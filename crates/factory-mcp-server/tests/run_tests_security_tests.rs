use factory_mcp_server::sandbox::SubprocessDriver;
use factory_mcp_server::tools::run_tests::RunTestsTool;
use factory_mcp_server::tools::Tool;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn test_run_tests_rejects_missing_runner_without_simulated_pass() {
    let tool = RunTestsTool::new(Arc::new(
        SubprocessDriver::new().with_force_production(false),
    ));

    // Execute a non-existent test runner command
    let res = tool
        .call(json!({
            "test_command": "non_existent_binary_xyz_12345 --all"
        }))
        .await
        .unwrap();

    // Invariant: Missing runner MUST NOT report success or exit_code 0
    let text = match &res.content[0] {
        factory_mcp_server::protocol::McpContent::Text { text } => text,
        _ => panic!("Expected text content"),
    };

    let parsed: serde_json::Value = serde_json::from_str(text).unwrap();

    assert!(
        res.is_error,
        "Tool call must be marked as error when test runner is missing"
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
    assert_eq!(
        parsed.get("outcome").and_then(|o| o.as_str()),
        Some("infrastructure_error"),
        "Outcome must be explicitly infrastructure_error"
    );
}
