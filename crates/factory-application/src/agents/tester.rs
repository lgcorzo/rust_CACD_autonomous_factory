use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Arc;
use factory_infrastructure::McpClient;
use crate::Agent;

pub struct TesterAgent {
    mcp_client: Arc<dyn McpClient>,
}

impl TesterAgent {
    pub fn new(mcp_client: Arc<dyn McpClient>) -> Self {
        Self { mcp_client }
    }
}

#[async_trait]
impl Agent for TesterAgent {
    fn name(&self) -> String {
        "tester".to_string()
    }

    async fn execute(&self, task_description: &str) -> anyhow::Result<Value> {
        self.run_tests(task_description).await
    }
}

impl TesterAgent {
    pub async fn run_tests(&self, description: &str) -> anyhow::Result<Value> {
        tracing::info!("[TesterAgent] Running tests: {}", description);
        
        let result = self.mcp_client.call_tool_json(
            "run_tests",
            json!({ 
                "changes": { "description": description },
                "workspace_path": "/tmp/sandbox" 
            })
        ).await?;
        
        Ok(result)
    }
}
