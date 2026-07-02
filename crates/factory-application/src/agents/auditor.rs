use crate::Agent;
use async_trait::async_trait;
use serde_json::{Value, json};

pub struct AuditorAgent {
    // Inject dependencies for Kafka and LiteLLM here when fully integrated
}

impl Default for AuditorAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditorAgent {
    pub fn new() -> Self {
        Self {}
    }

    /// Subscribes to Hatchet events via the Kafka event stream to fetch recent failed mission DAGs.
    pub async fn analyze_dag_logs(&self, mission_id: &str) -> anyhow::Result<Vec<Value>> {
        tracing::info!(
            "[AuditorAgent:{}] Fetching failed DAG logs via Kafka event stream",
            mission_id
        );

        // Mocking Kafka event stream consumption for now
        let mock_logs = vec![json!({
            "step": "validate_mission",
            "error": "Test 'test_verify_osr_calculation' failed. Expected 0.0, got 0.05",
            "timestamp": "2026-07-02T12:00:00Z"
        })];

        Ok(mock_logs)
    }

    /// Uses LiteLLM to process failures and output recommendations.
    pub async fn audit_mission(
        &self,
        mission_id: &str,
        failures: &[Value],
    ) -> anyhow::Result<Value> {
        tracing::info!(
            "[AuditorAgent:{}] Auditing mission failures using LiteLLM",
            mission_id
        );

        // Mocking LiteLLM call
        let _failures_json = serde_json::to_string(failures)?;

        let recommendations = json!([
            {
                "type": "prompt_adjustment",
                "target_agent": "doc_agent",
                "recommendation": "Instruct the agent to use Levenshtein distance for OSR calculation."
            },
            {
                "type": "tool_modification",
                "target_tool": "run_tests",
                "recommendation": "Ensure the test runner captures stdout for better diagnostics."
            }
        ]);

        Ok(recommendations)
    }
}

#[async_trait]
impl Agent for AuditorAgent {
    fn name(&self) -> String {
        "auditor".to_string()
    }

    async fn execute(&self, task_description: &str) -> anyhow::Result<Value> {
        tracing::info!("[AuditorAgent] Executing task: {}", task_description);
        let logs = self.analyze_dag_logs("default-id").await?;
        self.audit_mission("default-id", &logs).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_auditor_agent() {
        let agent = AuditorAgent::new();

        let logs = agent.analyze_dag_logs("test-mission").await.unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0]["step"], "validate_mission");

        let recommendations = agent.audit_mission("test-mission", &logs).await.unwrap();
        let array = recommendations.as_array().expect("Expected JSON array");
        assert_eq!(array.len(), 2);
        assert_eq!(array[0]["type"], "prompt_adjustment");
        assert_eq!(array[1]["type"], "tool_modification");
    }
}
