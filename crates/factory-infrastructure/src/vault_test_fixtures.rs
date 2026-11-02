use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use serde_json::json;

pub struct MockVaultServer {
    pub server: MockServer,
}

impl MockVaultServer {
    pub async fn start() -> Self {
        let server = MockServer::start().await;
        Self { server }
    }

    pub fn uri(&self) -> String {
        self.server.uri()
    }

    /// Mounts standard token creation mock scoped to a repository
    pub async fn mock_token_create(&self, repo: &str, agent_id: &str, token_value: &str) {
        let repo_policy = format!("repo-{}", repo.replace('/', "-"));
        let isolated_path = format!("secret/data/repos/{}/*", repo);

        Mock::given(method("POST"))
            .and(path("/v1/auth/token/create"))
            .and(header("X-Vault-Token", "valid-role-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "auth": {
                    "client_token": token_value,
                    "policies": [repo_policy],
                    "metadata": {
                        "audience": repo,
                        "agent_id": agent_id,
                        "isolated_path": isolated_path
                    },
                    "lease_duration": 300,
                    "renewable": false
                }
            })))
            .mount(&self.server)
            .await;
    }

    /// Mounts token lookup mock (validate_token)
    pub async fn mock_token_lookup(
        &self,
        token: &str,
        repo: &str,
        agent_id: &str,
        is_renewable: bool,
        ttl: i64,
        status_code: u16,
    ) {
        let repo_policy = format!("repo-{}", repo.replace('/', "-"));
        let isolated_path = format!("secret/data/repos/{}/*", repo);

        if status_code == 200 {
            Mock::given(method("GET"))
                .and(path("/v1/auth/token/lookup-self"))
                .and(header("X-Vault-Token", token))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "data": {
                        "policies": [repo_policy],
                        "meta": {
                            "audience": repo,
                            "agent_id": agent_id,
                            "isolated_path": isolated_path
                        },
                        "renewable": is_renewable,
                        "ttl": ttl,
                        "expire_time": "2026-11-02T12:00:00Z"
                    }
                })))
                .mount(&self.server)
                .await;
        } else {
            Mock::given(method("GET"))
                .and(path("/v1/auth/token/lookup-self"))
                .and(header("X-Vault-Token", token))
                .respond_with(ResponseTemplate::new(status_code).set_body_json(json!({
                    "errors": ["permission denied"]
                })))
                .mount(&self.server)
                .await;
        }
    }

    /// Mounts secret path access mock with simulated ACL enforcement
    pub async fn mock_secret_read(
        &self,
        token: &str,
        secret_path: &str,
        allowed_prefix: &str,
        secret_payload: serde_json::Value,
    ) {
        let full_path = format!("/v1/{}", secret_path);
        if secret_path.starts_with(allowed_prefix) {
            Mock::given(method("GET"))
                .and(path(full_path))
                .and(header("X-Vault-Token", token))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "data": {
                        "data": secret_payload
                    }
                })))
                .mount(&self.server)
                .await;
        } else {
            Mock::given(method("GET"))
                .and(path(full_path))
                .and(header("X-Vault-Token", token))
                .respond_with(ResponseTemplate::new(403).set_body_json(json!({
                    "errors": ["1 error occurred: permission denied by Vault ACL policy"]
                })))
                .mount(&self.server)
                .await;
        }
    }
}

