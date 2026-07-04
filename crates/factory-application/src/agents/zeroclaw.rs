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
        let is_rejected = if let Some(content) =
            sast_result["content"].as_array().and_then(|c| c.first())
        {
            if let Some(text) = content["text"].as_str() {
                if let Ok(parsed) = serde_json::from_str::<Value>(text) {
                    parsed["score"].as_f64().unwrap_or(0.0) < 8.0 || parsed["status"] == "rejected"
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

        // 2. Sandbox Orchestration (Skill)
        // Call MCP tool for execution
        let result = self
            .mcp_client
            .call_tool_json(
                "execute_code",
                json!({
                    "code": task_description,
                    "language": "python" // Assume python for now, or detect
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

            let result = self
                .mcp_client
                .call_tool_json(
                    "run_tests",
                    json!({
                        "mission_id": mission_id,
                        "command": test_command
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
