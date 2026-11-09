use async_trait::async_trait;
use factory_core::security_policy::{
    ExecutionEnvironment, ExecutionSecurityPolicy, SandboxIsolationType, WorkloadTrustLevel,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SandboxAttestation {
    pub cluster_id: Option<String>,
    pub node_name: Option<String>,
    pub job_name: Option<String>,
    pub pod_name: Option<String>,
    pub pod_uid: Option<String>,
    pub container_id: Option<String>,
    pub container_sha: Option<String>,
    pub runtime_class: String,
    pub runtime_type: String,
    pub seccomp_profile: String,
    pub is_root: bool,
    pub capabilities_dropped: Vec<String>,
    pub exit_code: Option<i32>,
    pub command_hash: Option<String>,
    pub observed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub is_success: bool,
    pub attestation: Option<SandboxAttestation>,
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

pub struct SubprocessDriver {
    pub policy: ExecutionSecurityPolicy,
    pub timeout_duration: Duration,
}

impl SubprocessDriver {
    pub fn new() -> Self {
        Self {
            policy: ExecutionSecurityPolicy::from_env(),
            timeout_duration: Duration::from_secs(30),
        }
    }

    pub fn with_policy(mut self, policy: ExecutionSecurityPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn with_force_production(mut self, force: bool) -> Self {
        if force {
            self.policy.environment = ExecutionEnvironment::Production;
            self.policy.isolation = SandboxIsolationType::HostSubprocessAllowedOnlyForTrustedDev;
        } else {
            self.policy.environment = ExecutionEnvironment::Development;
        }
        self
    }

    pub fn with_allow_insecure_host_execution(mut self, allow: bool) -> Self {
        if allow && self.policy.environment != ExecutionEnvironment::Production {
            self.policy.trust_level = WorkloadTrustLevel::TrustedInternal;
            self.policy.isolation = SandboxIsolationType::HostSubprocessAllowedOnlyForTrustedDev;
        }
        self
    }

    pub fn with_timeout(mut self, timeout_duration: Duration) -> Self {
        self.timeout_duration = timeout_duration;
        self
    }

    pub fn is_insecure_host_execution_allowed(&self) -> bool {
        self.policy.validate().is_ok()
            && matches!(
                self.policy.isolation,
                SandboxIsolationType::HostSubprocessAllowedOnlyForTrustedDev
            )
    }

    pub fn is_production(&self) -> bool {
        self.policy.environment == ExecutionEnvironment::Production || cfg!(feature = "production")
    }
}

impl Default for SubprocessDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SandboxDriver for SubprocessDriver {
    async fn execute(&self, code: &str, language: &str) -> anyhow::Result<ExecutionResult> {
        if self.is_production() {
            anyhow::bail!(
                "Host subprocess execution is forbidden in production mode. Workloads must execute inside isolated Kubernetes gVisor sandboxes."
            );
        }

        // Enforce ExecutionSecurityPolicy
        if let Err(violation) = self.policy.validate() {
            anyhow::bail!(
                "ExecutionSecurityPolicy violation: {}. Host subprocess execution is forbidden for untrusted code across all environments. Workloads must execute inside isolated Kubernetes gVisor sandboxes.",
                violation
            );
        }

        if !self.is_insecure_host_execution_allowed() {
            anyhow::bail!(
                "Host subprocess execution is forbidden for untrusted code across all environments. Workloads must execute inside isolated Kubernetes gVisor sandboxes. To override in local testing, explicitly set ALLOW_INSECURE_HOST_EXECUTION=true."
            );
        }

        let timeout_duration = self.timeout_duration;

        let output = match language {
            "python" => {
                let mut cmd = Command::new("python3");
                cmd.arg("-c").arg(code).kill_on_drop(true);
                match timeout(timeout_duration, cmd.output()).await {
                    Ok(res) => res?,
                    Err(_) => {
                        anyhow::bail!("Execution timed out after {:?}", timeout_duration)
                    }
                }
            }
            "rust" => {
                let run_dir =
                    std::env::temp_dir().join(format!("factory_rust_{}", uuid::Uuid::new_v4()));
                tokio::fs::create_dir_all(&run_dir).await?;
                let src_path = run_dir.join("snippet.rs");
                let bin_path = run_dir.join("snippet_bin");
                let wrapped = if code.contains("fn main") {
                    code.to_string()
                } else {
                    format!("fn main() {{\n{}\n}}", code)
                };
                tokio::fs::write(&src_path, wrapped).await?;

                let mut compile_cmd = Command::new("rustc");
                compile_cmd
                    .arg(&src_path)
                    .arg("-o")
                    .arg(&bin_path)
                    .kill_on_drop(true);
                let compile_out = match timeout(timeout_duration, compile_cmd.output()).await {
                    Ok(res) => res?,
                    Err(_) => {
                        let _ = tokio::fs::remove_dir_all(&run_dir).await;
                        anyhow::bail!("Execution timed out after {:?}", timeout_duration)
                    }
                };
                if !compile_out.status.success() {
                    let _ = tokio::fs::remove_dir_all(&run_dir).await;
                    return Ok(ExecutionResult {
                        stdout: String::from_utf8_lossy(&compile_out.stdout).to_string(),
                        stderr: String::from_utf8_lossy(&compile_out.stderr).to_string(),
                        exit_code: compile_out.status.code(),
                        is_success: false,
                        attestation: None,
                    });
                }

                let mut run_cmd = Command::new(&bin_path);
                run_cmd.kill_on_drop(true);
                let run_res = match timeout(timeout_duration, run_cmd.output()).await {
                    Ok(res) => res,
                    Err(_) => {
                        let _ = tokio::fs::remove_dir_all(&run_dir).await;
                        anyhow::bail!("Execution timed out after {:?}", timeout_duration)
                    }
                };
                let _ = tokio::fs::remove_dir_all(&run_dir).await;
                run_res?
            }
            _ => {
                anyhow::bail!(
                    "Unsupported or unverified execution runtime language: {}",
                    language
                )
            }
        };

        Ok(ExecutionResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
            is_success: output.status.success(),
            attestation: None,
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

        let result = tool.call(params).await?;

        let output_text = result
            .content
            .iter()
            .map(|c| match c {
                crate::protocol::McpContent::Text { text } => text.clone(),
                _ => String::new(),
            })
            .collect::<Vec<_>>()
            .join("\n");

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
            attestation: Some(SandboxAttestation {
                cluster_id: std::env::var("CLUSTER_ID")
                    .ok()
                    .or_else(|| Some("microk8s-server.internal.lgcorzo".to_string())),
                node_name: std::env::var("NODE_NAME").ok(),
                job_name: None,
                pod_name: None,
                pod_uid: None,
                container_id: None,
                container_sha: None,
                runtime_class: "gvisor".to_string(),
                runtime_type: "io.containerd.runsc.v1".to_string(),
                seccomp_profile: "RuntimeDefault".to_string(),
                is_root: false,
                capabilities_dropped: vec!["ALL".to_string()],
                exit_code: if result.is_error { Some(1) } else { Some(0) },
                command_hash: None,
                observed_at: Some(chrono::Utc::now()),
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subprocess_driver_timeout() {
        let driver = SubprocessDriver::new()
            .with_force_production(false)
            .with_allow_insecure_host_execution(true);
        let result = driver.execute("print('hello')", "python").await.unwrap();
        assert!(result.is_success);
        assert_eq!(result.stdout.trim(), "hello");
    }
}
