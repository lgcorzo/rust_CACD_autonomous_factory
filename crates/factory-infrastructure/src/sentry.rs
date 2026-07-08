use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrashEvent {
    pub event_id: String,
    pub project: String,
    pub message: String,
    pub level: String,
    pub timestamp: String,
    pub culprit: Option<String>,
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait SentryClient: Send + Sync {
    async fn fetch_recent_crashes(
        &self,
        project: &str,
        since_minutes: u64,
    ) -> anyhow::Result<Vec<CrashEvent>>;
}

pub struct HttpSentryClient {
    url: String,
    api_token: String,
    client: reqwest::Client,
}

impl HttpSentryClient {
    pub fn new(url: String, api_token: String) -> Self {
        Self {
            url,
            api_token,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl SentryClient for HttpSentryClient {
    async fn fetch_recent_crashes(
        &self,
        project: &str,
        since_minutes: u64,
    ) -> anyhow::Result<Vec<CrashEvent>> {
        let stats_period = format!("{}m", since_minutes);

        // Sentry API requires org-slug/project-slug in the path.
        // If the project string already contains '/', use it as-is.
        // Otherwise, prepend the org slug from SENTRY_ORG env var.
        let project_path = if project.contains('/') {
            project.to_string()
        } else {
            let org = std::env::var("SENTRY_ORG").unwrap_or_else(|_| "sentry".to_string());
            format!("{}/{}", org, project)
        };

        let search_url = format!(
            "{}/api/0/projects/{}/events/",
            self.url.trim_end_matches('/'),
            project_path
        );

        let res = self
            .client
            .get(&search_url)
            .bearer_auth(&self.api_token)
            .query(&[("statsPeriod", &stats_period)])
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            tracing::error!(
                "Sentry fetch failed with status {} (url: {})",
                status,
                search_url
            );
            anyhow::bail!("Sentry fetch failed with status {}", status);
        }

        let events: Vec<CrashEvent> = res.json().await?;
        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_sentry_fetch_success() {
        let mock_server = MockServer::start().await;
        let client = HttpSentryClient::new(mock_server.uri(), "test_token".to_string());

        let response_body = json!([
            {
                "event_id": "a1b2c3d4",
                "project": "my-project",
                "message": "ZeroDivisionError",
                "level": "error",
                "timestamp": "2026-07-05T09:00:00Z",
                "culprit": "main.py"
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/0/projects/my-org/my-project/events/"))
            .and(query_param("statsPeriod", "15m"))
            .and(header("Authorization", "Bearer test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let result = client
            .fetch_recent_crashes("my-org/my-project", 15)
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].event_id, "a1b2c3d4");
        assert_eq!(result[0].message, "ZeroDivisionError");
        assert_eq!(result[0].level, "error");
    }

    #[tokio::test]
    async fn test_sentry_fetch_unauthorized() {
        let mock_server = MockServer::start().await;
        let client = HttpSentryClient::new(mock_server.uri(), "bad_token".to_string());

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let result = client.fetch_recent_crashes("my-org/my-project", 15).await;
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("401 Unauthorized"));
    }

    #[tokio::test]
    async fn test_sentry_fetch_prepends_org_slug() {
        // When project doesn't contain '/', the org slug should be prepended.
        // We set SENTRY_ORG env var for this test.
        std::env::set_var("SENTRY_ORG", "test-org");

        let mock_server = MockServer::start().await;
        let client = HttpSentryClient::new(mock_server.uri(), "token".to_string());

        let response_body = json!([]);

        Mock::given(method("GET"))
            .and(path("/api/0/projects/test-org/my-project/events/"))
            .and(query_param("statsPeriod", "10m"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let result = client.fetch_recent_crashes("my-project", 10).await.unwrap();
        assert_eq!(result.len(), 0);

        // Clean up env var
        std::env::remove_var("SENTRY_ORG");
    }
}
