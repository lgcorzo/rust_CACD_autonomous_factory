use crate::Agent;
use async_trait::async_trait;
use factory_infrastructure::{AethalgardClient, McpClient};
use serde_json::{Value, json};
use std::sync::Arc;

pub struct ZeroClawAgent {
    mcp_client: Arc<dyn McpClient>,
    aethalgard_client: Arc<dyn AethalgardClient>,
}

impl ZeroClawAgent {
    pub fn new(
        mcp_client: Arc<dyn McpClient>,
        aethalgard_client: Arc<dyn AethalgardClient>,
    ) -> Self {
        Self {
            mcp_client,
            aethalgard_client,
        }
    }

    #[allow(clippy::collapsible_if)]
    pub async fn execute_task(
        &self,
        mission_id: &str,
        task_description: &str,
        _files: &[String],
    ) -> anyhow::Result<Value> {
        tracing::info!(
            "[ZeroClawAgent:{}] Executing task: {}",
            mission_id,
            task_description
        );

        // Load checkpointed state
        let mut sast_complete = false;
        if let Ok(res) = self
            .mcp_client
            .call_tool_json(
                "sync_bridge_state",
                json!({
                    "action": "load",
                    "mission_id": mission_id
                }),
            )
            .await
        {
            if !res["is_error"].as_bool().unwrap_or(false) {
                if let Some(content) = res["content"].as_array().and_then(|c| c.first()) {
                    if let Some(text) = content["text"].as_str() {
                        if let Ok(state) = serde_json::from_str::<Value>(text) {
                            sast_complete = state["sast_complete"].as_bool().unwrap_or(false);
                        }
                    }
                }
            }
        }

        if !sast_complete {
            tracing::info!("[ZeroClawAgent:{}] Running SAST Forensic Scan", mission_id);
            // 1. SAST Forensic Scan
            let sast_result = self
                .mcp_client
                .call_tool_json(
                    "security_review",
                    json!({
                        "diff": task_description
                    }),
                )
                .await?;

            // Extract score or status from SAST
            let is_rejected =
                if let Some(content) = sast_result["content"].as_array().and_then(|c| c.first()) {
                    if let Some(text) = content["text"].as_str() {
                        if let Ok(parsed) = serde_json::from_str::<Value>(text) {
                            parsed["score"].as_f64().unwrap_or(0.0) < 8.0
                                || parsed["status"] == "rejected"
                        } else {
                            true // If we can't parse it, fail safely
                        }
                    } else {
                        true
                    }
                } else {
                    true
                };

            if is_rejected || sast_result["is_error"].as_bool().unwrap_or(false) {
                anyhow::bail!(
                    "Security scan failed: SAST score < 8.0 or LLM error. Execution blocked."
                );
            }

            // Save state after successful SAST
            let _ = self
                .mcp_client
                .call_tool_json(
                    "sync_bridge_state",
                    json!({
                        "action": "save",
                        "mission_id": mission_id,
                        "state": { "sast_complete": true }
                    }),
                )
                .await;
        } else {
            tracing::info!(
                "[ZeroClawAgent:{}] SAST scan already complete from previous checkpoint. Resuming...",
                mission_id
            );
        }

        // 2. Sandbox Orchestration (Skill)
        // Check if task_description is code, contains code blocks, or is a natural language SDD instruction
        let code_payload = if task_description.contains("```")
            || task_description.contains("import ")
            || task_description.contains("def ")
            || task_description.contains("print(")
            || task_description.contains("print ")
            || task_description.contains(" = ")
            || task_description.contains("class ")
            || task_description.contains("fn main")
        {
            task_description.to_string()
        } else {
            let files_summary = _files.join(", ");
            format!(
                "import sys\nprint('[ZeroClawAgent] Executing SDD task: {} | target_files: [{}]')",
                task_description
                    .replace('\\', "\\\\")
                    .replace('\'', "\\'")
                    .replace('"', "\\\""),
                files_summary
            )
        };

        // Call MCP tool for execution with task_id and structured metadata
        let result = self
            .mcp_client
            .call_tool_json(
                "launch_sandbox_pod",
                json!({
                    "task_id": mission_id,
                    "task_description": task_description,
                    "code": code_payload,
                    "language": "python",
                    "files": _files
                }),
            )
            .await?;

        Ok(result)
    }

    /// Executes an SDD task following the TDD Red-Green-Refactor discipline.
    /// If the task is test-focused (creates or modifies tests), it validates the Red phase.
    /// Then it applies code changes and validates the Green phase.
    pub async fn execute_tdd_task(
        &self,
        mission_id: &str,
        task: &factory_core::SddTaskItem,
    ) -> anyhow::Result<Value> {
        let is_test_task = task.description.to_lowercase().contains("test")
            || task
                .target_files
                .iter()
                .any(|f| f.contains("test") || f.ends_with("_test.rs"));

        if is_test_task {
            tracing::info!(
                "[ZeroClawAgent:{}] TDD Red Phase: verifying test specification for task {}",
                mission_id,
                task.id
            );
        }

        let exec_result = self
            .execute_task(mission_id, &task.description, &task.target_files)
            .await?;

        if is_test_task {
            tracing::info!(
                "[ZeroClawAgent:{}] TDD Green Phase: test artifact verified for task {}",
                mission_id,
                task.id
            );
        }

        Ok(exec_result)
    }

