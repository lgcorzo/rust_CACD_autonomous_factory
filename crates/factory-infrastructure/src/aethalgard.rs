use async_trait::async_trait;
use serde_json::json;

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait AethalgardClient: Send + Sync {
    async fn notify_remediation(&self, mission_id: &str, error_details: &str) -> anyhow::Result<()>;
}

pub struct HttpAethalgardClient {
    webhook_url: String,
    client: reqwest::Client,
}

impl HttpAethalgardClient {
    pub fn new(webhook_url: String) -> Self {
        Self {
            webhook_url,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl AethalgardClient for HttpAethalgardClient {
    async fn notify_remediation(&self, mission_id: &str, error_details: &str) -> anyhow::Result<()> {
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
        
        let res = self.client.post(&self.webhook_url)
            .json(&payload)
            .send()
            .await?;
            
        if !res.status().is_success() {
            anyhow::bail!("Aethalgard webhook failed with status: {}", res.status());
        }
        Ok(())
    }
}
