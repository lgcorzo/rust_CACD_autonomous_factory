use crate::Agent;
use async_trait::async_trait;
use factory_core::FinOpsTag;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

pub struct FinOpsAgent {
    litellm_base_url: String,
    api_key: String,
    client: Client,
    tag: FinOpsTag,
}

impl Default for FinOpsAgent {
    fn default() -> Self {
        let tag = FinOpsTag {
            cost_center: std::env::var("FINOPS_COST_CENTER")
                .unwrap_or_else(|_| "engineering".to_string()),
            project_code: std::env::var("FINOPS_PROJECT_CODE")
                .unwrap_or_else(|_| "dg-factory".to_string()),
            owner: std::env::var("FINOPS_OWNER").unwrap_or_else(|_| "ai-agent".to_string()),
        };
        Self::new(
            std::env::var("LITELLM_BASE_URL").unwrap_or_default(),
            std::env::var("LITELLM_API_KEY").unwrap_or_default(),
            tag,
        )
    }
}

impl FinOpsAgent {
    pub fn new(litellm_base_url: String, api_key: String, tag: FinOpsTag) -> Self {
        Self {
            litellm_base_url,
            api_key,
            client: Client::new(),
            tag,
        }
    }

    pub async fn monitor_budget(&self) -> anyhow::Result<()> {
        if self.litellm_base_url.is_empty() {
            tracing::warn!("FinOpsAgent: LITELLM_BASE_URL is not set, monitoring disabled.");
            return Ok(());
        }

        let url = format!(
            "{}/v1/spend/logs",
            self.litellm_base_url.trim_end_matches('/')
        );

        loop {
            tracing::info!(
                "FinOpsAgent: Checking spend for project {}",
                self.tag.project_code
            );

            let response = self
                .client
                .get(&url)
                .bearer_auth(&self.api_key)
                .query(&[("project_code", &self.tag.project_code)])
                .send()
                .await;

            match response {
                Ok(resp) => {
                    if let Ok(json) = resp.json::<Value>().await {
                        // Attempt to read total_spend from LiteLLM spend response
                        if let Some(spend) = json.get("total_spend").and_then(|v| v.as_f64()) {
                            if spend > 10.0 {
                                tracing::error!("ANOMALY DETECTED! High token spend: ${}", spend);
                            } else {
                                tracing::info!("Current spend: ${}. Budget is healthy.", spend);
                            }
                        } else {
                            tracing::warn!(
                                "FinOpsAgent: Could not parse total_spend from LiteLLM response."
                            );
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(
                        "FinOpsAgent: Failed to fetch spend logs from LiteLLM: {}",
                        e
                    );
                }
            }

            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }
}

#[async_trait]
impl Agent for FinOpsAgent {
    fn name(&self) -> String {
        "FinOpsAgent".to_string()
    }

    async fn execute(&self, _task_description: &str) -> anyhow::Result<Value> {
        self.monitor_budget().await?;
        Ok(serde_json::json!({ "status": "monitoring_stopped" }))
    }
}
