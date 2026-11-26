use factory_application::agents::ZeroClawAgent;
use factory_application::workflows::autonomous_mission::partition_execution_waves;
use factory_application::workflows::develop_task::{
    TaskInput, TaskOutput, execute_develop_task_lane_with_clients,
};
use factory_application::workspaces::worktree::WorktreeManager;
use factory_core::SddTaskItem;
use factory_infrastructure::{MockAethalgardClient, MockMcpClient, MockS3Storage};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

struct TempDirGuard(PathBuf);

impl TempDirGuard {
    fn new(prefix: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("{}-{}", prefix, Uuid::new_v4()));
        std::fs::create_dir_all(&dir).expect("create temp dir");
        Self(dir)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn init_test_git_repo(repo_dir: &Path) {
    let run_git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo_dir)
            .output()
            .expect("git command failed to spawn");
        assert!(
            out.status.success(),
            "git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&out.stderr)
        );
    };

    run_git(&["init", "-b", "main"]);
    run_git(&["config", "user.email", "swarm-test@darkgravity.io"]);
    run_git(&["config", "user.name", "Swarm Test"]);
    std::fs::write(repo_dir.join("README.md"), "# Seed Repo\n").unwrap();
    run_git(&["add", "."]);
    run_git(&["commit", "-m", "chore: initial seed commit"]);
}

#[test]
fn test_partition_execution_waves_sequential_and_parallel() {
    let sequential_tasks = vec![
        SddTaskItem {
            id: "T001".to_string(),
            description: "First step".to_string(),
            is_parallel: false,
            dependencies: vec![],
            target_files: vec!["a.rs".to_string()],
        },
        SddTaskItem {
            id: "T002".to_string(),
            description: "Second step".to_string(),
            is_parallel: false,
            dependencies: vec!["T001".to_string()],
            target_files: vec!["b.rs".to_string()],
        },
    ];
    let seq_waves = partition_execution_waves(&sequential_tasks).unwrap();
    assert_eq!(
        seq_waves.len(),
        2,
        "FR-020: zero [P] tasks remain sequential"
    );
    assert_eq!(seq_waves[0].len(), 1);
    assert_eq!(seq_waves[1].len(), 1);

    let swarm_tasks = vec![
        SddTaskItem {
            id: "T001".to_string(),
            description: "Foundation".to_string(),
            is_parallel: false,
            dependencies: vec![],
            target_files: vec!["mod.rs".to_string()],
        },
        SddTaskItem {
            id: "T002".to_string(),
            description: "[P] Lane alpha".to_string(),
            is_parallel: true,
            dependencies: vec!["T001".to_string()],
            target_files: vec!["alpha.rs".to_string()],
        },
        SddTaskItem {
            id: "T003".to_string(),
            description: "[P] Lane beta".to_string(),
            is_parallel: true,
            dependencies: vec!["T001".to_string()],
            target_files: vec!["beta.rs".to_string()],
        },
        SddTaskItem {
            id: "T004".to_string(),
            description: "[P] Lane gamma".to_string(),
            is_parallel: true,
            dependencies: vec!["T001".to_string()],
            target_files: vec!["gamma.rs".to_string()],
        },
        SddTaskItem {
            id: "T005".to_string(),
            description: "Integration".to_string(),
            is_parallel: false,
            dependencies: vec!["T002".to_string(), "T003".to_string(), "T004".to_string()],
            target_files: vec!["lib.rs".to_string()],
        },
    ];

    let waves = partition_execution_waves(&swarm_tasks).unwrap();
    assert_eq!(waves.len(), 3);
    assert_eq!(
        waves[0].iter().map(|t| t.id.as_str()).collect::<Vec<_>>(),
        vec!["T001"]
    );
    assert_eq!(
        waves[1].iter().map(|t| t.id.as_str()).collect::<Vec<_>>(),
        vec!["T002", "T003", "T004"]
    );
    assert_eq!(
        waves[2].iter().map(|t| t.id.as_str()).collect::<Vec<_>>(),
        vec!["T005"]
    );
}

