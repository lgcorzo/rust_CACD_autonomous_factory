use async_trait::async_trait;
use factory_core::security::nhi::{AgentSubject, EphemeralSigningKeyBytes, VerifiableCredential};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait ZitiIdentity: Send + Sync {
    async fn get_token(&self) -> anyhow::Result<String>;
    fn service_name(&self) -> String;
}

pub struct OpenZitiIdentity {
    service: String,
    identity_file: String,
}

impl OpenZitiIdentity {
    pub fn new(service: &str, identity_file: &str) -> Self {
        Self {
            service: service.to_string(),
            identity_file: identity_file.to_string(),
        }
    }
}

#[async_trait]
impl ZitiIdentity for OpenZitiIdentity {
    async fn get_token(&self) -> anyhow::Result<String> {
        tracing::info!(
            "Initializing OpenZiti mTLS context for service {} using identity {}",
            self.service,
            self.identity_file
        );

        #[cfg(feature = "production")]
        {
            // Use real ziti-sdk to load the identity and authenticate
            // Note: ziti-sdk establishes mTLS via its C CGO wrapper internally.
            // We initialize the context here to ensure the identity is valid and enrolled.
            let _ctx = ziti_sdk::Context::from_file(&self.identity_file)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to initialize OpenZiti context: {:?}", e))?;

            // In a full implementation, we'd use this Context to create Ziti TCP streams.
            // For now, returning the authenticated session identifier if available.
            // ziti-sdk Rust bindings are still stabilizing, so we ensure the file loads
            // and fallback to returning a success marker indicating mTLS is ready.
            return Ok("ziti-mtls-session-active".to_string());
        }

        #[cfg(not(feature = "production"))]
        {
            // Fallback for demonstration / local testing
            let identity_json = tokio::fs::read_to_string(&self.identity_file)
                .await
                .unwrap_or_else(|_| "{}".to_string());

            let identity_val: serde_json::Value =
                serde_json::from_str(&identity_json).unwrap_or_default();
            if let Some(token) = identity_val["id"]["token"].as_str() {
                return Ok(token.to_string());
            }

            Ok(format!("ziti-token-for-{}", self.service))
        }
    }

