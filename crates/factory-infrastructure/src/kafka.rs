use async_trait::async_trait;
use chrono::Utc;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

#[async_trait]
pub trait KafkaClient: Send + Sync {
    async fn publish(&self, topic: &str, key: &str, payload: &[u8]) -> anyhow::Result<()>;
    async fn publish_thought(
        &self,
        mission_id: &str,
        thought: &str,
        agent: &str,
    ) -> anyhow::Result<()> {
        let payload = serde_json::json!({
            "mission_id": mission_id,
            "agent": agent,
            "thought": thought,
            "timestamp": Utc::now().to_rfc3339()
        });
        self.publish("agent-thought", mission_id, &serde_json::to_vec(&payload)?)
            .await
    }
}

pub struct RdKafkaClient {
    producer: FutureProducer,
}

impl RdKafkaClient {
    pub fn new(brokers: &str) -> anyhow::Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()?;
        Ok(RdKafkaClient { producer })
    }
}

#[async_trait]
impl KafkaClient for RdKafkaClient {
    async fn publish(&self, topic: &str, key: &str, payload: &[u8]) -> anyhow::Result<()> {
        let record = FutureRecord::to(topic).payload(payload).key(key);
        self.producer
            .send(record, Duration::from_secs(5))
            .await
            .map_err(|(err, _)| anyhow::anyhow!("Kafka publish failed: {:?}", err))?;
        Ok(())
    }
}

#[cfg(not(feature = "production"))]
pub struct SimpleMockKafkaClient;

#[cfg(not(feature = "production"))]
impl SimpleMockKafkaClient {
    pub fn new(_brokers: &str) -> anyhow::Result<Self> {
        Ok(SimpleMockKafkaClient)
    }
}

#[cfg(not(feature = "production"))]
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
