use factory_application::bridge::{BridgeState, StepCheckpoint};

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
