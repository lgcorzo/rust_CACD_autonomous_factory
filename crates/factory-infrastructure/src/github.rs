use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubIssue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub html_url: String,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubPullRequest {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub html_url: String,
    pub state: String,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GithubUser {
    pub login: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GithubReaction {
    pub id: u64,
    pub content: String,
    pub user: Option<GithubUser>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubComment {
    pub id: u64,
    pub body: String,
    pub user: GithubUser,
    pub html_url: String,
    pub updated_at: Option<DateTime<Utc>>,
}

// ──────────────────────────────────────────────────────────────────────────────
// GitHub Actions Workflow Run API Types
// ──────────────────────────────────────────────────────────────────────────────

/// GitHub Actions workflow run.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWorkflowRun {
    pub id: u64,
    pub name: Option<String>,
    pub status: String,
    pub conclusion: Option<String>,
    pub html_url: String,
    pub updated_at: Option<DateTime<Utc>>,
}

/// GitHub Actions workflow job.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWorkflowJob {
    pub id: u64,
    pub name: String,
    pub conclusion: Option<String>,
    #[serde(default)]
    pub steps: Vec<GithubWorkflowStep>,
}

/// GitHub Actions workflow step within a job.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWorkflowStep {
    pub name: String,
    pub conclusion: Option<String>,
}

/// Wrapper for workflow runs list response.
#[derive(Deserialize, Debug)]
pub struct GithubWorkflowRunsResponse {
    pub workflow_runs: Vec<GithubWorkflowRun>,
}

/// Wrapper for workflow jobs list response.
#[derive(Deserialize, Debug)]
pub struct GithubWorkflowJobsResponse {
    pub jobs: Vec<GithubWorkflowJob>,
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait GithubClient: Send + Sync {
    async fn create_issue(
        &self,
        repo: &str,
        title: &str,
        body: &str,
        labels: &[String],
    ) -> anyhow::Result<GithubIssue>;

    async fn list_open_issues(
        &self,
        repo: &str,
        labels: Option<String>,
    ) -> anyhow::Result<Vec<GithubIssue>>;

    async fn list_issues_updated_since(
        &self,
        repo: &str,
        labels: Option<String>,
        since: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<GithubIssue>>;

    async fn list_active_pull_requests(&self, repo: &str)
        -> anyhow::Result<Vec<GithubPullRequest>>;

    async fn list_pull_request_comments(
        &self,
        repo: &str,
        pr_number: u64,
        since: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<GithubComment>>;

    async fn post_pull_request_comment(
        &self,
        repo: &str,
        pr_number: u64,
        body: &str,
    ) -> anyhow::Result<GithubComment>;

    async fn create_pull_request(
        &self,
        repo: &str,
        title: &str,
        head: &str,
        base: &str,
        body: &str,
    ) -> anyhow::Result<String>;

    async fn post_issue_comment(
        &self,
        repo: &str,
        issue_number: u64,
        body: &str,
    ) -> anyhow::Result<GithubComment>;

    async fn create_branch(
        &self,
        repo: &str,
        branch: &str,
        base_branch: &str,
    ) -> anyhow::Result<String>;

    async fn add_comment_reaction(
        &self,
        repo: &str,
        comment_id: u64,
        reaction: &str,
    ) -> anyhow::Result<GithubReaction>;

    // ── Pipeline Error Remediation Methods ──

    /// List failed workflow runs for a repository.
    async fn list_failed_workflow_runs(
        &self,
        repo: &str,
        since: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<GithubWorkflowRun>>;

    /// Get jobs for a specific workflow run.
    async fn get_workflow_run_jobs(
        &self,
        repo: &str,
        run_id: u64,
    ) -> anyhow::Result<Vec<GithubWorkflowJob>>;

    /// Download the log for a specific job (truncated to 10KB).
    async fn get_job_log(
        &self,
        repo: &str,
        job_id: u64,
    ) -> anyhow::Result<String>;
}

pub struct HttpGithubClient {
    api_url: String,
    api_token: String,
    client: reqwest::Client,
}

impl HttpGithubClient {
    pub fn new(api_token: String) -> Self {
        Self::with_url("https://api.github.com".to_string(), api_token)
    }

    pub fn with_url(api_url: String, api_token: String) -> Self {
        Self {
            api_url,
            api_token,
            client: reqwest::Client::builder()
                .user_agent("dark-gravity-factory/1.0")
                .build()
                .unwrap_or_default(),
        }
    }
}

#[async_trait]
impl GithubClient for HttpGithubClient {
    async fn create_issue(
        &self,
        repo: &str,
        title: &str,
        body: &str,
        labels: &[String],
    ) -> anyhow::Result<GithubIssue> {
        let url = format!(
            "{}/repos/{}/issues",
            self.api_url.trim_end_matches('/'),
            repo
        );
        let mut payload = serde_json::json!({
            "title": title,
            "body": body
        });
        if !labels.is_empty() {
            payload["labels"] = serde_json::json!(labels);
        }

        let mut req = self.client.post(&url).json(&payload);
        if !self.api_token.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_token));
            req = req.header("Accept", "application/vnd.github.v3+json");
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitHub create issue failed with status {}", status);
            anyhow::bail!("GitHub create issue failed with status {}", status);
        }

        let issue: GithubIssue = res.json().await?;
        Ok(issue)
    }

    async fn list_open_issues(
        &self,
        repo: &str,
        labels: Option<String>,
    ) -> anyhow::Result<Vec<GithubIssue>> {
        self.list_issues_updated_since(repo, labels, None).await
    }

    async fn list_issues_updated_since(
        &self,
        repo: &str,
        labels: Option<String>,
        since: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<GithubIssue>> {
        let mut url = format!(
            "{}/repos/{}/issues?state=open&sort=updated&direction=desc",
            self.api_url.trim_end_matches('/'),
            repo
        );
        if let Some(lbl) = labels {
            url.push_str("&labels=");
            url.push_str(&urlencoding::encode(&lbl));
        }
        if let Some(s) = since {
            url.push_str("&since=");
            url.push_str(&urlencoding::encode(&s.to_rfc3339()));
        }

        let mut req = self.client.get(&url);
        if !self.api_token.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_token));
            req = req.header("Accept", "application/vnd.github.v3+json");
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitHub list issues failed with status {}", status);
            anyhow::bail!("GitHub list issues failed with status {}", status);
        }

