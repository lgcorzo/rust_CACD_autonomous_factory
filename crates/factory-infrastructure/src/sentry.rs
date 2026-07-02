use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrashEvent {
    pub event_id: String,
    pub title: String,
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
        let search_url = format!(
            "{}/api/0/projects/{}/events/",
            self.url.trim_end_matches('/'),
            project
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
            tracing::error!("Sentry fetch failed with status {}", status);
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
                "title": "ZeroDivisionError",
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
        assert_eq!(result[0].title, "ZeroDivisionError");
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
}