#[tokio::test]
async fn test_worktree_manager_3_lane_isolation_and_cleanup() {
    let base = TempDirGuard::new("swarm-wt-3lane");
    let repo_dir = base.path().join("repo");
    let wt_root = base.path().join("worktrees");
    std::fs::create_dir_all(&repo_dir).unwrap();
    init_test_git_repo(&repo_dir);

    let manager = WorktreeManager::new(&repo_dir, &wt_root);
    let mission_id = "mission-swarm-3lanes";

    // Provision 3 concurrent lanes cleanly
    let (l1, l2, l3) = tokio::try_join!(
        manager.provision_lane_worktree(mission_id, "lane-1", Some("main")),
        manager.provision_lane_worktree(mission_id, "lane-2", Some("main")),
        manager.provision_lane_worktree(mission_id, "lane-3", Some("main")),
    )
    .expect("3-lane parallel worktree provisioning must succeed");

    assert_ne!(l1.path, l2.path);
    assert_ne!(l2.path, l3.path);
    assert_eq!(l1.branch_name, "swarm/mission-swarm-3lanes/lane-1");
    assert_eq!(l2.branch_name, "swarm/mission-swarm-3lanes/lane-2");
    assert_eq!(l3.branch_name, "swarm/mission-swarm-3lanes/lane-3");

    // Mutate distinct files inside each isolated lane without touching repo_dir
    std::fs::write(l1.path.join("lane1.txt"), "alpha\n").unwrap();
    std::fs::write(l2.path.join("lane2.txt"), "beta\n").unwrap();
    std::fs::write(l3.path.join("lane3.txt"), "gamma\n").unwrap();

    assert!(!repo_dir.join("lane1.txt").exists());
    assert!(!l1.path.join("lane2.txt").exists());
    assert!(!l2.path.join("lane3.txt").exists());

    let sha1 = manager
        .commit_lane_changes(&l1, "feat(lane-1): add alpha")
        .await
        .unwrap();
    let sha2 = manager
        .commit_lane_changes(&l2, "feat(lane-2): add beta")
        .await
        .unwrap();
    let sha3 = manager
        .commit_lane_changes(&l3, "feat(lane-3): add gamma")
        .await
        .unwrap();
    assert!(sha1.is_some() && sha2.is_some() && sha3.is_some());

    let active = manager.list_mission_worktrees(mission_id).await.unwrap();
    assert_eq!(active.len(), 3);

    // Prune all worktrees after Fan-In
    let pruned = manager.prune_mission_worktrees(mission_id).await.unwrap();
    assert_eq!(pruned, 3);
    assert!(
        manager
            .list_mission_worktrees(mission_id)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(!l1.path.exists() && !l2.path.exists() && !l3.path.exists());
}

#[tokio::test]
async fn test_worktree_manager_rejects_path_traversal() {
    let base = TempDirGuard::new("swarm-wt-traversal");
    let repo_dir = base.path().join("repo");
    let wt_root = base.path().join("worktrees");
    std::fs::create_dir_all(&repo_dir).unwrap();
    init_test_git_repo(&repo_dir);

    let manager = WorktreeManager::new(&repo_dir, &wt_root);
    assert!(
        manager
            .provision_lane_worktree("../escape", "lane-1", Some("main"))
            .await
            .is_err()
    );
    assert!(
        manager
            .provision_lane_worktree("mission-1", "../../etc/passwd", Some("main"))
            .await
            .is_err()
    );
    assert!(
        manager
            .provision_lane_worktree("mission-1", "", Some("main"))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_zeroclaw_execute_task_in_worktree_attaches_isolation_and_priority() {
    let captured_launch = Arc::new(Mutex::new(None::<serde_json::Value>));
    let captured_clone = Arc::clone(&captured_launch);

    let mut mock_mcp = MockMcpClient::new();
    mock_mcp
        .expect_call_tool_json()
        .returning(move |tool, args| match tool {
            "sync_bridge_state" => Ok(json!({ "is_error": false, "content": [] })),
            "security_review" => Ok(json!({
                "is_error": false,
                "content": [{ "text": "{\"score\": 9.5, \"status\": \"approved\"}" }]
            })),
            "launch_sandbox_pod" => {
                *captured_clone.lock().unwrap() = Some(args.clone());
                Ok(json!({ "is_error": false, "content": [{ "text": "LANE_OK" }] }))
            }
            _ => Ok(json!({ "is_error": false })),
        });

    let agent = ZeroClawAgent::new(Arc::new(mock_mcp), Arc::new(MockAethalgardClient::new()));
    let wt_path = Path::new("/tmp/dg-worktrees/mission-42/lane-2");
    let res = agent
        .execute_task_in_worktree(
            "mission-42",
            "Implement isolated module",
            &["src/mod_b.rs".to_string()],
            Some(wt_path),
        )
        .await
        .expect("execute_task_in_worktree must succeed");

    assert_eq!(res["content"][0]["text"], "LANE_OK");
    let launch_args = captured_launch.lock().unwrap().clone().unwrap();
    assert_eq!(
        launch_args["worktree_path"],
        "/tmp/dg-worktrees/mission-42/lane-2"
    );
    assert_eq!(launch_args["inference_priority"], "Standard-Execution");
}

#[tokio::test]
async fn test_develop_task_lane_input_output_backward_compat_and_checkpoint() {
    let legacy_json = r#"{"task_id":"T009","description":"legacy task","relevant_files":["a.rs"]}"#;
    let legacy: TaskInput = serde_json::from_str(legacy_json).expect("legacy TaskInput parses");
    assert_eq!(legacy.task_id, "T009");
    assert!(legacy.mission_id.is_none());
    assert!(legacy.lane_id.is_none());

    let base = TempDirGuard::new("swarm-develop-lane");
    let repo_dir = base.path().join("repo");
    let wt_root = base.path().join("worktrees");
    std::fs::create_dir_all(&repo_dir).unwrap();
    init_test_git_repo(&repo_dir);
    let wt_manager = WorktreeManager::new(&repo_dir, &wt_root);

    let s3_store = Arc::new(Mutex::new(
        std::collections::HashMap::<String, Vec<u8>>::new(),
    ));
    let s3_put = Arc::clone(&s3_store);
    let s3_get = Arc::clone(&s3_store);
    let mut mock_s3 = MockS3Storage::new();
    mock_s3.expect_put_object().returning(move |_, key, data| {
        s3_put.lock().unwrap().insert(key.to_string(), data);
        Ok(())
    });
    mock_s3.expect_get_object().returning(move |_, key| {
        s3_get
            .lock()
            .unwrap()
            .get(key)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("missing"))
    });

    let mut mock_mcp = MockMcpClient::new();
    mock_mcp
        .expect_call_tool_json()
        .returning(|tool, _| match tool {
            "sync_bridge_state" => Ok(json!({ "is_error": false, "content": [] })),
            "security_review" => Ok(json!({
                "is_error": false,
                "content": [{ "text": "{\"score\": 9.1, \"status\": \"approved\"}" }]
            })),
            "launch_sandbox_pod" => Ok(json!({ "is_error": false, "content": [{ "text": "OK" }] })),
            _ => Ok(json!({ "is_error": false })),
        });

    let lane_input = TaskInput {
        task_id: "T009".to_string(),
        description: "Build lane module".to_string(),
        relevant_files: vec!["src/lane.rs".to_string()],
        mission_id: Some("mission-lane-exec".to_string()),
        lane_id: Some("lane-T009".to_string()),
        worktree_path: None,
        branch_name: None,
        inference_priority: Some("Standard-Execution".to_string()),
    };

    let output: TaskOutput = execute_develop_task_lane_with_clients(
        &lane_input,
        Arc::new(mock_mcp),
        Arc::new(MockAethalgardClient::new()),
        &mock_s3,
        "dg-factory-checkpoints",
        Some(&wt_manager),
    )
    .await
    .expect("lane execution must succeed");

    assert_eq!(output.lane_id.as_deref(), Some("lane-T009"));
    assert!(output.quarantined);
    assert_eq!(
        output.branch_name.as_deref(),
        Some("swarm/mission-lane-exec/lane-T009")
    );
    assert!(
        s3_store
            .lock()
            .unwrap()
            .contains_key("missions/mission-lane-exec/lanes/lane-T009/checkpoint.json"),
        "quarantined per-lane checkpoint must be written to S3"
    );

    // Crash-resume skip on second execution with same mission_id/lane_id
    let mock_mcp_second = MockMcpClient::new();
    let skipped = execute_develop_task_lane_with_clients(
        &lane_input,
        Arc::new(mock_mcp_second),
        Arc::new(MockAethalgardClient::new()),
        &mock_s3,
        "dg-factory-checkpoints",
        Some(&wt_manager),
    )
    .await
    .expect("resume should short-circuit cleanly");
    assert_eq!(skipped.result["status"], "already_done");

    wt_manager
        .prune_mission_worktrees("mission-lane-exec")
        .await
        .unwrap();
}
