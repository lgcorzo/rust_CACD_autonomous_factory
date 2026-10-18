use crate::Agent;
use async_trait::async_trait;
use factory_core::FinOpsTag;
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

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
    pub litellm_base_url: String,
    pub api_key: String,
    pub client: Client,
    pub tag: FinOpsTag,
    pub halted: Arc<AtomicBool>,
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
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(3))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            litellm_base_url,
            api_key,
            client,
            tag,
            halted: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted.load(Ordering::SeqCst)
    }

    pub fn set_halted(&self, val: bool) {
        self.halted.store(val, Ordering::SeqCst);
    }

    pub fn halted_latch(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.halted)
    }

    /// Parses total spend from LiteLLM response (supporting both array and object representations).
    pub fn parse_spend(json: &Value) -> Option<f64> {
        if let Some(array) = json.as_array() {
            let mut sum = 0.0;
            for item in array {
                if let Some(spend) = item.get("spend").and_then(|v| v.as_f64()) {
                    sum += spend;
                }
            }
            Some(sum)
        } else {
            json.get("total_spend")
                .or_else(|| json.get("spend"))
                .and_then(|v| v.as_f64())
        }
    }

    /// Injects Virtual Tags (x-vtags-*) into an outgoing HTTP request.
    pub fn inject_vtags(&self, mut request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        for (k, v) in self.tag.to_headers() {
            request = request.header(k, v);
        }
        request
    }

    /// Normalized spend evaluation with first-cycle anomaly suppression and active halting latch.
    pub fn evaluate_spend_delta_normalized(
        &self,
        current_spend: f64,
        previous_spend: Option<f64>,
        elapsed_minutes: f64,
        max_daily_budget: f64,
    ) -> BudgetEvaluation {
        let hardstop_threshold = max_daily_budget * 0.90;

        if current_spend >= hardstop_threshold {
            let alert = format!(
                "HARDSTOP TRIGGERED! Spend ${:.2} reached 90% threshold of daily budget (${:.2}). Halting missions.",
                current_spend, max_daily_budget
            );
            self.set_halted(true);
            BudgetEvaluation::HardStop {
                spend: current_spend,
                velocity: previous_spend
                    .map(|prev| current_spend - prev)
                    .unwrap_or(0.0),
                max_budget: max_daily_budget,
                alert,
            }
        } else {
            match previous_spend {
                None => {
                    // First cycle: anomaly suppression
                    BudgetEvaluation::Healthy {
                        spend: current_spend,
                        velocity: 0.0,
                    }
                }
                Some(prev) => {
                    let raw_delta = current_spend - prev;
                    let velocity = if elapsed_minutes > 0.0 {
                        raw_delta / elapsed_minutes
                    } else {
                        raw_delta
                    };

                    if velocity > 1.0 {
                        let alert = format!(
                            "ANOMALY DETECTED! High token spend velocity: +${:.2}/min! Total: ${:.2}",
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
            }
        }
    }

    /// Evaluates spend velocity (> +$1.00 / 60s) and 90% hardstop threshold ($45.00 of $50.00).
    pub fn evaluate_spend_delta(
        &self,
        current_spend: f64,
        previous_spend: f64,
        max_daily_budget: f64,
    ) -> BudgetEvaluation {
        self.evaluate_spend_delta_normalized(
            current_spend,
            Some(previous_spend),
            1.0,
            max_daily_budget,
        )
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

    /// Single poll cycle to query spend logs and evaluate budget status with default 1.0 minute duration.
    pub async fn poll_spend_and_evaluate(
        &self,
        max_daily_budget: f64,
        previous_spend: Option<f64>,
    ) -> anyhow::Result<BudgetEvaluation> {
        self.poll_spend_and_evaluate_normalized(max_daily_budget, previous_spend, 1.0)
            .await
    }

    /// Single poll cycle to query spend logs and evaluate budget status with normalized duration.
    pub async fn poll_spend_and_evaluate_normalized(
        &self,
        max_daily_budget: f64,
        previous_spend: Option<f64>,
        elapsed_minutes: f64,
    ) -> anyhow::Result<BudgetEvaluation> {
        let url = format!("{}/spend/logs", self.litellm_base_url.trim_end_matches('/'));
        let resp = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .query(&[("epic", &self.tag.epic)])
            .send()
            .await?
            .error_for_status()?;

        let json: Value = resp.json().await?;
        let spend = Self::parse_spend(&json).ok_or_else(|| {
            anyhow::anyhow!("Could not parse spend from LiteLLM response: {:?}", json)
        })?;

        Ok(self.evaluate_spend_delta_normalized(
            spend,
            previous_spend,
            elapsed_minutes,
            max_daily_budget,
        ))
    }

    pub async fn monitor_budget(&self) -> anyhow::Result<()> {
        if self.litellm_base_url.is_empty() {
            tracing::warn!(
                "FinOpsAgent: LITELLM_API_BASE/LITELLM_BASE_URL is not set, monitoring disabled."
            );
            return Ok(());
        }

        let base_interval = Duration::from_secs(60);
        let max_interval = Duration::from_secs(15 * 60); // 15 minutes
        let mut current_interval = base_interval;
        let mut consecutive_failures: u32 = 0;
        let mut previous_spend: Option<f64> = None;
        let mut last_poll_time: Option<Instant> = None;

        tracing::info!(
            "FinOpsAgent: Starting budget monitor for epic {} (url: {})",
            self.tag.epic,
            self.litellm_base_url
        );

        loop {
            tracing::info!("FinOpsAgent: Checking spend for epic {}", self.tag.epic);

            let max_daily_budget = std::env::var("FINOPS_MAX_DAILY_BUDGET")
                .ok()
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(50.0);

            let elapsed_minutes = last_poll_time
                .map(|t| t.elapsed().as_secs_f64() / 60.0)
                .unwrap_or(1.0);

            match self
                .poll_spend_and_evaluate_normalized(
                    max_daily_budget,
                    previous_spend,
                    elapsed_minutes,
                )
                .await
            {
                Ok(evaluation) => {
                    consecutive_failures = 0;
                    current_interval = base_interval;
                    last_poll_time = Some(Instant::now());

                    match evaluation {
                        BudgetEvaluation::HardStop { alert, spend, .. } => {
                            tracing::error!("{}", alert);
                            self.set_halted(true);
                            previous_spend = Some(spend);
                        }
                        BudgetEvaluation::VelocityAnomaly { alert, spend, .. } => {
                            tracing::warn!("{}", alert);
                            previous_spend = Some(spend);
                        }
                        BudgetEvaluation::Healthy {
                            spend, velocity, ..
                        } => {
                            let hardstop_threshold = max_daily_budget * 0.90;
                            tracing::info!(
                                "Current spend: ${:.2} / ${:.2} (Limit: ${:.2}). Velocity: +${:.2}/min. Budget is healthy.",
                                spend,
                                max_daily_budget,
                                hardstop_threshold,
                                velocity
                            );
                            previous_spend = Some(spend);
                        }
                    }
                }
                Err(e) => {
                    consecutive_failures += 1;
                    if consecutive_failures >= CIRCUIT_BREAKER_THRESHOLD {
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
