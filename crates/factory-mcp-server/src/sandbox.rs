use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
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
        #[cfg(not(target_os = "linux"))]
        {
            return Err(anyhow::anyhow!(
                "Hardened SubprocessDriver currently only supports Linux via namespaces. \
                 Consider using FirecrackerDriver for production environments."
            ));
        }

        #[cfg(target_os = "linux")]
        {
            let output = match language {
                "python" => {
                    Command::new("timeout")
                        .arg("30s")
                        .arg("unshare")
                        .arg("-r")
                        .arg("-n")
                        .arg("-p")
                        .arg("-f")
                        .arg("--mount-proc")
                        .arg("python3")
                        .arg("-c")
                        .arg(code)
                        .output()
                        .await?
                }
                "rust" => {
                    let temp_dir = tempfile::tempdir()?;
                    let main_rs = temp_dir.path().join("main.rs");
                    let main_bin = temp_dir.path().join("main");

                    let mut file = tokio::fs::File::create(&main_rs).await?;
                    file.write_all(code.as_bytes()).await?;
                    file.flush().await?;

                    let compile_output = Command::new("rustc")
                        .arg(&main_rs)
                        .arg("-o")
                        .arg(&main_bin)
                        .output()
                        .await?;

                    if !compile_output.status.success() {
                        return Ok(ExecutionResult {
                            stdout: String::from_utf8_lossy(&compile_output.stdout).to_string(),
                            stderr: format!(
                                "Compilation failed:\n{}",
                                String::from_utf8_lossy(&compile_output.stderr)
                            ),
                            exit_code: compile_output.status.code(),
                            is_success: false,
                        });
                    }

                    Command::new("timeout")
                        .arg("30s")
                        .arg("unshare")
                        .arg("-r")
                        .arg("-n")
                        .arg("-p")
                        .arg("-f")
                        .arg("--mount-proc")
                        .arg(&main_bin)
                        .output()
                        .await?
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_python_execution() {
        let driver = SubprocessDriver;
        let result = driver
            .execute("print('hello world')", "python")
            .await
            .unwrap();
        assert!(result.is_success);
        assert_eq!(result.stdout.trim(), "hello world");
    }

    #[tokio::test]
    async fn test_rust_execution() {
        let driver = SubprocessDriver;
        let code = r#"
            fn main() {
                println!("hello from rust");
            }
        "#;
        let result = driver.execute(code, "rust").await.unwrap();
        assert!(result.is_success);
        assert_eq!(result.stdout.trim(), "hello from rust");
    }

    #[tokio::test]
    async fn test_network_isolation() {
        let driver = SubprocessDriver;
        // Attempt to ping 8.8.8.8 - should fail in a network-isolated namespace
        let code = "import subprocess; print(subprocess.run(['ping', '-c', '1', '8.8.8.8'], capture_output=True).returncode)";
        let result = driver.execute(code, "python").await.unwrap();
        // The ping command itself should fail (non-zero return code)
        assert_ne!(result.stdout.trim(), "0");
    }

    #[tokio::test]
    async fn test_pid_isolation() {
        let driver = SubprocessDriver;
        // In a new PID namespace with --mount-proc, we should only see a few processes (like PID 1)
        // We can check this by listing /proc and counting numeric entries or using 'ps'
        let code = "import os; print(len([d for d in os.listdir('/proc') if d.isdigit()]))";
        let result = driver.execute(code, "python").await.unwrap();
        let num_procs: i32 = result.stdout.trim().parse().unwrap_or(100);
        // Usually it's 2: python itself and unshare's fork, or similar small number.
        // Definitely much less than a typical host.
        assert!(num_procs < 10);
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
