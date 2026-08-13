use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubIssue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub html_url: String,
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait GithubClient: Send + Sync {
    async fn create_issue(
        &self,
        repo: &str,
        title: &str,
        body: &str,
    ) -> anyhow::Result<GithubIssue>;

    async fn list_open_issues(
        &self,
        repo: &str,
        labels: Option<String>,
    ) -> anyhow::Result<Vec<GithubIssue>>;

    async fn create_pull_request(
        &self,
        repo: &str,
        title: &str,
        head: &str,
        base: &str,
        body: &str,
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
    ) -> anyhow::Result<GithubIssue> {
        let url = format!(
            "{}/repos/{}/issues",
            self.api_url.trim_end_matches('/'),
            repo
        );
        let payload = serde_json::json!({
            "title": title,
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
        let mut url = format!(
            "{}/repos/{}/issues?state=open",
            self.api_url.trim_end_matches('/'),
            repo
        );
        if let Some(lbl) = labels {
            url.push_str("&labels=");
            url.push_str(&urlencoding::encode(&lbl));
        }

        let mut req = self.client.get(&url);
        if !self.api_token.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_token));
            req = req.header("Accept", "application/vnd.github.v3+json");
        }

        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status();
            tracing::error!("GitHub list open issues failed with status {}", status);
            anyhow::bail!("GitHub list open issues failed with status {}", status);
        }

        let issues: Vec<GithubIssue> = res.json().await?;
        Ok(issues)
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
    async fn test_github_create_pull_request_success() {
        let mock_server = MockServer::start().await;

        let response = ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "html_url": "https://github.com/my-org/my-repo/pull/12"
        }));

        Mock::given(method("POST"))
            .and(path("/repos/my-org/my-repo/pulls"))
            .and(header("Authorization", "Bearer test-token"))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let client = HttpGithubClient::with_url(mock_server.uri(), "test-token".to_string());
        let pr_url = client
            .create_pull_request(
                "my-org/my-repo",
                "feat: autonomous mission completion",
                "feature/mission-1",
                "main",
                "PR description",
            )
            .await
            .unwrap();

        assert_eq!(pr_url, "https://github.com/my-org/my-repo/pull/12");
    }
}
