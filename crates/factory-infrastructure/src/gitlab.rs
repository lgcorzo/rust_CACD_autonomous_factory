use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitlabIssue {
    pub id: u64,
    pub iid: u64,
    pub title: String,
    pub description: Option<String>,
    pub web_url: String,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitlabMergeRequest {
    pub id: u64,
    pub iid: u64,
    pub title: String,
    pub description: Option<String>,
    pub web_url: String,
    pub state: String,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GitlabAuthor {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitlabNote {
    pub id: u64,
    pub body: String,
    pub author: GitlabAuthor,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GitlabCommitAction {
    pub action: String, // "create" | "delete" | "move" | "update" | "chmod"
    pub file_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GitlabAwardEmoji {
    pub id: u64,
    pub name: String,
    pub user: GitlabAuthor,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GitDeliveryResult {
    pub branch_name: String,
    pub commit_sha: Option<String>,
    pub mr_iid: u64,
    pub mr_web_url: String,
}

// ──────────────────────────────────────────────────────────────────────────────
// GitLab CI Pipeline API Types (T012)
// ──────────────────────────────────────────────────────────────────────────────

/// GitLab CI pipeline run.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitlabPipeline {
    pub id: u64,
    pub status: String,
    pub web_url: String,
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// GitLab CI pipeline job within a pipeline.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitlabPipelineJob {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub stage: String,
    pub web_url: String,
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait GitlabClient: Send + Sync {
    async fn create_issue(
        &self,
        project_id: &str,
        title: &str,
        description: &str,
    ) -> anyhow::Result<GitlabIssue>;

    async fn create_issue_with_labels(
        &self,
        project_id: &str,
        title: &str,
        description: &str,
        labels: &[String],
    ) -> anyhow::Result<GitlabIssue>;

    async fn list_open_issues(
        &self,
        project_id: &str,
        labels: Option<String>,
    ) -> anyhow::Result<Vec<GitlabIssue>>;

    async fn list_issues_updated_since(
        &self,
        project_id: &str,
        labels: Option<String>,
        since: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<GitlabIssue>>;

    async fn list_active_merge_requests(
        &self,
        project_id: &str,
    ) -> anyhow::Result<Vec<GitlabMergeRequest>>;

    async fn list_merge_request_notes(
        &self,
        project_id: &str,
        mr_iid: u64,
        since: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<GitlabNote>>;

    async fn post_merge_request_note(
        &self,
        project_id: &str,
        mr_iid: u64,
        body: &str,
    ) -> anyhow::Result<GitlabNote>;

    async fn post_issue_note(
        &self,
        project_id: &str,
        issue_iid: u64,
        body: &str,
    ) -> anyhow::Result<GitlabNote>;

    async fn create_branch(
        &self,
        project_id: &str,
        branch: &str,
        ref_branch: &str,
    ) -> anyhow::Result<String>;

    async fn create_commit_files(
        &self,
        project_id: &str,
        branch: &str,
        commit_message: &str,
        actions: &[GitlabCommitAction],
    ) -> anyhow::Result<String>;

    async fn create_merge_request(
        &self,
        project_id: &str,
        source_branch: &str,
        target_branch: &str,
        title: &str,
        description: &str,
    ) -> anyhow::Result<GitlabMergeRequest>;

    async fn add_merge_request_note_award_emoji(
        &self,
        project_id: &str,
        mr_iid: u64,
        note_id: u64,
        emoji_name: &str,
    ) -> anyhow::Result<GitlabAwardEmoji>;

    async fn check_current_user(&self) -> anyhow::Result<GitlabAuthor>;

    async fn get_project(&self, project_id: &str) -> anyhow::Result<bool>;

    // ── Pipeline Error Remediation Methods ──

    /// List failed CI pipeline runs for a GitLab project.
    async fn list_failed_pipelines(
        &self,
        project_id: &str,
        since: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<GitlabPipeline>>;

    /// Get the jobs for a specific pipeline.
    async fn get_pipeline_jobs(
        &self,
        project_id: &str,
        pipeline_id: u64,
    ) -> anyhow::Result<Vec<GitlabPipelineJob>>;

    /// Download the log trace for a specific job (truncated to 10KB).
    async fn get_job_trace(&self, project_id: &str, job_id: u64) -> anyhow::Result<String>;
}

pub struct HttpGitlabClient {
    url: String,
    api_token: String,
    client: reqwest::Client,
}

impl HttpGitlabClient {
    pub fn new(url: String, api_token: String) -> Self {
        Self {
            url,
            api_token,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl GitlabClient for HttpGitlabClient {
    async fn create_issue(
        &self,
        project_id: &str,
        title: &str,
        description: &str,
    ) -> anyhow::Result<GitlabIssue> {
        self.create_issue_with_labels(project_id, title, description, &[])
            .await
    }

    async fn create_issue_with_labels(
        &self,
        project_id: &str,
        title: &str,
        description: &str,
        labels: &[String],
    ) -> anyhow::Result<GitlabIssue> {
        let encoded_project_id = urlencoding::encode(project_id);
        let create_url = format!(
            "{}/api/v4/projects/{}/issues",
            self.url.trim_end_matches('/'),
            encoded_project_id
        );

        let mut payload = serde_json::json!({
            "title": title,
            "description": description
        });

        if !labels.is_empty() {
            payload["labels"] = serde_json::Value::String(labels.join(","));
        }

        let res = self
            .client
            .post(&create_url)
            .header("PRIVATE-TOKEN", &self.api_token)
            .json(&payload)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitLab create issue failed with status {}", status);
            anyhow::bail!("GitLab create issue failed with status {}", status);
        }

        let issue: GitlabIssue = res.json().await?;
        Ok(issue)
    }

    async fn list_open_issues(
        &self,
        project_id: &str,
        labels: Option<String>,
    ) -> anyhow::Result<Vec<GitlabIssue>> {
        self.list_issues_updated_since(project_id, labels, None)
            .await
    }

    async fn list_issues_updated_since(
        &self,
        project_id: &str,
        labels: Option<String>,
        since: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<GitlabIssue>> {
        let encoded_project_id = urlencoding::encode(project_id);
        let mut list_url = format!(
            "{}/api/v4/projects/{}/issues?state=opened&order_by=updated_at&sort=desc",
            self.url.trim_end_matches('/'),
            encoded_project_id
        );
        if let Some(lbl) = labels {
            list_url.push_str("&labels=");
            list_url.push_str(&urlencoding::encode(&lbl));
        }
        if let Some(s) = since {
            list_url.push_str("&updated_after=");
            list_url.push_str(&urlencoding::encode(&s.to_rfc3339()));
        }

        let res = self
            .client
            .get(&list_url)
            .header("PRIVATE-TOKEN", &self.api_token)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitLab list issues failed with status {}", status);
            anyhow::bail!("GitLab list issues failed with status {}", status);
        }

        let issues: Vec<GitlabIssue> = res.json().await?;
        Ok(issues)
    }

    async fn list_active_merge_requests(
        &self,
        project_id: &str,
    ) -> anyhow::Result<Vec<GitlabMergeRequest>> {
        let encoded_project_id = urlencoding::encode(project_id);
        let list_url = format!(
            "{}/api/v4/projects/{}/merge_requests?state=opened&order_by=updated_at&sort=desc",
            self.url.trim_end_matches('/'),
            encoded_project_id
        );

        let res = self
            .client
            .get(&list_url)
            .header("PRIVATE-TOKEN", &self.api_token)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitLab list MRs failed with status {}", status);
            anyhow::bail!("GitLab list MRs failed with status {}", status);
        }

        let mrs: Vec<GitlabMergeRequest> = res.json().await?;
        Ok(mrs)
    }

    async fn list_merge_request_notes(
        &self,
        project_id: &str,
        mr_iid: u64,
        since: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<GitlabNote>> {
        let encoded_project_id = urlencoding::encode(project_id);
        let mut list_url = format!(
            "{}/api/v4/projects/{}/merge_requests/{}/notes?sort=desc",
            self.url.trim_end_matches('/'),
            encoded_project_id,
            mr_iid
        );
        if let Some(s) = since {
            list_url.push_str("&updated_after=");
            list_url.push_str(&urlencoding::encode(&s.to_rfc3339()));
        }

        let res = self
            .client
            .get(&list_url)
            .header("PRIVATE-TOKEN", &self.api_token)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitLab list MR notes failed with status {}", status);
            anyhow::bail!("GitLab list MR notes failed with status {}", status);
        }

        let notes: Vec<GitlabNote> = res.json().await?;
        Ok(notes)
    }

    async fn post_merge_request_note(
        &self,
        project_id: &str,
        mr_iid: u64,
        body: &str,
    ) -> anyhow::Result<GitlabNote> {
        let encoded_project_id = urlencoding::encode(project_id);
        let post_url = format!(
            "{}/api/v4/projects/{}/merge_requests/{}/notes",
            self.url.trim_end_matches('/'),
            encoded_project_id,
            mr_iid
        );
        let payload = serde_json::json!({ "body": body });

        let res = self
            .client
            .post(&post_url)
            .header("PRIVATE-TOKEN", &self.api_token)
            .json(&payload)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitLab post MR note failed with status {}", status);
            anyhow::bail!("GitLab post MR note failed with status {}", status);
        }

        let note: GitlabNote = res.json().await?;
        Ok(note)
    }

    async fn post_issue_note(
        &self,
        project_id: &str,
        issue_iid: u64,
        body: &str,
    ) -> anyhow::Result<GitlabNote> {
        let encoded_project_id = urlencoding::encode(project_id);
        let post_url = format!(
            "{}/api/v4/projects/{}/issues/{}/notes",
            self.url.trim_end_matches('/'),
            encoded_project_id,
            issue_iid
        );
        let payload = serde_json::json!({ "body": body });

        let res = self
            .client
            .post(&post_url)
            .header("PRIVATE-TOKEN", &self.api_token)
            .json(&payload)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitLab post issue note failed with status {}", status);
            anyhow::bail!("GitLab post issue note failed with status {}", status);
        }

        let note: GitlabNote = res.json().await?;
        Ok(note)
    }

    async fn create_branch(
        &self,
        project_id: &str,
        branch: &str,
        ref_branch: &str,
    ) -> anyhow::Result<String> {
        let encoded_project_id = urlencoding::encode(project_id);
        let post_url = format!(
            "{}/api/v4/projects/{}/repository/branches",
            self.url.trim_end_matches('/'),
            encoded_project_id
        );
        let payload = serde_json::json!({
            "branch": branch,
            "ref": ref_branch
        });

        let res = self
            .client
            .post(&post_url)
            .header("PRIVATE-TOKEN", &self.api_token)
            .json(&payload)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitLab create branch failed with status {}", status);
            anyhow::bail!("GitLab create branch failed with status {}", status);
        }

        let body: serde_json::Value = res.json().await?;
        let name = body["name"].as_str().unwrap_or(branch).to_string();
        Ok(name)
    }

    async fn create_commit_files(
        &self,
        project_id: &str,
        branch: &str,
        commit_message: &str,
        actions: &[GitlabCommitAction],
    ) -> anyhow::Result<String> {
        let encoded_project_id = urlencoding::encode(project_id);
        let post_url = format!(
            "{}/api/v4/projects/{}/repository/commits",
            self.url.trim_end_matches('/'),
            encoded_project_id
        );
        let payload = serde_json::json!({
            "branch": branch,
            "commit_message": commit_message,
            "actions": actions
        });

        let res = self
            .client
            .post(&post_url)
            .header("PRIVATE-TOKEN", &self.api_token)
            .json(&payload)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitLab create commit failed with status {}", status);
            anyhow::bail!("GitLab create commit failed with status {}", status);
        }

        let body: serde_json::Value = res.json().await?;
        let commit_sha = body["id"].as_str().unwrap_or("").to_string();
        Ok(commit_sha)
    }

    async fn create_merge_request(
        &self,
        project_id: &str,
        source_branch: &str,
        target_branch: &str,
        title: &str,
        description: &str,
    ) -> anyhow::Result<GitlabMergeRequest> {
        let encoded_project_id = urlencoding::encode(project_id);
        let post_url = format!(
            "{}/api/v4/projects/{}/merge_requests",
            self.url.trim_end_matches('/'),
            encoded_project_id
        );
        let payload = serde_json::json!({
            "source_branch": source_branch,
            "target_branch": target_branch,
            "title": title,
            "description": description
        });

        let res = self
            .client
            .post(&post_url)
            .header("PRIVATE-TOKEN", &self.api_token)
            .json(&payload)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitLab create merge request failed with status {}", status);
            anyhow::bail!("GitLab create merge request failed with status {}", status);
        }

        let mr: GitlabMergeRequest = res.json().await?;
        Ok(mr)
    }

    async fn add_merge_request_note_award_emoji(
        &self,
        project_id: &str,
        mr_iid: u64,
        note_id: u64,
        emoji_name: &str,
    ) -> anyhow::Result<GitlabAwardEmoji> {
        let encoded_project_id = urlencoding::encode(project_id);
        let post_url = format!(
            "{}/api/v4/projects/{}/merge_requests/{}/notes/{}/award_emoji",
            self.url.trim_end_matches('/'),
            encoded_project_id,
            mr_iid,
            note_id
        );
        let payload = serde_json::json!({ "name": emoji_name });

        let res = self
            .client
            .post(&post_url)
            .header("PRIVATE-TOKEN", &self.api_token)
            .json(&payload)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitLab add award emoji failed with status {}", status);
            anyhow::bail!("GitLab add award emoji failed with status {}", status);
        }

        let emoji: GitlabAwardEmoji = res.json().await?;
        Ok(emoji)
    }

    async fn check_current_user(&self) -> anyhow::Result<GitlabAuthor> {
        let get_url = format!("{}/api/v4/user", self.url.trim_end_matches('/'));
        let res = self
            .client
            .get(&get_url)
            .header("PRIVATE-TOKEN", &self.api_token)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitLab check current user failed with status {}", status);
            anyhow::bail!("GitLab check current user failed with status {}", status);
        }

        let user: GitlabAuthor = res.json().await?;
        Ok(user)
    }

    async fn get_project(&self, project_id: &str) -> anyhow::Result<bool> {
        let encoded_project_id = urlencoding::encode(project_id);
        let get_url = format!(
            "{}/api/v4/projects/{}",
            self.url.trim_end_matches('/'),
            encoded_project_id
        );
        let res = self
            .client
            .get(&get_url)
            .header("PRIVATE-TOKEN", &self.api_token)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(true)
        } else if res.status() == reqwest::StatusCode::NOT_FOUND {
            Ok(false)
        } else {
            let status = res.status();
            tracing::error!("GitLab get project failed with status {}", status);
            anyhow::bail!("GitLab get project failed with status {}", status);
        }
    }

    // ── Pipeline Error Remediation Implementations (T024–T026) ──

    async fn list_failed_pipelines(
        &self,
        project_id: &str,
        since: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<GitlabPipeline>> {
        let encoded_project_id = urlencoding::encode(project_id);
        let mut url = format!(
            "{}/api/v4/projects/{}/pipelines?status=failed&order_by=updated_at&sort=desc",
            self.url.trim_end_matches('/'),
            encoded_project_id
        );
        if let Some(s) = since {
            url.push_str(&format!(
                "&updated_after={}",
                urlencoding::encode(&s.to_rfc3339())
            ));
        }

        let res = self
            .client
            .get(&url)
            .header("PRIVATE-TOKEN", &self.api_token)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitLab list failed pipelines failed with status {}", status);
            anyhow::bail!("GitLab list failed pipelines failed with status {}", status);
        }

        let pipelines: Vec<GitlabPipeline> = res.json().await?;
        Ok(pipelines)
    }

    async fn get_pipeline_jobs(
        &self,
        project_id: &str,
        pipeline_id: u64,
    ) -> anyhow::Result<Vec<GitlabPipelineJob>> {
        let encoded_project_id = urlencoding::encode(project_id);
        let url = format!(
            "{}/api/v4/projects/{}/pipelines/{}/jobs",
            self.url.trim_end_matches('/'),
            encoded_project_id,
            pipeline_id
        );

        let res = self
            .client
            .get(&url)
            .header("PRIVATE-TOKEN", &self.api_token)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitLab get pipeline jobs failed with status {}", status);
            anyhow::bail!("GitLab get pipeline jobs failed with status {}", status);
        }

        let jobs: Vec<GitlabPipelineJob> = res.json().await?;
        Ok(jobs)
    }

    async fn get_job_trace(&self, project_id: &str, job_id: u64) -> anyhow::Result<String> {
        let encoded_project_id = urlencoding::encode(project_id);
        let url = format!(
            "{}/api/v4/projects/{}/jobs/{}/trace",
            self.url.trim_end_matches('/'),
            encoded_project_id,
            job_id
        );

        let res = self
            .client
            .get(&url)
            .header("PRIVATE-TOKEN", &self.api_token)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitLab get job trace failed with status {}", status);
            anyhow::bail!("GitLab get job trace failed with status {}", status);
        }

        let trace = res.text().await?;
        // Truncate to 10KB
        let max_len = 10 * 1024;
        if trace.len() > max_len {
            Ok(trace[..max_len].to_string())
        } else {
            Ok(trace)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_gitlab_create_issue_success() {
        let mock_server = MockServer::start().await;
        let client = HttpGitlabClient::new(mock_server.uri(), "test_token".to_string());

        let response_body = json!({
            "id": 12345,
            "iid": 42,
            "title": "Crash: ZeroDivisionError",
            "description": "Details here",
            "web_url": "https://gitlab.com/my-org/my-project/-/issues/42"
        });

        let expected_payload = json!({
            "title": "Crash: ZeroDivisionError",
            "description": "Details here"
        });

        Mock::given(method("POST"))
            .and(path("/api/v4/projects/my-org%2Fmy-project/issues"))
            .and(header("PRIVATE-TOKEN", "test_token"))
            .and(body_json(expected_payload))
            .respond_with(ResponseTemplate::new(201).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let result = client
            .create_issue(
                "my-org/my-project",
                "Crash: ZeroDivisionError",
                "Details here",
            )
            .await
            .unwrap();
        assert_eq!(result.id, 12345);
        assert_eq!(result.iid, 42);
        assert_eq!(result.title, "Crash: ZeroDivisionError");
    }

    #[tokio::test]
    async fn test_gitlab_list_mr_notes_and_post() {
        let mock_server = MockServer::start().await;
        let client = HttpGitlabClient::new(mock_server.uri(), "test_token".to_string());

        let notes_resp = json!([
            {
                "id": 888,
                "body": "@dark-gravity /retry",
                "author": { "username": "qa-engineer" },
                "updated_at": "2026-08-15T20:15:00Z"
            }
        ]);

        Mock::given(method("GET"))
            .and(path(
                "/api/v4/projects/my-org%2Fmy-project/merge_requests/7/notes",
            ))
            .and(header("PRIVATE-TOKEN", "test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(notes_resp))
            .mount(&mock_server)
            .await;

        let post_note_resp = json!({
            "id": 889,
            "body": "DAG restarted.",
            "author": { "username": "dark-gravity-bot" },
            "updated_at": "2026-08-15T20:16:00Z"
        });

        Mock::given(method("POST"))
            .and(path(
                "/api/v4/projects/my-org%2Fmy-project/merge_requests/7/notes",
            ))
            .and(header("PRIVATE-TOKEN", "test_token"))
            .respond_with(ResponseTemplate::new(201).set_body_json(post_note_resp))
            .mount(&mock_server)
            .await;

        let notes = client
            .list_merge_request_notes("my-org/my-project", 7, None)
            .await
            .unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].author.username, "qa-engineer");
        assert_eq!(notes[0].body, "@dark-gravity /retry");

        let posted = client
            .post_merge_request_note("my-org/my-project", 7, "DAG restarted.")
            .await
            .unwrap();
        assert_eq!(posted.id, 889);
        assert_eq!(posted.body, "DAG restarted.");
    }

    #[tokio::test]
    async fn test_gitlab_post_issue_note() {
        let mock_server = MockServer::start().await;
        let client = HttpGitlabClient::new(mock_server.uri(), "test_token".to_string());

        let post_note_resp = json!({
            "id": 990,
            "body": "🚀 **Dark Gravity Mission Ingested**",
            "author": { "username": "dark-gravity-bot" },
            "updated_at": "2026-09-18T20:00:00Z"
        });

        Mock::given(method("POST"))
            .and(path("/api/v4/projects/my-org%2Fmy-project/issues/1/notes"))
            .and(header("PRIVATE-TOKEN", "test_token"))
            .and(body_json(
                json!({ "body": "🚀 **Dark Gravity Mission Ingested**" }),
            ))
            .respond_with(ResponseTemplate::new(201).set_body_json(post_note_resp))
            .mount(&mock_server)
            .await;

        let posted = client
            .post_issue_note(
                "my-org/my-project",
                1,
                "🚀 **Dark Gravity Mission Ingested**",
            )
            .await
            .unwrap();
        assert_eq!(posted.id, 990);
        assert_eq!(posted.body, "🚀 **Dark Gravity Mission Ingested**");
    }

    #[tokio::test]
    async fn test_gitlab_create_branch() {
        let mock_server = MockServer::start().await;
        let client = HttpGitlabClient::new(mock_server.uri(), "test_token".to_string());

        let branch_resp = json!({
            "name": "mission-1234",
            "merged": false,
            "protected": false,
            "default": false
        });

        Mock::given(method("POST"))
            .and(path(
                "/api/v4/projects/my-org%2Fmy-project/repository/branches",
            ))
            .and(header("PRIVATE-TOKEN", "test_token"))
            .and(body_json(json!({
                "branch": "mission-1234",
                "ref": "main"
            })))
            .respond_with(ResponseTemplate::new(201).set_body_json(branch_resp))
            .mount(&mock_server)
            .await;

        let branch = client
            .create_branch("my-org/my-project", "mission-1234", "main")
            .await
            .unwrap();
        assert_eq!(branch, "mission-1234");
    }

    #[tokio::test]
    async fn test_gitlab_create_commit_files() {
        let mock_server = MockServer::start().await;
        let client = HttpGitlabClient::new(mock_server.uri(), "test_token".to_string());

        let commit_resp = json!({
            "id": "ed899a888b945ce374c816c8555b1b68b062e1ac",
            "short_id": "ed899a88",
            "title": "feat: autonomous delivery",
            "message": "feat: autonomous delivery\n\nCloses #1"
        });

        let actions = vec![GitlabCommitAction {
            action: "create".to_string(),
            file_path: "src/lib.rs".to_string(),
            content: Some("// newly delivered code".to_string()),
        }];

        Mock::given(method("POST"))
            .and(path(
                "/api/v4/projects/my-org%2Fmy-project/repository/commits",
            ))
            .and(header("PRIVATE-TOKEN", "test_token"))
            .respond_with(ResponseTemplate::new(201).set_body_json(commit_resp))
            .mount(&mock_server)
            .await;

        let sha = client
            .create_commit_files(
                "my-org/my-project",
                "mission-1234",
                "feat: autonomous delivery\n\nCloses #1",
                &actions,
            )
            .await
            .unwrap();
        assert_eq!(sha, "ed899a888b945ce374c816c8555b1b68b062e1ac");
    }

    #[tokio::test]
    async fn test_gitlab_create_merge_request() {
        let mock_server = MockServer::start().await;
        let client = HttpGitlabClient::new(mock_server.uri(), "test_token".to_string());

        let mr_resp = json!({
            "id": 9991,
            "iid": 12,
            "title": "feat: autonomous mission delivery",
            "description": "Closes #1",
            "web_url": "https://gitlab.com/my-org/my-project/-/merge_requests/12",
            "state": "opened",
            "updated_at": "2026-09-18T20:00:00Z"
        });

        Mock::given(method("POST"))
            .and(path("/api/v4/projects/my-org%2Fmy-project/merge_requests"))
            .and(header("PRIVATE-TOKEN", "test_token"))
            .and(body_json(json!({
                "source_branch": "mission-1234",
                "target_branch": "main",
                "title": "feat: autonomous mission delivery",
                "description": "Closes #1"
            })))
            .respond_with(ResponseTemplate::new(201).set_body_json(mr_resp))
            .mount(&mock_server)
            .await;

        let mr = client
            .create_merge_request(
                "my-org/my-project",
                "mission-1234",
                "main",
                "feat: autonomous mission delivery",
                "Closes #1",
            )
            .await
            .unwrap();
        assert_eq!(mr.iid, 12);
        assert_eq!(
            mr.web_url,
            "https://gitlab.com/my-org/my-project/-/merge_requests/12"
        );
    }

    #[tokio::test]
    async fn test_gitlab_check_current_user() {
        let mock_server = MockServer::start().await;
        let client = HttpGitlabClient::new(mock_server.uri(), "test_token".to_string());

        let user_resp = json!({
            "username": "darkgravity-bot"
        });

        Mock::given(method("GET"))
            .and(path("/api/v4/user"))
            .and(header("PRIVATE-TOKEN", "test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(user_resp))
            .mount(&mock_server)
            .await;

        let user = client.check_current_user().await.unwrap();
        assert_eq!(user.username, "darkgravity-bot");
    }

    #[tokio::test]
    async fn test_gitlab_get_project() {
        let mock_server = MockServer::start().await;
        let client = HttpGitlabClient::new(mock_server.uri(), "test_token".to_string());

        Mock::given(method("GET"))
            .and(path("/api/v4/projects/my-org%2Fmy-project"))
            .and(header("PRIVATE-TOKEN", "test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": 123})))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/api/v4/projects/my-org%2Fmissing-project"))
            .and(header("PRIVATE-TOKEN", "test_token"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let exists = client.get_project("my-org/my-project").await.unwrap();
        assert!(exists);

        let missing = client.get_project("my-org/missing-project").await.unwrap();
        assert!(!missing);
    }

    // ── Pipeline Error Remediation Tests (T017–T018) ──

    #[tokio::test]
    async fn test_list_failed_gitlab_pipelines() {
        let mock_server = MockServer::start().await;
        let client = HttpGitlabClient::new(mock_server.uri(), "test_token".to_string());

        let pipelines_resp = json!([
            {
                "id": 201,
                "status": "failed",
                "web_url": "https://gitlab.com/my-org/my-project/-/pipelines/201",
                "ref_": "main",
                "updated_at": "2026-09-20T10:00:00Z"
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v4/projects/my-org%2Fmy-project/pipelines"))
            .and(header("PRIVATE-TOKEN", "test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(pipelines_resp))
            .mount(&mock_server)
            .await;

        let pipelines = client
            .list_failed_pipelines("my-org/my-project", None)
            .await
            .unwrap();

        assert_eq!(pipelines.len(), 1);
        assert_eq!(pipelines[0].id, 201);
        assert_eq!(pipelines[0].status, "failed");
    }

    #[tokio::test]
    async fn test_get_gitlab_job_trace() {
        let mock_server = MockServer::start().await;
        let client = HttpGitlabClient::new(mock_server.uri(), "test_token".to_string());

        // Test normal trace
        let trace_content = "error[E0308]: mismatched types\n  --> src/lib.rs:15:5";
        Mock::given(method("GET"))
            .and(path("/api/v4/projects/my-org%2Fmy-project/jobs/42/trace"))
            .and(header("PRIVATE-TOKEN", "test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_string(trace_content))
            .mount(&mock_server)
            .await;

        let trace = client.get_job_trace("my-org/my-project", 42).await.unwrap();
        assert_eq!(trace, trace_content);
        assert!(trace.len() <= 10 * 1024);

        // Test 10KB truncation
        let mock_server2 = MockServer::start().await;
        let client2 = HttpGitlabClient::new(mock_server2.uri(), "test_token".to_string());
        let large_trace = "x".repeat(20 * 1024); // 20KB
        Mock::given(method("GET"))
            .and(path("/api/v4/projects/my-org%2Fmy-project/jobs/99/trace"))
            .and(header("PRIVATE-TOKEN", "test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_string(large_trace.clone()))
            .mount(&mock_server2)
            .await;

        let truncated = client2
            .get_job_trace("my-org/my-project", 99)
            .await
            .unwrap();
        assert_eq!(truncated.len(), 10 * 1024);
    }

    #[tokio::test]
    async fn test_get_gitlab_pipeline_jobs() {
        let mock_server = MockServer::start().await;
        let client = HttpGitlabClient::new(mock_server.uri(), "test_token".to_string());

        let jobs_resp = json!([
            {
                "id": 301,
                "name": "rust-test",
                "status": "failed",
                "stage": "test",
                "web_url": "https://gitlab.com/my-org/my-project/-/jobs/301"
            }
        ]);

        Mock::given(method("GET"))
            .and(path(
                "/api/v4/projects/my-org%2Fmy-project/pipelines/201/jobs",
            ))
            .and(header("PRIVATE-TOKEN", "test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(jobs_resp))
            .mount(&mock_server)
            .await;

        let jobs = client
            .get_pipeline_jobs("my-org/my-project", 201)
            .await
            .unwrap();

        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].id, 301);
        assert_eq!(jobs[0].name, "rust-test");
        assert_eq!(jobs[0].status, "failed");
    }
}
