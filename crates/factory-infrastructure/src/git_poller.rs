use crate::cursor_store::CursorStore;
use crate::github::GithubClient;
use crate::gitlab::GitlabClient;
use chrono::Utc;
use factory_core::{PRCommentEvent, PRDirective, PolledIssueEvent, PollerSyncCursor};
use std::sync::Arc;

pub struct GitPlatformPoller {
    github_client: Option<Arc<dyn GithubClient>>,
    gitlab_client: Option<Arc<dyn GitlabClient>>,
    cursor_store: Arc<dyn CursorStore>,
    required_issue_labels: Vec<String>,
}

impl GitPlatformPoller {
    pub fn new(
        github_client: Option<Arc<dyn GithubClient>>,
        gitlab_client: Option<Arc<dyn GitlabClient>>,
        cursor_store: Arc<dyn CursorStore>,
    ) -> Self {
        Self {
            github_client,
            gitlab_client,
            cursor_store,
            required_issue_labels: vec![
                "autonomous-mission".to_string(),
                "dark-gravity".to_string(),
            ],
        }
    }

    pub fn with_labels(mut self, labels: Vec<String>) -> Self {
        self.required_issue_labels = labels;
        self
    }

    /// Polls GitHub repository for new/updated issues with control labels.
    pub async fn poll_github_issues(&self, repo: &str) -> anyhow::Result<Vec<PolledIssueEvent>> {
        let client = match &self.github_client {
            Some(c) => c,
            None => return Ok(vec![]),
        };

        let cursor_key = format!("github:{}:issues", repo);
        let cursor = self.cursor_store.get_cursor(&cursor_key).await?;
        let since = cursor.as_ref().map(|c| c.last_polled_at);

        let mut all_events = Vec::new();

        for label in &self.required_issue_labels {
            let issues = client
                .list_issues_updated_since(repo, Some(label.clone()), since)
                .await?;

            for issue in issues {
                let event_hash = format!("issue:{}:{}:{}", repo, issue.id, issue.number);
                if self
                    .cursor_store
                    .is_event_processed(&cursor_key, &event_hash)
                    .await?
                {
                    continue;
                }

                let body = issue.body.unwrap_or_default();
                let resource_limits = PolledIssueEvent::extract_resource_limits(&body);
                let updated_at = issue.updated_at.unwrap_or_else(Utc::now);

                let event = PolledIssueEvent {
                    source_platform: "github".to_string(),
                    repository: repo.to_string(),
                    issue_id: issue.id,
                    issue_number: issue.number,
                    title: issue.title,
                    body,
                    labels: vec![label.clone()],
                    resource_limits,
                    updated_at,
                    html_url: issue.html_url,
                };

                self.cursor_store
                    .mark_event_processed(&cursor_key, &event_hash)
                    .await?;
                all_events.push(event);
            }
        }

        let updated_cursor = PollerSyncCursor {
            source_key: cursor_key.clone(),
            last_polled_at: Utc::now(),
            last_processed_id: all_events.last().map(|e| e.issue_id).unwrap_or(0),
            processed_hashes: vec![],
        };
        self.cursor_store.save_cursor(&updated_cursor).await?;

        Ok(all_events)
    }

    /// Polls GitHub repository active PRs for directive comments.
    pub async fn poll_github_pr_comments(&self, repo: &str) -> anyhow::Result<Vec<PRCommentEvent>> {
        let client = match &self.github_client {
            Some(c) => c,
            None => return Ok(vec![]),
        };

        let cursor_key = format!("github:{}:pr_comments", repo);
        let cursor = self.cursor_store.get_cursor(&cursor_key).await?;
        let since = cursor.as_ref().map(|c| c.last_polled_at);

        let active_prs = client.list_active_pull_requests(repo).await?;
        let mut comment_events = Vec::new();

        for pr in active_prs {
            let comments = client
                .list_pull_request_comments(repo, pr.number, since)
                .await?;
            for comment in comments {
                let event_hash = format!("comment:{}:{}:{}", repo, pr.number, comment.id);
                if self
                    .cursor_store
                    .is_event_processed(&cursor_key, &event_hash)
                    .await?
                {
                    continue;
                }

                if let Some(directive) = PRDirective::parse(&comment.body) {
                    let updated_at = comment.updated_at.unwrap_or_else(Utc::now);
                    let event = PRCommentEvent {
                        source_platform: "github".to_string(),
                        repository: repo.to_string(),
                        pr_number: pr.number,
                        comment_id: comment.id,
                        author: comment.user.login,
                        body: comment.body,
                        directive,
                        updated_at,
                        html_url: comment.html_url,
                    };

                    self.cursor_store
                        .mark_event_processed(&cursor_key, &event_hash)
                        .await?;
                    comment_events.push(event);
                }
            }
        }

        let updated_cursor = PollerSyncCursor {
            source_key: cursor_key.clone(),
            last_polled_at: Utc::now(),
            last_processed_id: comment_events.last().map(|e| e.comment_id).unwrap_or(0),
            processed_hashes: vec![],
        };
        self.cursor_store.save_cursor(&updated_cursor).await?;

        Ok(comment_events)
    }

