use factory_application::bridge::BridgeState;

#[tokio::test]
async fn test_bridge_state_crash_resilience() {
    let mission_id = "test-mission-123";
    let mut state = BridgeState::new(mission_id.to_string());
    state.superpowers.completed_tasks.push("task-1".to_string());
    
    // We mock S3 Storage to store the state locally in a hashmap or just simulate successful save/load
    let mut mock_s3 = factory_infrastructure::MockS3Storage::new();
    
    let expected_state = serde_json::to_string_pretty(&state).unwrap().into_bytes();
    let expected_state_clone = expected_state.clone();
    
    mock_s3.expect_put_object()
        .with(mockall::predicate::eq("test-bucket"), mockall::predicate::eq(format!("bridge_state_{}.json", mission_id)), mockall::predicate::eq(expected_state))
        .times(1)
        .returning(|_, _, _| Ok(()));
        
    mock_s3.expect_get_object()
        .with(mockall::predicate::eq("test-bucket"), mockall::predicate::eq(format!("bridge_state_{}.json", mission_id)))
        .times(1)
        .returning(move |_, _| Ok(expected_state_clone.clone()));

    // Simulate save
    state
        .save_checkpoint(&mock_s3, "test-bucket")
        .await
        .expect("Failed to save checkpoint");

    // Simulate crash and load
    let recovered_state = BridgeState::load_checkpoint(mission_id, &mock_s3, "test-bucket")
        .await
        .expect("Failed to load checkpoint")
        .expect("Checkpoint file should exist");

    // Verify state is recovered
    assert_eq!(recovered_state.mission_id, mission_id);
    assert_eq!(recovered_state.superpowers.completed_tasks.len(), 1);
    assert_eq!(recovered_state.superpowers.completed_tasks[0], "task-1");
}
