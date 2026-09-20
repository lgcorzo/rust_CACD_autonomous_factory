use ed25519_dalek::SigningKey;
use factory_core::security::nhi::{AgentSubject, VerifiableCredential};
use factory_infrastructure::KafkaClient;
use std::sync::Arc;
use uuid::Uuid;

pub struct KafkaBridge {
    kafka_client: Arc<dyn KafkaClient>,
    signing_key: SigningKey,
    key_id: String,
    agent_id: String,
}

impl KafkaBridge {
    pub fn new(
        kafka_client: Arc<dyn KafkaClient>,
        signing_key: SigningKey,
        key_id: String,
        agent_id: String,
    ) -> Self {
        Self {
            kafka_client,
            signing_key,
            key_id,
            agent_id,
        }
    }

    /// Publishes a signed event to Kafka with attached W3C Verifiable Credential signature.
    pub async fn publish_signed_event(
        &self,
        topic: &str,
        key: &str,
        event_type: &str,
        payload: &serde_json::Value,
    ) -> anyhow::Result<()> {
        let subject = AgentSubject {
            id: self.agent_id.clone(),
            roles: vec!["event-publisher".to_string()],
            allowed_namespaces: vec!["factory-sandbox".to_string()],
        };

        let mut vc = VerifiableCredential::new(
            format!("urn:uuid:{}", Uuid::new_v4()),
            format!("did:darkgravity:{}", self.agent_id),
            subject,
        );

        vc.sign_async(self.signing_key.clone(), self.key_id.clone())
            .await
            .map_err(|e| anyhow::anyhow!("Signing error: {}", e))?;

        let envelope = serde_json::json!({
            "event_type": event_type,
            "credential": vc,
            "payload": payload,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        let bytes = serde_json::to_vec(&envelope)?;
        self.kafka_client.publish(topic, key, &bytes).await?;
        Ok(())
    }

    /// Publishes a batch of events with async concurrent batch signing.
    pub async fn publish_batch_signed_events(
        &self,
        topic: &str,
        key: &str,
        events: &[(String, serde_json::Value)],
    ) -> anyhow::Result<()> {
        let mut credentials: Vec<VerifiableCredential> = events
            .iter()
            .map(|_| {
                let subject = AgentSubject {
                    id: self.agent_id.clone(),
                    roles: vec!["event-publisher".to_string()],
                    allowed_namespaces: vec!["factory-sandbox".to_string()],
                };
                VerifiableCredential::new(
                    format!("urn:uuid:{}", Uuid::new_v4()),
                    format!("did:darkgravity:{}", self.agent_id),
                    subject,
                )
            })
            .collect();

        VerifiableCredential::sign_batch_async(&mut credentials, &self.signing_key, &self.key_id)
            .await
            .map_err(|e| anyhow::anyhow!("Batch signing error: {}", e))?;

        for (i, (event_type, payload)) in events.iter().enumerate() {
            let envelope = serde_json::json!({
                "event_type": event_type,
                "credential": &credentials[i],
                "payload": payload,
                "timestamp": chrono::Utc::now().to_rfc3339()
            });

            let bytes = serde_json::to_vec(&envelope)?;
            self.kafka_client.publish(topic, key, &bytes).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use factory_infrastructure::SimpleMockKafkaClient;
    use rand::rngs::OsRng;

    #[tokio::test]
    async fn test_kafka_bridge_publish_signed_event() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let mock_kafka = Arc::new(SimpleMockKafkaClient::new("localhost:9092").unwrap());

        let bridge = KafkaBridge::new(
            mock_kafka,
            signing_key,
            "key-ed25519-01".to_string(),
            "agent-zeroclaw-01".to_string(),
        );

        let payload = serde_json::json!({
            "action": "ast-mutation",
            "file": "src/lib.rs"
        });

        let res = bridge
            .publish_signed_event("factory-events", "key-1", "ast.mutated", &payload)
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_kafka_bridge_publish_batch_signed_events() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let mock_kafka = Arc::new(SimpleMockKafkaClient::new("localhost:9092").unwrap());

        let bridge = KafkaBridge::new(
            mock_kafka,
            signing_key,
            "key-ed25519-01".to_string(),
            "agent-zeroclaw-01".to_string(),
        );

        let events = vec![
            ("event.a".to_string(), serde_json::json!({"step": 1})),
            ("event.b".to_string(), serde_json::json!({"step": 2})),
            ("event.c".to_string(), serde_json::json!({"step": 3})),
        ];

        let res = bridge
            .publish_batch_signed_events("factory-events", "key-batch", &events)
            .await;
        assert!(res.is_ok());
    }
}