    /// Polls GitLab repository for new/updated issues.
    pub async fn poll_gitlab_issues(
        &self,
        project_id: &str,
    ) -> anyhow::Result<Vec<PolledIssueEvent>> {
        let client = match &self.gitlab_client {
            Some(c) => c,
            None => return Ok(vec![]),
        };

        let cursor_key = format!("gitlab:{}:issues", project_id);
        let cursor = self.cursor_store.get_cursor(&cursor_key).await?;
        let since = cursor.as_ref().map(|c| c.last_polled_at);

        let mut all_events = Vec::new();

        for label in &self.required_issue_labels {
            let issues = client
                .list_issues_updated_since(project_id, Some(label.clone()), since)
                .await?;

            for issue in issues {
                let event_hash = format!("issue:{}:{}:{}", project_id, issue.id, issue.iid);
                if self
                    .cursor_store
                    .is_event_processed(&cursor_key, &event_hash)
                    .await?
                {
                    continue;
                }

                let body = issue.description.unwrap_or_default();
                let resource_limits = PolledIssueEvent::extract_resource_limits(&body);
                let updated_at = issue.updated_at.unwrap_or_else(Utc::now);

                let event = PolledIssueEvent {
                    source_platform: "gitlab".to_string(),
                    repository: project_id.to_string(),
                    issue_id: issue.id,
                    issue_number: issue.iid,
                    title: issue.title,
                    body,
                    labels: vec![label.clone()],
                    resource_limits,
                    updated_at,
                    html_url: issue.web_url,
                };

                self.cursor_store
                    .mark_event_processed(&cursor_key, &event_hash)
                    .await?;
                all_events.push(event);
            }
        }

        let updated_cursor = PollerSyncCursor {
            source_key: cursor_key.clone(),
            last_polled_at: Utc::now(),
            last_processed_id: all_events.last().map(|e| e.issue_id).unwrap_or(0),
            processed_hashes: vec![],
        };
        self.cursor_store.save_cursor(&updated_cursor).await?;

        Ok(all_events)
    }

