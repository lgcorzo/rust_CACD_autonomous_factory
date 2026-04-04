use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Arc;
use factory_infrastructure::McpClient;
use crate::Agent;

pub struct DocAgent {
    mcp_client: Arc<dyn McpClient>,
}

impl DocAgent {
    pub fn new(mcp_client: Arc<dyn McpClient>) -> Self {
        Self { mcp_client }
    }
}

#[async_trait]
impl Agent for DocAgent {
    fn name(&self) -> String {
        "documentation".to_string()
    }

    async fn execute(&self, task_description: &str) -> anyhow::Result<Value> {
        self.generate_docs(task_description).await
    }
}

impl DocAgent {
    pub async fn generate_docs(&self, description: &str) -> anyhow::Result<Value> {
        tracing::info!("[DocAgent] Generating documentation: {}", description);
        
        let result = self.mcp_client.call_tool_json(
            "generate_mission_docs",
            json!({ 
                "mission_id": "mission-01", 
                "mission_context": { "description": description }
            })
        ).await?;
        
        Ok(result)
    }
}
