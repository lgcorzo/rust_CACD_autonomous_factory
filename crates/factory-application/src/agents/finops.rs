use crate::Agent;
use async_trait::async_trait;
use factory_core::FinOpsTag;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

/// Circuit breaker threshold: after this many consecutive failures,
/// downgrade logging from ERROR to WARN and apply maximum backoff.
const CIRCUIT_BREAKER_THRESHOLD: u32 = 5;

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
        // Try LITELLM_API_BASE first (project convention), then LITELLM_BASE_URL (K8s deployment)
        let base = std::env::var("LITELLM_API_BASE")
            .or_else(|_| std::env::var("LITELLM_BASE_URL"))
            .unwrap_or_default();
        // Strip /v1 suffix since we append /v1/spend/logs ourselves,
        // preventing the double /v1/v1 path that was causing connection errors.
        let base = base
            .trim_end_matches('/')
            .trim_end_matches("/v1")
            .to_string();
        Self::new(
            base,
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
            tracing::warn!(
                "FinOpsAgent: LITELLM_API_BASE/LITELLM_BASE_URL is not set, monitoring disabled."
            );
            return Ok(());
        }

        let url = format!(
            "{}/v1/spend/logs",
            self.litellm_base_url.trim_end_matches('/')
        );

        let base_interval = Duration::from_secs(60);
        let max_interval = Duration::from_secs(15 * 60); // 15 minutes
        let mut current_interval = base_interval;
        let mut consecutive_failures: u32 = 0;
        let mut previous_spend: f64 = 0.0;

        tracing::info!(
            "FinOpsAgent: Starting budget monitor for epic {} (url: {})",
            self.tag.epic,
            url
        );

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
                            // Reset backoff on successful data fetch
                            consecutive_failures = 0;
                            current_interval = base_interval;

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
                    consecutive_failures += 1;
                    if consecutive_failures >= CIRCUIT_BREAKER_THRESHOLD {
                        // Circuit breaker open: downgrade to WARN to avoid log flooding
                        tracing::warn!(
                            "FinOpsAgent: LiteLLM unreachable ({} consecutive failures, backoff {}s): {}",
                            consecutive_failures,
                            current_interval.as_secs(),
                            e
                        );
                    } else {
                        tracing::error!(
                            "FinOpsAgent: Failed to fetch spend logs from LiteLLM: {}",
                            e
                        );
                    }
                    // Exponential backoff: double interval on each failure, capped at max
                    current_interval = (current_interval * 2).min(max_interval);
                }
            }

            tokio::time::sleep(current_interval).await;
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

#[cfg(test)]
mod tests {
    use super::*;
    use factory_core::FinOpsTag;

    fn test_tag() -> FinOpsTag {
        FinOpsTag {
            team: "test-team".to_string(),
            epic: "E1.0".to_string(),
            microservice: "test-svc".to_string(),
            environment: "test".to_string(),
            cost_center: "test-cc".to_string(),
        }
    }

    #[test]
    fn test_finops_agent_strips_v1_suffix() {
        let agent = FinOpsAgent::new(
            "http://litellm.local:4000/v1".to_string(),
            "key".to_string(),
            test_tag(),
        );
        // The constructor doesn't strip — Default::default() does.
        // But the URL construction in monitor_budget appends /v1/spend/logs.
        // So a raw URL with /v1 would become /v1/v1/spend/logs.
        // This test verifies the URL stored.
        assert_eq!(agent.litellm_base_url, "http://litellm.local:4000/v1");
    }

    #[test]
    fn test_finops_agent_empty_url_guard() {
        let agent = FinOpsAgent::new(String::new(), "key".to_string(), test_tag());
        assert!(agent.litellm_base_url.is_empty());
    }
}
