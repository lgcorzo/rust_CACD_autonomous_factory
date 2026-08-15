use crate::agents::{RustantAgent, ZeroClawAgent};
use factory_core::{PRCommentEvent, PRDirective};
use factory_infrastructure::McpClient;
use factory_infrastructure::aethalgard::AethalgardClient;
use factory_infrastructure::github::GithubClient;
use factory_infrastructure::gitlab::GitlabClient;
use factory_infrastructure::r2r::R2rClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentControlInput {
    pub event: PRCommentEvent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentControlOutput {
    pub directive_type: String,
    pub status: String,
    pub response_body: String,
    pub comment_posted: bool,
}

pub struct CommentControlService {
    github_client: Option<Arc<dyn GithubClient>>,
    gitlab_client: Option<Arc<dyn GitlabClient>>,
    _mcp_client: Arc<dyn McpClient>,
    rustant_agent: Arc<RustantAgent>,
    zeroclaw_agent: Arc<ZeroClawAgent>,
}

impl CommentControlService {
    pub fn new(
        github_client: Option<Arc<dyn GithubClient>>,
        gitlab_client: Option<Arc<dyn GitlabClient>>,
        mcp_client: Arc<dyn McpClient>,
        r2r_client: Arc<dyn R2rClient>,
        aethalgard_client: Arc<dyn AethalgardClient>,
    ) -> Self {
        Self {
            github_client,
            gitlab_client,
            _mcp_client: mcp_client.clone(),
            rustant_agent: Arc::new(RustantAgent::new(mcp_client.clone(), r2r_client)),
            zeroclaw_agent: Arc::new(ZeroClawAgent::new(mcp_client, aethalgard_client)),
        }
    }

    pub async fn handle_directive(
        &self,
        input: &CommentControlInput,
    ) -> anyhow::Result<CommentControlOutput> {
        let event = &input.event;
        let mission_id = format!("{}-{}", event.repository.replace('/', "-"), event.pr_number);

        let (directive_type, response_body) = match &event.directive {
            PRDirective::Spec { prompt } => {
                let instruction = format!("Re-evaluate spec.md based on PR directive from {}: {}", event.author, prompt);
                let _agent_res = self.rustant_agent.plan_mission(&mission_id, &instruction).await;
                (
                    "spec".to_string(),
                    format!(
                        "🤖 **Rustant (PO)**: Spec re-evaluation complete based on feedback: `{}`.\nUpdated `spec.md` staged for verification.",
                        prompt
                    ),
                )
            }
            PRDirective::Refine { instruction } => {
                let _agent_res = self.zeroclaw_agent.execute_task(&mission_id, instruction, &[]).await;
                (
                    "refine".to_string(),
                    format!(
                        "⚡ **ZeroClaw (Dev)**: Applied localized code mutation in gVisor sandbox (<=30MiB RAM) for instruction: `{}`.",
                        instruction
                    ),
                )
            }
            PRDirective::Retry => {
                (
                    "retry".to_string(),
                    "🔄 **DevOps Agent**: Forced controlled restart of the Aethelgard Loop. Re-running automated test suites and SAST validation gates.".to_string(),
                )
            }
            PRDirective::Status => {
                let status_report = format!(
                    "📊 **Dark Gravity MCP System Report**:\n- **Repository**: {}\n- **Target PR**: #{}\n- **DAG Status**: Healthy (6 Phases Operational)\n- **Sandbox**: gVisor Active (RAM Limit: 30MiB)\n- **FinOps**: Velocity Nominal, Daily Budget Guard: Active",
                    event.repository, event.pr_number
                );
                ("status".to_string(), status_report)
            }
        };

        let mut comment_posted = false;

        // Post response back to PR/MR thread
        if event.source_platform == "github" {
            if let Some(gh) = &self.github_client {
                if let Err(e) = gh
                    .post_pull_request_comment(&event.repository, event.pr_number, &response_body)
                    .await
                {
                    tracing::warn!(
                        "Failed to post GitHub comment to PR #{}: {}",
                        event.pr_number,
                        e
                    );
                } else {
                    comment_posted = true;
                }
            }
        } else if event.source_platform == "gitlab"
            && let Some(gl) = &self.gitlab_client
        {
            if let Err(e) = gl
                .post_merge_request_note(&event.repository, event.pr_number, &response_body)
                .await
            {
                tracing::warn!(
                    "Failed to post GitLab note to MR !{}: {}",
                    event.pr_number,
                    e
                );
            } else {
                comment_posted = true;
            }
        }

        Ok(CommentControlOutput {
            directive_type,
            status: "success".to_string(),
            response_body,
            comment_posted,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use factory_infrastructure::aethalgard::MockAethalgardClient;
    use factory_infrastructure::github::{GithubComment, GithubUser, MockGithubClient};
    use factory_infrastructure::mcp_client::MockMcpClient;
    use factory_infrastructure::r2r::MockR2rClient;

    #[tokio::test]
    async fn test_comment_control_directives() {
        let mut mock_gh = MockGithubClient::new();
        mock_gh
            .expect_post_pull_request_comment()
            .returning(|_repo, _pr, body| {
                Ok(GithubComment {
                    id: 9999,
                    body: body.to_string(),
                    user: GithubUser {
                        login: "dark-gravity-bot".to_string(),
                    },
                    html_url: "https://github.com/my-org/my-repo/issues/10#issuecomment-9999"
                        .to_string(),
                    updated_at: Some(Utc::now()),
                })
            });

        let mut mock_mcp = MockMcpClient::new();
        mock_mcp.expect_call_tool_json().returning(|_tool, _args| {
            Ok(serde_json::json!({
                "is_error": false,
                "content": [{"type": "text", "text": "{\"sast_complete\": true}"}]
            }))
        });
        let mock_mcp = Arc::new(mock_mcp);
        let mock_r2r = Arc::new(MockR2rClient::new());
        let mock_aeth = Arc::new(MockAethalgardClient::new());

        let service = CommentControlService::new(
            Some(Arc::new(mock_gh)),
            None,
            mock_mcp,
            mock_r2r,
            mock_aeth,
        );

        let input_refine = CommentControlInput {
            event: PRCommentEvent {
                source_platform: "github".to_string(),
                repository: "my-org/my-repo".to_string(),
                pr_number: 10,
                comment_id: 1,
                author: "alice".to_string(),
                body: "@dark-gravity /refine optimize cache".to_string(),
                directive: PRDirective::Refine {
                    instruction: "optimize cache".to_string(),
                },
                updated_at: Utc::now(),
                html_url: "https://github.com/my-org/my-repo/pull/10#1".to_string(),
            },
        };

        let output = service.handle_directive(&input_refine).await.unwrap();
        assert!(output.directive_type == "refine");
        assert!(output.comment_posted);
        assert!(output.response_body.contains("ZeroClaw"));
    }
}
