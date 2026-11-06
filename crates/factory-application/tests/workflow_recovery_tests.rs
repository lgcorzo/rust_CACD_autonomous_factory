use factory_application::workflows::autonomous_mission::{
    ActiveMissionHardStopPolicy, MissionExecutionCoordinator, MissionInput, MissionLifecycleStatus,
};
use factory_infrastructure::finops_stop_store::{FileFinOpsStopStore, FinOpsStopStore};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[tokio::test]
async fn test_duplicate_event_delivery_is_idempotent_without_duplicate_side_effects() {
    let state_dir = std::env::temp_dir().join(format!("wf_recovery_{}", uuid::Uuid::new_v4()));
    let stop_store: Arc<dyn FinOpsStopStore> = Arc::new(FileFinOpsStopStore::new(
        state_dir.join("finops_stop.json"),
    ));
    let coordinator = MissionExecutionCoordinator::new(
        state_dir.join("missions_state.json"),
        stop_store,
        ActiveMissionHardStopPolicy::CancelAtStepBoundary,
    );

    let side_effect_count = Arc::new(AtomicUsize::new(0));
    let input = MissionInput {
        mission_id: Some("mission-dup-01".to_string()),
        goal: "Implement idempotent event handling".to_string(),
        repository: Some("lgcorzo/rust_CACD_autonomous_factory".to_string()),
        issue_number: Some(633),
        ..Default::default()
    };

    // First delivery executes the step
    let c1 = side_effect_count.clone();
    let rec1 = coordinator
        .process_event("evt-delivery-100", &input, || async move {
            c1.fetch_add(1, Ordering::SeqCst);
            Ok("step-1-complete".to_string())
        })
        .await
        .expect("First event delivery should succeed");

    assert_eq!(rec1.status, MissionLifecycleStatus::Completed);
    assert_eq!(side_effect_count.load(Ordering::SeqCst), 1);

    // Second (duplicate) delivery with same event key or completed mission ID MUST NOT re-run side effect
    let c2 = side_effect_count.clone();
    let rec2 = coordinator
        .process_event("evt-delivery-100", &input, || async move {
            c2.fetch_add(1, Ordering::SeqCst);
            Ok("should-not-run".to_string())
        })
        .await
        .expect("Duplicate event delivery should return existing mission state");

    assert_eq!(rec2.status, MissionLifecycleStatus::Completed);
    assert_eq!(
        side_effect_count.load(Ordering::SeqCst),
        1,
        "Duplicate delivery must not trigger duplicate side effects"
    );

    let _ = std::fs::remove_dir_all(&state_dir);
}

#[tokio::test]
async fn test_retry_after_transient_failure_advances_once_without_advancing_incomplete_work() {
    let state_dir = std::env::temp_dir().join(format!("wf_retry_{}", uuid::Uuid::new_v4()));
    let stop_store: Arc<dyn FinOpsStopStore> = Arc::new(FileFinOpsStopStore::new(
        state_dir.join("finops_stop.json"),
    ));
    let coordinator = MissionExecutionCoordinator::new(
        state_dir.join("missions_state.json"),
        stop_store,
        ActiveMissionHardStopPolicy::CancelAtStepBoundary,
    );

    let attempts = Arc::new(AtomicUsize::new(0));
    let input = MissionInput {
        mission_id: Some("mission-retry-01".to_string()),
        goal: "Recover from transient failure".to_string(),
        ..Default::default()
    };

    // Attempt 1: transient failure -> mission remains FailedRetryable, not Completed
    let a1 = attempts.clone();
    let err_res = coordinator
        .process_event("evt-retry-01", &input, || async move {
            a1.fetch_add(1, Ordering::SeqCst);
            Err(factory_core::error::FactoryError::Internal(
                "transient network timeout".to_string(),
            ))
        })
        .await;
    assert!(err_res.is_err());

    let state_after_fail = coordinator
        .get_mission_record("mission-retry-01")
        .unwrap()
        .expect("Mission record must be persisted");
    assert_eq!(
        state_after_fail.status,
        MissionLifecycleStatus::FailedRetryable
    );
    assert_eq!(state_after_fail.retry_count, 1);

    // Attempt 2: retry succeeds -> transitions to Completed with retry_count == 1
    let a2 = attempts.clone();
    let ok_res = coordinator
        .process_event("evt-retry-01", &input, || async move {
            a2.fetch_add(1, Ordering::SeqCst);
            Ok("recovered-ok".to_string())
        })
        .await
        .expect("Retry after transient failure should succeed");

    assert_eq!(ok_res.status, MissionLifecycleStatus::Completed);
    assert_eq!(ok_res.retry_count, 1);
    assert_eq!(attempts.load(Ordering::SeqCst), 2);

    let _ = std::fs::remove_dir_all(&state_dir);
}

#[tokio::test]
async fn test_cancellation_compensation_and_worker_restart_recovery() {
    let state_dir = std::env::temp_dir().join(format!("wf_restart_{}", uuid::Uuid::new_v4()));
    let state_file = state_dir.join("missions_state.json");
    let stop_file = state_dir.join("finops_stop.json");

    let stop_store: Arc<dyn FinOpsStopStore> = Arc::new(FileFinOpsStopStore::new(stop_file.clone()));
    let worker_1 = MissionExecutionCoordinator::new(
        state_file.clone(),
        stop_store.clone(),
        ActiveMissionHardStopPolicy::CancelAtStepBoundary,
    );

    let step_executions = Arc::new(AtomicUsize::new(0));
    let s1 = step_executions.clone();

    // Worker 1 completes step "plan" before crashing/restarting
    worker_1
        .execute_checkpointed_step("mission-restart-01", "rustant-plan", || async move {
            s1.fetch_add(1, Ordering::SeqCst);
            Ok("plan-artifact-v1".to_string())
        })
        .await
        .unwrap();

    // Simulate Worker 2 starting up from the same persisted state file
    let worker_2 = MissionExecutionCoordinator::new(
        state_file.clone(),
        stop_store.clone(),
        ActiveMissionHardStopPolicy::CancelAtStepBoundary,
    );

    // Re-running "rustant-plan" on Worker 2 skips execution and returns checkpointed result
    let s2 = step_executions.clone();
    let plan_out = worker_2
        .execute_checkpointed_step("mission-restart-01", "rustant-plan", || async move {
            s2.fetch_add(1, Ordering::SeqCst);
            Ok("should-not-reexecute".to_string())
        })
        .await
        .unwrap();
    assert_eq!(plan_out, "plan-artifact-v1");
    assert_eq!(
        step_executions.load(Ordering::SeqCst),
        1,
        "Completed step must not re-execute after worker restart"
    );

    // Cancel the mission on Worker 2 with explicit compensation record
    let cancelled = worker_2
        .cancel_mission(
            "mission-restart-01",
            "Operator requested cancellation",
            "Revoked JIT token and cleaned sandbox workspace",
        )
        .unwrap();
    assert_eq!(cancelled.status, MissionLifecycleStatus::Cancelled);
    assert_eq!(
        cancelled.compensation_record.as_deref(),
        Some("Revoked JIT token and cleaned sandbox workspace")
    );

    // Subsequent step attempt on a cancelled mission must be rejected
    let post_cancel = worker_2
        .execute_checkpointed_step("mission-restart-01", "zeroclaw-execute", || async move {
            Ok("should-not-run".to_string())
        })
        .await;
    assert!(post_cancel.is_err());

    let _ = std::fs::remove_dir_all(&state_dir);
}
