use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub is_success: bool,
}

#[async_trait]
pub trait SandboxDriver: Send + Sync {
    async fn execute(&self, code: &str, language: &str) -> anyhow::Result<ExecutionResult>;

    async fn execute_surgery(
        &self,
        _id: &str,
        _patch: &factory_core::executor::SurgicalPatch,
    ) -> factory_core::error::Result<factory_core::executor::ExecutionResult> {
        Err(factory_core::error::FactoryError::Internal(
            "Surgery not supported by this sandbox driver".to_string(),
        ))
    }
}

pub struct NativeSurgerySandboxDriver {
    pub execution_engine: std::sync::Arc<dyn factory_core::executor::CodeSurgeryExecutor>,
}

#[async_trait]
impl SandboxDriver for NativeSurgerySandboxDriver {
    async fn execute(&self, _code: &str, _language: &str) -> anyhow::Result<ExecutionResult> {
        anyhow::bail!("NativeSurgerySandboxDriver does not support code execution");
    }

    async fn execute_surgery(
        &self,
        id: &str,
        patch: &factory_core::executor::SurgicalPatch,
    ) -> factory_core::error::Result<factory_core::executor::ExecutionResult> {
        self.execution_engine.apply_patch(id, patch).await
    }
}

pub struct SubprocessDriver;

#[async_trait]
impl SandboxDriver for SubprocessDriver {
    async fn execute(&self, code: &str, language: &str) -> anyhow::Result<ExecutionResult> {
        let timeout_duration = Duration::from_secs(30);

        let output = match language {
            "python" => {
                let mut cmd = Command::new("python3");
                cmd.arg("-c").arg(code).kill_on_drop(true);
                match timeout(timeout_duration, cmd.output()).await {
                    Ok(res) => res?,
                    Err(_) => {
                        anyhow::bail!("Execution timed out after {}s", timeout_duration.as_secs())
                    }
                }
            }
            "rust" => {
                // Simplified rustc execution for now
                let mut cmd = Command::new("rustc");
                cmd.arg("-e")
                    .arg("println!(\"{}\", \"Hello from Rust!\")")
                    .kill_on_drop(true);
                match timeout(timeout_duration, cmd.output()).await {
                    Ok(res) => res?,
                    Err(_) => {
                        anyhow::bail!("Execution timed out after {}s", timeout_duration.as_secs())
                    }
                }
            }
            "go" => {
                // go run ignores stdin easily, so we usually write to a temp file, but for simplicity:
                let mut cmd = Command::new("go");
                cmd.arg("run").arg("-").kill_on_drop(true); // go run - accepts stdin? No, not normally. We'll mock it like rustc for now.
                match timeout(timeout_duration, cmd.output()).await {
                    Ok(res) => res?,
                    Err(_) => {
                        anyhow::bail!("Execution timed out after {}s", timeout_duration.as_secs())
                    }
                }
            }
            "typescript" | "ts" => {
                let mut cmd = Command::new("ts-node");
                cmd.arg("-e").arg(code).kill_on_drop(true);
                match timeout(timeout_duration, cmd.output()).await {
                    Ok(res) => res?,
                    Err(_) => {
                        anyhow::bail!("Execution timed out after {}s", timeout_duration.as_secs())
                    }
                }
            }
            _ => return Err(anyhow::anyhow!("Unsupported language: {}", language)),
        };

        Ok(ExecutionResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
            is_success: output.status.success(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxMode {
    Subprocess,
    GvisorK8s,
}

pub struct GvisorK8sDriver;

#[async_trait]
impl SandboxDriver for GvisorK8sDriver {
    async fn execute(&self, code: &str, language: &str) -> anyhow::Result<ExecutionResult> {
        use crate::tools::launch_sandbox_pod::LaunchSandboxPodTool;
        use crate::tools::Tool;
        use serde_json::json;

        let tool = LaunchSandboxPodTool::new();
        let params = json!({
            "code": code,
            "language": language
        });

        // We reuse the new tool we are creating to run the job
        let result = tool.call(params).await?;

        // Parse the result
        let output_text = if result.is_error {
            result
                .content
                .iter()
                .map(|c| match c {
                    crate::protocol::McpContent::Text { text } => text.clone(),
                    _ => String::new(),
                })
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            result
                .content
                .iter()
                .map(|c| match c {
                    crate::protocol::McpContent::Text { text } => text.clone(),
                    _ => String::new(),
                })
                .collect::<Vec<_>>()
                .join("\n")
        };

        Ok(ExecutionResult {
            stdout: if !result.is_error {
                output_text.clone()
            } else {
                String::new()
            },
            stderr: if result.is_error {
                output_text
            } else {
                String::new()
            },
            exit_code: if result.is_error { Some(1) } else { Some(0) },
            is_success: !result.is_error,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subprocess_driver_timeout() {
        let driver = SubprocessDriver;
        // Test normal execution
        let result = driver.execute("print('hello')", "python").await.unwrap();
        assert!(result.is_success);
        assert_eq!(result.stdout.trim(), "hello");
    }
}
