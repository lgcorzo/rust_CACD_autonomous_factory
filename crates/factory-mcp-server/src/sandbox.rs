use async_trait::async_trait;
use tokio::process::Command;
use serde::{Serialize, Deserialize};

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
}

pub struct SubprocessDriver;

#[async_trait]
impl SandboxDriver for SubprocessDriver {
    async fn execute(&self, code: &str, language: &str) -> anyhow::Result<ExecutionResult> {
        let output = match language {
            "python" => {
                Command::new("python3")
                    .arg("-c")
                    .arg(code)
                    .output()
                    .await?
            }
            "rust" => {
                // Simplified rustc execution for now
                Command::new("rustc")
                    .arg("-e")
                    .arg("println!(\"{}\", \"Hello from Rust!\")")
                    .output()
                    .await
                    .map_err(|_| anyhow::anyhow!("Rustc execution not fully implemented in local driver"))?
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
