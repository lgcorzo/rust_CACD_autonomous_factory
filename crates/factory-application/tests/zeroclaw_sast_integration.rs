use factory_application::agents::ZeroClawAgent;
use factory_infrastructure::MockMcpClient;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn test_zeroclaw_blocks_execution_on_sast_failure() {
    let mut mock_mcp = MockMcpClient::new();

    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "sync_bridge_state")
        .times(1)
        .returning(|_, _| Ok(json!({"is_error": false, "content": []})));

    // Mock the SAST call to return a score < 8.0
    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "security_review")
        .times(1)
        .returning(|_, _| {
            Ok(json!({
                "content": [{
                    "text": json!({
                        "status": "rejected",
                        "score": 4.0,
                        "findings": ["High severity injection vulnerability found."]
                    }).to_string()
                }],
                "is_error": false
            }))
        });

    // launch_sandbox_pod should NEVER be called since SAST failed
    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "launch_sandbox_pod")
        .times(0)
        .returning(|_, _| Ok(json!({})));

    let mock_aethalgard = factory_infrastructure::MockAethalgardClient::new();
    let agent = ZeroClawAgent::new(Arc::new(mock_mcp), Arc::new(mock_aethalgard));

    let malicious_code = "import os; os.system('rm -rf /')";
    let result = agent.execute_task("mission-123", malicious_code, &[]).await;

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Security scan failed"));
    assert!(err_msg.contains("SAST score < 8.0"));
}

#[tokio::test]
async fn test_zeroclaw_allows_execution_on_sast_pass() {
    let mut mock_mcp = MockMcpClient::new();

    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "sync_bridge_state")
        .times(2)
        .returning(|_, _| Ok(json!({"is_error": false, "content": []})));

    // Mock the SAST call to return a score >= 8.0
    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "security_review")
        .times(1)
        .returning(|_, _| {
            Ok(json!({
                "content": [{
                    "text": json!({
                        "status": "approved",
                        "score": 10.0,
                        "findings": []
                    }).to_string()
                }],
                "is_error": false
            }))
        });

    // launch_sandbox_pod SHOULD be called
    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "launch_sandbox_pod")
        .times(1)
        .returning(|_, _| {
            Ok(json!({
                "is_success": true,
                "stdout": "success"
            }))
        });

    let mock_aethalgard = factory_infrastructure::MockAethalgardClient::new();
    let agent = ZeroClawAgent::new(Arc::new(mock_mcp), Arc::new(mock_aethalgard));

    let safe_code = "print('hello world')";
    let result = agent.execute_task("mission-123", safe_code, &[]).await;

    assert!(result.is_ok());
}
