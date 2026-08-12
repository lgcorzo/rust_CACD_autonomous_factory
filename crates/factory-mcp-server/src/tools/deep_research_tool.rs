use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
use factory_infrastructure::KafkaClient;
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

pub struct DeepResearchTool {
    kafka_client: Arc<dyn KafkaClient>,
}

impl DeepResearchTool {
    pub fn new(kafka_client: Arc<dyn KafkaClient>) -> Self {
        Self { kafka_client }
    }
}

#[async_trait]
impl Tool for DeepResearchTool {
    fn name(&self) -> String {
        "dispatch_deep_research".to_string()
    }

    fn description(&self) -> String {
        "Dispatches an asynchronous deep research task in the background using the Hatchet DAG. Use this when you need to research a complex topic, learn an API, or find comprehensive information that might take a long time to gather. This is a fire-and-forget tool: it returns a job_id immediately and does NOT block your execution.".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The complex topic or question to investigate deeply."
                }
            },
            "required": ["query"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let query = params["query"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("query is required"))?;

        let job_id = Uuid::new_v4().to_string();

        // Push an event to Kafka. The research worker or Hatchet event listener will pick up "research:requested"
        match self
            .kafka_client
            .publish_thought(&job_id, query, "research:requested")
            .await
        {
            Ok(_) => {
                let response_text = format!(
                    "Deep research initiated successfully in the background. Job ID: {}. You can continue with other tasks or enter sleep mode. Once the research is complete, the knowledge will be automatically ingested into R2R (GraphRAG) and available for retrieval.",
                    job_id
                );

                Ok(CallToolResult {
                    content: vec![McpContent::Text {
                        text: response_text,
                    }],
                    is_error: false,
                })
            }
            Err(e) => {
                tracing::error!("Failed to dispatch research to event stream: {}", e);
                Ok(CallToolResult {
                    content: vec![McpContent::Text {
                        text: format!("Failed to dispatch research: {}", e),
                    }],
                    is_error: true,
                })
            }
        }
    }
}
