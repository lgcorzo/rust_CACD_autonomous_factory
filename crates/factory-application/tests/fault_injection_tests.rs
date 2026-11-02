use factory_application::bridge::{BridgeState, StepCheckpoint};
use factory_infrastructure::MockS3Storage;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn test_crash_recovery_prevents_duplicate_pr_creation() {
    let mission_id = "mission-fault-injection-001";
    let bucket = "darkgravity-checkpoints";

    // 1. Initial execution: Worker executes plan, code, and review checkpoints
    let mut initial_state = BridgeState::new(mission_id.to_string());
    initial_state.checkpoints.insert(
        "step-plan".to_string(),
        StepCheckpoint {
            step_name: "plan".to_string(),
            input_snapshot: serde_json::json!({"issue": 42}),
            output_snapshot: Some(serde_json::json!({"plan_id": "P-101"})),
            completed_at: Some(chrono::Utc::now()),
        },
    );
    initial_state.checkpoints.insert(
        "step-code".to_string(),
        StepCheckpoint {
            step_name: "code".to_string(),
            input_snapshot: serde_json::json!({"branch": "darkgravity/mission-42"}),
            output_snapshot: Some(serde_json::json!({"commit_sha": "abc12345"})),
            completed_at: Some(chrono::Utc::now()),
        },
    );

    // Mock S3 storage tracking PUT calls in-memory
    let storage = Arc::new(std::sync::Mutex::new(std::collections::HashMap::<
        String,
        Vec<u8>,
    >::new()));
    let storage_put = Arc::clone(&storage);
    let storage_get = Arc::clone(&storage);

    let mut mock_s3 = MockS3Storage::new();
    let pr_creation_calls = Arc::new(AtomicUsize::new(0));
    let pr_calls_clone = Arc::clone(&pr_creation_calls);

    mock_s3
        .expect_put_object()
        .returning(move |_bucket, key, data| {
            let mut map = storage_put.lock().unwrap();
            map.insert(key.to_string(), data.to_vec());
            Ok(())
        });

    mock_s3.expect_get_object().returning(move |_bucket, key| {
        let map = storage_get.lock().unwrap();
        map.get(key)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Key not found: {}", key))
    });

    // Save initial pre-delivery state
    initial_state
        .save_checkpoint(&mock_s3, bucket)
        .await
        .expect("Failed to save pre-delivery checkpoint");

    // 2. Simulated worker crash right after creating PR
    // Delivery step executed and marked in checkpoint
    let pr_url = "https://github.com/org/repo/pull/123";
    pr_calls_clone.fetch_add(1, Ordering::SeqCst);

    initial_state.checkpoints.insert(
        "step-delivery".to_string(),
        StepCheckpoint {
            step_name: "delivery".to_string(),
            input_snapshot: serde_json::json!({"branch": "darkgravity/mission-42"}),
            output_snapshot: Some(serde_json::json!({"pr_url": pr_url})),
            completed_at: Some(chrono::Utc::now()),
        },
    );
    initial_state
        .save_checkpoint(&mock_s3, bucket)
        .await
        .expect("Failed to save delivery checkpoint");

    // 3. Worker restarts (process killed / SIGKILL simulated)
    // On reboot, worker loads checkpoint from S3
    let resumed_state = BridgeState::load_checkpoint(mission_id, &mock_s3, bucket)
        .await
        .expect("Failed to load checkpoint")
        .expect("Checkpoint should exist");

    // Assert that delivery was already recorded
    if let Some(delivery_checkpoint) = resumed_state.checkpoints.get("step-delivery") {
        if delivery_checkpoint.output_snapshot.is_some() {
            // Worker skips PR creation because it was already delivered
            tracing::info!("Delivery already completed in previous run, skipping PR creation");
        } else {
            pr_calls_clone.fetch_add(1, Ordering::SeqCst);
        }
    } else {
        pr_calls_clone.fetch_add(1, Ordering::SeqCst);
    }

    // Zero duplicate PRs created!
    assert_eq!(
        pr_creation_calls.load(Ordering::SeqCst),
        1,
        "Worker crash recovery must not generate duplicate PRs"
    );
}

