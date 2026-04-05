use async_trait::async_trait;
use serde_json::json;
use crate::R2rClient;

pub struct HttpR2rClient {
    url: String,
    user: String,
    pwd: String,
    client: reqwest::Client,
}

impl HttpR2rClient {
    pub fn new(url: String, user: String, pwd: String) -> Self {
        Self {
            url,
            user,
            pwd,
            client: reqwest::Client::new(),
        }
    }

    async fn get_token(&self) -> anyhow::Result<String> {
        let login_url = format!("{}/v3/users/login", self.url.trim_end_matches('/'));
        let login_res = self.client
            .post(&login_url)
            .form(&[("username", &self.user), ("password", &self.pwd)])
            .send()
            .await?;

        let login_status = login_res.status();
        let login_body = login_res.text().await?;

        if !login_status.is_success() {
            anyhow::bail!(
                "R2R login failed with status {}. Body: {}",
                login_status,
                login_body
            );
        }

        let login_data: serde_json::Value = serde_json::from_str(&login_body).map_err(|e| {
            anyhow::anyhow!(
                "Failed to decode R2R login response: {}. Body: {}",
                e,
                login_body
            )
        })?;

        let token = login_data["results"]["access_token"]
            .as_str()
            .or_else(|| login_data["results"]["access_token"]["token"].as_str())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Failed to retrieve access token from R2R. Response was: {}",
                    login_data
                )
            })?;

        Ok(token.to_string())
    }
}

#[async_trait]
impl R2rClient for HttpR2rClient {
    async fn search(&self, query: &str) -> anyhow::Result<String> {
        let token = self.get_token().await?;

        let search_url = format!("{}/v3/retrieval/search", self.url.trim_end_matches('/'));
        let search_res = self.client
            .post(&search_url)
            .bearer_auth(token)
            .json(&json!({
                "query": query,
                "stream": false,
                "search_settings": {
                    "use_vector_search": true,
                    "search_filters": {},
                    "search_limit": 3
                }
            }))
            .send()
            .await?;

        let status = search_res.status();
        let body_text = search_res.text().await?;

        if !status.is_success() {
            anyhow::bail!(
                "R2R search query failed with status {}. Body: {}",
                status,
                body_text
            );
        }

        let search_data: serde_json::Value = serde_json::from_str(&body_text).map_err(|e| {
            anyhow::anyhow!(
                "Failed to decode R2R search response: {}. Body: {}",
                e,
                body_text
            )
        })?;

        let mut combined_results = String::new();
        if let Some(chunks) = search_data["results"]["chunk_search_results"].as_array() {
            for chunk in chunks {
                if let Some(text) = chunk["text"].as_str() {
                    if !combined_results.is_empty() {
                        combined_results.push_str("\n\n---\n\n");
                    }
                    combined_results.push_str(text);
                }
            }
        }

        if combined_results.is_empty() {
            combined_results = "No internal r2r result found".to_string();
        }

        Ok(combined_results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use serde_json::json;

    #[tokio::test]
    async fn test_r2r_search_success() {
        let mock_server = MockServer::start().await;
        let client = HttpR2rClient::new(
            mock_server.uri(),
            "admin".to_string(),
            "admin".to_string(),
        );

        // Mock login
        Mock::given(method("POST"))
            .and(path("/v3/users/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "results": {
                    "access_token": "mock_token"
                }
            })))
            .mount(&mock_server)
            .await;

        // Mock search
        let search_response = json!({
            "results": {
                "chunk_search_results": [
                    {
                        "text": "Pattern 1 content"
                    },
                    {
                        "text": "Pattern 2 content"
                    }
                ]
            }
        });

        Mock::given(method("POST"))
            .and(path("/v3/retrieval/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(search_response))
            .mount(&mock_server)
            .await;

        let result = client.search("test").await.unwrap();
        assert!(result.contains("Pattern 1 content"));
        assert!(result.contains("Pattern 2 content"));
        assert!(result.contains("---"));
    }

    #[tokio::test]
    async fn test_r2r_login_failure() {
        let mock_server = MockServer::start().await;
        let client = HttpR2rClient::new(
            mock_server.uri(),
            "admin".to_string(),
            "admin".to_string(),
        );

        Mock::given(method("POST"))
            .and(path("/v3/users/login"))
            .respond_with(ResponseTemplate::new(401).set_body_string("Invalid credentials"))
            .mount(&mock_server)
            .await;

        let result = client.search("test").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("401 Unauthorized"));
    }

    #[tokio::test]
    async fn test_r2r_search_failure_after_login() {
        let mock_server = MockServer::start().await;
        let client = HttpR2rClient::new(
            mock_server.uri(),
            "admin".to_string(),
            "admin".to_string(),
        );

        // Mock login
        Mock::given(method("POST"))
            .and(path("/v3/users/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "results": { "access_token": "mock_token" }
            })))
            .mount(&mock_server)
            .await;

        // Mock search failure
        Mock::given(method("POST"))
            .and(path("/v3/retrieval/search"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal error"))
            .mount(&mock_server)
            .await;

        let result = client.search("test").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("500 Internal Server Error"));
    }
}