    /// Polls GitLab merge requests for comments/notes with directives.
    pub async fn poll_gitlab_mr_notes(
        &self,
        project_id: &str,
    ) -> anyhow::Result<Vec<PRCommentEvent>> {
        let client = match &self.gitlab_client {
            Some(c) => c,
            None => return Ok(vec![]),
        };

        let cursor_key = format!("gitlab:{}:mr_notes", project_id);
        let cursor = self.cursor_store.get_cursor(&cursor_key).await?;
        let since = cursor.as_ref().map(|c| c.last_polled_at);

        let mrs = client.list_active_merge_requests(project_id).await?;
        let mut note_events = Vec::new();

        for mr in mrs {
            let notes = client
                .list_merge_request_notes(project_id, mr.iid, since)
                .await?;
            for note in notes {
                let event_hash = format!("note:{}:{}:{}", project_id, mr.iid, note.id);
                if self
                    .cursor_store
                    .is_event_processed(&cursor_key, &event_hash)
                    .await?
                {
                    continue;
                }

                if let Some(directive) = PRDirective::parse(&note.body) {
                    let updated_at = note.updated_at.unwrap_or_else(Utc::now);
                    let event = PRCommentEvent {
                        source_platform: "gitlab".to_string(),
                        repository: project_id.to_string(),
                        pr_number: mr.iid,
                        comment_id: note.id,
                        author: note.author.username,
                        body: note.body,
                        directive,
                        updated_at,
                        html_url: format!("{}/#note_{}", mr.web_url, note.id),
                    };

                    self.cursor_store
                        .mark_event_processed(&cursor_key, &event_hash)
                        .await?;
                    note_events.push(event);
                }
            }
        }

        let updated_cursor = PollerSyncCursor {
            source_key: cursor_key.clone(),
            last_polled_at: Utc::now(),
            last_processed_id: note_events.last().map(|e| e.comment_id).unwrap_or(0),
            processed_hashes: vec![],
        };
        self.cursor_store.save_cursor(&updated_cursor).await?;

        Ok(note_events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cursor_store::InMemoryCursorStore;
    use crate::github::{
        GithubComment, GithubIssue, GithubPullRequest, GithubUser, MockGithubClient,
    };
    use crate::gitlab::{
        GitlabAuthor, GitlabIssue, GitlabMergeRequest, GitlabNote, MockGitlabClient,
    };

    #[tokio::test]
    async fn test_github_poller_issue_and_directive_flow() {
        let mut mock_gh = MockGithubClient::new();

        mock_gh
            .expect_list_issues_updated_since()
            .returning(|_repo, _label, _since| {
                Ok(vec![GithubIssue {
                    id: 1001,
                    number: 55,
                    title: "Implement zero-trust polling".to_string(),
                    body: Some(
                        "Build native Rust poller [RESOURCE_LIMIT: RAM <= 30Mi]".to_string(),
                    ),
                    html_url: "https://github.com/my-org/my-repo/issues/55".to_string(),
                    updated_at: Some(Utc::now()),
                }])
            });

        mock_gh
            .expect_list_active_pull_requests()
            .returning(|_repo| {
                Ok(vec![GithubPullRequest {
                    id: 2001,
                    number: 10,
                    title: "feat: add poller".to_string(),
                    body: Some("PR body".to_string()),
                    html_url: "https://github.com/my-org/my-repo/pull/10".to_string(),
                    state: "open".to_string(),
                    updated_at: Some(Utc::now()),
                }])
            });

        mock_gh
            .expect_list_pull_request_comments()
            .returning(|_repo, _pr, _since| {
                Ok(vec![
                    GithubComment {
                        id: 3001,
                        body: "@dark-gravity /refine add exponential backoff".to_string(),
                        user: GithubUser {
                            login: "senior-dev".to_string(),
                        },
                        html_url: "https://github.com/my-org/my-repo/pull/10#comment-3001"
                            .to_string(),
                        updated_at: Some(Utc::now()),
                    },
                    GithubComment {
                        id: 3002,
                        body: "LGTM! Non directive comment.".to_string(),
                        user: GithubUser {
                            login: "reviewer".to_string(),
                        },
                        html_url: "https://github.com/my-org/my-repo/pull/10#comment-3002"
                            .to_string(),
                        updated_at: Some(Utc::now()),
                    },
                ])
            });

        let cursor_store = Arc::new(InMemoryCursorStore::new());
        let poller = GitPlatformPoller::new(Some(Arc::new(mock_gh)), None, cursor_store.clone())
            .with_labels(vec!["autonomous-mission".to_string()]);

        // First poll: extracts the issue
        let issues = poller.poll_github_issues("my-org/my-repo").await.unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].issue_number, 55);
        assert_eq!(issues[0].resource_limits, Some("RAM <= 30Mi".to_string()));

        // Second poll: deduplicated, yields 0 new issues
        let issues_second = poller.poll_github_issues("my-org/my-repo").await.unwrap();
        assert_eq!(issues_second.len(), 0);

        // Poll comments: extracts only directive comment
        let comments = poller
            .poll_github_pr_comments("my-org/my-repo")
            .await
            .unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].author, "senior-dev");
        assert_eq!(
            comments[0].directive,
            PRDirective::Refine {
                instruction: "add exponential backoff".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_gitlab_poller_issue_and_directive_flow() {
        let mut mock_gl = MockGitlabClient::new();

        mock_gl
            .expect_list_issues_updated_since()
            .returning(|_project, _label, _since| {
                Ok(vec![GitlabIssue {
                    id: 9901,
                    iid: 77,
                    title: "Security fix issue".to_string(),
                    description: Some("Implement fix [RESOURCE_LIMIT: RAM <= 30Mi]".to_string()),
                    web_url: "https://gitlab.com/my-org/my-proj/-/issues/77".to_string(),
                    updated_at: Some(Utc::now()),
                }])
            });

        mock_gl
            .expect_list_active_merge_requests()
            .returning(|_project| {
                Ok(vec![GitlabMergeRequest {
                    id: 8801,
                    iid: 15,
                    title: "MR 15".to_string(),
                    description: Some("MR desc".to_string()),
                    web_url: "https://gitlab.com/my-org/my-proj/-/merge_requests/15".to_string(),
                    state: "opened".to_string(),
                    updated_at: Some(Utc::now()),
                }])
            });

        mock_gl
            .expect_list_merge_request_notes()
            .returning(|_project, _mr, _since| {
                Ok(vec![GitlabNote {
                    id: 4401,
                    body: "@dark-gravity /retry".to_string(),
                    author: GitlabAuthor {
                        username: "devops-engineer".to_string(),
                    },
                    updated_at: Some(Utc::now()),
                }])
            });

        let cursor_store = Arc::new(InMemoryCursorStore::new());
        let poller = GitPlatformPoller::new(None, Some(Arc::new(mock_gl)), cursor_store.clone())
            .with_labels(vec!["dark-gravity".to_string()]);

        let issues = poller.poll_gitlab_issues("my-org/my-proj").await.unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].issue_number, 77);

        let notes = poller.poll_gitlab_mr_notes("my-org/my-proj").await.unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].author, "devops-engineer");
        assert_eq!(notes[0].directive, PRDirective::Retry);
    }
}
