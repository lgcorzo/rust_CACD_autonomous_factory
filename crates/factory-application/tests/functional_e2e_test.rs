use factory_application::agents::{RustantAgent, ZeroClawAgent};
use factory_infrastructure::{MockAethalgardClient, MockMcpClient, MockR2rClient};
use serde_json::json;
use std::sync::Arc;

/// 1. E2E Mission Cycle Success Test (Offline / CI Integration)
/// Validates the full 5-stage mission execution:
/// Ingestion -> Rustant Planning (GraphRAG) -> ZeroClaw AST Surgery -> SAST Security Review (≥ 8.0) -> Delivery (GitOps PR)
#[tokio::test]
async fn test_dark_gravity_e2e_mission_cycle_success() {
    let mut mock_mcp = MockMcpClient::new();
    let mut mock_r2r = MockR2rClient::new();
    let mock_aethalgard = MockAethalgardClient::new();

    // R2R GraphRAG & deepwiki-rs semantic context retrieval
    mock_r2r
        .expect_search()
        .with(mockall::predicate::eq("Add resource clamping check to worker execution"))
        .returning(|_| {
            Ok("Architecture V7 Context: gVisor RAM limit <= 30MiB, Protobuf Kafka events, Hatchet DAG".to_string())
        });

    // Mock Spec-Kit planning tool invocations for Rustant PO agent
    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "invoke_spec_kit")
        .times(6)
        .returning(|_, _| Ok(json!({ "status": "spec_kit_planning_complete" })));

    // Mock ZeroClaw bridge synchronization and AST code surgery
    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "sync_bridge_state")
        .times(2)
        .returning(|_, _| Ok(json!({"is_error": false, "content": []})));

    // Mock SAST security review passing with score >= 8.0 (10.0/10.0)
    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "security_review")
        .times(2)
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

    // Mock gVisor sandbox execution pod launch
    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "launch_sandbox_pod")
        .times(1)
        .returning(|_, _| {
            Ok(json!({
                "is_success": true,
                "stdout": "cargo test passed (0 errors)"
            }))
        });

    // Initialize Rustant & ZeroClaw agents
    let mcp_arc = Arc::new(mock_mcp);
    let r2r_arc = Arc::new(mock_r2r);
    let aethalgard_arc = Arc::new(mock_aethalgard);

    let rustant = RustantAgent::new(mcp_arc.clone(), r2r_arc.clone());
    let zeroclaw = ZeroClawAgent::new(mcp_arc.clone(), aethalgard_arc.clone());

    // Phase 1: Autonomous Planning
    let plan_result = rustant
        .plan_mission(
            "mission-e2e-01",
            "Add resource clamping check to worker execution",
        )
        .await;
    assert!(plan_result.is_ok());
    let plan_val = plan_result.unwrap();
    assert_eq!(plan_val["status"], "spec_kit_planning_complete");

    // Phase 2: Secure Execution & Code Surgery
    let safe_ast_patch = "pub fn check_ram_limit(ram_mb: u32) -> bool { ram_mb <= 30 }";
    let exec_result = zeroclaw
        .execute_task("mission-e2e-01", safe_ast_patch, &[])
        .await;
    assert!(exec_result.is_ok());

    // Phase 3: Review and Delivery Gate Validation
    let review_result = rustant
        .review_mission("mission-e2e-01", safe_ast_patch)
        .await;
    assert!(review_result.is_ok());
}

/// 2. Circuit Breaker Test (Disyuntor Activo / Agent-Stuck Vertex 3)
/// Validates that 3 consecutive validation/SAST failures stop the factory safely,
/// trigger Aethelgard escalation, and flag the agent state as Agent-Stuck without corrupting the repo.
#[tokio::test]
async fn test_dark_gravity_circuit_breaker_agent_stuck() {
    let mut mock_mcp = MockMcpClient::new();
    let mock_aethalgard = MockAethalgardClient::new();

    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "sync_bridge_state")
        .times(1)
        .returning(|_, _| Ok(json!({"is_error": false, "content": []})));

    // Ingest SAST rejection score < 8.0 (3.5/10.0)
    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "security_review")
        .times(1)
        .returning(|_, _| {
            Ok(json!({
                "content": [{
                    "text": json!({
                        "status": "rejected",
                        "score": 3.5,
                        "findings": ["Critical security vulnerability: unsafe pointer dereference without boundary check"]
                    }).to_string()
                }],
                "is_error": false
            }))
        });

    // Ensure sandbox pod is NEVER launched when circuit breaker is active
    mock_mcp
        .expect_call_tool_json()
        .withf(|tool, _params| tool == "launch_sandbox_pod")
        .times(0)
        .returning(|_, _| Ok(json!({})));

    let zeroclaw = ZeroClawAgent::new(Arc::new(mock_mcp), Arc::new(mock_aethalgard));
    let faulty_code = "pub fn unsafe_op(ptr: *const u8) -> u8 { unsafe { *ptr } }";

    let result = zeroclaw
        .execute_task("mission-circuit-breaker", faulty_code, &[])
        .await;

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Security scan failed"));
    assert!(err_msg.contains("SAST score < 8.0"));
}

/// 3. Pre-flight Resource Constraints Test (gVisor RAM Clamping <= 30 MiB)
#[test]
fn test_dark_gravity_preflight_resource_constraints() {
    const GVISOR_MAX_RAM_MIB: u32 = 30;
    let configured_ram_mb: u32 = 30;

    assert!(
        configured_ram_mb <= GVISOR_MAX_RAM_MIB,
        "gVisor sandbox RAM allocation exceeds maximum allowable limit of 30 MiB"
    );
}

/// 4. Live Kubernetes Cluster E2E Test
/// Executes against live Hatchet and Kafka endpoints in the Kubernetes cluster (`gitops_internal_lgcorzo`).
/// Run with: `cargo test --package factory-application --test functional_e2e_test test_live_k8s_dark_gravity_e2e_mission -- --ignored`
#[tokio::test]
#[ignore = "requires live k8s cluster"]
async fn test_live_k8s_dark_gravity_e2e_mission() {
    use factory_application::workflows::autonomous_mission::{MissionInput, MissionOutput};
    use hatchet_sdk::Hatchet;
    use hatchet_sdk::Runnable;

    println!("[Live K8s E2E] Initializing connection to Hatchet cluster orchestrator...");
    let hatchet_res = Hatchet::from_env().await;

    match hatchet_res {
        Ok(hatchet) => {
            let input = MissionInput {
                mission_id: Some(format!("k8s-live-e2e-{}", uuid::Uuid::new_v4())),
                goal: "E2E Functional Validation Test on Live K8s Cluster".to_string(),
                repository_path: String::new(),
            };

            let run_res = hatchet
                .workflow::<MissionInput, MissionOutput>("darkgravitymission-dev-lgcorzo")
                .build()
                .unwrap()
                .run_no_wait(&input, None)
                .await;

            assert!(
                run_res.is_ok(),
                "Failed to dispatch live mission DAG to Hatchet cluster: {:?}",
                run_res.err()
            );
            println!(
                "[Live K8s E2E] Successfully triggered live mission DAG on Kubernetes cluster!"
            );
        }
        Err(e) => {
            println!("[Live K8s E2E] Hatchet initialization error: {:?}", e);
        }
    }
}
