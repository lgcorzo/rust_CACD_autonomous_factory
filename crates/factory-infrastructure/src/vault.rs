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

    /// Issues an ephemeral JIT token scoped to a repository path (secret/data/repos/<repo>/*) and agent ID.
    pub async fn issue_jit_token_for_repo(
        &self,
        repo: &str,
        agent_id: &str,
    ) -> factory_core::error::Result<JitToken> {
        let isolated_path = format!("secret/data/repos/{}/*", repo);
        let repo_policy = format!("repo-{}", repo.replace('/', "-"));
        let url = format!("{}/v1/auth/token/create", self.vault_addr);

        let res = self
            .client
            .post(&url)
            .header("X-Vault-Token", &self.role_token)
            .json(&json!({
                "ttl": "300s",
                "explicit_max_ttl": "300s",
                "renewable": false,
                "policies": [repo_policy],
                "meta": {
                    "audience": repo,
                    "agent_id": agent_id,
                    "isolated_path": isolated_path
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
                    // Reject if renewable - zero-trust policy strictly requires non-renewable tokens
                    if body["data"]["renewable"].as_bool().unwrap_or(false) {
                        tracing::warn!("Token is renewable; strict non-renewable required");
                        return Ok(false);
                    }
                    // Check if token has not expired
                    if let Some(expire_time_str) = body["data"]["expire_time"].as_str() {
                        if let Ok(expire_time) =
                            chrono::DateTime::parse_from_rfc3339(expire_time_str)
                        {
                            if expire_time.with_timezone(&chrono::Utc) < chrono::Utc::now() {
                                tracing::warn!("Token has expired at {}", expire_time_str);
                                return Ok(false);
                            }
                        }
                    }
                    // Check TTL is positive
                    if let Some(ttl) = body["data"]["ttl"].as_i64() {
                        if ttl <= 0 {
                            tracing::warn!("Token TTL is <= 0");
                            return Ok(false);
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
        self.issue_jit_token_for_repo(audience, "zeroclaw-worker")
            .await
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
                    "renewable": false,
                    "ttl": 300,
                    "expire_time": "2099-01-01T00:00:00Z"
                }
            })))
            .mount(&mock_server)
            .await;

        let bounds = VaultSecurityBounds::new(mock_server.uri(), "root-token".to_string());

        let token = bounds.issue_jit_token("org/my-repo").await.unwrap();
        assert_eq!(token.token, "s.testtoken123");

        let is_valid = bounds.validate_token(&token).await.unwrap();
        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_vault_expired_token_rejected() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/auth/token/lookup"))
            .and(header("X-Vault-Token", "root-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": {
                    "renewable": false,
                    "ttl": 0,
                    "expire_time": "2020-01-01T00:00:00Z"
                }
            })))
            .mount(&mock_server)
            .await;

        let bounds = VaultSecurityBounds::new(mock_server.uri(), "root-token".to_string());
        let expired_token = JitToken {
            token: "s.expired_token".to_string(),
        };

        let is_valid = bounds.validate_token(&expired_token).await.unwrap();
        assert!(!is_valid);
    }

    #[tokio::test]
    async fn test_vault_renewable_token_rejected() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/auth/token/lookup"))
            .and(header("X-Vault-Token", "root-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": {
                    "renewable": true,
                    "ttl": 300,
                    "expire_time": "2099-01-01T00:00:00Z"
                }
            })))
            .mount(&mock_server)
            .await;

        let bounds = VaultSecurityBounds::new(mock_server.uri(), "root-token".to_string());
        let renewable_token = JitToken {
            token: "s.renewable_token".to_string(),
        };

        let is_valid = bounds.validate_token(&renewable_token).await.unwrap();
        assert!(!is_valid);
    }
}
