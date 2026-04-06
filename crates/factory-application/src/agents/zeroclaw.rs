use crate::Agent;
use async_trait::async_trait;
use factory_infrastructure::McpClient;
use serde_json::{Value, json};
use std::sync::Arc;

pub struct ZeroClawAgent {
    mcp_client: Arc<dyn McpClient>,
}

impl ZeroClawAgent {
    pub fn new(mcp_client: Arc<dyn McpClient>) -> Self {
        Self { mcp_client }
    }

    pub async fn execute_task(
        &self,
        mission_id: &str,
        task_description: &str,
        files: &[String],
    ) -> anyhow::Result<Value> {
        tracing::info!(
            "[ZeroClawAgent:{}] Executing task: {}",
            mission_id,
            task_description
        );

        // 1. Sandbox Orchestration (Skill)
        // 2. Call MCP tool for execution
        let result = self
            .mcp_client
            .call_tool_json(
                "execute_code",
                json!({
                    "mission_id": mission_id,
                    "task": {
                        "description": task_description,
                        "files": files
                    },
                    "workspace_path": format!("/tmp/sandbox/zeroclaw/{}", mission_id)
                }),
            )
            .await?;

        Ok(result)
    }

    pub async fn validate_mission(
        &self,
        mission_id: &str,
        test_command: &str,
    ) -> anyhow::Result<Value> {
        tracing::info!(
            "[ZeroClawAgent:{}] Validating mission with tests: {}",
            mission_id,
            test_command
        );

        let result = self
            .mcp_client
            .call_tool_json(
                "run_tests",
                json!({
                    "mission_id": mission_id,
                    "command": test_command
                }),
            )
            .await?;

        Ok(result)
    }

    pub async fn introspect_k8s(&self, mission_id: &str) -> anyhow::Result<Value> {
        tracing::info!(
            "[ZeroClawAgent:{}] Performing K8s introspection skill",
            mission_id
        );

        let result = self
            .mcp_client
            .call_tool_json("introspect_k8s", json!({ "mission_id": mission_id }))
            .await?;

        Ok(result)
    }
}

#[async_trait]
impl Agent for ZeroClawAgent {
    fn name(&self) -> String {
        "zeroclaw".to_string()
    }

    async fn execute(&self, task_description: &str) -> anyhow::Result<Value> {
        // Default to executing a general task with a temporary ID if no specific action specified
        self.execute_task("default-id", task_description, &[]).await
    }
}
