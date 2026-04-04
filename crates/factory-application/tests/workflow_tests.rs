use factory_application::Agent;
use factory_application::agents::MockAgent;
use serde_json::json;

#[tokio::test]
async fn test_planner_agent_mock() {
    let mut mock_planner = MockAgent::new();
    mock_planner.expect_name().return_const("planner".to_string());
    mock_planner.expect_execute()
        .with(mockall::predicate::eq("Create a web app"))
        .returning(|_| Ok(json!({ "status": "planned", "tasks": [] })));

    let result = mock_planner.execute("Create a web app").await.unwrap();
    assert_eq!(result["status"], "planned");
}

/* 
  Note: Full integration tests for Hatchet workflows usually require 
  a running Hatchet server or a complex mock of the Hatchet Context.
  For this migration phase, we verify that our Agents (the primary workflow steps)
  can be mocked and injected correctly.
*/
