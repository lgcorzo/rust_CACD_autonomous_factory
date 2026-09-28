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

        let mut parsed_spec = Value::Null;
        let mut parsed_plan = Value::Null;
        let mut parsed_tasks = Value::Null;

        let mut sdd_tasks = Vec::new();
        if let Some(spec_dir) = target_spec_dir {
            if let Ok(s) = std::fs::read_to_string(spec_dir.join("spec.md")) {
                parsed_spec = serde_json::from_str(&s).unwrap_or(json!(s));
            }
            if let Ok(s) = std::fs::read_to_string(spec_dir.join("plan.md")) {
                parsed_plan = serde_json::from_str(&s).unwrap_or(json!(s));
            }
            if let Ok(s) = std::fs::read_to_string(spec_dir.join("tasks.md")) {
                sdd_tasks = Self::parse_sdd_tasks(&s);
                parsed_tasks = serde_json::from_str(&s).unwrap_or(json!(s));
            }
        }

        let sdd_plan = factory_core::SddMissionPlan {
            mission_id: mission_id.to_string(),
            spec_version: "1.0.0".to_string(),
            total_tasks: sdd_tasks.len(),
            tasks: sdd_tasks,
        };

        Ok(json!({
            "status": "spec_kit_planning_complete",
            "spec": parsed_spec,
            "plan": parsed_plan,
            "tasks": parsed_tasks,
            "sdd_plan": sdd_plan,
            "summary": "Implement the task described in the parsed plan and tasks artifacts."
        }))
    }

    pub fn parse_sdd_tasks(content: &str) -> Vec<factory_core::SddTaskItem> {
        let task_regex =
            regex::Regex::new(r"(?m)^-\s*\[[ xX]\]\s*(T\d+)\s*(\[P\])?\s*(?:\[[^\]]+\])?\s*(.+)$")
                .unwrap();
        let file_regex = regex::Regex::new(r"`([^`]+(?:\.[a-zA-Z0-9]+|Cargo\.toml))`").unwrap();

        let mut tasks = Vec::new();
        let mut previous_task_id: Option<String> = None;

        for cap in task_regex.captures_iter(content) {
            let id = cap.get(1).map_or("", |m| m.as_str()).to_string();
            let is_parallel = cap.get(2).is_some();
            let description = cap.get(3).map_or("", |m| m.as_str()).trim().to_string();

            let mut target_files = Vec::new();
            for file_cap in file_regex.captures_iter(&description) {
                if let Some(m) = file_cap.get(1) {
                    let path = m.as_str().to_string();
                    if !target_files.contains(&path) {
                        target_files.push(path);
                    }
                }
            }

            let mut dependencies = Vec::new();
            if !is_parallel {
                if let Some(prev) = &previous_task_id {
                    dependencies.push(prev.clone());
                }
            }
            previous_task_id = Some(id.clone());

            tasks.push(factory_core::SddTaskItem {
                id,
                description,
                is_parallel,
                dependencies,
                target_files,
            });
        }

        tasks
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sdd_tasks_from_markdown() {
        let tasks_md = r#"
# Tasks: Pipeline Error Remediation
- [ ] T001 Add `regex = "1"` to `[workspace.dependencies]` in `Cargo.toml`
- [ ] T002 [P] Create empty module file `crates/factory-infrastructure/src/pipeline_classifier.rs` with module doc comment
- [ ] T003 [P] Create empty module file `crates/factory-application/src/workflows/pipeline_remediation.rs`
- [ ] T006 [P] Add `ErrorCategory` enum to `crates/factory-core/src/lib.rs` (per data-model.md)
"#;

        let parsed = RustantAgent::parse_sdd_tasks(tasks_md);
        assert_eq!(parsed.len(), 4);
        assert_eq!(parsed[0].id, "T001");
        assert!(!parsed[0].is_parallel);
        assert!(parsed[0].target_files.contains(&"Cargo.toml".to_string()));

        assert_eq!(parsed[1].id, "T002");
        assert!(parsed[1].is_parallel);
        assert!(parsed[1].target_files.contains(&"crates/factory-infrastructure/src/pipeline_classifier.rs".to_string()));
    }
}

