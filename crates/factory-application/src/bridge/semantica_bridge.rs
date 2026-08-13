use std::sync::Arc;
use factory_infrastructure::{DecisionRecord, SemanticaClient};
use tracing::{error, info};

pub struct SemanticaBridge {
    semantica_client: Arc<dyn SemanticaClient>,
}

impl SemanticaBridge {
    pub fn new(semantica_client: Arc<dyn SemanticaClient>) -> Self {
        Self { semantica_client }
    }

    pub async fn process_agent_thought_event(
        &self,
        event_payload: &str,
    ) -> anyhow::Result<()> {
        let record: DecisionRecord = match serde_json::from_str(event_payload) {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to parse agent-thought payload: {}", e);
                anyhow::bail!("Invalid agent-thought event schema");
            }
        };

        info!(
            "Recording agent decision in Semantica: decision_id={}, agent={}",
            record.decision_id, record.agent_id
        );

        self.semantica_client.record_decision(&record).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use factory_infrastructure::MockSemanticaClient;

    #[tokio::test]
    async fn test_process_agent_thought_event_success() {
        let mut mock = MockSemanticaClient::new();
        mock.expect_record_decision()
            .times(1)
            .returning(|_| Ok(()));

        let bridge = SemanticaBridge::new(Arc::new(mock));
        let payload = serde_json::json!({
            "decision_id": "dec-999",
            "agent_id": "zeroclaw",
            "mission_id": "mis-100",
            "reasoning": "Refactored security validator",
            "ast_node_ids": ["fn_validate"],
            "timestamp": "2026-08-13T17:10:00Z"
        })
        .to_string();

        let res = bridge.process_agent_thought_event(&payload).await;
        assert!(res.is_ok());
    }
}
