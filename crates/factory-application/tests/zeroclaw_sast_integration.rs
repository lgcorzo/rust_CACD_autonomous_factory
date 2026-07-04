use factory_application::agents::ZeroClawAgent;
use factory_infrastructure::MockMcpClient;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn test_zeroclaw_blocks_execution_on_sast_failure() {
    let mut mock_mcp = MockMcpClient::new();

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

    // execute_code should NEVER be called since SAST failed
    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "execute_code")
        .times(0)
        .returning(|_, _| Ok(json!({})));

    let agent = ZeroClawAgent::new(Arc::new(mock_mcp));

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

    // execute_code SHOULD be called
    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "execute_code")
        .times(1)
        .returning(|_, _| {
            Ok(json!({
                "is_success": true,
                "stdout": "success"
            }))
        });

    let agent = ZeroClawAgent::new(Arc::new(mock_mcp));

    let safe_code = "print('hello world')";
    let result = agent.execute_task("mission-123", safe_code, &[]).await;

    assert!(result.is_ok());
}
