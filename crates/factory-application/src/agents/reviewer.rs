use crate::Agent;
use async_trait::async_trait;
use factory_infrastructure::McpClient;
use serde_json::{Value, json};
use std::sync::Arc;

pub struct ReviewerAgent {
    mcp_client: Arc<dyn McpClient>,
}

impl ReviewerAgent {
    pub fn new(mcp_client: Arc<dyn McpClient>) -> Self {
        Self { mcp_client }
    }
}

#[async_trait]
impl Agent for ReviewerAgent {
    fn name(&self) -> String {
        "reviewer".to_string()
    }

    async fn execute(&self, task_description: &str) -> anyhow::Result<Value> {
        self.review_changes(task_description).await
    }
}

impl ReviewerAgent {
    pub async fn review_changes(&self, diff: &str) -> anyhow::Result<Value> {
        tracing::info!("[ReviewerAgent] Reviewing changes: {}", diff);

        let result = self
            .mcp_client
            .call_tool_json(
                "security_review",
                json!({
                    "diff": diff
                }),
            )
            .await?;

        Ok(result)
    }
}
