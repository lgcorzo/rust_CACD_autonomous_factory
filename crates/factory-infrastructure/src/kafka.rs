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

pub struct RealKafkaClient {
    producer: FutureProducer,
}

impl RealKafkaClient {
    pub fn new(brokers: &str) -> anyhow::Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()?;

        Ok(Self { producer })
    }
}

#[async_trait]
impl KafkaClient for RealKafkaClient {
    async fn publish(&self, topic: &str, key: &str, payload: &[u8]) -> anyhow::Result<()> {
        let record = FutureRecord::to(topic).key(key).payload(payload);

        self.producer
            .send(record, Duration::from_secs(0))
            .await
            .map_err(|(e, _)| anyhow::anyhow!("Kafka send error: {}", e))?;

        Ok(())
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
