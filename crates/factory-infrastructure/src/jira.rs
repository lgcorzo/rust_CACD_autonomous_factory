use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait JiraClient: Send + Sync {
    async fn search_issues(&self, query: &str) -> anyhow::Result<String>;
}

pub struct HttpJiraClient {
    url: String,
    username: String,
    api_token: String,
    client: reqwest::Client,
}

impl HttpJiraClient {
    pub fn new(url: String, username: String, api_token: String) -> Self {
        Self {
            url,
            username,
            api_token,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl JiraClient for HttpJiraClient {
    async fn search_issues(&self, query: &str) -> anyhow::Result<String> {
        let sanitized_query = query.replace('\\', "\\\\").replace('"', "\\\"");
        let jql = format!(
            "summary ~ \"{}\" OR description ~ \"{}\"",
            sanitized_query, sanitized_query
        );

        let search_url = format!("{}/rest/api/2/search", self.url.trim_end_matches('/'));
        let res = self
            .client
            .get(&search_url)
            .basic_auth(&self.username, Some(&self.api_token))
            .query(&[("jql", &jql)])
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            anyhow::bail!("Jira search failed with status {}. Body: {}", status, body);
        }

        let data: serde_json::Value = res.json().await?;
        let issues = data["issues"].as_array();

        match issues {
            Some(list) if !list.is_empty() => {
                let res_list: Vec<String> = list
                    .iter()
                    .map(|issue| {
                        let key = issue["key"].as_str().unwrap_or("UNKNOWN");
                        let summary = issue["fields"]["summary"].as_str().unwrap_or("No summary");
                        format!("[{}] {}", key, summary)
                    })
                    .collect();
                Ok(res_list.join("\n"))
            }
            _ => Ok("No se encontraron resultados en Jira.".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_jira_search_success() {
        let mock_server = MockServer::start().await;
        let client =
            HttpJiraClient::new(mock_server.uri(), "user".to_string(), "token".to_string());

        let response_body = json!({
            "issues": [
                {
                    "key": "PROJ-123",
                    "fields": {
                        "summary": "Test issue"
                    }
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/rest/api/2/search"))
            .and(query_param(
                "jql",
                "summary ~ \"test\" OR description ~ \"test\"",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let result = client.search_issues("test").await.unwrap();
        assert_eq!(result, "[PROJ-123] Test issue");
    }

    #[tokio::test]
    async fn test_jira_search_no_results() {
        let mock_server = MockServer::start().await;
        let client =
            HttpJiraClient::new(mock_server.uri(), "user".to_string(), "token".to_string());

        let response_body = json!({
            "issues": []
        });

        Mock::given(method("GET"))
            .and(path("/rest/api/2/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let result = client.search_issues("test").await.unwrap();
        assert_eq!(result, "No se encontraron resultados en Jira.");
    }

    #[tokio::test]
    async fn test_jira_search_unauthorized() {
        let mock_server = MockServer::start().await;
        let client =
            HttpJiraClient::new(mock_server.uri(), "user".to_string(), "token".to_string());

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(401).set_body_string("Unauthorized"))
            .mount(&mock_server)
            .await;

        let result = client.search_issues("test").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("401 Unauthorized"));
    }

    #[tokio::test]
    async fn test_jira_search_server_error() {
        let mock_server = MockServer::start().await;
        let client =
            HttpJiraClient::new(mock_server.uri(), "user".to_string(), "token".to_string());

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Server Error"))
            .mount(&mock_server)
            .await;

        let result = client.search_issues("test").await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("500 Internal Server Error"));
    }
}
