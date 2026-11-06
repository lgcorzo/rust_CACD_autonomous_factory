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
    Mocked,
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
    driver: Arc<dyn SandboxDriver>,
}

impl RunTestsTool {
    pub fn new(driver: Arc<dyn SandboxDriver>) -> Self {
        Self { driver }
    }
}

/// Validates and parses an untrusted test command into `(executable, args, inferred_language)`.
/// Rejects shell metacharacters, command chaining, subshells, and unapproved executables.
fn validate_and_parse_test_command(
    raw_cmd: &str,
    explicit_lang: Option<&str>,
) -> Result<(String, Vec<String>, String), String> {
    let trimmed = raw_cmd.trim();
    if trimmed.is_empty() {
        return Err("Empty test command is not permitted".to_string());
    }

    // Reject shell metacharacters and control operators to prevent shell-injection bypass
    const FORBIDDEN_PATTERNS: &[&str] = &[
        ";", "&&", "||", "|", "`", "$(", "${", "\n", "\r", ">", "<", "&",
    ];
    for pat in FORBIDDEN_PATTERNS {
        if trimmed.contains(pat) {
            return Err(format!(
                "Shell metacharacter or operator '{}' is forbidden in test command",
                pat
            ));
        }
    }

    if let Some(lang) = explicit_lang {
        if lang != "rust" && lang != "python" {
            return Err(format!(
                "Unsupported runtime language '{}'; only 'rust' and 'python' are verified",
                lang
            ));
        }
    }

    let tokens: Vec<String> = trimmed.split_whitespace().map(|s| s.to_string()).collect();
    let exe = tokens[0].as_str();
    let args = tokens[1..].to_vec();

    // Reject dangerous flags that execute arbitrary commands or modify manifest paths outside repo
    for arg in &args {
        if arg.contains("..") || arg.starts_with("--config") {
            return Err(format!("Argument '{}' is not permitted in sandbox test runner", arg));
        }
    }

    let inferred_lang = match exe {
        "cargo" | "rustc" => "rust",
        "pytest" | "python3" | "python" => "python",
        _ => {
            return Err(format!(
                "Unapproved test runner executable '{}'; allowed runners are cargo, pytest, python3",
                exe
            ))
        }
    };

    let final_lang = explicit_lang.unwrap_or(inferred_lang).to_string();
    Ok((exe.to_string(), args, final_lang))
}

/// Builds a safe Python subprocess invocation snippet for the validated executable and argument list
/// so the command runs inside the injected `SandboxDriver` with argument boundaries preserved.
fn build_driver_snippet(exe: &str, args: &[String]) -> String {
    let mut argv = vec![exe.to_string()];
    argv.extend_from_slice(args);
    let argv_json = serde_json::to_string(&argv).unwrap_or_else(|_| "[]".to_string());
    format!(
        "import json, subprocess, sys\nargv = json.loads({:?})\ntry:\n    res = subprocess.run(argv, capture_output=True, text=True)\n    sys.stdout.write(res.stdout)\n    sys.stderr.write(res.stderr)\n    sys.exit(res.returncode)\nexcept FileNotFoundError as e:\n    sys.stderr.write(f'Test runner binary not found: {{e}}\\n')\n    sys.exit(127)\n",
        argv_json
    )
}

#[async_trait]
impl Tool for RunTestsTool {
    fn name(&self) -> String {
        "run_tests".to_string()
    }

