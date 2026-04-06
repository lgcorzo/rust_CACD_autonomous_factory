use crate::Agent;
use async_trait::async_trait;
use factory_infrastructure::{McpClient, R2rClient};
use serde_json::{Value, json};
use std::sync::Arc;

pub struct RustantAgent {
    mcp_client: Arc<dyn McpClient>,
    r2r_client: Arc<dyn R2rClient>,
}

impl RustantAgent {
    pub fn new(mcp_client: Arc<dyn McpClient>, r2r_client: Arc<dyn R2rClient>) -> Self {
        Self {
            mcp_client,
            r2r_client,
        }
    }

    pub async fn plan_mission(&self, goal: &str) -> anyhow::Result<Value> {
        tracing::info!("[RustantAgent] Planning mission for goal: {}", goal);

        // 1. Context Pruning (Skill)
        let context = self.r2r_client.search(goal).await?;

        // 2. Call MCP tool with pruned context
        let result = self
            .mcp_client
            .call_tool_json(
                "plan_mission",
                json!({
                    "mission_description": goal,
                    "context": context
                }),
            )
            .await?;

        Ok(result)
    }

    pub async fn review_mission(&self, mission_results: &str) -> anyhow::Result<Value> {
        tracing::info!("[RustantAgent] Reviewing mission results");

        let result = self
            .mcp_client
            .call_tool_json("security_review", json!({ "artifacts": mission_results }))
            .await?;

        Ok(result)
    }
}

#[async_trait]
impl Agent for RustantAgent {
    fn name(&self) -> String {
        "rustant".to_string()
    }

    async fn execute(&self, task_description: &str) -> anyhow::Result<Value> {
        // Default to planning if no specific action is provided
        self.plan_mission(task_description).await
    }
}
