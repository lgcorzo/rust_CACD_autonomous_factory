use async_trait::async_trait;
use serde_json::{json, Value};
use factory_infrastructure::McpHttpClient;
use crate::Agent;

pub struct PlannerAgent {
    mcp_client: McpHttpClient,
}

impl PlannerAgent {
    pub fn new(mcp_client: McpHttpClient) -> Self {
        Self { mcp_client }
    }
}

#[async_trait]
impl Agent for PlannerAgent {
    fn name(&self) -> String {
        "planner".to_string()
    }

    async fn execute(&self, task_description: &str) -> anyhow::Result<Value> {
        tracing::info!("[PlannerAgent] Creating plan for goal: {}", task_description);
        
        let result = self.mcp_client.call_tool(
            "plan_mission",
            json!({ "mission_description": task_description })
        ).await?;
        
        Ok(result)
    }
}
