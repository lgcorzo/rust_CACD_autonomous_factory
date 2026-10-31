use factory_cli::operator::crd::{AgentWorkspace, AgentWorkspaceSpec, AgentWorkspaceStatus, WorkspacePhase, WorkspaceResourceLimits};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{Condition, ObjectMeta, Time};
use k8s_openapi::jiff::Timestamp;

#[test]
fn test_agent_workspace_serialization_and_columns() {
    let now = Time(Timestamp::now());
    let ws = AgentWorkspace {
        metadata: ObjectMeta {
            name: Some("test-workspace".to_string()),
            namespace: Some("development".to_string()),
            ..Default::default()
        },
        spec: AgentWorkspaceSpec {
            repository_url: "https://github.com/lgcorzo/rust_CACD_autonomous_factory".to_string(),
            target_branch: "main".to_string(),
            constitution_ref: "docs/constitution.md".to_string(),
            mission_ref: Some("mission-001".to_string()),
            resource_limits: WorkspaceResourceLimits {
                max_ram: "30Mi".to_string(),
                max_cpu: "250m".to_string(),
            },
        },
        status: Some(AgentWorkspaceStatus {
            phase: WorkspacePhase::Running,
            active_mission_ref: Some("mission-001".to_string()),
            active_mission_commit: Some("c8efca85".to_string()),
            hatchet_workflow_run_id: Some("wf-run-12345".to_string()),
            observed_generation: Some(1),
            conditions: vec![
                Condition {
                    type_: "MissionAccepted".to_string(),
                    status: "True".to_string(),
                    reason: "Dispatched".to_string(),
                    message: "Dispatched to Hatchet".to_string(),
                    last_transition_time: now.clone(),
                    observed_generation: Some(1),
                },
                Condition {
                    type_: "Synced".to_string(),
                    status: "True".to_string(),
                    reason: "Current".to_string(),
                    message: "Status projected".to_string(),
                    last_transition_time: now,
                    observed_generation: Some(1),
                },
            ],
        }),
    };

    // Serialize to YAML & JSON
    let json_val = serde_json::to_value(&ws).expect("Serialization to json failed");
    assert_eq!(json_val["status"]["phase"], "Running");
    assert_eq!(json_val["status"]["activeMissionRef"], "mission-001");
    assert_eq!(json_val["status"]["activeMissionCommit"], "c8efca85");
    assert_eq!(json_val["status"]["conditions"][0]["reason"], "Dispatched");
    assert_eq!(json_val["status"]["conditions"][1]["status"], "True");
}

