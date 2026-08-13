use crate::semantica::SemanticaClient;
use async_trait::async_trait;
use serde_json::json;

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait AethalgardClient: Send + Sync {
    async fn notify_remediation(&self, mission_id: &str, error_details: &str)
        -> anyhow::Result<()>;
    async fn verify_causal_provenance(&self, patch_id: &str) -> anyhow::Result<bool>;
}

pub struct HttpAethalgardClient {
    webhook_url: String,
    semantica_endpoint: Option<String>,
    client: reqwest::Client,
}

impl HttpAethalgardClient {
    pub fn new(webhook_url: String) -> Self {
        Self {
            webhook_url,
            semantica_endpoint: std::env::var("SEMANTICA_ENDPOINT").ok(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_semantica_endpoint(mut self, endpoint: String) -> Self {
        self.semantica_endpoint = Some(endpoint);
        self
    }
}

#[async_trait]
impl AethalgardClient for HttpAethalgardClient {
    async fn notify_remediation(
        &self,
        mission_id: &str,
        error_details: &str,
    ) -> anyhow::Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "method": "notify_remediation",
            "params": {
                "mission_id": mission_id,
                "error": error_details,
                "source": "dark-gravity-factory"
            },
            "id": uuid::Uuid::new_v4().to_string()
        });

        let res = self
            .client
            .post(&self.webhook_url)
            .json(&payload)
            .send()
            .await?;

        if !res.status().is_success() {
            anyhow::bail!("Aethalgard webhook failed with status: {}", res.status());
        }
        Ok(())
    }

    async fn verify_causal_provenance(&self, patch_id: &str) -> anyhow::Result<bool> {
        let endpoint = match &self.semantica_endpoint {
            Some(ep) => ep,
            None => return Ok(true),
        };
        let semantica_client = crate::HttpSemanticaClient::new(endpoint.clone(), None);
        let report = semantica_client.verify_provenance(patch_id).await?;
        if !report.is_valid || !report.policy_violations.is_empty() {
            tracing::warn!(
                "Causal provenance verification failed for patch {}: violations={:?}",
                patch_id,
                report.policy_violations
            );
            return Ok(false);
        }
        Ok(true)
    }
}
