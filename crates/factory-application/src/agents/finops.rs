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
            team: std::env::var("FINOPS_TEAM").unwrap_or_else(|_| "dark-gravity-ops".to_string()),
            epic: std::env::var("FINOPS_EPIC").unwrap_or_else(|_| "E6.3".to_string()),
            microservice: std::env::var("FINOPS_MICROSERVICE")
                .unwrap_or_else(|_| "factory-application".to_string()),
            environment: std::env::var("FINOPS_ENVIRONMENT")
                .unwrap_or_else(|_| "staging".to_string()),
            cost_center: std::env::var("FINOPS_COST_CENTER")
                .unwrap_or_else(|_| "eu-rd-grants".to_string()),
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

        let mut previous_spend: f64 = 0.0;

        loop {
            tracing::info!("FinOpsAgent: Checking spend for epic {}", self.tag.epic);

            let response = self
                .client
                .get(&url)
                .bearer_auth(&self.api_key)
                .query(&[("epic", &self.tag.epic)])
                .send()
                .await;

            match response {
                Ok(resp) => {
                    if let Ok(json) = resp.json::<Value>().await {
                        // Attempt to read total_spend from LiteLLM spend response
                        if let Some(spend) = json.get("total_spend").and_then(|v| v.as_f64()) {
                            // Preventative Anomaly Detection: check velocity
                            let spend_velocity = spend - previous_spend;
                            if spend_velocity > 1.0 {
                                tracing::error!(
                                    "ANOMALY DETECTED! High token spend velocity: +${} in 60s! Total: ${}",
                                    spend_velocity,
                                    spend
                                );
                            } else if spend > 10.0 {
                                tracing::error!("LIMIT REACHED! High token spend: ${}", spend);
                            } else {
                                tracing::info!(
                                    "Current spend: ${}. Velocity: +${}. Budget is healthy.",
                                    spend,
                                    spend_velocity
                                );
                            }
                            previous_spend = spend;
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
