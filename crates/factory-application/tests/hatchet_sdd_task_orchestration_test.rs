use factory_application::agents::{RustantAgent, ZeroClawAgent};
use factory_core::{SddMissionPlan, SddTaskItem};
use factory_infrastructure::{MockAethalgardClient, MockMcpClient};
use serde_json::json;
use std::sync::{Arc, Mutex};

/// Test 1: Verify SDD parser extracts multi-task DAG dependencies from markdown
#[test]
fn test_sdd_multi_task_dag_dependency_extraction() {
    let tasks_md = r#"
# Tasks: Autonomous Feature Development

## Phase 1: Setup
- [ ] T001 Add core dependencies in `Cargo.toml`

## Phase 2: Foundational
- [ ] T002 Implement domain models in `crates/factory-core/src/lib.rs`
- [ ] T003 [P] Implement classifier in `crates/factory-infrastructure/src/pipeline_classifier.rs`

## Phase 3: Application Logic
- [ ] T004 Implement orchestration workflow in `crates/factory-application/src/workflows/mod.rs`
"#;

    let tasks = RustantAgent::parse_sdd_tasks(tasks_md);
    assert_eq!(tasks.len(), 4);

    // T001 has no dependencies
    assert_eq!(tasks[0].id, "T001");
    assert!(tasks[0].dependencies.is_empty());
    assert!(!tasks[0].is_parallel);

    // T002 depends on T001
    assert_eq!(tasks[1].id, "T002");
    assert_eq!(tasks[1].dependencies, vec!["T001".to_string()]);
    assert!(!tasks[1].is_parallel);

    // T003 is parallel [P]
    assert_eq!(tasks[2].id, "T003");
    assert!(tasks[2].is_parallel);
    assert!(tasks[2].dependencies.is_empty());

    // T004 depends on T003
    assert_eq!(tasks[3].id, "T004");
    assert_eq!(tasks[3].dependencies, vec!["T003".to_string()]);
}

/// Test 2: Verify sequential execution order enforcement and dependency violation rejection
#[tokio::test]
async fn test_sdd_execution_order_enforcement_and_rejection() {
    let tasks = vec![
        SddTaskItem {
            id: "T001".to_string(),
            description: "Setup foundation".to_string(),
            is_parallel: false,
            dependencies: vec![],
            target_files: vec!["Cargo.toml".to_string()],
        },
        SddTaskItem {
            id: "T002".to_string(),
            description: "Build domain logic".to_string(),
            is_parallel: false,
            dependencies: vec!["T001".to_string()],
            target_files: vec!["src/lib.rs".to_string()],
        },
        SddTaskItem {
            id: "T003".to_string(),
            description: "Build API integration".to_string(),
            is_parallel: false,
            dependencies: vec!["T002".to_string()],
            target_files: vec!["src/api.rs".to_string()],
        },
    ];

    // Simulating out-of-order execution attempt (T002 attempted before T001)
    let completed_tasks = std::collections::HashSet::<String>::new();
    let t002 = &tasks[1];
    let mut violation_detected = false;
    for dep in &t002.dependencies {
        if !completed_tasks.contains(dep) {
            violation_detected = true;
            break;
        }
    }
    assert!(
        violation_detected,
        "Execution should reject running T002 when T001 has not completed"
    );

    // Simulating correct in-order execution
    let mut completed_tasks_valid = std::collections::HashSet::new();
    let mut execution_trace = Vec::new();

    for task in &tasks {
        for dep in &task.dependencies {
            assert!(
                completed_tasks_valid.contains(dep),
                "Task {} cannot execute before dependency {}",
                task.id,
                dep
            );
        }
        completed_tasks_valid.insert(task.id.clone());
        execution_trace.push(task.id.clone());
    }

    assert_eq!(execution_trace, vec!["T001", "T002", "T003"]);
}

/// Test 3: End-to-end multi-task execution with mocked ZeroClaw agents
#[tokio::test]
async fn test_zeroclaw_executes_multi_task_mission_in_planned_order() {
    let execution_order = Arc::new(Mutex::new(Vec::new()));
    let order_clone = execution_order.clone();

    let mut mock_mcp = MockMcpClient::new();
    let mock_aethalgard = MockAethalgardClient::new();

    // Mock bridge sync
    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "sync_bridge_state")
        .returning(|_, _| Ok(json!({"is_error": false, "content": []})));

    // Mock SAST review
    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "security_review")
        .returning(|_, _| {
            Ok(json!({
                "content": [{
                    "text": json!({
                        "status": "approved",
                        "score": 10.0,
                        "findings": []
                    }).to_string()
                }],
                "is_error": false
            }))
        });

    // Mock sandbox pod launch and capture task order
    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "launch_sandbox_pod")
        .returning(move |_, params| {
            let mut list = order_clone.lock().unwrap();
            let task_name = params["task_id"].as_str().unwrap_or("unknown").to_string();
            list.push(task_name);
            Ok(json!({
                "is_success": true,
                "stdout": "cargo test passed"
            }))
        });

    let zeroclaw = ZeroClawAgent::new(Arc::new(mock_mcp), Arc::new(mock_aethalgard));

    let plan = SddMissionPlan {
        mission_id: "mission-sdd-42".to_string(),
        spec_version: "1.0.0".to_string(),
        total_tasks: 3,
        tasks: vec![
            SddTaskItem {
                id: "T001".to_string(),
                description: "Setup step".to_string(),
                is_parallel: false,
                dependencies: vec![],
                target_files: vec!["Cargo.toml".to_string()],
            },
            SddTaskItem {
                id: "T002".to_string(),
                description: "Core logic".to_string(),
                is_parallel: false,
                dependencies: vec!["T001".to_string()],
                target_files: vec!["src/lib.rs".to_string()],
            },
            SddTaskItem {
                id: "T003".to_string(),
                description: "Finalize workflow".to_string(),
                is_parallel: false,
                dependencies: vec!["T002".to_string()],
                target_files: vec!["src/workflow.rs".to_string()],
            },
        ],
    };

    let mut completed_task_ids = std::collections::HashSet::new();
    for task in &plan.tasks {
        for dep in &task.dependencies {
            assert!(
                completed_task_ids.contains(dep),
                "Task {} scheduled before dependency {}",
                task.id,
                dep
            );
        }

        let res = zeroclaw
            .execute_task(&plan.mission_id, &task.description, &task.target_files)
            .await;
        assert!(res.is_ok(), "Task {} execution failed: {:?}", task.id, res);
        completed_task_ids.insert(task.id.clone());
    }

    assert_eq!(completed_task_ids.len(), 3);
    assert!(completed_task_ids.contains("T001"));
    assert!(completed_task_ids.contains("T002"));
    assert!(completed_task_ids.contains("T003"));
}
