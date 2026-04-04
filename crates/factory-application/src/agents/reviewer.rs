use async_trait::async_trait;
use serde_json::{json, Value};
use factory_infrastructure::McpHttpClient;
use crate::Agent;

pub struct ReviewerAgent {
    mcp_client: McpHttpClient,
}

impl ReviewerAgent {
    pub fn new(mcp_client: McpHttpClient) -> Self {
        Self { mcp_client }
    }
}

#[async_trait]
impl Agent for ReviewerAgent {
    fn name(&self) -> String {
        "reviewer".to_string()
    }

    async fn execute(&self, task_description: &str) -> anyhow::Result<Value> {
        tracing::info!("[ReviewerAgent] Reviewing changes: {}", task_description);
        
        let result = self.mcp_client.call_tool(
            "security_review",
            json!({ 
                "diff": task_description 
            })
        ).await?;
        
        Ok(result)
    }
}
