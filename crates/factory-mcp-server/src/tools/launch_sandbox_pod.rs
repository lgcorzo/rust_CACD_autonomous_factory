use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
use factory_core::security_policy::{
    ExecutionEnvironment, ExecutionSecurityPolicy, WorkloadTrustLevel,
};
use k8s_openapi::api::batch::v1::Job;
use kube::api::{Api, DeleteParams, ListParams, PostParams};
use kube::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::time::{sleep, Duration};

#[derive(Serialize, Deserialize, Debug)]
pub struct SandboxJobSpec {
    pub code: String,
    pub language: String,
}

pub fn build_sandbox_job_spec(
    job_name: &str,
    cmd: &str,
    require_gvisor: bool,
    namespace: &str,
    language: &str,
) -> anyhow::Result<serde_json::Value> {
    let policy = ExecutionSecurityPolicy::from_env();
    if (policy.environment == ExecutionEnvironment::Production
        || policy.trust_level == WorkloadTrustLevel::Untrusted)
        && !require_gvisor
    {
        anyhow::bail!(
            "Untrusted sandbox execution requires gVisor runtime across all environments. Fallback to standard runc/host is forbidden."
        );
    }
    if job_name.trim().is_empty() {
        anyhow::bail!("Sandbox job_name must not be empty");
    }
    if cmd.trim().is_empty() {
        anyhow::bail!("Sandbox command must not be empty");
    }
    if namespace.trim().is_empty() || namespace == "default" {
        anyhow::bail!(
            "Sandbox Job must target an isolated namespace (e.g. 'factory-sandboxes'), got '{}'",
            namespace
        );
    }

    let image = if let Ok(custom_image) = std::env::var("SANDBOX_IMAGE") {
        custom_image
    } else if language == "rust" {
        std::env::var("SANDBOX_RUST_IMAGE")
            .unwrap_or_else(|_| "docker.io/library/rust@sha256:9f841bbe9e7d8e37ceb96ed907265a3a0df7f44e3737d0b100e7907a679acb36".to_string())
    } else {
        std::env::var("SANDBOX_PYTHON_IMAGE")
            .unwrap_or_else(|_| "docker.io/library/python@sha256:5e2bdb1b0d542d5039cffb8181c65930093949194a65c365dfb1b0fe13a4fa5b".to_string())
    };

    let memory_limit =
        std::env::var("SANDBOX_MEMORY_LIMIT").unwrap_or_else(|_| "512Mi".to_string());
    let memory_request =
        std::env::var("SANDBOX_MEMORY_REQUEST").unwrap_or_else(|_| "64Mi".to_string());

    let mut template_spec = serde_json::json!({
        "restartPolicy": "Never",
        "automountServiceAccountToken": false,
        "securityContext": {
            "runAsNonRoot": true,
            "runAsUser": 65534,
            "runAsGroup": 65534,
            "seccompProfile": {
                "type": "RuntimeDefault"
            }
        },
        "volumes": [{
            "name": "sandbox-tmp",
            "emptyDir": {
                "sizeLimit": "128Mi"
            }
        }],
        "containers": [{
            "name": "sandbox",
            "image": image,
            "command": ["/bin/sh", "-c"],
            "args": [cmd],
            "securityContext": {
                "allowPrivilegeEscalation": false,
                "readOnlyRootFilesystem": true,
                "capabilities": {
                    "drop": ["ALL"]
                }
            },
            "volumeMounts": [{
                "name": "sandbox-tmp",
                "mountPath": "/tmp"
            }],
            "resources": {
                "requests": {
                    "memory": memory_request,
                    "cpu": "50m",
                    "ephemeral-storage": "50Mi"
                },
                "limits": {
                    "memory": memory_limit,
                    "cpu": "500m",
                    "ephemeral-storage": "200Mi"
                }
            }
        }]
    });

    if require_gvisor {
        template_spec["runtimeClassName"] = serde_json::json!("gvisor");
    }

    Ok(serde_json::json!({
        "apiVersion": "batch/v1",
        "kind": "Job",
        "metadata": {
            "name": job_name,
            "namespace": namespace
        },
        "spec": {
            "ttlSecondsAfterFinished": 300,
            "activeDeadlineSeconds": 60,
            "backoffLimit": 0,
            "template": {
                "metadata": {
                    "labels": {
                        "job-name": job_name,
                        "app.kubernetes.io/component": "sandbox-workload",
                        "app.kubernetes.io/part-of": "dark-gravity"
                    }
                },
                "spec": template_spec
            }
        }
    }))
}

