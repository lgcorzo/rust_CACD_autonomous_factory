use async_trait::async_trait;
use serde_json::{json, Value};
use factory_infrastructure::McpHttpClient;
use crate::Agent;

pub struct DocAgent {
    mcp_client: McpHttpClient,
}

impl DocAgent {
    pub fn new(mcp_client: McpHttpClient) -> Self {
        Self { mcp_client }
    }
}

#[async_trait]
impl Agent for DocAgent {
    fn name(&self) -> String {
        "documentation".to_string()
    }

    async fn execute(&self, task_description: &str) -> anyhow::Result<Value> {
        tracing::info!("[DocAgent] Generating documentation: {}", task_description);
        
        let result = self.mcp_client.call_tool(
            "generate_mission_docs",
            json!({ 
                "mission_id": "mission-01", 
                "mission_context": { "description": task_description }
            })
        ).await?;
        
        Ok(result)
    }
}
