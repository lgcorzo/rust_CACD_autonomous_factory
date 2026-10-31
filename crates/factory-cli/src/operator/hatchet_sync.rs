use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchedMissionEvent {
    pub workspace_uid: String,
    pub workspace_name: String,
    pub mission_ref: String,
    pub commit_sha: String,
    pub constitution_hash: String,
    pub max_ram: String,
    pub max_cpu: String,
    pub idempotency_key: String,
}

pub struct HatchetSync;

impl HatchetSync {
    /// Dispatches mission to Hatchet / Kafka bus using composite idempotency key (workspace_uid:mission_ref)
    /// and queueing concurrency strategy per workspace.
    pub async fn dispatch_mission(event: DispatchedMissionEvent) -> Result<String> {
        info!(
            workspace_uid = %event.workspace_uid,
            mission_ref = %event.mission_ref,
            idempotency_key = %event.idempotency_key,
            commit_sha = %event.commit_sha,
            "Dispatching mission with Hatchet queueing concurrency"
        );

        // Generates deterministic simulated run ID based on idempotency key
        let run_id = format!(
            "wf-run-{}",
            uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_DNS, event.idempotency_key.as_bytes())
                .simple()
        );
        Ok(run_id)
    }

    /// Checks if a workflow run for given (workspace_uid, mission_ref) is currently running or completed
    pub async fn check_mission_status(
        _workspace_uid: &str,
        _mission_ref: &str,
    ) -> Result<Option<String>> {
        // Query simulation: in production, queries Hatchet gRPC API
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_idempotent_dispatch_key() {
        let event = DispatchedMissionEvent {
            workspace_uid: "uid-12345".to_string(),
            workspace_name: "test-ws".to_string(),
            mission_ref: "mission-01".to_string(),
            commit_sha: "7a8f9c2100000000000000000000000000000000".to_string(),
            constitution_hash: "hash123".to_string(),
            max_ram: "30Mi".to_string(),
            max_cpu: "250m".to_string(),
            idempotency_key: "uid-12345:mission-01".to_string(),
        };

        let run_id1 = HatchetSync::dispatch_mission(event.clone()).await.unwrap();
        let run_id2 = HatchetSync::dispatch_mission(event).await.unwrap();
        assert_eq!(run_id1, run_id2);
        assert!(run_id1.starts_with("wf-run-"));
    }
}
