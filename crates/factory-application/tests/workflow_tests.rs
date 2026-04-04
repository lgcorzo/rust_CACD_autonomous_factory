use factory_application::agents::PlannerAgent;
use factory_infrastructure::MockMcpClient;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn test_planner_agent_with_mock_mcp() {
    let mut mock_mcp = MockMcpClient::new();

    mock_mcp
        .expect_call_tool_json()
        .with(
            mockall::predicate::eq("plan_mission"),
            mockall::predicate::always(),
        )
        .returning(|_, _| Ok(json!({ "status": "planned", "tasks": [] })));

    let planner = PlannerAgent::new(Arc::new(mock_mcp));
    let result: serde_json::Value = planner.create_plan("Create a web app").await.unwrap();

    assert_eq!(result["status"], "planned");
}

/*
  Note: Full integration tests for Hatchet workflows usually require
  a running Hatchet server or a complex mock of the Hatchet Context.
  For this migration phase, we verify that our Agents (the primary workflow steps)
  can be mocked and injected correctly via the MCP client.
*/
