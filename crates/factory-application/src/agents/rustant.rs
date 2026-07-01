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

    pub async fn plan_mission(&self, mission_id: &str, goal: &str) -> anyhow::Result<Value> {
        tracing::info!(
            "[RustantAgent:{}] Planning mission for goal: {}",
            mission_id,
            goal
        );

        // 1. Context Pruning (R2R)
        let context = self.r2r_client.search(goal).await?;

        // 2. 6-Phase Spec-Kit sequence via MCP
        let phases = vec!["init", "specify", "plan", "execute", "verify", "git-commit"];

        for phase in phases {
            let mut args = vec![];

            // Inject R2R context into specify
            if phase == "specify" {
                args.push(format!("--context={}", context));
            }

            self.mcp_client
                .call_tool_json(
                    "invoke_spec_kit",
                    json!({
                        "command": phase,
                        "args": args
                    }),
                )
                .await?;
        }

        // Return a mock result to conform to legacy requirements (could be populated from tasks.md)
        Ok(json!({ "status": "spec_kit_planning_complete" }))
    }

    pub async fn review_mission(
        &self,
        mission_id: &str,
        mission_results: &str,
    ) -> anyhow::Result<Value> {
        tracing::info!("[RustantAgent:{}] Reviewing mission results", mission_id);

        let result = self
            .mcp_client
            .call_tool_json(
                "security_review",
                json!({
                    "mission_id": mission_id,
                    "artifacts": mission_results
                }),
            )
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
        // Default to planning with a temporary ID if no specific action is provided
        self.plan_mission("default-id", task_description).await
    }
}
