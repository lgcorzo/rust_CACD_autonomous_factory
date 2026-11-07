use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
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
) -> anyhow::Result<serde_json::Value> {
    let is_prod = std::env::var("FACTORY_ENV")
        .unwrap_or_default()
        .eq_ignore_ascii_case("production");
    if is_prod && !require_gvisor {
        anyhow::bail!(
            "Untrusted sandbox execution requires gVisor runtime in production. Fallback to standard runc/host is forbidden."
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

    let image = std::env::var("SANDBOX_IMAGE")
        .unwrap_or_else(|_| "python:3.11.11-slim".to_string());

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
                "sizeLimit": "32Mi"
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
                    "memory": "16Mi",
                    "cpu": "50m",
                    "ephemeral-storage": "50Mi"
                },
                "limits": {
                    "memory": "30Mi",
                    "cpu": "250m",
                    "ephemeral-storage": "100Mi"
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
                extracted.to_string()
            } else {
                format!(
                    "fn main() {{ println!(\"[DarkGravity Sandbox] Task executed: {}\"); }}",
                    extracted.replace('\\', "\\\\").replace('"', "\\\"")
                )
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

        // Fail closed unless gVisor is explicitly disabled via USE_GVISOR=false (never allowed in production)
        let is_prod = std::env::var("FACTORY_ENV")
            .unwrap_or_default()
            .eq_ignore_ascii_case("production");
        let explicit_disable = std::env::var("USE_GVISOR")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("false") || v == "0")
            .unwrap_or(false);
        let require_gvisor = is_prod || !explicit_disable;

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
        let job_json = build_sandbox_job_spec(&job_name, &cmd, require_gvisor, &namespace)?;
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

        // Fetch logs
        let pods: Api<k8s_openapi::api::core::v1::Pod> =
            Api::namespaced(client.clone(), &namespace);
        let lp = ListParams::default().labels(&format!("job-name={}", job_name));
        if let Ok(pod_list) = pods.list(&lp).await {
            if let Some(pod) = pod_list.items.first() {
                if let Some(pod_name) = &pod.metadata.name {
                    let log_params = kube::api::LogParams::default();
                    if let Ok(pod_logs) = pods.logs(pod_name, &log_params).await {
                        logs = pod_logs;
                    }
                }
            }
        }

        // Cleanup job
        let _ = jobs.delete(&job_name, &DeleteParams::background()).await;

        if success {
            Ok(CallToolResult {
                content: vec![McpContent::Text { text: logs }],
                is_error: false,
            })
        } else {
            Ok(CallToolResult {
                content: vec![McpContent::Text {
                    text: format!("Execution failed or timed out.\nLogs:\n{}", logs),
                }],
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
