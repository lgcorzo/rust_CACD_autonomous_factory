use factory_application::agents::finops::{FinOpsAgent, MissionAdmissionGate};
use factory_application::workflows::autonomous_mission::{MissionInput, admit_mission_with_store};
use factory_core::FinOpsTag;
use factory_infrastructure::finops_stop_store::{
    FailingFinOpsStopStore, FileFinOpsStopStore, FinOpsStopStore,
};
use std::sync::Arc;

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
async fn test_finops_hard_stop_blocks_mission_admission_across_workers_with_version() {
    let state_path =
        std::env::temp_dir().join(format!("finops_stop_state_{}.json", uuid::Uuid::new_v4()));

    // Monitor worker and separate admission worker share the same durable file-backed store
    let monitor_store: Arc<dyn FinOpsStopStore> =
        Arc::new(FileFinOpsStopStore::new(state_path.clone()));
    let admission_worker_store: Arc<dyn FinOpsStopStore> =
        Arc::new(FileFinOpsStopStore::new(state_path.clone()));

    let monitor_agent = FinOpsAgent::with_stop_store(
        "http://mock-litellm".to_string(),
        "key".to_string(),
        create_test_tag(),
        monitor_store.clone(),
    );

    let admission_agent = FinOpsAgent::with_stop_store(
        "http://mock-litellm".to_string(),
        "key".to_string(),
        create_test_tag(),
        admission_worker_store.clone(),
    );

    // 1. Healthy state (version 0): admission succeeds
    let initial_decision = admission_agent
        .evaluate_admission("mission-01")
        .expect("Mission must be admitted before hard stop");
    assert!(initial_decision.admitted);
    assert_eq!(initial_decision.state_version, 0);

    // 2. Monitor evaluates spend reaching 90% ($46.00 of $50.00) -> trips durable hard stop
    let eval = monitor_agent.evaluate_spend_delta(46.00, 44.00, 50.0);
    assert!(matches!(
        eval,
        factory_application::agents::finops::BudgetEvaluation::HardStop { .. }
    ));

    let observed_state = admission_worker_store.read_state().unwrap();
    assert!(observed_state.halted);
    assert!(observed_state.state_version >= 1);

    // 3. Separate admission worker evaluates concurrent mission admission requests
    let mut handles = Vec::new();
    for idx in 0..8 {
        let store_clone = admission_worker_store.clone();
        handles.push(tokio::spawn(async move {
            let input = MissionInput {
                mission_id: Some(format!("concurrent-mission-{}", idx)),
                goal: "Test concurrent admission after hard stop".to_string(),
                ..Default::default()
            };
            admit_mission_with_store(&input, store_clone.as_ref())
        }));
    }

    for h in handles {
        let res = h.await.unwrap();
        assert!(
            res.is_err(),
            "Every concurrent admission request after hard stop MUST be rejected"
        );
        let err_msg = res.unwrap_err().to_string();
        assert!(
            err_msg.contains("FinOps budget hard-stop latch is active"),
            "Expected hard-stop message, got: {}",
            err_msg
        );
        assert!(
            err_msg.contains(&format!("version={}", observed_state.state_version)),
            "Rejection error must include observed stop-state version={}, got: {}",
            observed_state.state_version,
            err_msg
        );
    }

    // Also verify MissionAdmissionGate trait method on admission_agent
    let gate_res = admission_agent.can_admit_mission("mission-after-stop");
    assert!(gate_res.is_err());

    let _ = std::fs::remove_file(&state_path);
}

#[tokio::test]
async fn test_finops_admission_fails_closed_on_store_read_error() {
    let failing_store: Arc<dyn FinOpsStopStore> = Arc::new(FailingFinOpsStopStore::new(
        "disk I/O failure reading stop state",
    ));
    let agent = FinOpsAgent::with_stop_store(
        "http://mock-litellm".to_string(),
        "key".to_string(),
        create_test_tag(),
        failing_store,
    );

    let res = agent.can_admit_mission("mission-store-error");
    assert!(
        res.is_err(),
        "Mission admission MUST fail closed when the authoritative stop store cannot be read"
    );
    let msg = res.unwrap_err().to_string();
    assert!(msg.contains("disk I/O failure"));
}