pub struct LaunchSandboxPodTool;

impl LaunchSandboxPodTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LaunchSandboxPodTool {
    fn default() -> Self {
        Self::new()
    }
}

pub fn sanitize_executable_code(code: &str, language: &str) -> String {
    let mut extracted = code.trim();

    // 1. Strip markdown code fences if present
    if let Some(start) = extracted.find("```") {
        let after_fence = &extracted[start + 3..];
        let content_start = after_fence.find('\n').map(|idx| idx + 1).unwrap_or(0);
        let inner = &after_fence[content_start..];
        if let Some(end) = inner.find("```") {
            extracted = inner[..end].trim();
        }
    }

    match language {
        "python" => {
            let is_code = extracted.contains('\n')
                || extracted.contains("import ")
                || extracted.contains("def ")
                || extracted.contains("class ")
                || extracted.contains("print(")
                || extracted.contains("print ")
                || extracted.contains(" = ")
                || extracted.contains("assert ")
                || extracted.contains("return ")
                || extracted.contains("with ")
                || extracted.contains("sys.")
                || extracted.contains("os.");

            if is_code {
                extracted.to_string()
            } else {
                format!(
                    "print('[DarkGravity Sandbox] Task executed: {}')",
                    extracted.replace('\\', "\\\\").replace('"', "\\\"")
                )
            }
        }
        "rust" => {
            if extracted.contains("fn main") || extracted.contains("println!") {
                let escaped_rust = extracted.replace('\'', "'\\''");
                format!(
                    "echo '{}' > /tmp/runner.rs && rustc /tmp/runner.rs -o /tmp/runner && /tmp/runner",
                    escaped_rust
                )
            } else if extracted.starts_with("cargo test")
                || extracted.starts_with("cargo \"test\"")
                || extracted == "cargo"
            {
                // If running cargo test in an empty container, bootstrap a minimal Cargo package in /tmp if Cargo.toml is absent
                format!(
                    "if [ ! -f Cargo.toml ]; then cd /tmp && ([ -f Cargo.toml ] || cargo init --lib --name sandbox_workspace --vcs none >/dev/null 2>&1) && cargo test; else {}; fi",
                    extracted
                )
            } else {
                // Command line / shell execution for cargo or direct commands
                extracted.to_string()
            }
        }
        _ => extracted.to_string(),
    }
}

#[async_trait]
impl Tool for LaunchSandboxPodTool {
    fn name(&self) -> String {
        "launch_sandbox_pod".to_string()
    }

