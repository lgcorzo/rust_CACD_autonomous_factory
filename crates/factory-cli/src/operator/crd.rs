use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Custom Resource Definition for AgentWorkspace (darkgravity.io/v1alpha1)
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema, PartialEq)]
#[kube(
    group = "darkgravity.io",
    version = "v1alpha1",
    kind = "AgentWorkspace",
    plural = "agentworkspaces",
    namespaced,
    status = "AgentWorkspaceStatus",
    shortname = "agws",
    printcolumn = r#"{"name":"Phase", "type":"string", "jsonPath":".status.phase"}"#,
    printcolumn = r#"{"name":"Mission", "type":"string", "jsonPath":".status.activeMissionRef"}"#,
    printcolumn = r#"{"name":"Commit", "type":"string", "jsonPath":".status.activeMissionCommit"}"#,
    printcolumn = r#"{"name":"Accepted", "type":"string", "jsonPath":".status.conditions[?(@.type=='MissionAccepted')].reason"}"#,
    printcolumn = r#"{"name":"Synced", "type":"string", "jsonPath":".status.conditions[?(@.type=='Synced')].status"}"#,
    printcolumn = r#"{"name":"Run", "type":"string", "jsonPath":".status.hatchetWorkflowRunId", "priority":1}"#,
    printcolumn = r#"{"name":"Age", "type":"date", "jsonPath":".metadata.creationTimestamp"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct AgentWorkspaceSpec {
    /// Git repository clone URL (immutable)
    pub repository_url: String,

    /// Target branch pointer (immutable)
    pub target_branch: String,

    /// Constitution document reference path
    pub constitution_ref: String,

    /// Explicit user-requested mission reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission_ref: Option<String>,

    /// Resource bounds for agent tasks
    pub resource_limits: WorkspaceResourceLimits,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceResourceLimits {
    /// Maximum container memory ceiling (e.g., "30Mi")
    pub max_ram: String,

    /// Maximum container CPU ceiling (e.g., "250m")
    pub max_cpu: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentWorkspaceStatus {
    /// High-level lifecycle phase
    #[serde(default = "default_phase")]
    pub phase: WorkspacePhase,

    /// Active mission reference being orchestrated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_mission_ref: Option<String>,

    /// Pinned commit SHA resolved at mission dispatch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_mission_commit: Option<String>,

    /// Associated Hatchet workflow run ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatchet_workflow_run_id: Option<String>,

    /// Observed spec generation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    /// Standard k8s lifecycle conditions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<Condition>,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema)]
pub enum WorkspacePhase {
    #[default]
    Pending,
    Initializing,
    Running,
    Paused,
    Completed,
    Failed,
}

fn default_phase() -> WorkspacePhase {
    WorkspacePhase::Pending
}

#[cfg(test)]
mod tests {
    use super::*;
    use kube::CustomResourceExt;

    #[test]
    fn test_crd_schema_generation() {
        let crd = AgentWorkspace::crd();
        assert_eq!(crd.spec.group, "darkgravity.io");
        assert_eq!(crd.spec.names.kind, "AgentWorkspace");
        assert_eq!(crd.spec.names.plural, "agentworkspaces");
        assert_eq!(crd.spec.names.singular.as_deref(), Some("agentworkspace"));
        
        let yaml = serde_yaml::to_string(&crd).expect("failed to serialize CRD to YAML");
        assert!(yaml.contains("darkgravity.io"));
        assert!(yaml.contains("AgentWorkspace"));
    }
}