        let issues: Vec<GithubIssue> = res.json().await?;
        Ok(issues)
    }

    async fn list_active_pull_requests(
        &self,
        repo: &str,
    ) -> anyhow::Result<Vec<GithubPullRequest>> {
        let url = format!(
            "{}/repos/{}/pulls?state=open&sort=updated&direction=desc",
            self.api_url.trim_end_matches('/'),
            repo
        );

        let mut req = self.client.get(&url);
        if !self.api_token.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_token));
            req = req.header("Accept", "application/vnd.github.v3+json");
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitHub list PRs failed with status {}", status);
            anyhow::bail!("GitHub list PRs failed with status {}", status);
        }

        let prs: Vec<GithubPullRequest> = res.json().await?;
        Ok(prs)
    }

    async fn list_pull_request_comments(
        &self,
        repo: &str,
        pr_number: u64,
        since: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<GithubComment>> {
        let mut url = format!(
            "{}/repos/{}/issues/{}/comments?sort=updated&direction=desc",
            self.api_url.trim_end_matches('/'),
            repo,
            pr_number
        );
        if let Some(s) = since {
            url.push_str("&since=");
            url.push_str(&urlencoding::encode(&s.to_rfc3339()));
        }

        let mut req = self.client.get(&url);
        if !self.api_token.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_token));
            req = req.header("Accept", "application/vnd.github.v3+json");
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitHub list PR comments failed with status {}", status);
            anyhow::bail!("GitHub list PR comments failed with status {}", status);
        }

        let comments: Vec<GithubComment> = res.json().await?;
        Ok(comments)
    }

    async fn post_pull_request_comment(
        &self,
        repo: &str,
        pr_number: u64,
        body: &str,
    ) -> anyhow::Result<GithubComment> {
        let url = format!(
            "{}/repos/{}/issues/{}/comments",
            self.api_url.trim_end_matches('/'),
            repo,
            pr_number
        );
        let payload = serde_json::json!({ "body": body });

        let mut req = self.client.post(&url).json(&payload);
        if !self.api_token.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_token));
            req = req.header("Accept", "application/vnd.github.v3+json");
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitHub post PR comment failed with status {}", status);
            anyhow::bail!("GitHub post PR comment failed with status {}", status);
        }

        let comment: GithubComment = res.json().await?;
        Ok(comment)
    }

    async fn create_pull_request(
        &self,
        repo: &str,
        title: &str,
        head: &str,
        base: &str,
        body: &str,
    ) -> anyhow::Result<String> {
        let url = format!(
            "{}/repos/{}/pulls",
            self.api_url.trim_end_matches('/'),
            repo
        );
        let payload = serde_json::json!({
            "title": title,
            "head": head,
            "base": base,
            "body": body
        });

        let mut req = self.client.post(&url).json(&payload);
        if !self.api_token.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_token));
            req = req.header("Accept", "application/vnd.github.v3+json");
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitHub create PR failed with status {}", status);
            anyhow::bail!("GitHub create PR failed with status {}", status);
        }

        let body_val: serde_json::Value = res.json().await?;
        let html_url = body_val["html_url"].as_str().unwrap_or("").to_string();
        Ok(html_url)
    }

    async fn post_issue_comment(
        &self,
        repo: &str,
        issue_number: u64,
        body: &str,
    ) -> anyhow::Result<GithubComment> {
        let url = format!(
            "{}/repos/{}/issues/{}/comments",
            self.api_url.trim_end_matches('/'),
            repo,
            issue_number
        );
        let payload = serde_json::json!({
            "body": body
        });

        let mut req = self.client.post(&url).json(&payload);
        if !self.api_token.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_token));
            req = req.header("Accept", "application/vnd.github.v3+json");
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitHub post issue comment failed with status {}", status);
            anyhow::bail!("GitHub post issue comment failed with status {}", status);
        }

        let comment: GithubComment = res.json().await?;
        Ok(comment)
    }

    async fn create_branch(
        &self,
        repo: &str,
        branch: &str,
        base_branch: &str,
    ) -> anyhow::Result<String> {
        let ref_url = format!(
            "{}/repos/{}/git/ref/heads/{}",
            self.api_url.trim_end_matches('/'),
            repo,
            base_branch
        );
        let mut req = self.client.get(&ref_url);
        if !self.api_token.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_token));
            req = req.header("Accept", "application/vnd.github.v3+json");
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitHub get base branch ref failed with status {}", status);
            anyhow::bail!("GitHub get base branch ref failed with status {}", status);
        }

        let ref_data: serde_json::Value = res.json().await?;
        let sha = ref_data["object"]["sha"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing commit SHA in base ref"))?;

        let create_url = format!(
            "{}/repos/{}/git/refs",
            self.api_url.trim_end_matches('/'),
            repo
        );
        let payload = serde_json::json!({
            "ref": format!("refs/heads/{}", branch),
            "sha": sha
        });

        let mut create_req = self.client.post(&create_url).json(&payload);
        if !self.api_token.is_empty() {
            create_req = create_req.header("Authorization", format!("Bearer {}", self.api_token));
            create_req = create_req.header("Accept", "application/vnd.github.v3+json");
        }

        let create_res = create_req.send().await?;
        if !create_res.status().is_success() {
            let status = create_res.status();
            tracing::error!("GitHub create branch failed with status {}", status);
            anyhow::bail!("GitHub create branch failed with status {}", status);
        }

        Ok(branch.to_string())
    }

    async fn add_comment_reaction(
        &self,
        repo: &str,
        comment_id: u64,
        reaction: &str,
    ) -> anyhow::Result<GithubReaction> {
        let url = format!(
            "{}/repos/{}/issues/comments/{}/reactions",
            self.api_url.trim_end_matches('/'),
            repo,
            comment_id
        );
        let payload = serde_json::json!({ "content": reaction });

        let mut req = self.client.post(&url).json(&payload);
        if !self.api_token.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_token));
        }
        req = req.header("Accept", "application/vnd.github+json");
        req = req.header("User-Agent", "DarkGravity-Factory");

        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitHub add comment reaction failed with status {}", status);
            anyhow::bail!("GitHub add comment reaction failed with status {}", status);
        }

        let reaction_obj: GithubReaction = res.json().await?;
        Ok(reaction_obj)
    }

    // ── Pipeline Error Remediation Methods ──

    async fn list_failed_workflow_runs(
        &self,
        repo: &str,
        since: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<GithubWorkflowRun>> {
        let mut url = format!(
            "{}/repos/{}/actions/runs?status=failure",
            self.api_url.trim_end_matches('/'),
            repo
        );
        if let Some(s) = since {
            url.push_str(&format!("&created=>={}", urlencoding::encode(&s.to_rfc3339())));
        }

        let mut req = self.client.get(&url);
        if !self.api_token.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_token));
            req = req.header("Accept", "application/vnd.github.v3+json");
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitHub list failed workflow runs failed with status {}", status);
            anyhow::bail!("GitHub list failed workflow runs failed with status {}", status);
        }

        let response: GithubWorkflowRunsResponse = res.json().await?;
        Ok(response.workflow_runs)
    }

    async fn get_workflow_run_jobs(
        &self,
        repo: &str,
        run_id: u64,
    ) -> anyhow::Result<Vec<GithubWorkflowJob>> {
        let url = format!(
            "{}/repos/{}/actions/runs/{}/jobs",
            self.api_url.trim_end_matches('/'),
            repo,
            run_id
        );

        let mut req = self.client.get(&url);
        if !self.api_token.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_token));
            req = req.header("Accept", "application/vnd.github.v3+json");
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitHub get workflow run jobs failed with status {}", status);
            anyhow::bail!("GitHub get workflow run jobs failed with status {}", status);
        }

        let response: GithubWorkflowJobsResponse = res.json().await?;
        Ok(response.jobs)
    }

    async fn get_job_log(
        &self,
        repo: &str,
        job_id: u64,
    ) -> anyhow::Result<String> {
        let url = format!(
            "{}/repos/{}/actions/jobs/{}/logs",
            self.api_url.trim_end_matches('/'),
            repo,
            job_id
        );

        let mut req = self.client.get(&url);
        if !self.api_token.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_token));
            req = req.header("Accept", "application/vnd.github.v3+json");
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitHub get job log failed with status {}", status);
            anyhow::bail!("GitHub get job log failed with status {}", status);
        }

        let log_text = res.text().await?;
        // Truncate to 10KB
        let max_len = 10 * 1024;
        if log_text.len() > max_len {
            Ok(log_text[..max_len].to_string())
        } else {
            Ok(log_text)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_github_create_issue_success() {
        let mock_server = MockServer::start().await;

        let response = ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 101,
            "number": 42,
            "title": "Bug in production",
            "body": "Detailed crash report",
            "html_url": "https://github.com/my-org/my-repo/issues/42"
        }));

        Mock::given(method("POST"))
            .and(path("/repos/my-org/my-repo/issues"))
            .and(header("Authorization", "Bearer test-token"))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let client = HttpGithubClient::with_url(mock_server.uri(), "test-token".to_string());
        let issue = client
            .create_issue(
                "my-org/my-repo",
                "Bug in production",
                "Detailed crash report",
                &[],
            )
            .await
            .unwrap();

        assert_eq!(issue.number, 42);
        assert_eq!(
            issue.html_url,
            "https://github.com/my-org/my-repo/issues/42"
        );
    }

    #[tokio::test]
    async fn test_github_list_issues_and_comments() {
        let mock_server = MockServer::start().await;

        let issues_resp = ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 101,
                "number": 42,
                "title": "feat: mission issue",
                "body": "Autonomous mission [RESOURCE_LIMIT: RAM <= 30Mi]",
                "html_url": "https://github.com/my-org/my-repo/issues/42",
                "updated_at": "2026-08-15T20:00:00Z"
            }
        ]));

        Mock::given(method("GET"))
            .and(path("/repos/my-org/my-repo/issues"))
            .respond_with(issues_resp)
            .mount(&mock_server)
            .await;

        let comments_resp = ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 501,
                "body": "@dark-gravity /refine optimize memory",
                "user": { "login": "engineer-1" },
                "html_url": "https://github.com/my-org/my-repo/issues/42#issuecomment-501",
                "updated_at": "2026-08-15T20:05:00Z"
            }
        ]));

        Mock::given(method("GET"))
            .and(path("/repos/my-org/my-repo/issues/42/comments"))
            .respond_with(comments_resp)
            .mount(&mock_server)
            .await;

        let client = HttpGithubClient::with_url(mock_server.uri(), "test-token".to_string());
        let issues = client
            .list_issues_updated_since(
                "my-org/my-repo",
                Some("autonomous-mission".to_string()),
                None,
            )
            .await
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].number, 42);

        let comments = client
            .list_pull_request_comments("my-org/my-repo", 42, None)
            .await
            .unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].user.login, "engineer-1");
        assert_eq!(comments[0].body, "@dark-gravity /refine optimize memory");
    }

    #[tokio::test]
    async fn test_github_post_pr_comment() {
        let mock_server = MockServer::start().await;

        let comment_resp = ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 502,
            "body": "Aethelgard status: DAG healthy, 0 errors.",
            "user": { "login": "dark-gravity-bot" },
            "html_url": "https://github.com/my-org/my-repo/issues/42#issuecomment-502",
            "updated_at": "2026-08-15T20:10:00Z"
        }));

        Mock::given(method("POST"))
            .and(path("/repos/my-org/my-repo/issues/42/comments"))
            .respond_with(comment_resp)
            .mount(&mock_server)
            .await;

        let client = HttpGithubClient::with_url(mock_server.uri(), "test-token".to_string());
        let comment = client
            .post_pull_request_comment(
                "my-org/my-repo",
                42,
                "Aethelgard status: DAG healthy, 0 errors.",
            )
            .await
            .unwrap();

        assert_eq!(comment.id, 502);
        assert_eq!(comment.body, "Aethelgard status: DAG healthy, 0 errors.");
    }

    #[tokio::test]
    async fn test_github_post_issue_comment() {
        let mock_server = MockServer::start().await;

        let comment_resp = ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 601,
            "body": "🚀 **Dark Gravity Mission Ingested**",
            "user": { "login": "dark-gravity-bot" },
            "html_url": "https://github.com/my-org/my-repo/issues/42#issuecomment-601",
            "updated_at": "2026-09-18T20:00:00Z"
        }));

        Mock::given(method("POST"))
            .and(path("/repos/my-org/my-repo/issues/42/comments"))
            .respond_with(comment_resp)
            .mount(&mock_server)
            .await;

        let client = HttpGithubClient::with_url(mock_server.uri(), "test-token".to_string());
        let comment = client
            .post_issue_comment("my-org/my-repo", 42, "🚀 **Dark Gravity Mission Ingested**")
            .await
            .unwrap();

        assert_eq!(comment.id, 601);
        assert_eq!(comment.body, "🚀 **Dark Gravity Mission Ingested**");
    }

    #[tokio::test]
    async fn test_github_create_branch() {
        let mock_server = MockServer::start().await;

        let ref_resp = ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "ref": "refs/heads/main",
            "object": {
                "sha": "aa218f56b14c9653891f9e74264a383fa43fefbd",
                "type": "commit"
            }
        }));

        Mock::given(method("GET"))
            .and(path("/repos/my-org/my-repo/git/ref/heads/main"))
            .respond_with(ref_resp)
            .mount(&mock_server)
            .await;

        let create_resp = ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "ref": "refs/heads/mission-1234",
            "object": {
                "sha": "aa218f56b14c9653891f9e74264a383fa43fefbd"
            }
        }));

        Mock::given(method("POST"))
            .and(path("/repos/my-org/my-repo/git/refs"))
            .respond_with(create_resp)
            .mount(&mock_server)
            .await;

        let client = HttpGithubClient::with_url(mock_server.uri(), "test-token".to_string());
        let branch = client
            .create_branch("my-org/my-repo", "mission-1234", "main")
            .await
            .unwrap();

        assert_eq!(branch, "mission-1234");
    }
}
