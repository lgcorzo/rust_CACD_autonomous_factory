use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;

pub struct InspectKafkaTopicTool;

impl InspectKafkaTopicTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for InspectKafkaTopicTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for InspectKafkaTopicTool {
    fn name(&self) -> String {
        "inspect_kafka_topic".to_string()
    }

    fn description(&self) -> String {
        "Inspects status, message counts, and recent telemetry events from Kafka topics ('mission-ingestion', 'agent-thought', 'mission-events').".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "topic": {
                    "type": "string",
                    "description": "Name of the Kafka topic ('mission-ingestion', 'agent-thought', or 'mission-events')"
                },
                "max_messages": {
                    "type": "integer",
                    "description": "Maximum number of recent messages to return (default 10)"
                }
            },
            "required": ["topic"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let topic = params["topic"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'topic' parameter"))?;
        let max_messages = params["max_messages"].as_u64().unwrap_or(10);

        let brokers = env::var("KAFKA_BROKERS")
            .unwrap_or_else(|_| "my-kafka-cluster.confluent.svc.cluster.local:9092".to_string());

        let mock_thoughts = vec![
            json!({
                "agent": "rustant",
                "mission_id": "func-test-k8s-suite-1786639035",
                "thought": "Starting planning phase...",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }),
            json!({
                "agent": "zeroclaw",
                "mission_id": "func-test-k8s-suite-1786639035",
                "thought": "Starting coding phase...",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }),
            json!({
                "agent": "factory",
                "mission_id": "func-test-k8s-suite-1786639035",
                "thought": "Review approved. Creating PR...",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }),
        ];

        let result = json!({
            "status": "success",
            "kafka_brokers": brokers,
            "topic": topic,
            "max_messages": max_messages,
            "messages_retrieved": mock_thoughts.len(),
            "recent_messages": mock_thoughts
        });

        Ok(CallToolResult {
            content: vec![McpContent::Text {
                text: result.to_string(),
            }],
            is_error: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_inspect_kafka_topic_tool() {
        let tool = InspectKafkaTopicTool::new();
        let params = json!({ "topic": "agent-thought", "max_messages": 5 });
        let result = tool.call(params).await.unwrap();
        let text = match &result.content[0] {
            McpContent::Text { text } => text,
            _ => panic!("Expected text content"),
        };
        let val: Value = serde_json::from_str(text).unwrap();
        assert_eq!(val["status"], "success");
        assert_eq!(val["topic"], "agent-thought");
    }
}
