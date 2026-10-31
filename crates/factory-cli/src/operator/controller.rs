use crate::operator::crd::{AgentWorkspace, AgentWorkspaceStatus, WorkspacePhase};
use crate::operator::git_resolver::GitResolver;
use crate::operator::hatchet_sync::{DispatchedMissionEvent, HatchetSync};
use anyhow::Result;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{Condition, Time};
use k8s_openapi::jiff::Timestamp;
use kube::{
    Client,
    api::{Api, Patch, PatchParams, ResourceExt},
    runtime::controller::Action,
};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

pub struct ContextData {
    pub client: Client,
    pub namespace: String,
}

pub struct WorkspaceController;

impl WorkspaceController {
    /// Evaluates if status patch should be emitted (strictly throttled to major phase transitions)
    pub fn should_update_status(
        current: &AgentWorkspaceStatus,
        next_phase: WorkspacePhase,
    ) -> bool {
        current.phase != next_phase
    }

    /// Primary reconciler loop for AgentWorkspace objects
    pub async fn reconcile(ws: Arc<AgentWorkspace>, ctx: Arc<ContextData>) -> Result<Action> {
        let name = ws.name_any();
        let namespace = ws.namespace().unwrap_or_else(|| ctx.namespace.clone());
        let spec = &ws.spec;
        let status = ws.status.clone().unwrap_or_default();
        let uid = ws.uid().unwrap_or_default();

        info!(name = %name, namespace = %namespace, phase = ?status.phase, "Reconciling AgentWorkspace");

        let api: Api<AgentWorkspace> = Api::namespaced(ctx.client.clone(), &namespace);

        // Check if deletion is in progress
        if ws.metadata.deletion_timestamp.is_some() {
            info!(name = %name, "Deletion timestamp detected; deferring to finalizer sequence");
            return Ok(Action::requeue(Duration::from_secs(5)));
        }

        // If missionRef is specified, check dispatch state
        if let Some(ref mref) = spec.mission_ref {
            // Check if already dispatched under this missionRef
            if status.active_mission_ref.as_deref() == Some(mref)
                && status.phase == WorkspacePhase::Running
            {
                info!(name = %name, mission_ref = %mref, "Mission already actively running");
                return Ok(Action::requeue(Duration::from_secs(60)));
            }

            // Resolve Git commit & constitution hash snapshot
            let git_info = GitResolver::resolve_commit_and_constitution(
                &spec.repository_url,
                &spec.target_branch,
                &spec.constitution_ref,
            )?;

            // Check if already dispatched under this commit or executed
            if status.active_mission_commit.as_deref() == Some(&git_info.commit_sha) {
                info!(name = %name, mission_ref = %mref, commit = %git_info.commit_sha, "Mission commit already executed");
                let now = Time(Timestamp::now());
                let condition = Condition {
                    type_: "MissionAccepted".to_string(),
                    status: "False".to_string(),
                    reason: "AlreadyExecuted".to_string(),
                    message: format!(
                        "Commit {} already executed for mission {}",
                        git_info.commit_sha, mref
                    ),
                    last_transition_time: now,
                    observed_generation: ws.metadata.generation,
                };
                let patch_json = serde_json::json!({
                    "status": {
                        "conditions": [condition]
                    }
                });
                let pp = PatchParams::apply("factory-operator");
                api.patch_status(&name, &pp, &Patch::Merge(&patch_json))
                    .await?;
                return Ok(Action::requeue(Duration::from_secs(60)));
            }

            // Check if active mission is currently running in another run (deferred mission)
            if status.phase == WorkspacePhase::Running
                && status.active_mission_ref.as_deref() != Some(mref)
            {
                info!(name = %name, active = ?status.active_mission_ref, pending = %mref, "Mission active; deferring until complete");
                let now = Time(Timestamp::now());
                let condition = Condition {
                    type_: "MissionAccepted".to_string(),
                    status: "False".to_string(),
                    reason: "MissionActive".to_string(),
                    message: format!(
                        "Mission {} deferred because active mission is running",
                        mref
                    ),
                    last_transition_time: now,
                    observed_generation: ws.metadata.generation,
                };
                let patch_json = serde_json::json!({
                    "status": {
                        "conditions": [condition]
                    }
                });
                let pp = PatchParams::apply("factory-operator");
                api.patch_status(&name, &pp, &Patch::Merge(&patch_json))
                    .await?;
                return Ok(Action::requeue(Duration::from_secs(60)));
            }

            // Dispatch mission to Hatchet
            let event = DispatchedMissionEvent {
                workspace_uid: uid.clone(),
                workspace_name: name.clone(),
                mission_ref: mref.clone(),
                commit_sha: git_info.commit_sha.clone(),
                constitution_hash: git_info.constitution_hash.clone(),
                max_ram: spec.resource_limits.max_ram.clone(),
                max_cpu: spec.resource_limits.max_cpu.clone(),
                idempotency_key: format!("{}:{}", uid, mref),
            };

            let run_id = HatchetSync::dispatch_mission(event).await?;

            // Patch status with throttled one-way projection
            let now = Time(Timestamp::now());
            let condition = Condition {
                type_: "MissionAccepted".to_string(),
                status: "True".to_string(),
                reason: "Dispatched".to_string(),
                message: format!("Mission {} dispatched to Hatchet run {}", mref, run_id),
                last_transition_time: now.clone(),
                observed_generation: ws.metadata.generation,
            };

            let synced_cond = Condition {
                type_: "Synced".to_string(),
                status: "True".to_string(),
                reason: "Current".to_string(),
                message: "Workspace status projected from Hatchet SSOT".to_string(),
                last_transition_time: now,
                observed_generation: ws.metadata.generation,
            };

            let patch_json = serde_json::json!({
                "status": {
                    "phase": "Running",
                    "activeMissionRef": mref,
                    "activeMissionCommit": git_info.commit_sha,
                    "hatchetWorkflowRunId": run_id,
                    "observedGeneration": ws.metadata.generation,
                    "conditions": [condition, synced_cond]
                }
            });

            let pp = PatchParams::apply("factory-operator");
            api.patch_status(&name, &pp, &Patch::Merge(&patch_json))
                .await?;
            info!(name = %name, mission_ref = %mref, run_id = %run_id, "Updated AgentWorkspace status to Running");
        }

        // Requeue periodic poller to resume deferred tasks every 60s
        Ok(Action::requeue(Duration::from_secs(60)))
    }

    pub fn error_policy(
        _ws: Arc<AgentWorkspace>,
        err: &anyhow::Error,
        _ctx: Arc<ContextData>,
    ) -> Action {
        error!(error = %err, "Reconciliation error occurred");
        Action::requeue(Duration::from_secs(10))
    }
}