    fn service_name(&self) -> String {
        self.service.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct A2aHeaders {
    pub target: String,
    pub hop: u8,
    pub trace: Vec<String>,
    pub vtag_mission: Option<String>,
    pub vtag_epic: Option<String>,
    pub vtag_task: Option<String>,
    pub vtags_team: Option<String>,
    pub vtags_environment: Option<String>,
    pub vtags_cost_center: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2aDispatchRequest {
    pub skill_name: String,
    pub arguments: serde_json::Value,
    pub headers: A2aHeaders,
    pub caller_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2aDispatchResponse {
    pub status: String,
    pub result: serde_json::Value,
    #[serde(default)]
    pub trace_id: Option<String>,
}

pub struct ZitiA2aClient {
    endpoint: String,
    ziti_identity: Arc<dyn ZitiIdentity>,
    signing_key: Option<EphemeralSigningKeyBytes>,
    client: reqwest::Client,
}

impl ZitiA2aClient {
    pub fn new(endpoint: String, ziti_identity: Arc<dyn ZitiIdentity>) -> Self {
        Self {
            endpoint,
            ziti_identity,
            signing_key: None,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_signing_key(mut self, key: EphemeralSigningKeyBytes) -> Self {
        self.signing_key = Some(key);
        self
    }

    pub async fn discover_skills(&self) -> anyhow::Result<serde_json::Value> {
        let token = self.ziti_identity.get_token().await?;
        let url = format!(
            "{}/mcp/skills-manifest",
            self.endpoint.trim_end_matches('/')
        );
        let res = self
            .client
            .get(&url)
            .header("X-Ziti-Token", token)
            .send()
            .await?;
        if !res.status().is_success() {
            anyhow::bail!("A2A discover_skills failed with status: {}", res.status());
        }
        Ok(res.json().await?)
    }

    pub async fn dispatch(&self, req: &A2aDispatchRequest) -> anyhow::Result<A2aDispatchResponse> {
        if req.headers.hop > 3 {
            anyhow::bail!("A2A hop limit exceeded: {} > 3", req.headers.hop);
        }
        if req.headers.trace.iter().any(|t| t == &req.headers.target) {
            anyhow::bail!(
                "A2A cycle detected: target '{}' already in trace",
                req.headers.target
            );
        }

        let ziti_token = self.ziti_identity.get_token().await?;
        let subject = AgentSubject::new(
            req.caller_id.clone(),
            vec!["a2a-caller".to_string()],
            vec![req.headers.target.clone()],
        )
        .with_caller_skill(req.skill_name.clone());

        let mut vc = VerifiableCredential::new(
            format!("urn:uuid:{}", uuid::Uuid::new_v4()),
            format!("did:ziti:{}", self.ziti_identity.service_name()),
            subject,
        );
        if let Some(ref key) = self.signing_key {
            let signing_key = key.signing_key();
            let vm = format!("did:ziti:{}#key-1", self.ziti_identity.service_name());
            vc.sign(&signing_key, &vm)
                .map_err(|e| anyhow::anyhow!("Failed to sign A2A credential: {:?}", e))?;
        }
        let vc_json = serde_json::to_string(&vc)?;

        let url = format!("{}/mcp/a2a/dispatch", self.endpoint.trim_end_matches('/'));
        let mut http_req = self
            .client
            .post(&url)
            .header("X-Ziti-Token", ziti_token)
            .header("X-NHI-Credential", vc_json)
            .header("X-A2A-Target", &req.headers.target)
            .header("X-A2A-Hop", req.headers.hop.to_string())
            .header("X-A2A-Trace", req.headers.trace.join(","));

        if let Some(ref v) = req.headers.vtag_mission {
            http_req = http_req.header("X-Vtag-Mission", v);
        }
        if let Some(ref v) = req.headers.vtag_epic {
            http_req = http_req.header("X-Vtag-Epic", v);
        }
        if let Some(ref v) = req.headers.vtag_task {
            http_req = http_req.header("X-Vtag-Task", v);
        }
        if let Some(ref v) = req.headers.vtags_team {
            http_req = http_req.header("x-vtags-team", v);
        }
        if let Some(ref v) = req.headers.vtags_environment {
            http_req = http_req.header("x-vtags-environment", v);
        }
        if let Some(ref v) = req.headers.vtags_cost_center {
            http_req = http_req.header("x-vtags-cost-center", v);
        }

        let res = http_req.json(req).send().await?;
        if !res.status().is_success() {
            anyhow::bail!("A2A dispatch failed with status: {}", res.status());
        }
        Ok(res.json().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_open_ziti_identity_new() {
        let service = "test-service";
        let identity_file = "test-identity.json";
        let identity = OpenZitiIdentity::new(service, identity_file);

        assert_eq!(identity.service, service);
        assert_eq!(identity.identity_file, identity_file);
    }

    #[tokio::test]
    async fn test_open_ziti_identity_trait_methods() {
        let service = "test-service";
        let identity_file = "test-identity.json";
        let identity = OpenZitiIdentity::new(service, identity_file);

        assert_eq!(identity.service_name(), service);

        let token = identity.get_token().await.unwrap();
        assert_eq!(token, format!("ziti-token-for-{}", service));
    }

    #[tokio::test]
    async fn test_ziti_a2a_client_hop_limit_and_cycle_detection() {
        let identity = Arc::new(OpenZitiIdentity::new("rustant-agent", "id.json"));
        let client = ZitiA2aClient::new("http://127.0.0.1:9999".to_string(), identity);

        let over_hop = A2aDispatchRequest {
            skill_name: "security_review".to_string(),
            arguments: serde_json::json!({}),
            headers: A2aHeaders {
                target: "aethelgard".to_string(),
                hop: 4,
                trace: vec!["rustant".to_string()],
                ..Default::default()
            },
            caller_id: "rustant-1".to_string(),
        };
        let err = client.dispatch(&over_hop).await.unwrap_err();
        assert!(err.to_string().contains("hop limit exceeded"));

        let cycle = A2aDispatchRequest {
            skill_name: "security_review".to_string(),
            arguments: serde_json::json!({}),
            headers: A2aHeaders {
                target: "aethelgard".to_string(),
                hop: 2,
                trace: vec!["rustant".to_string(), "aethelgard".to_string()],
                ..Default::default()
            },
            caller_id: "rustant-1".to_string(),
        };
        let err = client.dispatch(&cycle).await.unwrap_err();
        assert!(err.to_string().contains("cycle detected"));
    }

    #[tokio::test]
    async fn test_ziti_a2a_client_dispatch_with_wiremock() {
        use wiremock::matchers::{header, header_exists, method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        let identity = Arc::new(OpenZitiIdentity::new("rustant-agent", "id.json"));
        let key = EphemeralSigningKeyBytes::new([7u8; 32]);
        let client = ZitiA2aClient::new(server.uri(), identity).with_signing_key(key);

        Mock::given(method("POST"))
            .and(path("/mcp/a2a/dispatch"))
            .and(header("X-A2A-Target", "aethelgard"))
            .and(header("X-A2A-Hop", "1"))
            .and(header("X-A2A-Trace", "rustant"))
            .and(header("X-Vtag-Mission", "m-42"))
            .and(header("x-vtags-team", "platform"))
            .and(header_exists("X-NHI-Credential"))
            .and(header_exists("X-Ziti-Token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "result": { "passed": true },
                "trace_id": "tr-1"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let req = A2aDispatchRequest {
            skill_name: "security_review".to_string(),
            arguments: serde_json::json!({ "diff": "fn main() {}" }),
            headers: A2aHeaders {
                target: "aethelgard".to_string(),
                hop: 1,
                trace: vec!["rustant".to_string()],
                vtag_mission: Some("m-42".to_string()),
                vtags_team: Some("platform".to_string()),
                ..Default::default()
            },
            caller_id: "rustant-1".to_string(),
        };
        let resp = client.dispatch(&req).await.unwrap();
        assert_eq!(resp.status, "ok");
        assert_eq!(resp.trace_id.as_deref(), Some("tr-1"));
    }
}
