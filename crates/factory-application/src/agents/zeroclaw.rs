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
        task_description: &str,
        files: &[String],
    ) -> anyhow::Result<Value> {
        tracing::info!("[ZeroClawAgent] Executing task: {}", task_description);

        // 1. Sandbox Orchestration (Skill)
        // 2. Call MCP tool for execution
        let result = self
            .mcp_client
            .call_tool_json(
                "execute_code",
                json!({
                    "task": {
                        "description": task_description,
                        "files": files
                    },
                    "workspace_path": "/tmp/sandbox/zeroclaw"
                }),
            )
            .await?;

        Ok(result)
    }

    pub async fn validate_mission(&self, test_command: &str) -> anyhow::Result<Value> {
        tracing::info!(
            "[ZeroClawAgent] Validating mission with tests: {}",
            test_command
        );

        let result = self
            .mcp_client
            .call_tool_json("run_tests", json!({ "command": test_command }))
            .await?;

        Ok(result)
    }

    pub async fn introspect_k8s(&self) -> anyhow::Result<Value> {
        tracing::info!("[ZeroClawAgent] Performing K8s introspection skill");

        let result = self
            .mcp_client
            .call_tool_json("introspect_k8s", json!({}))
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
        // Default to executing a general task if no action specified
        self.execute_task(task_description, &[]).await
    }
}
