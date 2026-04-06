use async_trait::async_trait;
use chrono::Utc;

#[async_trait]
pub trait KafkaClient: Send + Sync {
    async fn publish(&self, topic: &str, key: &str, payload: &[u8]) -> anyhow::Result<()>;
    async fn publish_thought(&self, mission_id: &str, thought: &str) -> anyhow::Result<()> {
        let payload = serde_json::json!({
            "mission_id": mission_id,
            "thought": thought,
            "timestamp": Utc::now().to_rfc3339()
        });
        self.publish("agent-thought", mission_id, &serde_json::to_vec(&payload)?)
            .await
    }
}

pub struct SimpleMockKafkaClient;

impl SimpleMockKafkaClient {
    pub fn new(_brokers: &str) -> anyhow::Result<Self> {
        Ok(SimpleMockKafkaClient)
    }
}

#[async_trait]
impl KafkaClient for SimpleMockKafkaClient {
    async fn publish(&self, topic: &str, key: &str, payload: &[u8]) -> anyhow::Result<()> {
        tracing::info!(
            "Mock Kafka publish: topic={}, key={}, payload_len={}",
            topic,
            key,
            payload.len()
        );
        Ok(())
    }
}
