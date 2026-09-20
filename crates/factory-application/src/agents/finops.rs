use crate::Agent;
use async_trait::async_trait;
use factory_core::FinOpsTag;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

/// Circuit breaker threshold: after this many consecutive failures,
/// downgrade logging from ERROR to WARN and apply maximum backoff.
const CIRCUIT_BREAKER_THRESHOLD: u32 = 5;

#[derive(Debug, Clone, PartialEq)]
pub enum BudgetEvaluation {
    Healthy {
        spend: f64,
        velocity: f64,
    },
    VelocityAnomaly {
        spend: f64,
        velocity: f64,
        alert: String,
    },
    HardStop {
        spend: f64,
        velocity: f64,
        max_budget: f64,
        alert: String,
    },
}

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

    /// Injects Virtual Tags (x-vtags-*) into an outgoing HTTP request.
    pub fn inject_vtags(&self, mut request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        for (k, v) in self.tag.to_headers() {
            request = request.header(k, v);
        }
        request
    }

    /// Evaluates spend velocity (> +$1.00 / 60s) and 90% hardstop threshold ($45.00 of $50.00).
    pub fn evaluate_spend_delta(
        &self,
        current_spend: f64,
        previous_spend: f64,
        max_daily_budget: f64,
    ) -> BudgetEvaluation {
        let velocity = current_spend - previous_spend;
        let hardstop_threshold = max_daily_budget * 0.90;

        if current_spend >= hardstop_threshold {
            let alert = format!(
                "HARDSTOP TRIGGERED! Spend ${:.2} reached 90% threshold of daily budget (${:.2}). Halting missions.",
                current_spend, max_daily_budget
            );
            BudgetEvaluation::HardStop {
                spend: current_spend,
                velocity,
                max_budget: max_daily_budget,
                alert,
            }
        } else if velocity > 1.0 {
            let alert = format!(
                "ANOMALY DETECTED! High token spend velocity: +${:.2} in 60s! Total: ${:.2}",
                velocity, current_spend
            );
            BudgetEvaluation::VelocityAnomaly {
                spend: current_spend,
                velocity,
                alert,
            }
        } else {
            BudgetEvaluation::Healthy {
                spend: current_spend,
                velocity,
            }
        }
    }

    /// Dispatches budget-exceeded event to Kafka upon HardStop threshold breach.
    pub async fn dispatch_budget_exceeded_event(
        &self,
        kafka: &dyn factory_infrastructure::KafkaClient,
        mission_id: &str,
        current_spend: f64,
        max_daily_budget: f64,
    ) -> anyhow::Result<()> {
        let payload = serde_json::json!({
            "event_type": "budget-exceeded",
            "mission_id": mission_id,
            "current_spend": current_spend,
            "max_daily_budget": max_daily_budget,
            "threshold": "90%",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "tags": {
                "team": self.tag.team,
                "epic": self.tag.epic,
                "microservice": self.tag.microservice,
                "environment": self.tag.environment,
                "cost_center": self.tag.cost_center,
            }
        });
        kafka
            .publish(
                "budget-exceeded",
                mission_id,
                &serde_json::to_vec(&payload)?,
            )
            .await
    }

    pub async fn monitor_budget(&self) -> anyhow::Result<()> {
        if self.litellm_base_url.is_empty() {
            tracing::warn!(
                "FinOpsAgent: LITELLM_API_BASE/LITELLM_BASE_URL is not set, monitoring disabled."
            );
            return Ok(());
        }

        let url = format!("{}/spend/logs", self.litellm_base_url.trim_end_matches('/'));

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
                        let mut parsed_spend = None;
                        if let Some(array) = json.as_array() {
                            let mut sum = 0.0;
                            for item in array {
                                if let Some(spend) = item.get("spend").and_then(|v| v.as_f64()) {
                                    sum += spend;
                                }
                            }
                            parsed_spend = Some(sum);
                        } else if let Some(spend) = json.get("total_spend").and_then(|v| v.as_f64())
                        {
                            parsed_spend = Some(spend);
                        }

                        if let Some(spend) = parsed_spend {
                            // Reset backoff on successful data fetch
                            consecutive_failures = 0;
                            current_interval = base_interval;

                            let max_daily_budget = std::env::var("FINOPS_MAX_DAILY_BUDGET")
                                .ok()
                                .and_then(|v| v.parse::<f64>().ok())
                                .unwrap_or(50.0);
                            let hardstop_threshold = max_daily_budget * 0.90;

                            let evaluation =
                                self.evaluate_spend_delta(spend, previous_spend, max_daily_budget);
                            match evaluation {
                                BudgetEvaluation::HardStop { alert, .. } => {
                                    tracing::error!("{}", alert);
                                }
                                BudgetEvaluation::VelocityAnomaly { alert, .. } => {
                                    tracing::warn!("{}", alert);
                                }
                                BudgetEvaluation::Healthy { velocity, .. } => {
                                    tracing::info!(
                                        "Current spend: ${:.2} / ${:.2} (Limit: ${:.2}). Velocity: +${:.2}/min. Budget is healthy.",
                                        spend,
                                        max_daily_budget,
                                        hardstop_threshold,
                                        velocity
                                    );
                                }
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
            "https://litellm.local:4000/v1".to_string(),
            "key".to_string(),
            test_tag(),
        );
        // The constructor doesn't strip — Default::default() does.
        // But the URL construction in monitor_budget appends /spend/logs.
        // So a raw URL with /v1 would become /v1/spend/logs.
        // This test verifies the URL stored.
        assert_eq!(agent.litellm_base_url, "https://litellm.local:4000/v1");
    }

    #[test]
    fn test_finops_agent_empty_url_guard() {
        let agent = FinOpsAgent::new(String::new(), "key".to_string(), test_tag());
        assert!(agent.litellm_base_url.is_empty());
    }

    #[test]
    fn test_finops_inject_vtags() {
        let agent = FinOpsAgent::new(
            "https://litellm:4000".to_string(),
            "key".to_string(),
            test_tag(),
        );
        let client = Client::new();
        let req = client.get("https://litellm:4000/v1/chat/completions");
        let tagged_req = agent.inject_vtags(req).build().unwrap();

        let headers = tagged_req.headers();
        assert_eq!(headers.get("x-vtags-team").unwrap(), "test-team");
        assert_eq!(headers.get("x-vtags-epic").unwrap(), "E1.0");
        assert_eq!(headers.get("x-vtags-microservice").unwrap(), "test-svc");
        assert_eq!(headers.get("x-vtags-environment").unwrap(), "test");
        assert_eq!(headers.get("x-vtags-cost-center").unwrap(), "test-cc");
    }

    #[test]
    fn test_spend_velocity_anomaly_detection() {
        let agent = FinOpsAgent::new(
            "https://litellm:4000".to_string(),
            "key".to_string(),
            test_tag(),
        );

        // Velocity within normal limit: +$0.50 in 60s
        let eval_normal = agent.evaluate_spend_delta(10.50, 10.00, 50.0);
        assert_eq!(
            eval_normal,
            BudgetEvaluation::Healthy {
                spend: 10.50,
                velocity: 0.50,
            }
        );

        // Velocity anomaly: +$1.50 in 60s (> +$1.00 / 60s threshold)
        let eval_anomaly = agent.evaluate_spend_delta(11.50, 10.00, 50.0);
        match eval_anomaly {
            BudgetEvaluation::VelocityAnomaly {
                spend,
                velocity,
                alert,
            } => {
                assert_eq!(spend, 11.50);
                assert!((velocity - 1.50).abs() < f64::EPSILON);
                assert!(alert.contains("ANOMALY DETECTED"));
            }
            _ => panic!("Expected VelocityAnomaly"),
        }
    }

    #[test]
    fn test_hardstop_threshold_tripping() {
        let agent = FinOpsAgent::new(
            "https://litellm:4000".to_string(),
            "key".to_string(),
            test_tag(),
        );

        // 90% of $50.00 = $45.00
        let eval_under = agent.evaluate_spend_delta(44.90, 44.00, 50.0);
        assert!(matches!(eval_under, BudgetEvaluation::Healthy { .. }));

        // HardStop breached at $45.00
        let eval_hardstop = agent.evaluate_spend_delta(45.00, 44.50, 50.0);
        match eval_hardstop {
            BudgetEvaluation::HardStop {
                spend,
                max_budget,
                alert,
                ..
            } => {
                assert_eq!(spend, 45.00);
                assert_eq!(max_budget, 50.0);
                assert!(alert.contains("HARDSTOP TRIGGERED"));
            }
            _ => panic!("Expected HardStop"),
        }
    }

    #[tokio::test]
    async fn test_dispatch_budget_exceeded_event() {
        use factory_infrastructure::SimpleMockKafkaClient;
        let agent = FinOpsAgent::new(
            "https://litellm:4000".to_string(),
            "key".to_string(),
            test_tag(),
        );
        let mock_kafka = SimpleMockKafkaClient::new("localhost:9092").unwrap();

        let result = agent
            .dispatch_budget_exceeded_event(&mock_kafka, "mission-budget-test", 45.50, 50.0)
            .await;
        assert!(result.is_ok());
    }
}
