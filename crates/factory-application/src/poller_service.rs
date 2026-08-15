use crate::workflows::comment_control::{CommentControlInput, CommentControlService};
use ed25519_dalek::SigningKey;
use factory_core::security::nhi::{AgentSubject, VerifiableCredential};
use factory_core::{PRCommentEvent, PolledIssueEvent};
use factory_infrastructure::git_poller::GitPlatformPoller;
use factory_infrastructure::kafka::KafkaClient;
use factory_infrastructure::semantica::SemanticaClient;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PollerCycleStats {
    pub issues_ingested: usize,
    pub directives_processed: usize,
    pub errors: Vec<String>,
}

pub struct PollerDaemonService {
    poller: Arc<GitPlatformPoller>,
    kafka_client: Arc<dyn KafkaClient>,
    semantica_client: Option<Arc<dyn SemanticaClient>>,
    comment_service: Arc<CommentControlService>,
    signing_key: SigningKey,
    key_id: String,
}

impl PollerDaemonService {
    pub fn new(
        poller: Arc<GitPlatformPoller>,
        kafka_client: Arc<dyn KafkaClient>,
        semantica_client: Option<Arc<dyn SemanticaClient>>,
        comment_service: Arc<CommentControlService>,
    ) -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);

        Self {
            poller,
            kafka_client,
            semantica_client,
            comment_service,
            signing_key,
            key_id: "did:key:dark-gravity-poller-nhi#1".to_string(),
        }
    }

    pub fn with_signing_key(mut self, signing_key: SigningKey, key_id: String) -> Self {
        self.signing_key = signing_key;
        self.key_id = key_id;
        self
    }

    /// Executes a single polling cycle across configured repositories.
    pub async fn poll_once(
        &self,
        github_repos: &[String],
        gitlab_projects: &[String],
    ) -> PollerCycleStats {
        let mut stats = PollerCycleStats::default();

        // 1. GitHub Issues
        for repo in github_repos {
            match self.poller.poll_github_issues(repo).await {
                Ok(issues) => {
                    for issue in issues {
                        if let Err(e) = self.ingest_issue(&issue).await {
                            stats
                                .errors
                                .push(format!("GitHub issue ingest error for {}: {}", repo, e));
                        } else {
                            stats.issues_ingested += 1;
                        }
                    }
                }
                Err(e) => stats
                    .errors
                    .push(format!("GitHub issue poll error for {}: {}", repo, e)),
            }

            // GitHub PR Comments
            match self.poller.poll_github_pr_comments(repo).await {
                Ok(comments) => {
                    for comment in comments {
                        if let Err(e) = self.process_comment_directive(&comment).await {
                            stats
                                .errors
                                .push(format!("GitHub comment error for {}: {}", repo, e));
                        } else {
                            stats.directives_processed += 1;
                        }
                    }
                }
                Err(e) => stats
                    .errors
                    .push(format!("GitHub comment poll error for {}: {}", repo, e)),
            }
        }

        // 2. GitLab Issues
        for project in gitlab_projects {
            match self.poller.poll_gitlab_issues(project).await {
                Ok(issues) => {
                    for issue in issues {
                        if let Err(e) = self.ingest_issue(&issue).await {
                            stats
                                .errors
                                .push(format!("GitLab issue ingest error for {}: {}", project, e));
                        } else {
                            stats.issues_ingested += 1;
                        }
                    }
                }
                Err(e) => stats
                    .errors
                    .push(format!("GitLab issue poll error for {}: {}", project, e)),
            }

            // GitLab MR Notes
            match self.poller.poll_gitlab_mr_notes(project).await {
                Ok(notes) => {
                    for note in notes {
                        if let Err(e) = self.process_comment_directive(&note).await {
                            stats
                                .errors
                                .push(format!("GitLab note error for {}: {}", project, e));
                        } else {
                            stats.directives_processed += 1;
                        }
                    }
                }
                Err(e) => stats
                    .errors
                    .push(format!("GitLab note poll error for {}: {}", project, e)),
            }
        }

        stats
    }

    async fn ingest_issue(&self, issue: &PolledIssueEvent) -> anyhow::Result<()> {
        // Create NHI Verifiable Credential signed with Ed25519
        let subject = AgentSubject {
            id: format!("nhi:issue:{}:{}", issue.source_platform, issue.issue_number),
            roles: vec!["mission_initiator".to_string()],
            allowed_namespaces: vec!["agents".to_string(), "production".to_string()],
        };

        let mut vc = VerifiableCredential::new(
            format!("vc:mission:{}:{}", issue.repository, issue.issue_number),
            "did:factory:dark-gravity-euskadi".to_string(),
            subject,
        );

        vc.sign(&self.signing_key, &self.key_id)
            .map_err(|e| anyhow::anyhow!("NHI Ed25519 signing failed: {}", e))?;

        let payload = serde_json::json!({
            "mission_id": format!("{}-{}", issue.repository.replace('/', "-"), issue.issue_number),
            "source_platform": issue.source_platform,
            "repository": issue.repository,
            "issue_id": issue.issue_id,
            "issue_number": issue.issue_number,
            "title": issue.title,
            "body": issue.body,
            "labels": issue.labels,
            "resource_limits": issue.resource_limits,
            "verifiable_credential": vc,
        });

        let payload_bytes = serde_json::to_vec(&payload)?;
        let msg_key = format!("{}:{}", issue.repository, issue.issue_number);
        self.kafka_client
            .publish("mission-input", &msg_key, &payload_bytes)
            .await?;

        // Semantica-AGI Causal Provenance Record
        if let Some(semantica) = &self.semantica_client {
            let record = factory_infrastructure::DecisionRecord {
                decision_id: format!(
                    "dec-{}-{}",
                    issue.repository.replace('/', "-"),
                    issue.issue_number
                ),
                agent_id: "poller-daemon".to_string(),
                mission_id: format!(
                    "{}-{}",
                    issue.repository.replace('/', "-"),
                    issue.issue_number
                ),
                reasoning: format!(
                    "Mission ingested via native outbound polling from {} issue #{}",
                    issue.source_platform, issue.issue_number
                ),
                ast_node_ids: vec![],
                timestamp: chrono::Utc::now().to_rfc3339(),
            };
            let _ = semantica.record_decision(&record).await;
        }

        Ok(())
    }

    async fn process_comment_directive(&self, comment: &PRCommentEvent) -> anyhow::Result<()> {
        let input = CommentControlInput {
            event: comment.clone(),
        };
        self.comment_service.handle_directive(&input).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use factory_infrastructure::aethalgard::MockAethalgardClient;
    use factory_infrastructure::cursor_store::InMemoryCursorStore;
    use factory_infrastructure::github::{
        GithubComment, GithubIssue, GithubPullRequest, GithubUser, MockGithubClient,
    };
    use factory_infrastructure::kafka::SimpleMockKafkaClient;
    use factory_infrastructure::mcp_client::MockMcpClient;
    use factory_infrastructure::r2r::MockR2rClient;

    #[tokio::test]
    async fn test_poller_daemon_full_cycle() {
        let mut mock_gh = MockGithubClient::new();

        mock_gh
            .expect_list_issues_updated_since()
            .returning(|_repo, _lbl, _since| {
                Ok(vec![GithubIssue {
                    id: 777,
                    number: 88,
                    title: "Deploy Zero-Trust Service".to_string(),
                    body: Some("Description here [RESOURCE_LIMIT: RAM <= 30Mi]".to_string()),
                    html_url: "https://github.com/my-org/my-repo/issues/88".to_string(),
                    updated_at: Some(Utc::now()),
                }])
            });

        mock_gh
            .expect_list_active_pull_requests()
            .returning(|_repo| {
                Ok(vec![GithubPullRequest {
                    id: 111,
                    number: 22,
                    title: "feat: mission".to_string(),
                    body: Some("body".to_string()),
                    html_url: "https://github.com/my-org/my-repo/pull/22".to_string(),
                    state: "open".to_string(),
                    updated_at: Some(Utc::now()),
                }])
            });

        mock_gh
            .expect_list_pull_request_comments()
            .returning(|_repo, _pr, _since| {
                Ok(vec![GithubComment {
                    id: 333,
                    body: "@dark-gravity /status".to_string(),
                    user: GithubUser {
                        login: "lead-architect".to_string(),
                    },
                    html_url: "https://github.com/my-org/my-repo/pull/22#issuecomment-333"
                        .to_string(),
                    updated_at: Some(Utc::now()),
                }])
            });

        mock_gh
            .expect_post_pull_request_comment()
            .returning(|_repo, _pr, body| {
                Ok(GithubComment {
                    id: 444,
                    body: body.to_string(),
                    user: GithubUser {
                        login: "dark-gravity-bot".to_string(),
                    },
                    html_url: "https://github.com/my-org/my-repo/pull/22#issuecomment-444"
                        .to_string(),
                    updated_at: Some(Utc::now()),
                })
            });

        let gh_arc = Arc::new(mock_gh);
        let cursor_store = Arc::new(InMemoryCursorStore::new());
        let poller = Arc::new(
            GitPlatformPoller::new(Some(gh_arc.clone()), None, cursor_store)
                .with_labels(vec!["autonomous-mission".to_string()]),
        );

        let kafka = Arc::new(SimpleMockKafkaClient::new("mock").unwrap());
        let mcp = Arc::new(MockMcpClient::new());
        let r2r = Arc::new(MockR2rClient::new());
        let aeth = Arc::new(MockAethalgardClient::new());

        let comment_service = Arc::new(CommentControlService::new(
            Some(gh_arc),
            None,
            mcp,
            r2r,
            aeth,
        ));

        let daemon = PollerDaemonService::new(poller, kafka, None, comment_service);

        let stats = daemon.poll_once(&["my-org/my-repo".to_string()], &[]).await;
        assert_eq!(stats.issues_ingested, 1);
        assert_eq!(stats.directives_processed, 1);
        assert!(stats.errors.is_empty());
    }
}
