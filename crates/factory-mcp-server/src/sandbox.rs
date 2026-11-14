use async_trait::async_trait;
#[cfg(any(test, feature = "test-insecure-host"))]
use factory_core::security_policy::WorkloadTrustLevel;
use factory_core::security_policy::{
    ExecutionEnvironment, ExecutionSecurityPolicy, SandboxIsolationType,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

/// Sandbox policy parameters requested in the Kubernetes Job/Pod specification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct RequestedSandboxSpec {
    pub runtime_class_name: Option<String>,
    #[serde(default)]
    pub runtime_handler: Option<String>,
    pub seccomp_profile: String,
    pub run_as_non_root: bool,
    pub run_as_user: Option<i64>,
    #[serde(default)]
    pub run_as_group: Option<i64>,
    pub read_only_root_filesystem: bool,
    pub allow_privilege_escalation: bool,
    pub capabilities_dropped: Vec<String>,
    #[serde(default)]
    pub image_digest: Option<String>,
    #[serde(default)]
    pub memory_limit: Option<String>,
    #[serde(default)]
    pub cpu_limit: Option<String>,
}

/// Runtime facts directly observed from the Kubernetes Pod status and container runtime.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ObservedRuntimeFacts {
    #[serde(default)]
    pub job_name: Option<String>,
    #[serde(default)]
    pub job_uid: Option<String>,
    pub node_name: Option<String>,
    pub pod_name: Option<String>,
    pub pod_uid: Option<String>,
    pub pod_phase: Option<String>,
    pub container_id: Option<String>,
    pub container_image_digest: Option<String>,
    #[serde(default)]
    pub termination_reason: Option<String>,
    pub exit_code: Option<i32>,
    #[serde(default)]
    pub observed_runtime_class_name: Option<String>,
    #[serde(default)]
    pub observed_run_as_non_root: Option<bool>,
    #[serde(default)]
    pub observed_run_as_user: Option<i64>,
    #[serde(default)]
    pub observed_read_only_root_filesystem: Option<bool>,
    #[serde(default)]
    pub observed_seccomp_profile: Option<String>,
    #[serde(default)]
    pub observed_capabilities_dropped: Vec<String>,
    /// Containerd runtime handler observed on the host node when inspected directly (`None` when only K8s Pod status API is queried).
    pub host_runtime_handler_observed: Option<String>,
}

/// Independently verified runtime properties and overall verification outcome (`VERIFIED` or `UNVERIFIED`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct VerifiedSandboxChecks {
    /// Either `"VERIFIED"` (when all mandatory runtime + host handler checks are corroborated) or `"UNVERIFIED"`.
    pub verification_status: String,
    pub effective_runtime_is_gvisor: bool,
    pub expected_image_digest_matched: bool,
    pub ran_as_non_root: bool,
    pub expected_capabilities_dropped: bool,
    pub root_filesystem_read_only: bool,
    pub expected_seccomp_applied: bool,
    pub negative_isolation_probes_passed: Option<bool>,
    #[serde(default)]
    pub failures: Vec<String>,
}

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
    /// Explicitly separates policy requested in the K8s API spec from node/status facts.
    #[serde(default)]
    pub requested_spec: RequestedSandboxSpec,
    /// Explicitly records only facts observed from live K8s Pod status or node CRI inspection.
    #[serde(default)]
    pub observed_runtime: ObservedRuntimeFacts,
    /// Independently evaluated verification checks comparing requested vs observed facts.
    #[serde(default)]
    pub verified_checks: VerifiedSandboxChecks,
}

