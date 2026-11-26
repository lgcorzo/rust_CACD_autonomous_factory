use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum BridgeStatus {
    #[default]
    Idle,
    Running,
    Completed,
    Failed,
}

fn default_quarantined() -> bool {
    true
}

fn validate_safe_path_segment(label: &str, value: &str) -> anyhow::Result<()> {
    if value.is_empty()
        || value.contains("..")
        || value.contains('/')
        || value.contains('\\')
        || value.contains('\0')
    {
        anyhow::bail!("Invalid {} segment in checkpoint key: '{}'", label, value);
    }
    Ok(())
}

/// Quarantine-aware checkpoint state for a workflow step or Swarm lane.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepCheckpoint {
    pub step_name: String,
    pub input_snapshot: serde_json::Value,
    pub output_snapshot: Option<serde_json::Value>,
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(default = "default_quarantined")]
    pub quarantined: bool,
    #[serde(default)]
    pub diff_sha256: Option<String>,
}

impl Default for StepCheckpoint {
    fn default() -> Self {
        Self {
            step_name: String::new(),
            input_snapshot: serde_json::Value::Null,
            output_snapshot: None,
            completed_at: None,
            quarantined: true,
            diff_sha256: None,
        }
    }
}

/// A2A peer invocation lineage record persisted alongside bridge state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct A2aBridgeEntry {
    pub trace_id: String,
    pub target_agent: String,
    pub caller_skill: String,
    pub hop_count: u8,
    pub receipt_id: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Blocks quarantined or incomplete bridge checkpoints from R2R promotion.
pub fn assert_bridge_checkpoint_promotable(cp: &StepCheckpoint) -> anyhow::Result<()> {
    if cp.quarantined {
        anyhow::bail!(
            "Step checkpoint '{}' is quarantined and cannot be promoted to R2R",
            cp.step_name
        );
    }
    if cp.output_snapshot.is_none() {
        anyhow::bail!(
            "Step checkpoint '{}' has no output snapshot for R2R promotion",
            cp.step_name
        );
    }
    Ok(())
}

/// Releases a completed bridge checkpoint from quarantine after Stage 5 verification.
pub fn mark_bridge_checkpoint_verified(cp: &mut StepCheckpoint) -> anyhow::Result<()> {
    if cp.output_snapshot.is_none() {
        anyhow::bail!(
            "Cannot unquarantine step checkpoint '{}' without an output snapshot",
            cp.step_name
        );
    }
    cp.quarantined = false;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeState {
    pub session_id: String,
    pub spec_version: String,
    pub current_step: String,
    pub checkpoints: HashMap<String, StepCheckpoint>,
    pub run_status: BridgeStatus,
    pub last_updated: u64,
    #[serde(default)]
    pub a2a_entries: Vec<A2aBridgeEntry>,
}

impl BridgeState {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            spec_version: "1.0".to_string(),
            current_step: "init".to_string(),
            checkpoints: HashMap::new(),
            run_status: BridgeStatus::Idle,
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            a2a_entries: Vec::new(),
        }
    }

    fn get_checkpoint_key(session_id: &str) -> String {
        format!("bridge_state_{}.json", session_id)
    }

    /// Returns the isolated per-lane S3 checkpoint object key.
    pub fn lane_checkpoint_key(mission_id: &str, lane_id: &str) -> anyhow::Result<String> {
        validate_safe_path_segment("mission_id", mission_id)?;
        validate_safe_path_segment("lane_id", lane_id)?;
        Ok(format!(
            "missions/{}/lanes/{}/checkpoint.json",
            mission_id, lane_id
        ))
    }

    /// Appends an A2A delegation lineage entry to this state snapshot.
    pub fn record_a2a_entry(&mut self, entry: A2aBridgeEntry) {
        self.a2a_entries.push(entry);
    }

    /// Returns true when the given step key is already durably completed for crash-resume skipping.
    pub fn should_skip_completed_step(&self, step_key: &str) -> bool {
        self.checkpoints
            .get(step_key)
            .is_some_and(|cp| cp.completed_at.is_some() && cp.output_snapshot.is_some())
    }

    async fn load_from_key(
        key: &str,
        s3: &dyn factory_infrastructure::S3Storage,
        bucket: &str,
    ) -> anyhow::Result<Option<Self>> {
        match s3.get_object(bucket, key).await {
            Ok(data) => {
                let content = String::from_utf8(data)?;
                let state: Self = serde_json::from_str(&content)?;
                Ok(Some(state))
            }
            Err(e) => {
                tracing::debug!("Checkpoint not found or error loading {}: {}", key, e);
                Ok(None)
            }
        }
    }

    async fn save_to_key(
        &mut self,
        key: &str,
        s3: &dyn factory_infrastructure::S3Storage,
        bucket: &str,
    ) -> anyhow::Result<()> {
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let content = serde_json::to_string_pretty(self)?;
        s3.put_object(bucket, key, content.into_bytes()).await?;
        Ok(())
    }

    pub async fn load_checkpoint(
        session_id: &str,
        s3: &dyn factory_infrastructure::S3Storage,
        bucket: &str,
    ) -> anyhow::Result<Option<Self>> {
        let key = Self::get_checkpoint_key(session_id);
        Self::load_from_key(&key, s3, bucket).await
    }

    pub async fn save_checkpoint(
        &mut self,
        s3: &dyn factory_infrastructure::S3Storage,
        bucket: &str,
    ) -> anyhow::Result<()> {
        let key = Self::get_checkpoint_key(&self.session_id);
        self.save_to_key(&key, s3, bucket).await
    }

    /// Loads a Swarm lane checkpoint at `missions/<mission_id>/lanes/<lane_id>/checkpoint.json`.
    pub async fn load_lane_checkpoint(
        mission_id: &str,
        lane_id: &str,
        s3: &dyn factory_infrastructure::S3Storage,
        bucket: &str,
    ) -> anyhow::Result<Option<Self>> {
        let key = Self::lane_checkpoint_key(mission_id, lane_id)?;
        Self::load_from_key(&key, s3, bucket).await
    }

    /// Persists a Swarm lane checkpoint at `missions/<mission_id>/lanes/<lane_id>/checkpoint.json`.
    pub async fn save_lane_checkpoint(
        &mut self,
        mission_id: &str,
        lane_id: &str,
        s3: &dyn factory_infrastructure::S3Storage,
        bucket: &str,
    ) -> anyhow::Result<()> {
        let key = Self::lane_checkpoint_key(mission_id, lane_id)?;
        self.save_to_key(&key, s3, bucket).await
    }
}
