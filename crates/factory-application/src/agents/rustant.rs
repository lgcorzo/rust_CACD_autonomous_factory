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

        // 3. Parse generated artifacts
        let mut target_spec_dir = None;

        // Try reading .specify/init-options.json
        let config_str = std::fs::read_to_string(".specify/init-options.json").ok();
        let config: Option<serde_json::Value> =
            config_str.and_then(|s| serde_json::from_str(&s).ok());
        if let Some(specs_dir) = config
            .as_ref()
            .and_then(|c| c.get("specs_dir"))
            .and_then(|v| v.as_str())
        {
            let path = std::path::PathBuf::from(specs_dir);
            if path.exists() {
                target_spec_dir = Some(path);
            }
        }

        // Fall back to latest directory in specs/
        if target_spec_dir.is_none() {
            let entries = std::fs::read_dir("specs").ok();
            if let Some(entries) = entries {
                let mut dirs: Vec<_> = entries.filter_map(|e| e.ok()).collect();
                dirs.sort_by_key(|dir| {
                    dir.metadata()
                        .and_then(|m| m.modified())
                        .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                });
                if let Some(latest) = dirs.last() {
                    target_spec_dir = Some(latest.path());
                }
            }
        }

        let mut parsed_spec = String::new();
        let mut parsed_plan = String::new();
        let mut parsed_tasks = String::new();

        if let Some(spec_dir) = target_spec_dir {
            parsed_spec = std::fs::read_to_string(spec_dir.join("spec.md")).unwrap_or_default();
            parsed_plan = std::fs::read_to_string(spec_dir.join("plan.md")).unwrap_or_default();
            parsed_tasks = std::fs::read_to_string(spec_dir.join("tasks.md")).unwrap_or_default();
        }

        Ok(json!({
            "status": "spec_kit_planning_complete",
            "spec": parsed_spec,
            "plan": parsed_plan,
            "tasks": parsed_tasks,
            "summary": "Implement the task described in the parsed plan and tasks artifacts."
        }))
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
