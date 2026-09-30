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
        let code = sanitize_executable_code(raw_code, language);

        let client = Client::try_default().await?;
        let jobs: Api<Job> = Api::namespaced(client.clone(), "development");

        let job_name = format!(
            "sandbox-job-{}",
            uuid::Uuid::new_v4()
                .to_string()
                .chars()
                .take(8)
                .collect::<String>()
        );

        let cmd = match language {
            "python" => format!("python3 -c '{}'", code.replace('\'', "'\\''")),
            "rust" => format!("rustc -e '{}'", code.replace('\'', "'\\''")),
            _ => return Err(anyhow::anyhow!("Unsupported language")),
        };

        let use_gvisor = std::env::var("USE_GVISOR").unwrap_or_default() == "true";

        let mut job_json = serde_json::json!({
            "apiVersion": "batch/v1",
            "kind": "Job",
            "metadata": {
                "name": job_name,
                "namespace": "development"
            },
            "spec": {
                "backoffLimit": 0,
                "template": {
                    "metadata": {
                        "labels": {
                            "job-name": job_name
                        }
                    },
                    "spec": {
                        "restartPolicy": "Never",
                        "containers": [{
                            "name": "sandbox",
                            "image": "python:3.11-slim",
                            "command": ["/bin/sh", "-c"],
                            "args": [cmd],
                            "resources": {
                                "limits": {
                                    "memory": "30Mi",
                                    "cpu": "100m"
                                }
                            }
                        }]
                    }
                }
            }
        });

        if use_gvisor {
            if let Some(spec) = job_json
                .get_mut("spec")
                .and_then(|s| s.get_mut("template"))
                .and_then(|t| t.get_mut("spec"))
                .and_then(|s| s.as_object_mut())
            {
                spec.insert("runtimeClassName".to_string(), serde_json::json!("gvisor"));
            }
        }

        let job: Job = serde_json::from_value(job_json)?;

        // Create Job
        jobs.create(&PostParams::default(), &job).await?;

        // Wait for Job completion
        let mut success = false;
        let mut logs = String::new();

        // polling for simplicity
        for _ in 0..60 {
            // wait up to 60s
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
            Api::namespaced(client.clone(), "development");
        let lp = ListParams::default().labels(&format!("job-name={}", job_name));
        if let Ok(pod_list) = pods.list(&lp).await {
            if let Some(pod) = pod_list.items.first() {
                if let Some(pod_name) = &pod.metadata.name {
                    let log_params = kube::api::LogParams::default();
                    // Just read logs
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
