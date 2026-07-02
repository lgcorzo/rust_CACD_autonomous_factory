use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitlabIssue {
    pub id: u64,
    pub iid: u64,
    pub title: String,
    pub description: Option<String>,
    pub web_url: String,
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait GitlabClient: Send + Sync {
    async fn create_issue(&self, project_id: &str, title: &str, description: &str) -> anyhow::Result<GitlabIssue>;
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
    async fn create_issue(&self, project_id: &str, title: &str, description: &str) -> anyhow::Result<GitlabIssue> {
        // Project ID in GitLab API can be URL-encoded path like `group%2Fproject` or numeric ID
        let encoded_project_id = urlencoding::encode(project_id);
        let create_url = format!("{}/api/v4/projects/{}/issues", self.url.trim_end_matches('/'), encoded_project_id);
        
        let payload = serde_json::json!({
            "title": title,
            "description": description
        });

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path, header, body_json};
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

        let result = client.create_issue("my-org/my-project", "Crash: ZeroDivisionError", "Details here").await.unwrap();
        assert_eq!(result.id, 12345);
        assert_eq!(result.iid, 42);
        assert_eq!(result.title, "Crash: ZeroDivisionError");
    }

    #[tokio::test]
    async fn test_gitlab_create_issue_unauthorized() {
        let mock_server = MockServer::start().await;
        let client = HttpGitlabClient::new(mock_server.uri(), "bad_token".to_string());

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let result = client.create_issue("my-org/my-project", "Crash", "Details").await;
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("401 Unauthorized"));
    }
}
