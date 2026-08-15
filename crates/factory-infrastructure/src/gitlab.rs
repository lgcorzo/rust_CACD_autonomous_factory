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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
        self.list_issues_updated_since(project_id, labels, None).await
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
            .and(path("/api/v4/projects/my-org%2Fmy-project/merge_requests/7/notes"))
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
            .and(path("/api/v4/projects/my-org%2Fmy-project/merge_requests/7/notes"))
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
}

