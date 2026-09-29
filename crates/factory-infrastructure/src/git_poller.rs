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
    bot_username: Option<String>,
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
            bot_username: None,
        }
    }

    pub fn with_labels(mut self, labels: Vec<String>) -> Self {
        self.required_issue_labels = labels;
        self
    }

    pub fn with_bot_username(mut self, bot_username: impl Into<String>) -> Self {
        self.bot_username = Some(bot_username.into());
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
            for comment in &comments {
                if let Some(bot) = &self.bot_username {
                    if comment.user.login.eq_ignore_ascii_case(bot) {
                        continue;
                    }
                }

                let event_hash = format!("comment:{}:{}:{}", repo, pr.number, comment.id);
                if self
                    .cursor_store
                    .is_event_processed(&cursor_key, &event_hash)
                    .await?
                {
                    continue;
                }

                if let Some(directive) = PRDirective::parse(&comment.body) {
                    if let Err(e) = client.add_comment_reaction(repo, comment.id, "eyes").await {
                        tracing::warn!(
                            "Failed to add eyes reaction to GitHub comment {}: {}",
                            comment.id,
                            e
                        );
                    }

                    let updated_at = comment.updated_at.unwrap_or_else(Utc::now);
                    let thread_context: Vec<String> = comments
                        .iter()
                        .take_while(|c| c.id != comment.id)
                        .map(|c| format!("{}: {}", c.user.login, c.body))
                        .collect();

                    let event = PRCommentEvent {
                        source_platform: "github".to_string(),
                        repository: repo.to_string(),
                        pr_number: pr.number,
                        comment_id: comment.id,
                        author: comment.user.login.clone(),
                        body: comment.body.clone(),
                        directive,
                        updated_at,
                        html_url: comment.html_url.clone(),
                        thread_context,
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
            for note in &notes {
                if let Some(bot) = &self.bot_username {
                    if note.author.username.eq_ignore_ascii_case(bot) {
                        continue;
                    }
                }

                let event_hash = format!("note:{}:{}:{}", project_id, mr.iid, note.id);
                if self
                    .cursor_store
                    .is_event_processed(&cursor_key, &event_hash)
                    .await?
                {
                    continue;
                }

                if let Some(directive) = PRDirective::parse(&note.body) {
                    if let Err(e) = client
                        .add_merge_request_note_award_emoji(project_id, mr.iid, note.id, "eyes")
                        .await
                    {
                        tracing::warn!(
                            "Failed to add eyes reaction to GitLab MR note {}: {}",
                            note.id,
                            e
                        );
                    }

                    let updated_at = note.updated_at.unwrap_or_else(Utc::now);
                    let thread_context: Vec<String> = notes
                        .iter()
                        .take_while(|n| n.id != note.id)
                        .map(|n| format!("{}: {}", n.author.username, n.body))
                        .collect();

                    let event = PRCommentEvent {
                        source_platform: "gitlab".to_string(),
                        repository: project_id.to_string(),
                        pr_number: mr.iid,
                        comment_id: note.id,
                        author: note.author.username.clone(),
                        body: note.body.clone(),
                        directive,
                        updated_at,
                        html_url: format!("{}/#note_{}", mr.web_url, note.id),
                        thread_context,
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

    // ── Pipeline Error Remediation Polling (T027–T028) ──

    /// Poll GitHub Actions for newly failed workflow runs in a repository.
    ///
    /// Uses the cursor key `github:{repo}:pipelines` for idempotent tracking.
    /// Fetches the failing jobs and their logs to build [`PipelineFailureEvent`]s.
    pub async fn poll_github_pipeline_runs(
        &self,
        repo: &str,
    ) -> anyhow::Result<Vec<factory_core::PipelineFailureEvent>> {
        use factory_core::{PipelineFailureEvent, PipelineScopeFilter, PipelineScopeVerdict};

        let client = match &self.github_client {
            Some(c) => c,
            None => return Ok(vec![]),
        };

        // Fetch active pull requests first
        let active_prs = client
            .list_active_pull_requests(repo)
            .await
            .unwrap_or_default();
        if active_prs.is_empty() {
            tracing::debug!(
                repo,
                "No active PRs found; skipping GitHub pipeline failure polling"
            );
            return Ok(vec![]);
        }

        let pr_tuples: Vec<(u64, String, Option<String>)> = active_prs
            .iter()
            .map(|pr| {
                let branch = pr
                    .head
                    .as_ref()
                    .and_then(|h| h.branch_ref.clone())
                    .unwrap_or_default();
                let sha = pr.head.as_ref().and_then(|h| h.sha.clone());
                (pr.number, branch, sha)
            })
            .collect();

        let cursor_key = format!("github:{}:pipelines", repo);
        let cursor = self.cursor_store.get_cursor(&cursor_key).await?;
        let since = cursor.as_ref().map(|c| c.last_polled_at);

        let runs = client.list_failed_workflow_runs(repo, since).await?;
        let mut events = Vec::new();
        let mut last_id = cursor.as_ref().map(|c| c.last_processed_id).unwrap_or(0);

        for run in runs {
            let event_hash = format!("pipeline:{}:{}", repo, run.id);
            if self
                .cursor_store
                .is_event_processed(&cursor_key, &event_hash)
                .await?
            {
                continue;
            }

            // Scope filter: Check whether run is associated with an active PR
            let run_pr_numbers: Vec<u64> = run.pull_requests.iter().map(|pr| pr.number).collect();
            let verdict = PipelineScopeFilter::evaluate_github(
                run.event.as_deref(),
                run.head_branch.as_deref(),
                run.head_sha.as_deref(),
                &run_pr_numbers,
                &pr_tuples,
            );

            let (pr_number, head_branch, head_sha) = match verdict {
                PipelineScopeVerdict::Included {
                    pr_number,
                    head_branch,
                    head_sha,
                } => (Some(pr_number), Some(head_branch), head_sha),
                verdict => {
                    tracing::debug!(
                        repo,
                        run_id = run.id,
                        ?verdict,
                        "Skipping non-PR GitHub workflow run and marking as processed"
                    );
                    self.cursor_store
                        .mark_event_processed(&cursor_key, &event_hash)
                        .await?;
                    last_id = run.id.max(last_id);
                    continue;
                }
            };

            // Fetch failing jobs for this run
            let jobs = client.get_workflow_run_jobs(repo, run.id).await?;
            let failing_job = jobs
                .iter()
                .find(|j| j.conclusion.as_deref() == Some("failure"))
                .or_else(|| jobs.first());

            let (job_name, failing_step, job_id) = if let Some(job) = failing_job {
                let step_name = job
                    .steps
                    .iter()
                    .find(|s| s.conclusion.as_deref() == Some("failure"))
                    .map(|s| s.name.clone());
                (job.name.clone(), step_name, Some(job.id))
            } else {
                ("unknown".to_string(), None, None)
            };

            // Fetch log (best-effort)
            let error_log = if let Some(jid) = job_id {
                client.get_job_log(repo, jid).await.unwrap_or_default()
            } else {
                String::new()
            };

            let event = PipelineFailureEvent {
                source_platform: "github".to_string(),
                repository: repo.to_string(),
                run_id: run.id,
                workflow_name: run.name.unwrap_or_else(|| "unknown".to_string()),
                failing_job: job_name,
                failing_step,
                error_log,
                run_url: run.html_url,
                detected_at: Utc::now(),
                pr_number,
                head_branch,
                head_sha,
            };

            self.cursor_store
                .mark_event_processed(&cursor_key, &event_hash)
                .await?;
            last_id = run.id.max(last_id);
            events.push(event);
        }

        let updated_cursor = factory_core::PollerSyncCursor {
            source_key: cursor_key.clone(),
            last_polled_at: Utc::now(),
            last_processed_id: last_id,
            processed_hashes: vec![],
        };
        self.cursor_store.save_cursor(&updated_cursor).await?;

        Ok(events)
    }

    /// Poll GitLab CI for newly failed pipeline runs in a project.
    ///
    /// Uses the cursor key `gitlab:{project}:pipelines` for idempotent tracking.
    pub async fn poll_gitlab_pipeline_runs(
        &self,
        project: &str,
    ) -> anyhow::Result<Vec<factory_core::PipelineFailureEvent>> {
        use factory_core::{PipelineFailureEvent, PipelineScopeFilter, PipelineScopeVerdict};

        let client = match &self.gitlab_client {
            Some(c) => c,
            None => return Ok(vec![]),
        };

        // Fetch active merge requests first
        let active_mrs = client
            .list_active_merge_requests(project)
            .await
            .unwrap_or_default();
        if active_mrs.is_empty() {
            tracing::debug!(
                project,
                "No active MRs found; skipping GitLab pipeline failure polling"
            );
            return Ok(vec![]);
        }

        let mr_tuples: Vec<(u64, String, Option<String>)> = active_mrs
            .iter()
            .map(|mr| {
                (
                    mr.iid,
                    mr.source_branch.clone().unwrap_or_default(),
                    mr.sha.clone(),
                )
            })
            .collect();

        let cursor_key = format!("gitlab:{}:pipelines", project);
        let cursor = self.cursor_store.get_cursor(&cursor_key).await?;
        let since = cursor.as_ref().map(|c| c.last_polled_at);

        let pipelines = client.list_failed_pipelines(project, since).await?;
        let mut events = Vec::new();
        let mut last_id = cursor.as_ref().map(|c| c.last_processed_id).unwrap_or(0);

        for pipeline in pipelines {
            let event_hash = format!("pipeline:{}:{}", project, pipeline.id);
            if self
                .cursor_store
                .is_event_processed(&cursor_key, &event_hash)
                .await?
            {
                continue;
            }

            // Scope filter: Check whether pipeline is associated with an active MR
            let verdict = PipelineScopeFilter::evaluate_gitlab(
                pipeline.ref_.as_deref(),
                pipeline.sha.as_deref(),
                pipeline.source.as_deref(),
                &mr_tuples,
            );

            let (pr_number, head_branch, head_sha) = match verdict {
                PipelineScopeVerdict::Included {
                    pr_number,
                    head_branch,
                    head_sha,
                } => (Some(pr_number), Some(head_branch), head_sha),
                verdict => {
                    tracing::debug!(
                        project,
                        pipeline_id = pipeline.id,
                        ?verdict,
                        "Skipping non-MR GitLab pipeline and marking as processed"
                    );
                    self.cursor_store
                        .mark_event_processed(&cursor_key, &event_hash)
                        .await?;
                    last_id = pipeline.id.max(last_id);
                    continue;
                }
            };

            // Fetch failing jobs for this pipeline
            let jobs = client.get_pipeline_jobs(project, pipeline.id).await?;
            let failing_job = jobs
                .iter()
                .find(|j| j.status == "failed")
                .or_else(|| jobs.first());

            let (job_name, job_id) = if let Some(job) = failing_job {
                (job.name.clone(), Some(job.id))
            } else {
                ("unknown".to_string(), None)
            };

            // Fetch log trace (best-effort)
            let error_log = if let Some(jid) = job_id {
                client.get_job_trace(project, jid).await.unwrap_or_default()
            } else {
                String::new()
            };

            let event = PipelineFailureEvent {
                source_platform: "gitlab".to_string(),
                repository: project.to_string(),
                run_id: pipeline.id,
                workflow_name: pipeline.ref_.unwrap_or_else(|| "unknown".to_string()),
                failing_job: job_name,
                failing_step: None, // GitLab doesn't have steps like GitHub
                error_log,
                run_url: pipeline.web_url,
                detected_at: Utc::now(),
                pr_number,
                head_branch,
                head_sha,
            };

            self.cursor_store
                .mark_event_processed(&cursor_key, &event_hash)
                .await?;
            last_id = pipeline.id.max(last_id);
            events.push(event);
        }

        let updated_cursor = factory_core::PollerSyncCursor {
            source_key: cursor_key.clone(),
            last_polled_at: Utc::now(),
            last_processed_id: last_id,
            processed_hashes: vec![],
        };
        self.cursor_store.save_cursor(&updated_cursor).await?;

        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cursor_store::InMemoryCursorStore;
    use crate::github::{
        GithubComment, GithubIssue, GithubPullRequest, GithubReaction, GithubUser, MockGithubClient,
    };
    use crate::gitlab::{
        GitlabAuthor, GitlabAwardEmoji, GitlabIssue, GitlabMergeRequest, GitlabNote,
        MockGitlabClient,
    };

    #[tokio::test]
    async fn test_github_poller_issue_and_directive_flow() {
        let mut mock_gh = MockGithubClient::new();

        mock_gh
            .expect_add_comment_reaction()
            .returning(|_repo, _comment_id, reaction| {
                Ok(GithubReaction {
                    id: 1,
                    content: reaction.to_string(),
                    user: None,
                })
            });

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
                    head: None,
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
            .expect_add_merge_request_note_award_emoji()
            .returning(|_project, _mr_iid, _note_id, name| {
                Ok(GitlabAwardEmoji {
                    id: 1,
                    name: name.to_string(),
                    user: GitlabAuthor {
                        username: "darkgravity-bot".to_string(),
                    },
                })
            });

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
                    source_branch: None,
                    sha: None,
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

    // ── Pipeline Polling Tests (T019–T020) ──

    #[tokio::test]
    async fn test_poll_github_pipeline_runs() {
        use crate::github::{
            GithubPullRequest, GithubWorkflowBranchRef, GithubWorkflowJob, GithubWorkflowRun,
            GithubWorkflowRunPr, GithubWorkflowStep,
        };

        let mut mock_gh = MockGithubClient::new();

        mock_gh
            .expect_list_active_pull_requests()
            .returning(|_repo| {
                Ok(vec![GithubPullRequest {
                    id: 101,
                    number: 404,
                    title: "PR 404".to_string(),
                    body: None,
                    html_url: "https://github.com/my-org/my-repo/pull/404".to_string(),
                    state: "open".to_string(),
                    updated_at: Some(Utc::now()),
                    head: Some(GithubWorkflowBranchRef {
                        branch_ref: Some("feat/pr-scope".to_string()),
                        sha: Some("abc1234".to_string()),
                    }),
                }])
            });

        mock_gh
            .expect_list_failed_workflow_runs()
            .returning(|_repo, _since| {
                Ok(vec![GithubWorkflowRun {
                    id: 9001,
                    name: Some("CI/CD Pipeline".to_string()),
                    status: "completed".to_string(),
                    conclusion: Some("failure".to_string()),
                    html_url: "https://github.com/my-org/my-repo/actions/runs/9001".to_string(),
                    updated_at: Some(Utc::now()),
                    pull_requests: vec![GithubWorkflowRunPr {
                        id: 101,
                        number: 404,
                        head: None,
                        base: None,
                    }],
                    head_branch: Some("feat/pr-scope".to_string()),
                    head_sha: Some("abc1234".to_string()),
                    event: Some("pull_request".to_string()),
                }])
            });

        mock_gh
            .expect_get_workflow_run_jobs()
            .returning(|_repo, _run_id| {
                Ok(vec![GithubWorkflowJob {
                    id: 42,
                    name: "Rust CI (Lint & Test)".to_string(),
                    conclusion: Some("failure".to_string()),
                    steps: vec![GithubWorkflowStep {
                        name: "Lint with Clippy".to_string(),
                        conclusion: Some("failure".to_string()),
                    }],
                }])
            });

        mock_gh.expect_get_job_log().returning(|_repo, _job_id| {
            Ok("error[E0308]: mismatched types\n  --> src/lib.rs:15:5".to_string())
        });

        let cursor_store = Arc::new(InMemoryCursorStore::new());
        let poller = GitPlatformPoller::new(Some(Arc::new(mock_gh)), None, cursor_store.clone());

        let events = poller
            .poll_github_pipeline_runs("my-org/my-repo")
            .await
            .unwrap();

        assert_eq!(events.len(), 1);
        let event = &events[0];
        assert_eq!(event.run_id, 9001);
        assert_eq!(event.source_platform, "github");
        assert_eq!(event.repository, "my-org/my-repo");
        assert_eq!(event.workflow_name, "CI/CD Pipeline");
        assert_eq!(event.failing_job, "Rust CI (Lint & Test)");
        assert_eq!(event.failing_step.as_deref(), Some("Lint with Clippy"));
        assert!(event.error_log.contains("error[E0308]"));
        assert_eq!(event.pr_number, Some(404));
        assert_eq!(event.head_branch.as_deref(), Some("feat/pr-scope"));
        assert_eq!(event.head_sha.as_deref(), Some("abc1234"));
    }

    #[tokio::test]
    async fn test_pipeline_cursor_idempotency() {
        use crate::github::{
            GithubPullRequest, GithubWorkflowBranchRef, GithubWorkflowJob, GithubWorkflowRun,
            GithubWorkflowRunPr,
        };

        let mut mock_gh = MockGithubClient::new();

        mock_gh
            .expect_list_active_pull_requests()
            .returning(|_repo| {
                Ok(vec![GithubPullRequest {
                    id: 101,
                    number: 404,
                    title: "PR 404".to_string(),
                    body: None,
                    html_url: "https://github.com/my-org/my-repo/pull/404".to_string(),
                    state: "open".to_string(),
                    updated_at: Some(Utc::now()),
                    head: Some(GithubWorkflowBranchRef {
                        branch_ref: Some("feat/pr-scope".to_string()),
                        sha: Some("abc1234".to_string()),
                    }),
                }])
            });

        // list_failed_workflow_runs called twice, returns same run both times
        mock_gh
            .expect_list_failed_workflow_runs()
            .times(2)
            .returning(|_repo, _since| {
                Ok(vec![GithubWorkflowRun {
                    id: 9001,
                    name: Some("CI/CD Pipeline".to_string()),
                    status: "completed".to_string(),
                    conclusion: Some("failure".to_string()),
                    html_url: "https://github.com/my-org/my-repo/actions/runs/9001".to_string(),
                    updated_at: Some(Utc::now()),
                    pull_requests: vec![GithubWorkflowRunPr {
                        id: 101,
                        number: 404,
                        head: None,
                        base: None,
                    }],
                    head_branch: Some("feat/pr-scope".to_string()),
                    head_sha: Some("abc1234".to_string()),
                    event: Some("pull_request".to_string()),
                }])
            });

        // get_workflow_run_jobs and get_job_log are called only once (first poll)
        mock_gh
            .expect_get_workflow_run_jobs()
            .times(1)
            .returning(|_repo, _run_id| {
                Ok(vec![GithubWorkflowJob {
                    id: 42,
                    name: "rust-test".to_string(),
                    conclusion: Some("failure".to_string()),
                    steps: vec![],
                }])
            });

        mock_gh
            .expect_get_job_log()
            .times(1)
            .returning(|_repo, _job_id| Ok("error log".to_string()));

        let cursor_store = Arc::new(InMemoryCursorStore::new());
        let poller = GitPlatformPoller::new(Some(Arc::new(mock_gh)), None, cursor_store.clone());

        // First poll: detects the failure
        let events_first = poller
            .poll_github_pipeline_runs("my-org/my-repo")
            .await
            .unwrap();
        assert_eq!(
            events_first.len(),
            1,
            "First poll should detect the failure"
        );

        // Second poll: same run_id is idempotently skipped
        let events_second = poller
            .poll_github_pipeline_runs("my-org/my-repo")
            .await
            .unwrap();
        assert_eq!(
            events_second.len(),
            0,
            "Second poll should not re-process the same run"
        );
    }

    #[tokio::test]
    async fn test_poll_github_pipeline_runs_early_exit_on_empty_prs() {
        let mut mock_gh = MockGithubClient::new();

        // 0 active PRs returned -> early exit without querying failed runs
        mock_gh
            .expect_list_active_pull_requests()
            .times(1)
            .returning(|_repo| Ok(vec![]));

        mock_gh.expect_list_failed_workflow_runs().never();

        let cursor_store = Arc::new(InMemoryCursorStore::new());
        let poller = GitPlatformPoller::new(Some(Arc::new(mock_gh)), None, cursor_store);

        let events = poller
            .poll_github_pipeline_runs("my-org/my-repo")
            .await
            .unwrap();
        assert!(
            events.is_empty(),
            "Must return empty when no active PRs exist"
        );
    }

    #[tokio::test]
    async fn test_poll_github_pipeline_runs_skips_non_pr_runs() {
        use crate::github::{GithubPullRequest, GithubWorkflowBranchRef, GithubWorkflowRun};

        let mut mock_gh = MockGithubClient::new();

        // Active PR on feature branch
        mock_gh
            .expect_list_active_pull_requests()
            .returning(|_repo| {
                Ok(vec![GithubPullRequest {
                    id: 101,
                    number: 404,
                    title: "PR 404".to_string(),
                    body: None,
                    html_url: "https://github.com/my-org/my-repo/pull/404".to_string(),
                    state: "open".to_string(),
                    updated_at: Some(Utc::now()),
                    head: Some(GithubWorkflowBranchRef {
                        branch_ref: Some("feat/pr-scope".to_string()),
                        sha: Some("abc1234".to_string()),
                    }),
                }])
            });

        // Run is on main (push event), NOT associated with PR 404
        mock_gh
            .expect_list_failed_workflow_runs()
            .returning(|_repo, _since| {
                Ok(vec![GithubWorkflowRun {
                    id: 9999,
                    name: Some("CI on main".to_string()),
                    status: "completed".to_string(),
                    conclusion: Some("failure".to_string()),
                    html_url: "https://github.com/my-org/my-repo/actions/runs/9999".to_string(),
                    updated_at: Some(Utc::now()),
                    pull_requests: vec![],
                    head_branch: Some("main".to_string()),
                    head_sha: Some("deadbeef".to_string()),
                    event: Some("push".to_string()),
                }])
            });

        // Job log and jobs should never be fetched for a skipped run
        mock_gh.expect_get_workflow_run_jobs().never();
        mock_gh.expect_get_job_log().never();

        let cursor_store = Arc::new(InMemoryCursorStore::new());
        let poller = GitPlatformPoller::new(Some(Arc::new(mock_gh)), None, cursor_store.clone());

        let events = poller
            .poll_github_pipeline_runs("my-org/my-repo")
            .await
            .unwrap();

        assert!(events.is_empty(), "Non-PR workflow runs must be skipped");

        // Verify skipped run was marked as processed in cursor store (T011)
        let is_processed = cursor_store
            .is_event_processed(
                "github:my-org/my-repo:pipelines",
                "pipeline:my-org/my-repo:9999",
            )
            .await
            .unwrap();
        assert!(
            is_processed,
            "Skipped run must be marked as processed to avoid re-evaluation"
        );
    }

    #[tokio::test]
    async fn test_poll_gitlab_pipeline_runs_early_exit_on_empty_mrs() {
        let mut mock_gl = MockGitlabClient::new();

        // 0 active MRs returned -> early exit without querying failed pipelines
        mock_gl
            .expect_list_active_merge_requests()
            .times(1)
            .returning(|_project| Ok(vec![]));

        mock_gl.expect_list_failed_pipelines().never();

        let cursor_store = Arc::new(InMemoryCursorStore::new());
        let poller = GitPlatformPoller::new(None, Some(Arc::new(mock_gl)), cursor_store);

        let events = poller
            .poll_gitlab_pipeline_runs("my-org/my-proj")
            .await
            .unwrap();
        assert!(
            events.is_empty(),
            "Must return empty when no active MRs exist"
        );
    }

    #[tokio::test]
    async fn test_poll_gitlab_pipeline_runs_filters_by_mr_source_branch() {
        use crate::gitlab::{GitlabMergeRequest, GitlabPipeline, GitlabPipelineJob};

        let mut mock_gl = MockGitlabClient::new();

        // 1 active MR on "feature/fix-engine"
        mock_gl
            .expect_list_active_merge_requests()
            .returning(|_project| {
                Ok(vec![GitlabMergeRequest {
                    id: 501,
                    iid: 42,
                    title: "Fix Engine MR".to_string(),
                    description: None,
                    web_url: "https://gitlab.com/my-org/my-proj/-/merge_requests/42".to_string(),
                    state: "opened".to_string(),
                    updated_at: Some(Utc::now()),
                    source_branch: Some("feature/fix-engine".to_string()),
                    sha: Some("sha-engine".to_string()),
                }])
            });

        // 2 failed pipelines: one on main (schedule), one on MR branch
        mock_gl
            .expect_list_failed_pipelines()
            .returning(|_project, _since| {
                Ok(vec![
                    GitlabPipeline {
                        id: 1001,
                        status: "failed".to_string(),
                        web_url: "https://gitlab.com/my-org/my-proj/-/pipelines/1001".to_string(),
                        ref_: Some("main".to_string()),
                        updated_at: Some(Utc::now()),
                        sha: Some("sha-main".to_string()),
                        source: Some("schedule".to_string()),
                    },
                    GitlabPipeline {
                        id: 1002,
                        status: "failed".to_string(),
                        web_url: "https://gitlab.com/my-org/my-proj/-/pipelines/1002".to_string(),
                        ref_: Some("feature/fix-engine".to_string()),
                        updated_at: Some(Utc::now()),
                        sha: Some("sha-engine".to_string()),
                        source: Some("merge_request_event".to_string()),
                    },
                ])
            });

        // Only pipeline 1002 jobs & trace should be fetched
        mock_gl
            .expect_get_pipeline_jobs()
            .times(1)
            .returning(|_project, _pipeline_id| {
                Ok(vec![GitlabPipelineJob {
                    id: 77,
                    name: "cargo-clippy".to_string(),
                    status: "failed".to_string(),
                    stage: "test".to_string(),
                    web_url: "https://gitlab.com/my-org/my-proj/-/jobs/77".to_string(),
                }])
            });

        mock_gl
            .expect_get_job_trace()
            .times(1)
            .returning(|_project, _job_id| Ok("error: unused import".to_string()));

        let cursor_store = Arc::new(InMemoryCursorStore::new());
        let poller = GitPlatformPoller::new(None, Some(Arc::new(mock_gl)), cursor_store.clone());

        let events = poller
            .poll_gitlab_pipeline_runs("my-org/my-proj")
            .await
            .unwrap();

        assert_eq!(
            events.len(),
            1,
            "Only the active MR pipeline should be emitted"
        );
        let event = &events[0];
        assert_eq!(event.run_id, 1002);
        assert_eq!(event.pr_number, Some(42));
        assert_eq!(event.head_branch.as_deref(), Some("feature/fix-engine"));
        assert_eq!(event.head_sha.as_deref(), Some("sha-engine"));

        // Verify non-MR pipeline 1001 was marked processed in cursor store
        let is_1001_processed = cursor_store
            .is_event_processed(
                "gitlab:my-org/my-proj:pipelines",
                "pipeline:my-org/my-proj:1001",
            )
            .await
            .unwrap();
        assert!(
            is_1001_processed,
            "Non-MR pipeline must be marked as processed"
        );
    }
}
