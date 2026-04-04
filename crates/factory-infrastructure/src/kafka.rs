use async_trait::async_trait;
use crate::KafkaClient;

pub struct SimpleMockKafkaClient;

impl SimpleMockKafkaClient {
    pub fn new(_brokers: &str) -> anyhow::Result<Self> {
        Ok(Self)
    }
}

#[async_trait]
impl KafkaClient for SimpleMockKafkaClient {
    async fn publish(&self, topic: &str, key: &str, payload: &[u8]) -> anyhow::Result<()> {
        tracing::info!("Mock Kafka publish: topic={}, key={}, payload_len={}", topic, key, payload.len());
        Ok(())
    }
}
