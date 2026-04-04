use async_trait::async_trait;
use serde_json::{json, Value};
use factory_infrastructure::McpHttpClient;
use crate::Agent;

pub struct TesterAgent {
    mcp_client: McpHttpClient,
}

impl TesterAgent {
    pub fn new(mcp_client: McpHttpClient) -> Self {
        Self { mcp_client }
    }
}

#[async_trait]
impl Agent for TesterAgent {
    fn name(&self) -> String {
        "tester".to_string()
    }

    async fn execute(&self, task_description: &str) -> anyhow::Result<Value> {
        tracing::info!("[TesterAgent] Running tests: {}", task_description);
        
        let result = self.mcp_client.call_tool(
            "run_tests",
            json!({ 
                "changes": { "description": task_description },
                "workspace_path": "/tmp/sandbox" 
            })
        ).await?;
        
        Ok(result)
    }
}