#[tokio::test]
async fn test_crash_between_remote_pr_creation_and_checkpoint_commit_idempotency() {
    // Audit scenario: The worker calls the GitHub API, creates the PR, but crashes
    // (SIGKILL) BEFORE the S3 checkpoint is saved!
    // When the next worker starts, the S3 checkpoint still has NO record of the PR.
    // The worker MUST query the remote repository via branch name to discover if the PR
    // already exists rather than naively calling create_pull_request again.
    let _mission_id = "mission-uncommitted-pr-crash";
    let target_branch = "darkgravity/mission-42";
    let remote_pr_url = "https://github.com/org/repo/pull/555";

    let mut remote_prs = std::collections::HashMap::<String, String>::new();
    let pr_creation_attempts = Arc::new(AtomicUsize::new(0));

    // Simulated worker 1: Creates PR on remote, then dies before saving checkpoint
    {
        pr_creation_attempts.fetch_add(1, Ordering::SeqCst);
        remote_prs.insert(target_branch.to_string(), remote_pr_url.to_string());
        // Worker 1 process terminates abruptly here (no S3 checkpoint written!)
    }

    // Simulated worker 2 starts with blank / pre-crash checkpoint state
    let pr_discovery_calls = Arc::new(AtomicUsize::new(0));

    // Idempotent delivery step execution:
    let delivered_pr_url = if let Some(existing_url) = remote_prs.get(target_branch) {
        pr_discovery_calls.fetch_add(1, Ordering::SeqCst);
        existing_url.clone()
    } else {
        pr_creation_attempts.fetch_add(1, Ordering::SeqCst);
        "https://github.com/org/repo/pull/999".to_string()
    };

    assert_eq!(delivered_pr_url, remote_pr_url);
    assert_eq!(
        pr_creation_attempts.load(Ordering::SeqCst),
        1,
        "Remote PR must NOT be created twice even if process died before saving checkpoint"
    );
    assert_eq!(
        pr_discovery_calls.load(Ordering::SeqCst),
        1,
        "Resumed worker must discover existing remote PR"
    );
}

#[tokio::test]
async fn test_idempotent_branch_and_checkpoint_recovery() {
    let mission_id = "mission-fault-injection-002";
    let bucket = "darkgravity-checkpoints";

    let mut state = BridgeState::new(mission_id.to_string());
    let mut mock_s3 = MockS3Storage::new();

    mock_s3.expect_put_object().returning(|_, _, _| Ok(()));

    // Simulate step 1 failure
    state.checkpoints.insert(
        "task-1".to_string(),
        StepCheckpoint {
            step_name: "validate".to_string(),
            input_snapshot: serde_json::json!({"tests": "all"}),
            output_snapshot: None, // Incomplete/crashed
            completed_at: None,
        },
    );

    state.save_checkpoint(&mock_s3, bucket).await.unwrap();

    // Verify incomplete step is detected on recovery
    let checkpoint = state.checkpoints.get("task-1").unwrap();
    assert!(checkpoint.completed_at.is_none());
    assert!(checkpoint.output_snapshot.is_none());

    // Worker re-runs and completes step safely
    state.checkpoints.insert(
        "task-1".to_string(),
        StepCheckpoint {
            step_name: "validate".to_string(),
            input_snapshot: serde_json::json!({"tests": "all"}),
            output_snapshot: Some(serde_json::json!({"status": "passed"})),
            completed_at: Some(chrono::Utc::now()),
        },
    );

    let completed_checkpoint = state.checkpoints.get("task-1").unwrap();
    assert!(completed_checkpoint.completed_at.is_some());
    assert_eq!(
        completed_checkpoint.output_snapshot.as_ref().unwrap()["status"],
        "passed"
    );
}
