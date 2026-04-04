use async_trait::async_trait;
use serde_json::{json, Value};
use factory_infrastructure::McpHttpClient;
use crate::Agent;

pub struct CoderAgent {
    mcp_client: McpHttpClient,
}

impl CoderAgent {
    pub fn new(mcp_client: McpHttpClient) -> Self {
        Self { mcp_client }
    }
}

#[async_trait]
impl Agent for CoderAgent {
    fn name(&self) -> String {
        "coder".to_string()
    }

    async fn execute(&self, task_description: &str) -> anyhow::Result<Value> {
        tracing::info!("[CoderAgent] Executing task: {}", task_description);
        
        let result = self.mcp_client.call_tool(
            "execute_code",
            json!({ 
                "task": { "description": task_description },
                "workspace_path": "/tmp/sandbox" 
            })
        ).await?;
        
        Ok(result)
    }
}