    fn description(&self) -> String {
        "Runs test suites (cargo test, pytest) in an isolated sandbox.".to_string()
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
        let explicit_lang = params["language"].as_str();

        // Mocked execution requests are explicitly classified as `Mocked`, NEVER `Passed` (`FR-005`, `FR-006`)
        if test_cmd.trim_start().starts_with("mock") {
            let report = TestExecutionReport {
                status: "mocked".to_string(),
                outcome: TestExecutionOutcome::Mocked,
                command: test_cmd.to_string(),
                exit_code: None,
                stdout: "Mock execution recorded (not a real test execution).".to_string(),
                stderr: String::new(),
                duration_ms: 0,
                execution_environment: "mock".to_string(),
            };
            return Ok(CallToolResult {
                content: vec![McpContent::Text {
                    text: serde_json::to_string(&report)?,
                }],
                is_error: false,
            });
        }

        // Validate command and argument boundaries before invoking the sandbox driver
        let (exe, args, language) = match validate_and_parse_test_command(test_cmd, explicit_lang) {
            Ok(parsed) => parsed,
            Err(validation_err) => {
                let report = TestExecutionReport {
                    status: "rejected".to_string(),
                    outcome: TestExecutionOutcome::NotExecuted,
                    command: test_cmd.to_string(),
                    exit_code: None,
                    stdout: String::new(),
                    stderr: validation_err,
                    duration_ms: 0,
                    execution_environment: "sandbox-policy-gate".to_string(),
                };
                return Ok(CallToolResult {
                    content: vec![McpContent::Text {
                        text: serde_json::to_string(&report)?,
                    }],
                    is_error: true,
                });
            }
        };

        let start_time = std::time::Instant::now();
        let snippet = build_driver_snippet(&exe, &args);

        // Execute strictly through the injected SandboxDriver (using python argv runner inside sandbox)
        let driver_lang = if language == "rust" || language == "python" {
            "python"
        } else {
            language.as_str()
        };

        let report = match self.driver.execute(&snippet, driver_lang).await {
            Ok(exec_res) => {
                let exit_code = exec_res.exit_code;
                let is_missing_runner = exit_code == Some(127)
                    || exit_code == Some(126)
                    || exec_res.stderr.contains("Test runner binary not found")
                    || exec_res.stderr.contains("not found");

                if !exec_res.is_success && is_missing_runner {
                    TestExecutionReport {
                        status: "failed".to_string(),
                        outcome: TestExecutionOutcome::InfrastructureError,
                        command: test_cmd.to_string(),
                        exit_code,
                        stdout: exec_res.stdout,
                        stderr: exec_res.stderr,
                        duration_ms: start_time.elapsed().as_millis() as u64,
                        execution_environment: "sandbox-driver".to_string(),
                    }
                } else if exec_res.is_success {
                    TestExecutionReport {
                        status: "success".to_string(),
                        outcome: TestExecutionOutcome::Passed,
                        command: test_cmd.to_string(),
                        exit_code: exit_code.or(Some(0)),
                        stdout: exec_res.stdout,
                        stderr: exec_res.stderr,
                        duration_ms: start_time.elapsed().as_millis() as u64,
                        execution_environment: "sandbox-driver".to_string(),
                    }
                } else {
                    TestExecutionReport {
                        status: "failed".to_string(),
                        outcome: TestExecutionOutcome::Failed,
                        command: test_cmd.to_string(),
                        exit_code,
                        stdout: exec_res.stdout,
                        stderr: exec_res.stderr,
                        duration_ms: start_time.elapsed().as_millis() as u64,
                        execution_environment: "sandbox-driver".to_string(),
                    }
                }
            }
            Err(err) => {
                let err_msg = err.to_string();
                let is_timeout = err_msg.to_lowercase().contains("timed out")
                    || err_msg.to_lowercase().contains("timeout");
                TestExecutionReport {
                    status: "failed".to_string(),
                    outcome: if is_timeout {
                        TestExecutionOutcome::Timeout
                    } else {
                        TestExecutionOutcome::InfrastructureError
                    },
                    command: test_cmd.to_string(),
                    exit_code: if is_timeout { Some(124) } else { None },
                    stdout: String::new(),
                    stderr: format!("Sandbox driver execution failed: {}", err_msg),
                    duration_ms: start_time.elapsed().as_millis() as u64,
                    execution_environment: "sandbox-driver".to_string(),
                }
            }
        };

        let is_error = report.outcome != TestExecutionOutcome::Passed;
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
        let tool = RunTestsTool::new(Arc::new(
            SubprocessDriver::new().with_force_production(false),
        ));
        let res = tool.call(json!({"command": "mock test"})).await.unwrap();
        assert!(!res.is_error);
        let text = match &res.content[0] {
            McpContent::Text { text } => text,
            _ => panic!("expected text"),
        };
        let parsed: Value = serde_json::from_str(text).unwrap();
        assert_eq!(parsed["outcome"], "mocked");
        assert_eq!(parsed["status"], "mocked");
    }

    #[tokio::test]
    async fn test_run_tests_tool_python_command() {
        let tool = RunTestsTool::new(Arc::new(
            SubprocessDriver::new().with_force_production(false),
        ));
        let res = tool
            .call(json!({"test_command": "python3 --version"}))
            .await
            .unwrap();
        assert!(!res.is_error);
    }
}