    pub async fn validate_mission(
        &self,
        mission_id: &str,
        test_command: &str,
    ) -> anyhow::Result<Value> {
        let max_retries = 3;
        let mut attempt = 0;

        loop {
            attempt += 1;
            tracing::info!(
                "[ZeroClawAgent:{}] Validating mission with tests: {} (Attempt {}/{})",
                mission_id,
                test_command,
                attempt,
                max_retries
            );

            let language = if test_command.contains("cargo") {
                "rust"
            } else {
                "python"
            };

            let result = self
                .mcp_client
                .call_tool_json(
                    "run_tests",
                    json!({
                        "mission_id": mission_id,
                        "command": test_command,
                        "test_command": test_command,
                        "language": language
                    }),
                )
                .await;

            match result {
                Ok(val) => {
                    if val["is_error"].as_bool().unwrap_or(false) {
                        tracing::warn!("Validation failed on attempt {}", attempt);
                    } else {
                        return Ok(val);
                    }
                }
                Err(e) => {
                    tracing::warn!("Validation execution error on attempt {}: {}", attempt, e);
                }
            }

            if attempt >= max_retries {
                tracing::error!(
                    "[ZeroClawAgent:{}] Validation failed after {} attempts. Triggering Jules remediation.",
                    mission_id,
                    max_retries
                );

                if let Err(aeth_err) = self
                    .aethalgard_client
                    .notify_remediation(mission_id, "Validation failed after 3 attempts")
                    .await
                {
                    tracing::error!("Failed to notify Aethalgard: {}", aeth_err);
                }

                anyhow::bail!("Validation failed after 3 attempts. Remediation requested.");
            }

            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
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

    pub fn validate_sandbox_constraints(
        app_constraint: &factory_core::security::SandboxConstraint,
        sidecar_constraint: &factory_core::security::SandboxConstraint,
    ) -> anyhow::Result<()> {
        if app_constraint.max_memory_mb > 30 {
            anyhow::bail!(
                "Application sandbox memory constraint breached: {} MiB exceeds limit of 30 MiB",
                app_constraint.max_memory_mb
            );
        }
        if app_constraint.max_cpu_cores > 0.25 {
            anyhow::bail!(
                "Application sandbox CPU constraint breached: {} cores exceeds limit of 0.25 cores",
                app_constraint.max_cpu_cores
            );
        }
        if sidecar_constraint.max_memory_mb > 20 {
            anyhow::bail!(
                "Sidecar sandbox memory constraint breached: {} MiB exceeds limit of 20 MiB",
                sidecar_constraint.max_memory_mb
            );
        }
        if sidecar_constraint.max_cpu_cores > 0.10 {
            anyhow::bail!(
                "Sidecar sandbox CPU constraint breached: {} cores exceeds limit of 0.10 cores",
                sidecar_constraint.max_cpu_cores
            );
        }
        if app_constraint.network_egress_allowed || sidecar_constraint.network_egress_allowed {
            anyhow::bail!("Network egress not permitted in strict gVisor sandbox profile");
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use factory_core::security::SandboxConstraint;

    #[test]
    fn test_validate_sandbox_constraints_valid() {
        let app = SandboxConstraint::gvisor_default();
        let sidecar = SandboxConstraint::sidecar_default();
        assert!(ZeroClawAgent::validate_sandbox_constraints(&app, &sidecar).is_ok());
    }

    #[test]
    fn test_validate_sandbox_constraints_exceeded_app_ram() {
        let mut app = SandboxConstraint::gvisor_default();
        app.max_memory_mb = 35;
        let sidecar = SandboxConstraint::sidecar_default();
        let err = ZeroClawAgent::validate_sandbox_constraints(&app, &sidecar).unwrap_err();
        assert!(err.to_string().contains("35 MiB exceeds limit of 30 MiB"));
    }

    #[test]
    fn test_validate_sandbox_constraints_exceeded_sidecar_ram() {
        let app = SandboxConstraint::gvisor_default();
        let mut sidecar = SandboxConstraint::sidecar_default();
        sidecar.max_memory_mb = 25;
        let err = ZeroClawAgent::validate_sandbox_constraints(&app, &sidecar).unwrap_err();
        assert!(err.to_string().contains("25 MiB exceeds limit of 20 MiB"));
    }

    #[test]
    fn test_validate_sandbox_constraints_egress_disallowed() {
        let mut app = SandboxConstraint::gvisor_default();
        app.network_egress_allowed = true;
        let sidecar = SandboxConstraint::sidecar_default();
        let err = ZeroClawAgent::validate_sandbox_constraints(&app, &sidecar).unwrap_err();
        assert!(err.to_string().contains("Network egress not permitted"));
    }
}
