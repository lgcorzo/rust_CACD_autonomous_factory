use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

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
            "python" => Command::new("python3").arg("-c").arg(code).output().await?,
            "rust" => {
                // Simplified rustc execution for now
                Command::new("rustc")
                    .arg("-e")
                    .arg("println!(\"{}\", \"Hello from Rust!\")")
                    .output()
                    .await
                    .map_err(|_| {
                        anyhow::anyhow!("Rustc execution not fully implemented in local driver")
                    })?
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

pub struct FirecrackerDriver {
    kvm_enabled: bool,
}

impl Default for FirecrackerDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl FirecrackerDriver {
    pub fn new() -> Self {
        Self { kvm_enabled: true }
    }
}

#[async_trait]
impl SandboxDriver for FirecrackerDriver {
    async fn execute(&self, code: &str, language: &str) -> anyhow::Result<ExecutionResult> {
        tracing::info!(
            "[FirecrackerDriver] Spawning micro-VM for {} execution",
            language
        );

        if !self.kvm_enabled {
            anyhow::bail!("KVM not enabled, cannot spawn Firecracker VM");
        }

        // Implementation detail:
        // 1. Create VM config (microvm.json)
        // 2. Start firecracker process
        // 3. Inyect code via API or vsock
        // 4. Capture output and return

        // For now, we mock the success of the VM execution
        Ok(ExecutionResult {
            stdout: format!(
                "Simulation of {} executed in Firecracker VM: {}",
                language, code
            ),
            stderr: "".to_string(),
            exit_code: Some(0),
            is_success: true,
        })
    }
}
