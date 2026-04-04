use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Arc;
use factory_infrastructure::McpClient;
use crate::Agent;

pub struct CoderAgent {
    mcp_client: Arc<dyn McpClient>,
}

impl CoderAgent {
    pub fn new(mcp_client: Arc<dyn McpClient>) -> Self {
        Self { mcp_client }
    }
}

#[async_trait]
impl Agent for CoderAgent {
    fn name(&self) -> String {
        "coder".to_string()
    }

    async fn execute(&self, task_description: &str) -> anyhow::Result<Value> {
        self.execute_task(task_description).await
    }
}

impl CoderAgent {
    pub async fn execute_task(&self, description: &str) -> anyhow::Result<Value> {
        tracing::info!("[CoderAgent] Executing task: {}", description);
        
        let result = self.mcp_client.call_tool_json(
            "execute_code",
            json!({ 
                "task": { "description": description },
                "workspace_path": "/tmp/sandbox" 
            })
        ).await?;
        
        Ok(result)
    }
}
