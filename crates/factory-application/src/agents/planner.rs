use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Arc;
use factory_infrastructure::McpClient;
use crate::Agent;

pub struct PlannerAgent {
    mcp_client: Arc<dyn McpClient>,
}

impl PlannerAgent {
    pub fn new(mcp_client: Arc<dyn McpClient>) -> Self {
        Self { mcp_client }
    }
}

#[async_trait]
impl Agent for PlannerAgent {
    fn name(&self) -> String {
        "planner".to_string()
    }

    async fn execute(&self, task_description: &str) -> anyhow::Result<Value> {
        self.create_plan(task_description).await
    }
}

impl PlannerAgent {
    pub async fn create_plan(&self, goal: &str) -> anyhow::Result<Value> {
        tracing::info!("[PlannerAgent] Creating plan for goal: {}", goal);
        
        let result = self.mcp_client.call_tool_json(
            "plan_mission",
            json!({ "mission_description": goal })
        ).await?;
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use factory_infrastructure::MockMcpClient;

    #[tokio::test]
    async fn test_planner_agent_execute() {
        let mut mock_client = MockMcpClient::new();
        let expected_result = json!({ "tasks": [] });
        
        mock_client.expect_call_tool_json()
            .with(mockall::predicate::eq("plan_mission"), mockall::predicate::always())
            .returning(move |_, _| {
                let res = json!({ "tasks": [] });
                Ok(res)
            });

        let agent = PlannerAgent::new(Arc::new(mock_client));
        let result = agent.execute("Test goal").await.unwrap();
        
        assert_eq!(result, expected_result);
    }
}
