use crate::Agent;
use async_trait::async_trait;
use factory_infrastructure::{McpClient, R2rClient};
use serde_json::{Value, json};
use std::sync::Arc;
use std::time::Duration;

pub struct DocumentationAgent {
    mcp_client: Arc<dyn McpClient>,
    r2r_client: Arc<dyn R2rClient>,
    superpowers_skills_root: std::path::PathBuf,
}

impl DocumentationAgent {
    pub fn new(
        mcp_client: Arc<dyn McpClient>,
        r2r_client: Arc<dyn R2rClient>,
        superpowers_skills_root: std::path::PathBuf,
    ) -> Self {
        Self {
            mcp_client,
            r2r_client,
            superpowers_skills_root,
        }
    }

    pub async fn run_post_merge_pipeline(&self, mission_id: &str) -> anyhow::Result<Value> {
        tracing::info!(
            "[DocumentationAgent:{}] Starting post-merge documentation pipeline",
            mission_id
        );

        let skills = vec![
            "using-superpowers",
            "updating-c4-models",
            "writing-wiki-markdown",
            "subagent-driven-development",
            "verification-before-completion",
            "finishing-a-development-branch",
        ];

        let max_retries = 2;
        let mut attempt = 0;

        loop {
            attempt += 1;
            tracing::info!(
                "Documentation pipeline attempt {}/{}",
                attempt,
                max_retries + 1
            );

            if attempt == 1 {
                for skill in &skills {
                    self.mcp_client
                        .call_tool_json(
                            "invoke_spec_kit",
                            json!({
                                "command": skill,
                                "args": [self.superpowers_skills_root.to_string_lossy().to_string()]
                            }),
                        )
                        .await?;
                }
            } else {
                self.mcp_client
                    .call_tool_json(
                        "invoke_spec_kit",
                        json!({
                            "command": "subagent-driven-development",
                            "args": [self.superpowers_skills_root.to_string_lossy().to_string()]
                        }),
                    )
                    .await?;
            }

            let osr_value = self.verify_osr().await?;

            let commit_sha = std::process::Command::new("git")
                .args(["rev-parse", "HEAD"])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| "unknown".to_string());

            let metric = factory_core::OsrMetric {
                mission_id: mission_id.to_string(),
                osr_value,
                wiki_commit_sha: commit_sha,
                timestamp: chrono::Utc::now(),
            };

            // Push metric to R2R
            if let Err(e) = self.r2r_client.push_osr_metric(&metric).await {
                tracing::warn!("Failed to push OSR metric: {}", e);
            }

            if osr_value <= 0.05 {
                tracing::info!("OSR validation passed with {}%", osr_value * 100.0);
                return Ok(json!({
                    "status": "success",
                    "osr": osr_value
                }));
            } else {
                tracing::warn!("OSR validation failed with {}%", osr_value * 100.0);
                if attempt > max_retries {
                    tracing::error!(
                        "HITL Escalation: OSR remained > 5% after {} retries",
                        max_retries
                    );
                    let _ = self
                        .mcp_client
                        .call_tool_json(
                            "update_mission_status",
                            json!({
                                "mission_id": mission_id,
                                "status": "doc_escalation"
                            }),
                        )
                        .await;
                    anyhow::bail!(
                        "HITL Escalation: Documentation remains out of sync (OSR: {})",
                        osr_value
                    );
                }

                // Simulate wait before retry
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    #[allow(clippy::collapsible_if)]
    async fn verify_osr(&self) -> anyhow::Result<f32> {
        let context = self.r2r_client.search("documentation sync state").await?;
        let r2r_text = serde_json::to_string(&context).unwrap_or_default();

        let mut wiki_content = String::new();
        if let Ok(entries) = std::fs::read_dir("wiki") {
            for entry in entries.flatten() {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("md") {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        wiki_content.push_str(&content);
                    }
                }
            }
        }

        // DESIGN DECISION: Using Levenshtein distance for OSR calculation.
        // In the future, this can be changed to a more advanced approach using LiteLLM
        // to analyze the diff and return a confidence penalty.
        let osr = crate::utils::osr::calculate_osr(&wiki_content, &r2r_text);

        Ok(osr)
    }
}

#[async_trait]
impl Agent for DocumentationAgent {
    fn name(&self) -> String {
        "documentation_agent".to_string()
    }

    async fn execute(&self, task_description: &str) -> anyhow::Result<Value> {
        self.run_post_merge_pipeline(task_description).await
    }
}
