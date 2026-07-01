use factory_application::bridge::BridgeState;

#[tokio::test]
async fn test_bridge_state_crash_resilience() {
    let mission_id = "test-mission-crash-123";

    // 1. Initial State: Agent starts task
    let mut initial_state = BridgeState::new(mission_id.to_string());
    assert!(initial_state.superpowers.completed_tasks.is_empty());

    // Agent completes a sub-task and saves checkpoint
    initial_state
        .superpowers
        .completed_tasks
        .push("subtask-1".to_string());
    initial_state
        .save_checkpoint()
        .expect("Failed to save checkpoint");

    // 2. Simulate Crash
    // The program crashes here. All memory is lost.

    // 3. Recovery: Agent restarts and loads checkpoint
    let recovered_state = BridgeState::load_checkpoint(mission_id)
        .expect("Failed to load checkpoint")
        .expect("Checkpoint file should exist");

    // Verify state is recovered
    assert_eq!(recovered_state.mission_id, mission_id);
    assert_eq!(recovered_state.superpowers.completed_tasks.len(), 1);
    assert_eq!(recovered_state.superpowers.completed_tasks[0], "subtask-1");

    // Cleanup
    let path = format!("/tmp/bridge_state_{}.json", mission_id);
    std::fs::remove_file(path).ok();
}
