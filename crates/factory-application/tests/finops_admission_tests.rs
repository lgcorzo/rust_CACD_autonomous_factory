use factory_application::agents::finops::{FinOpsAgent, MissionAdmissionGate};
use factory_core::FinOpsTag;

fn create_test_tag() -> FinOpsTag {
    FinOpsTag {
        team: "core".to_string(),
        epic: "audit".to_string(),
        microservice: "mcp".to_string(),
        environment: "test".to_string(),
        cost_center: "cc-100".to_string(),
    }
}

#[tokio::test]
async fn test_finops_hard_stop_blocks_mission_admission() {
    let agent = FinOpsAgent::new(
        "http://mock-litellm".to_string(),
        "key".to_string(),
        create_test_tag(),
    );

    // Initial state: not halted -> admission must be granted
    assert!(
        agent.can_admit_mission("mission-01").is_ok(),
        "Missions must be admitted when budget is healthy"
    );

    // Simulate budget threshold breach -> HardStop triggered
    agent.set_halted(true);

    // HardStop state -> admission MUST be rejected with QuotaExceeded / FinOps error
    let admission_res = agent.can_admit_mission("mission-02");
    assert!(
        admission_res.is_err(),
        "New missions MUST be rejected after FinOps hard-stop threshold breach"
    );

    let err_str = admission_res.unwrap_err().to_string();
    assert!(
        err_str.contains("FinOps budget hard-stop latch is active"),
        "Expected hard-stop error message, got: {}",
        err_str
    );
}
