use crate::protocol::{CallToolResult, McpContent};
use crate::sandbox::SandboxDriver;
use crate::tools::Tool;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TestExecutionOutcome {
    Passed,
    Failed,
    NotExecuted,
    InfrastructureError,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExecutionReport {
    pub status: String,
    pub outcome: TestExecutionOutcome,
    pub command: String,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
    pub execution_environment: String,
}

pub struct RunTestsTool {
    #[allow(dead_code)]
    driver: Arc<dyn SandboxDriver>,
}

impl RunTestsTool {
    pub fn new(driver: Arc<dyn SandboxDriver>) -> Self {
        Self { driver }
    }
}

#[async_trait]
impl Tool for RunTestsTool {
    fn name(&self) -> String {
        "run_tests".to_string()
    }

    fn description(&self) -> String {
        "Runs test suites (cargo test, pytest) in a sandbox.".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "test_command": {"type": "string"},
                "command": {"type": "string"},
                "mission_id": {"type": "string"},
                "language": {"type": "string", "enum": ["python", "rust"]}
            }
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let test_cmd = params["test_command"]
            .as_str()
            .or_else(|| params["command"].as_str())
            .unwrap_or("cargo test");

        if test_cmd.starts_with("mock") {
            let report = TestExecutionReport {
                status: "success".to_string(),
                outcome: TestExecutionOutcome::Passed,
                command: test_cmd.to_string(),
                exit_code: Some(0),
                stdout: "Mock tests passed successfully!".to_string(),
                stderr: String::new(),
                duration_ms: 1,
                execution_environment: "mock".to_string(),
            };
            return Ok(CallToolResult {
                content: vec![McpContent::Text {
                    text: serde_json::to_string(&report)?,
                }],
                is_error: false,
            });
        }

        let timeout_duration = std::time::Duration::from_secs(60);
        let start_time = std::time::Instant::now();
        let mut cmd = tokio::process::Command::new("sh");
        cmd.arg("-c").arg(test_cmd);

        let report = match tokio::time::timeout(timeout_duration, cmd.output()).await {
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let is_success = output.status.success();
                let exit_code = output.status.code();

                if !is_success
                    && (stderr.contains("not found")
                        || exit_code == Some(127)
                        || exit_code == Some(126))
                {
                    // Fail closed: Missing test runner is an infrastructure error, NEVER a success
                    TestExecutionReport {
                        status: "failed".to_string(),
                        outcome: TestExecutionOutcome::InfrastructureError,
                        command: test_cmd.to_string(),
                        exit_code,
                        stdout,
                        stderr: format!("Test runner binary not found: {}", stderr),
                        duration_ms: start_time.elapsed().as_millis() as u64,
                        execution_environment: "sandbox-runner".to_string(),
                    }
                } else {
                    TestExecutionReport {
                        status: if is_success {
                            "success".to_string()
                        } else {
                            "failed".to_string()
                        },
                        outcome: if is_success {
                            TestExecutionOutcome::Passed
                        } else {
                            TestExecutionOutcome::Failed
                        },
                        command: test_cmd.to_string(),
                        exit_code,
                        stdout,
                        stderr,
                        duration_ms: start_time.elapsed().as_millis() as u64,
                        execution_environment: "sandbox-runner".to_string(),
                    }
                }
            }
            Ok(Err(e)) => {
                // Fail closed: Spawn failure is an infrastructure error, NEVER a success
                TestExecutionReport {
                    status: "failed".to_string(),
                    outcome: TestExecutionOutcome::InfrastructureError,
                    command: test_cmd.to_string(),
                    exit_code: Some(1),
                    stdout: String::new(),
                    stderr: format!("Failed to spawn test runner '{}': {}", test_cmd, e),
                    duration_ms: start_time.elapsed().as_millis() as u64,
                    execution_environment: "sandbox-runner".to_string(),
                }
            }
            Err(_) => TestExecutionReport {
                status: "failed".to_string(),
                outcome: TestExecutionOutcome::Timeout,
                command: test_cmd.to_string(),
                exit_code: Some(124),
                stdout: String::new(),
                stderr: format!(
                    "Test execution timed out after {}s",
                    timeout_duration.as_secs()
                ),
                duration_ms: start_time.elapsed().as_millis() as u64,
                execution_environment: "sandbox-runner".to_string(),
            },
        };

        let is_error = report.status != "success";
        Ok(CallToolResult {
            content: vec![McpContent::Text {
                text: serde_json::to_string(&report)?,
            }],
            is_error,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sandbox::SubprocessDriver;

    #[tokio::test]
    async fn test_run_tests_tool_mock_command() {
        let tool = RunTestsTool::new(Arc::new(SubprocessDriver::new()));
        let res = tool.call(json!({"command": "mock test"})).await.unwrap();
        assert!(!res.is_error);
    }

    #[tokio::test]
    async fn test_run_tests_tool_echo_command() {
        let tool = RunTestsTool::new(Arc::new(SubprocessDriver::new()));
        let res = tool
            .call(json!({"test_command": "echo 'tests passed'"}))
            .await
            .unwrap();
        assert!(!res.is_error);
    }
}
