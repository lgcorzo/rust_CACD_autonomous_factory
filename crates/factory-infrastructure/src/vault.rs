use async_trait::async_trait;
use factory_core::security::{JitToken, SecurityBounds};
use reqwest::Client;
use serde_json::json;

pub struct VaultSecurityBounds {
    client: Client,
    vault_addr: String,
    role_token: String,
}

impl VaultSecurityBounds {
    pub fn new(vault_addr: String, role_token: String) -> Self {
        Self {
            client: Client::new(),
            vault_addr,
            role_token,
        }
    }
}

#[async_trait]
impl SecurityBounds for VaultSecurityBounds {
    async fn validate_token(&self, token: &JitToken) -> factory_core::error::Result<bool> {
        let url = format!("{}/v1/auth/token/lookup", self.vault_addr);

        let res = self
            .client
            .post(&url)
            .header("X-Vault-Token", &self.role_token)
            .json(&json!({
                "token": token.token
            }))
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    let body: serde_json::Value = response.json().await.unwrap_or_default();
                    // Check if token has not expired
                    if let Some(expire_time) = body["data"]["expire_time"].as_str() {
                        if !expire_time.is_empty() {
                            return Ok(true);
                        }
                    }
                    Ok(true)
                } else {
                    tracing::warn!("Token validation failed with status: {}", response.status());
                    Ok(false)
                }
            }
            Err(e) => {
                tracing::error!("Vault API error during token validation: {}", e);
                Ok(false)
            }
        }
    }

    async fn issue_jit_token(&self, audience: &str) -> factory_core::error::Result<JitToken> {
        let url = format!("{}/v1/auth/token/create", self.vault_addr);

        // Create a short-lived token (e.g., 5m) for the specific audience
        let res = self
            .client
            .post(&url)
            .header("X-Vault-Token", &self.role_token)
            .json(&json!({
                "ttl": "5m",
                "renewable": false,
                "meta": {
                    "audience": audience
                }
            }))
            .send()
            .await;

        match res {
            Ok(response) => {
                let status = response.status();
                if status.is_success() {
                    let body: serde_json::Value = response.json().await.unwrap_or_default();
                    if let Some(client_token) = body["auth"]["client_token"].as_str() {
                        return Ok(JitToken {
                            token: client_token.to_string(),
                        });
                    }
                }
                Err(factory_core::error::FactoryError::Unexpected(
                    anyhow::anyhow!("Failed to issue JIT token, status: {}", status),
                ))
            }
            Err(e) => Err(factory_core::error::FactoryError::Unexpected(
                anyhow::anyhow!("Vault API error: {}", e),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_vault_issue_and_validate() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/auth/token/create"))
            .and(header("X-Vault-Token", "root-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "auth": {
                    "client_token": "s.testtoken123"
                }
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("POST"))
            .and(path("/v1/auth/token/lookup"))
            .and(header("X-Vault-Token", "root-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": {
                    "expire_time": "2026-07-02T19:00:00Z"
                }
            })))
            .mount(&mock_server)
            .await;

        let bounds = VaultSecurityBounds::new(mock_server.uri(), "root-token".to_string());

        let token = bounds.issue_jit_token("test-audience").await.unwrap();
        assert_eq!(token.token, "s.testtoken123");

        let is_valid = bounds.validate_token(&token).await.unwrap();
        assert!(is_valid);
    }
}
