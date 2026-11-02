use async_trait::async_trait;
use factory_core::security::{JitToken, SecurityBounds};
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

#[derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop)]
pub struct VaultRoleToken {
    token: String,
}

impl VaultRoleToken {
    pub fn new(token: impl Into<String>) -> Self {
        Self { token: token.into() }
    }

    pub fn expose(&self) -> &str {
        &self.token
    }
}

impl std::fmt::Debug for VaultRoleToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VaultRoleToken")
            .field("token", &"[REDACTED_ROLE_TOKEN]")
            .finish()
    }
}

pub struct VaultSecurityBounds {
    client: Client,
    vault_addr: String,
    role_token: VaultRoleToken,
}

impl VaultSecurityBounds {
    pub fn new(vault_addr: String, role_token: impl Into<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(3))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            client,
            vault_addr,
            role_token: VaultRoleToken::new(role_token),
        }
    }

    /// Read secret from Vault path using the supplied JIT token, enforcing Vault's HTTP status & ACL
    pub async fn read_secret(
        &self,
        token: &JitToken,
        secret_path: &str,
    ) -> factory_core::error::Result<serde_json::Value> {
        let clean_path = secret_path.trim_start_matches('/');
        let url = format!("{}/v1/{}", self.vault_addr, clean_path);

        let res = self
            .client
            .get(&url)
            .header("X-Vault-Token", token.expose_token())
            .send()
            .await
            .map_err(|e| {
                factory_core::error::FactoryError::Unexpected(anyhow::anyhow!("Vault request failed: {}", e))
            })?;

        let status = res.status();
        if status.is_success() {
            let body: serde_json::Value = res.json().await.map_err(|e| {
                factory_core::error::FactoryError::Unexpected(anyhow::anyhow!("Invalid json response: {}", e))
            })?;
            Ok(body)
        } else if status == reqwest::StatusCode::FORBIDDEN {
            Err(factory_core::error::FactoryError::Security(
                "Access denied by Vault ACL policy".to_string(),
            ))
        } else {
            Err(factory_core::error::FactoryError::Unexpected(anyhow::anyhow!(
                "Vault returned status {}",
                status
            )))
        }
    }

    /// Issues an ephemeral JIT token scoped to a repository path (secret/data/repos/<repo>/*) and agent ID.
    /// Validates token and checks authorization claims against expected repository policy, audience, and agent ID
    pub async fn validate_token_with_claims(
        &self,
        token: &JitToken,
        expected_repo: Option<&str>,
        expected_agent_id: Option<&str>,
    ) -> factory_core::error::Result<bool> {
        // Support both /lookup (called with role token) and /lookup-self (called with client token)
        let lookup_url = format!("{}/v1/auth/token/lookup-self", self.vault_addr);

        let mut res = self
            .client
            .get(&lookup_url)
            .header("X-Vault-Token", token.expose_token())
            .send()
            .await;

        // Fallback to /v1/auth/token/lookup if /lookup-self is 404
        if let Ok(ref resp) = res {
            if resp.status() == reqwest::StatusCode::NOT_FOUND {
                let post_url = format!("{}/v1/auth/token/lookup", self.vault_addr);
                res = self
                    .client
                    .post(&post_url)
                    .header("X-Vault-Token", self.role_token.expose())
                    .json(&json!({
                        "token": token.expose_token()
                    }))
                    .send()
                    .await;
            }
        }

        match res {
            Ok(response) => {
                if !response.status().is_success() {
                    tracing::warn!("Token validation failed with status: {}", response.status());
                    return Ok(false);
                }
                let body: serde_json::Value = response.json().await.unwrap_or_default();
                let data = &body["data"];

                // 1. Strictly non-renewable
                if data["renewable"].as_bool().unwrap_or(false) {
                    tracing::warn!("Token is renewable; strict non-renewable required");
                    return Ok(false);
                }

                // 2. Check expiration
                if let Some(expire_time_str) = data["expire_time"].as_str() {
                    if let Ok(expire_time) = chrono::DateTime::parse_from_rfc3339(expire_time_str) {
                        if expire_time.with_timezone(&chrono::Utc) < chrono::Utc::now() {
                            tracing::warn!("Token has expired at {}", expire_time_str);
                            return Ok(false);
                        }
                    }
                }

                // 3. Positive TTL
                if let Some(ttl) = data["ttl"].as_i64() {
                    if ttl <= 0 {
                        tracing::warn!("Token TTL is <= 0");
                        return Ok(false);
                    }
                }

                // 4. Policy enforcement check
                if let Some(repo) = expected_repo {
                    let expected_policy = format!("repo-{}", repo.replace('/', "-"));
                    let policies: Vec<String> = data["policies"]
                        .as_array()
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                        .unwrap_or_default();

                    if !policies.contains(&expected_policy) {
                        tracing::warn!(
                            "Token missing required repository policy: {} (found: {:?})",
                            expected_policy,
                            policies
                        );
                        return Ok(false);
                    }

                    // 5. Audience check in meta
                    if let Some(aud) = data["meta"]["audience"].as_str() {
                        if aud != repo {
                            tracing::warn!("Token audience mismatch: {} != {}", aud, repo);
                            return Ok(false);
                        }
                    } else {
                        tracing::warn!("Token metadata missing audience claim");
                        return Ok(false);
                    }
                }

                // 6. Agent ID check in meta
                if let Some(agent_id) = expected_agent_id {
                    if let Some(claimed_agent) = data["meta"]["agent_id"].as_str() {
                        if claimed_agent != agent_id {
                            tracing::warn!("Token agent_id mismatch: {} != {}", claimed_agent, agent_id);
                            return Ok(false);
                        }
                    } else {
                        tracing::warn!("Token metadata missing agent_id claim");
                        return Ok(false);
                    }
                }

                Ok(true)
            }
            Err(e) => {
                tracing::error!("Vault API error during token validation: {}", e);
                Ok(false)
            }
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
            .header("X-Vault-Token", self.role_token.expose())
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
                        return Ok(JitToken::new(client_token));
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
        self.validate_token_with_claims(token, None, None).await
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
        assert_eq!(token.expose_token(), "s.testtoken123");

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
        let expired_token = JitToken::new("s.expired_token");

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
        let renewable_token = JitToken::new("s.renewable_token");

        let is_valid = bounds.validate_token(&renewable_token).await.unwrap();
        assert!(!is_valid);
    }
}
