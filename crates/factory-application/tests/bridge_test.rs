use factory_application::bridge::{
    A2aBridgeEntry, BridgeState, StepCheckpoint, assert_bridge_checkpoint_promotable,
};
use factory_application::workspaces::checkpoints::{
    CheckpointHelper, StepCheckpoint as WorkspaceStepCheckpoint, assert_promotable_to_r2r,
    mark_stage5_verified,
};

#[tokio::test]
async fn test_bridge_state_crash_resilience() {
    let session_id = "test-mission-123";
    let mut state = BridgeState::new(session_id.to_string());

    state.checkpoints.insert(
        "task-1".to_string(),
        StepCheckpoint {
            step_name: "code".to_string(),
            input_snapshot: serde_json::json!({"test": true}),
            output_snapshot: None,
            completed_at: None,
            quarantined: true,
            diff_sha256: None,
        },
    );

    // We mock S3 Storage to store the state locally in a hashmap or just simulate successful save/load
    let mut mock_s3 = factory_infrastructure::MockS3Storage::new();

    // Mocking serialization logic because times(1) with exact byte match is flaky if timestamp changes.
    // So we just use `withf` or skip byte matching.
    mock_s3
        .expect_put_object()
        .withf(move |bucket, key, _data| {
            bucket == "test-bucket" && key == format!("bridge_state_{}.json", session_id)
        })
        .times(1)
        .returning(|_, _, _| Ok(()));

    // For the get_object, we need to return the bytes of the serialized state
    let expected_state = serde_json::to_string_pretty(&state).unwrap().into_bytes();
    mock_s3
        .expect_get_object()
        .with(
            mockall::predicate::eq("test-bucket"),
            mockall::predicate::eq(format!("bridge_state_{}.json", session_id)),
        )
        .times(1)
        .returning(move |_, _| Ok(expected_state.clone()));

    // Simulate save
    state
        .save_checkpoint(&mock_s3, "test-bucket")
        .await
        .expect("Failed to save checkpoint");

    // Simulate crash and load
    let recovered_state = BridgeState::load_checkpoint(session_id, &mock_s3, "test-bucket")
        .await
        .expect("Failed to load checkpoint")
        .expect("Checkpoint file should exist");

    // Verify state is recovered
    assert_eq!(recovered_state.session_id, session_id);
    assert_eq!(recovered_state.checkpoints.len(), 1);
    assert!(recovered_state.checkpoints.contains_key("task-1"));
}

#[test]
fn test_bridge_state_legacy_json_defaults_quarantined_to_true() {
    let legacy_json = r#"{
        "session_id": "legacy-mission-1",
        "spec_version": "1.0",
        "current_step": "code",
        "checkpoints": {
            "task-1": {
                "step_name": "code",
                "input_snapshot": {"task": "T001"},
                "output_snapshot": {"status": "done"},
                "completed_at": "2026-01-01T00:00:00Z"
            }
        },
        "run_status": "Completed",
        "last_updated": 1700000000
    }"#;

    let state: BridgeState =
        serde_json::from_str(legacy_json).expect("legacy BridgeState JSON must deserialize");
    assert!(state.a2a_entries.is_empty());
    let cp = state.checkpoints.get("task-1").unwrap();
    assert!(cp.quarantined, "quarantined must fail-closed to true");
    assert!(cp.diff_sha256.is_none());
    assert!(
        assert_bridge_checkpoint_promotable(cp).is_err(),
        "quarantined checkpoint must block promotion"
    );
}

#[tokio::test]
async fn test_lane_checkpoint_s3_key_and_a2a_bridge_entries() {
    let key = BridgeState::lane_checkpoint_key("mission-42", "lane-1").unwrap();
    assert_eq!(key, "missions/mission-42/lanes/lane-1/checkpoint.json");
    assert!(BridgeState::lane_checkpoint_key("../escape", "lane-1").is_err());
    assert!(BridgeState::lane_checkpoint_key("mission-42", "lane/../../bad").is_err());

    let mut lane_state = BridgeState::new("mission-42".to_string());
    lane_state.record_a2a_entry(A2aBridgeEntry {
        trace_id: "trace-abc-123".to_string(),
        target_agent: "rustant".to_string(),
        caller_skill: "security_review".to_string(),
        hop_count: 1,
        receipt_id: Some("rcpt-001".to_string()),
        timestamp: chrono::Utc::now(),
    });
    lane_state.checkpoints.insert(
        "lane-1".to_string(),
        StepCheckpoint {
            step_name: "develop-task-lane".to_string(),
            input_snapshot: serde_json::json!({"task_id": "T009"}),
            output_snapshot: Some(serde_json::json!({"branch": "swarm/mission-42/lane-1"})),
            completed_at: Some(chrono::Utc::now()),
            quarantined: true,
            diff_sha256: Some("deadbeef".to_string()),
        },
    );

    let expected_bytes = serde_json::to_vec_pretty(&lane_state).unwrap();
    let mut mock_s3 = factory_infrastructure::MockS3Storage::new();
    mock_s3
        .expect_put_object()
        .withf(|bucket, obj_key, _| {
            bucket == "dg-factory-checkpoints"
                && obj_key == "missions/mission-42/lanes/lane-1/checkpoint.json"
        })
        .times(1)
        .returning(|_, _, _| Ok(()));
    mock_s3
        .expect_get_object()
        .with(
            mockall::predicate::eq("dg-factory-checkpoints"),
            mockall::predicate::eq("missions/mission-42/lanes/lane-1/checkpoint.json"),
        )
        .times(1)
        .returning(move |_, _| Ok(expected_bytes.clone()));

    lane_state
        .save_lane_checkpoint("mission-42", "lane-1", &mock_s3, "dg-factory-checkpoints")
        .await
        .expect("save_lane_checkpoint should succeed");

    let loaded = BridgeState::load_lane_checkpoint(
        "mission-42",
        "lane-1",
        &mock_s3,
        "dg-factory-checkpoints",
    )
    .await
    .expect("load_lane_checkpoint should not error")
    .expect("lane checkpoint should exist");

    assert_eq!(loaded.a2a_entries.len(), 1);
    assert_eq!(loaded.a2a_entries[0].trace_id, "trace-abc-123");
    assert!(loaded.should_skip_completed_step("lane-1"));
    assert!(!loaded.should_skip_completed_step("lane-2"));
}

#[test]
fn test_pre_merge_quarantine_guard_blocks_unverified_r2r_promotion() {
    let mut snapshot = std::collections::HashMap::new();
    snapshot.insert("lib.rs".to_string(), "sha-1".to_string());

    let mut cp = WorkspaceStepCheckpoint {
        step_id: "lane-1".to_string(),
        task_id: "T009".to_string(),
        execution_id: "exec-1".to_string(),
        step_index: 1,
        status: "COMPLETED".to_string(),
        output_payload_hash: CheckpointHelper::compute_payload_hash("diff-content"),
        state_snapshot: snapshot,
        quarantined: true,
    };

    let err = assert_promotable_to_r2r(&cp).unwrap_err();
    assert!(
        err.to_string().to_lowercase().contains("quarantin"),
        "expected quarantine rejection, got: {}",
        err
    );

    mark_stage5_verified(&mut cp).expect("completed valid checkpoint should promote");
    assert!(!cp.quarantined);
    assert_eq!(cp.status, "VERIFIED");
    assert_promotable_to_r2r(&cp).expect("Stage 5 verified checkpoint must pass R2R guard");
}