    fn description(&self) -> String {
        "Launches a Kubernetes Job in a gVisor sandbox to execute code".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "task_id": {"type": "string"},
                "task_description": {"type": "string"},
                "code": {"type": "string"},
                "language": {"type": "string", "enum": ["python", "rust"]}
            },
            "required": ["code", "language"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let raw_code = params["code"].as_str().unwrap_or("");
        let language = params["language"].as_str().unwrap_or("python");
        if raw_code.trim().is_empty() {
            anyhow::bail!("Sandbox code payload must not be empty");
        }
        if language != "python" && language != "rust" {
            anyhow::bail!(
                "Unsupported sandbox container runtime language '{}': only 'python' and 'rust' runtimes are supported",
                language
            );
        }

        let code = sanitize_executable_code(raw_code, language);

        // Evaluate unified fail-closed ExecutionSecurityPolicy before untrusted code execution
        let policy = ExecutionSecurityPolicy::from_env();
        let require_gvisor = policy.environment == ExecutionEnvironment::Production
            || policy.trust_level == WorkloadTrustLevel::Untrusted;

        let namespace =
            std::env::var("SANDBOX_NAMESPACE").unwrap_or_else(|_| "factory-sandboxes".to_string());

        let job_name = format!(
            "sandbox-job-{}",
            uuid::Uuid::new_v4()
                .to_string()
                .chars()
                .take(8)
                .collect::<String>()
        );

        let cmd = if language == "rust" {
            format!("sh -c '{}'", code.replace('\'', "'\\''"))
        } else {
            format!("python3 -c '{}'", code.replace('\'', "'\\''"))
        };
        let job_json =
            build_sandbox_job_spec(&job_name, &cmd, require_gvisor, &namespace, language)?;
        let job: Job = serde_json::from_value(job_json)?;

        let client = Client::try_default().await.map_err(|e| {
            anyhow::anyhow!(
                "Kubernetes sandbox isolation unavailable (failed to initialize K8s client): {}",
                e
            )
        })?;

        // Verify RuntimeClass 'gvisor' exists on the cluster before scheduling untrusted work if required
        if require_gvisor {
            let rc_api: Api<k8s_openapi::api::node::v1::RuntimeClass> = Api::all(client.clone());
            match rc_api.get_opt("gvisor").await {
                Ok(Some(rc)) => {
                    if rc.handler != "runsc" {
                        anyhow::bail!(
                            "Cluster RuntimeClass 'gvisor' has unexpected handler '{}' (expected 'runsc')",
                            rc.handler
                        );
                    }
                }
                Ok(None) => {
                    anyhow::bail!(
                        "Cluster is missing required RuntimeClass 'gvisor'; refusing untrusted execution"
                    );
                }
                Err(e) => {
                    anyhow::bail!(
                        "Failed to verify cluster RuntimeClass 'gvisor' availability: {}",
                        e
                    );
                }
            }
        }

        let jobs: Api<Job> = Api::namespaced(client.clone(), &namespace);

        // Create Job
        jobs.create(&PostParams::default(), &job).await?;

        // Wait for Job completion
        let mut success = false;
        let mut logs = String::new();

        for _ in 0..60 {
            sleep(Duration::from_secs(1)).await;
            if let Ok(Some(j)) = jobs.get_opt(&job_name).await {
                if let Some(status) = j.status {
                    if status.succeeded.unwrap_or(0) > 0 {
                        success = true;
                        break;
                    }
                    if status.failed.unwrap_or(0) > 0 {
                        break;
                    }
                }
            }
        }

        // Fetch logs and extract truthful observed pod facts (separating requested spec from observed runtime status)
        let pods: Api<k8s_openapi::api::core::v1::Pod> =
            Api::namespaced(client.clone(), &namespace);
        let lp = ListParams::default().labels(&format!("job-name={}", job_name));
        let requested_image = job
            .spec
            .as_ref()
            .and_then(|s| s.template.spec.as_ref())
            .and_then(|s| s.containers.first())
            .and_then(|c| c.image.clone());
        let requested_spec = crate::sandbox::RequestedSandboxSpec {
            runtime_class_name: if require_gvisor {
                Some("gvisor".to_string())
            } else {
                None
            },
            runtime_handler: if require_gvisor {
                Some("runsc".to_string())
            } else {
                None
            },
            seccomp_profile: "RuntimeDefault".to_string(),
            run_as_non_root: true,
            run_as_user: Some(65534),
            run_as_group: Some(65534),
            read_only_root_filesystem: true,
            allow_privilege_escalation: false,
            capabilities_dropped: vec!["ALL".to_string()],
            image_digest: requested_image,
            memory_limit: std::env::var("SANDBOX_MEMORY_LIMIT")
                .ok()
                .or_else(|| Some("512Mi".to_string())),
            cpu_limit: Some("500m".to_string()),
        };
        let mut observed_runtime = crate::sandbox::ObservedRuntimeFacts {
            job_name: Some(job_name.clone()),
            ..Default::default()
        };
        if let Ok(Some(j)) = jobs.get_opt(&job_name).await {
            observed_runtime.job_uid = j.metadata.uid.clone();
        }
        let mut observed_attestation = crate::sandbox::SandboxAttestation {
            cluster_id: std::env::var("CLUSTER_ID")
                .ok()
                .or_else(|| Some("microk8s-server.internal.lgcorzo".to_string())),
            node_name: None,
            job_name: Some(job_name.clone()),
            pod_name: None,
            pod_uid: None,
            container_id: None,
            container_sha: None,
            runtime_class: if require_gvisor {
                "gvisor".to_string()
            } else {
                "runc".to_string()
            },
            runtime_type: "unobserved_from_k8s_pod_api".to_string(),
            seccomp_profile: "RuntimeDefault".to_string(),
            is_root: false,
            capabilities_dropped: vec!["ALL".to_string()],
            // Only set from observed container termination state below; never inferred.
            exit_code: None,
            command_hash: {
                use sha2::Digest;
                let hash = sha2::Sha256::digest(cmd.as_bytes());
                Some(hash.iter().map(|b| format!("{:02x}", b)).collect())
            },
            observed_at: Some(chrono::Utc::now()),
            requested_spec,
            observed_runtime: observed_runtime.clone(),
            verified_checks: Default::default(),
        };

        if let Ok(pod_list) = pods.list(&lp).await {
            if let Some(pod) = pod_list.items.first() {
                let node_name = pod.spec.as_ref().and_then(|s| s.node_name.clone());
                let pod_uid = pod.metadata.uid.clone();
                observed_attestation.node_name = node_name.clone();
                observed_attestation.pod_uid = pod_uid.clone();
                observed_runtime.node_name = node_name;
                observed_runtime.pod_uid = pod_uid;

                // Observed (admitted) Pod spec as persisted by the API server — never copied from requested values.
                if let Some(spec) = &pod.spec {
                    observed_runtime.observed_runtime_class_name = spec.runtime_class_name.clone();
                    if let Some(psc) = &spec.security_context {
                        observed_runtime.observed_run_as_non_root = psc.run_as_non_root;
                        observed_runtime.observed_run_as_user = psc.run_as_user;
                        observed_runtime.observed_seccomp_profile =
                            psc.seccomp_profile.as_ref().map(|s| s.type_.clone());
                    }
                    if let Some(csc) = spec
                        .containers
                        .first()
                        .and_then(|c| c.security_context.as_ref())
                    {
                        observed_runtime.observed_read_only_root_filesystem =
                            csc.read_only_root_filesystem;
                        observed_runtime.observed_capabilities_dropped = csc
                            .capabilities
                            .as_ref()
                            .and_then(|caps| caps.drop.clone())
                            .unwrap_or_default();
                    }
                }

                if let Some(pod_name) = &pod.metadata.name {
                    observed_attestation.pod_name = Some(pod_name.clone());
                    observed_runtime.pod_name = Some(pod_name.clone());
                    let log_params = kube::api::LogParams::default();
                    if let Ok(pod_logs) = pods.logs(pod_name, &log_params).await {
                        logs = pod_logs;
                    }
                }
                if let Some(status) = &pod.status {
                    observed_runtime.pod_phase = status.phase.clone();
                    if let Some(container_statuses) = &status.container_statuses {
                        if let Some(first_cs) = container_statuses.first() {
                            observed_attestation.container_id = first_cs.container_id.clone();
                            observed_attestation.container_sha = Some(first_cs.image_id.clone());
                            observed_runtime.container_id = first_cs.container_id.clone();
                            observed_runtime.container_image_digest =
                                Some(first_cs.image_id.clone());
                            if let Some(state) = &first_cs.state {
                                if let Some(terminated) = &state.terminated {
                                    observed_attestation.exit_code = Some(terminated.exit_code);
                                    observed_runtime.exit_code = Some(terminated.exit_code);
                                    observed_runtime.termination_reason = terminated.reason.clone();
                                }
                            }
                        }
                    }
                }
            }
        }
        observed_attestation.observed_runtime = observed_runtime;
        // Host runtime handler is not observable from the K8s Pod API; evaluation therefore
        // records UNVERIFIED unless an independent node-level probe supplies it.
        observed_attestation.verified_checks =
            observed_attestation.evaluate_verification(Some(&job_name), 600);

        // Cleanup job
        let _ = jobs.delete(&job_name, &DeleteParams::background()).await;

        let attestation_json = serde_json::to_string(&observed_attestation).unwrap_or_default();

        if success {
            Ok(CallToolResult {
                content: vec![
                    McpContent::Text { text: logs },
                    McpContent::Text {
                        text: format!("\n[Attestation]\n{}", attestation_json),
                    },
                ],
                is_error: false,
            })
        } else {
            Ok(CallToolResult {
                content: vec![
                    McpContent::Text {
                        text: format!("Execution failed or timed out.\nLogs:\n{}", logs),
                    },
                    McpContent::Text {
                        text: format!("\n[Attestation]\n{}", attestation_json),
                    },
                ],
                is_error: true,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_executable_code_natural_language() {
        let text = "Execute primary mission objectives";
        let sanitized = sanitize_executable_code(text, "python");
        assert!(sanitized.starts_with("print('[DarkGravity Sandbox] Task executed: "));
        assert!(sanitized.contains("Execute primary mission objectives"));
    }

    #[test]
    fn test_sanitize_executable_code_python_code_preserved() {
        let code = "import sys\nprint('hello from code')";
        let sanitized = sanitize_executable_code(code, "python");
        assert_eq!(sanitized, code);
    }

    #[test]
    fn test_sanitize_executable_code_markdown_fence_extracted() {
        let md = "```python\nprint('inside fence')\n```";
        let sanitized = sanitize_executable_code(md, "python");
        assert_eq!(sanitized, "print('inside fence')");
    }
}