impl SandboxAttestation {
    /// Evaluates whether the observed runtime facts independently corroborate the requested spec
    /// and expected Job name (`DG-RVP-014` §4.1–§4.3). Any missing, stale, or contradictory
    /// observation prevents `VERIFIED` and records `UNVERIFIED` with detailed failure reasons.
    pub fn evaluate_verification(
        &self,
        expected_job_name: Option<&str>,
        max_age_seconds: i64,
    ) -> VerifiedSandboxChecks {
        let mut failures = Vec::new();
        let req = &self.requested_spec;
        let obs = &self.observed_runtime;

        if let Some(exp_job) = expected_job_name {
            let actual_job = obs.job_name.as_deref().or(self.job_name.as_deref());
            if actual_job != Some(exp_job) {
                failures.push(format!(
                    "Job correlation mismatch: expected job '{}', observed {:?}",
                    exp_job, actual_job
                ));
            }
        }

        if obs.pod_uid.as_deref().unwrap_or_default().trim().is_empty() {
            failures.push("Missing observed pod_uid".to_string());
        }
        if obs
            .container_id
            .as_deref()
            .unwrap_or_default()
            .trim()
            .is_empty()
        {
            failures.push("Missing observed container_id".to_string());
        }

        if let Some(obs_at) = self.observed_at {
            let age = (chrono::Utc::now() - obs_at).num_seconds();
            if age > max_age_seconds {
                failures.push(format!(
                    "Stale attestation observation: age {}s exceeds max_age_seconds {}s",
                    age, max_age_seconds
                ));
            }
        } else {
            failures.push("Missing observed_at timestamp".to_string());
        }

        let rc_matched = match (&req.runtime_class_name, &obs.observed_runtime_class_name) {
            (Some(r), Some(o)) => r == o,
            (None, None) => true,
            _ => false,
        };
        if !rc_matched {
            failures.push(format!(
                "RuntimeClass mismatch: requested {:?}, observed {:?}",
                req.runtime_class_name, obs.observed_runtime_class_name
            ));
        }

        let handler_ok = match &obs.host_runtime_handler_observed {
            Some(h) if h.contains("runsc") => true,
            Some(h) => {
                failures.push(format!(
                    "Runtime handler mismatch: expected runsc/gVisor, observed '{}'",
                    h
                ));
                false
            }
            None => {
                failures.push(
                    "Runtime observation unavailable: host_runtime_handler_observed is None (K8s Pod API alone cannot prove host runsc handler)".to_string(),
                );
                false
            }
        };
        let effective_runtime_is_gvisor = rc_matched && handler_ok;

        let expected_image_digest_matched = match (&req.image_digest, &obs.container_image_digest) {
            (Some(req_img), Some(obs_img)) => {
                if let Some((_, sha_part)) = req_img.split_once("@sha256:") {
                    obs_img.contains(sha_part)
                } else {
                    req_img == obs_img
                }
            }
            (None, Some(obs_img)) => obs_img.contains("sha256:"),
            _ => false,
        };
        if !expected_image_digest_matched {
            failures.push(format!(
                "Unexpected image digest: requested {:?}, observed {:?}",
                req.image_digest, obs.container_image_digest
            ));
        }

        if obs.exit_code != Some(0) || obs.pod_phase.as_deref() != Some("Succeeded") {
            failures.push(format!(
                "Container termination failure: phase={:?}, exit_code={:?}",
                obs.pod_phase, obs.exit_code
            ));
        }

        let ran_as_non_root = req.run_as_non_root
            && obs.observed_run_as_non_root == Some(true)
            && obs.observed_run_as_user.unwrap_or(0) > 0;
        if !ran_as_non_root {
            failures.push(format!(
                "Non-root verification failed: requested run_as_non_root={}, observed non_root={:?}, uid={:?}",
                req.run_as_non_root, obs.observed_run_as_non_root, obs.observed_run_as_user
            ));
        }

        let expected_capabilities_dropped = obs
            .observed_capabilities_dropped
            .iter()
            .any(|c| c.eq_ignore_ascii_case("ALL"));
        if !expected_capabilities_dropped {
            failures.push(format!(
                "Capabilities drop mismatch: observed {:?}",
                obs.observed_capabilities_dropped
            ));
        }

        let root_filesystem_read_only =
            req.read_only_root_filesystem && obs.observed_read_only_root_filesystem == Some(true);
        if !root_filesystem_read_only {
            failures.push(format!(
                "Read-only rootfs mismatch: requested={}, observed={:?}",
                req.read_only_root_filesystem, obs.observed_read_only_root_filesystem
            ));
        }

        let expected_seccomp_applied =
            obs.observed_seccomp_profile.as_deref() == Some(req.seccomp_profile.as_str());
        if !expected_seccomp_applied {
            failures.push(format!(
                "Seccomp profile mismatch: requested '{}', observed {:?}",
                req.seccomp_profile, obs.observed_seccomp_profile
            ));
        }

        let verification_status = if failures.is_empty() {
            "VERIFIED".to_string()
        } else {
            "UNVERIFIED".to_string()
        };

        VerifiedSandboxChecks {
            verification_status,
            effective_runtime_is_gvisor,
            expected_image_digest_matched,
            ran_as_non_root,
            expected_capabilities_dropped,
            root_filesystem_read_only,
            expected_seccomp_applied,
            negative_isolation_probes_passed: self.verified_checks.negative_isolation_probes_passed,
            failures,
        }
    }
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

    #[cfg(any(test, feature = "test-insecure-host"))]
    pub fn with_allow_insecure_host_execution(mut self, allow: bool) -> Self {
        if allow && self.policy.environment != ExecutionEnvironment::Production {
            self.policy.trust_level = WorkloadTrustLevel::TrustedInternal;
            self.policy.isolation = SandboxIsolationType::HostSubprocessAllowedOnlyForTrustedDev;
        }
        self
    }

    #[cfg(not(any(test, feature = "test-insecure-host")))]
    pub fn with_allow_insecure_host_execution(self, _allow: bool) -> Self {
        // B1 (DG-RVP-018): Host execution override is disabled in non-test builds.
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

        let mut parsed_attestation: Option<SandboxAttestation> = None;
        let mut stdout_parts = Vec::new();
        for c in &result.content {
            if let crate::protocol::McpContent::Text { text } = c {
                if let Some(att_str) = text.strip_prefix("\n[Attestation]\n") {
                    parsed_attestation = serde_json::from_str::<SandboxAttestation>(att_str).ok();
                } else {
                    stdout_parts.push(text.clone());
                }
            }
        }
        let output_text = stdout_parts.join("\n");
        let exit_code = parsed_attestation
            .as_ref()
            .and_then(|a| a.exit_code)
            .or(if result.is_error { Some(1) } else { Some(0) });

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
            exit_code,
            is_success: !result.is_error,
            attestation: parsed_attestation,
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
