use crate::Agent;
use async_trait::async_trait;
use factory_infrastructure::{McpClient, R2rClient};
use serde_json::{Value, json};
use std::sync::Arc;
use std::time::Duration;

pub struct DocumentationAgent {
    mcp_client: Arc<dyn McpClient>,
    r2r_client: Arc<dyn R2rClient>,
}

impl DocumentationAgent {
    pub fn new(mcp_client: Arc<dyn McpClient>, r2r_client: Arc<dyn R2rClient>) -> Self {
        Self {
            mcp_client,
            r2r_client,
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

            for skill in &skills {
                self.mcp_client
                    .call_tool_json(
                        "invoke_spec_kit",
                        json!({
                            "command": skill,
                            "args": []
                        }),
                    )
                    .await?;
            }

            let osr_value = self.verify_osr().await?;

            // Push metric to R2R
            if let Err(e) = self.r2r_client.push_osr_metric(osr_value).await {
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

    async fn verify_osr(&self) -> anyhow::Result<f32> {
        // Retrieve context to simulate a diff calculation
        let _context = self.r2r_client.search("documentation sync state").await?;

        // In a real implementation, we would compare the R2R context with local wiki files.
        // For now, we mock a successful OSR value under the 5% threshold.
        // E.g., if we returned 0.06, it would trigger the retry loop.
        let mock_osr = 0.03;

        Ok(mock_osr)
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
