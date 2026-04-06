use factory_application::agents::RustantAgent;
use factory_infrastructure::{MockMcpClient, MockR2rClient};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn test_rustant_agent_with_mock_mcp() {
    let mut mock_mcp = MockMcpClient::new();
    let mut mock_r2r = MockR2rClient::new();

    mock_r2r
        .expect_search()
        .with(mockall::predicate::eq("Create a web app"))
        .returning(|_| Ok("Retrieved context for web app".to_string()));

    mock_mcp
        .expect_call_tool_json()
        .with(
            mockall::predicate::eq("plan_mission"),
            mockall::predicate::always(),
        )
        .returning(|_, _| Ok(json!({ "status": "planned", "tasks": [] })));

    let rustant = RustantAgent::new(Arc::new(mock_mcp), Arc::new(mock_r2r));
    let result: serde_json::Value = rustant
        .plan_mission("test-id", "Create a web app")
        .await
        .unwrap();

    assert_eq!(result["status"], "planned");
}
