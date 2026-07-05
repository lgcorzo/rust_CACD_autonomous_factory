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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepCheckpoint {
    pub step_name: String,
    pub input_snapshot: serde_json::Value,
    pub output_snapshot: Option<serde_json::Value>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeState {
    pub session_id: String,
    pub spec_version: String,
    pub current_step: String,
    pub checkpoints: HashMap<String, StepCheckpoint>,
    pub run_status: BridgeStatus,
    pub last_updated: u64,
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
        }
    }

    fn get_checkpoint_key(session_id: &str) -> String {
        format!("bridge_state_{}.json", session_id)
    }

    pub async fn load_checkpoint(
        session_id: &str,
        s3: &dyn factory_infrastructure::S3Storage,
        bucket: &str,
    ) -> anyhow::Result<Option<Self>> {
        let key = Self::get_checkpoint_key(session_id);
        match s3.get_object(bucket, &key).await {
            Ok(data) => {
                let content = String::from_utf8(data)?;
                let state: Self = serde_json::from_str(&content)?;
                Ok(Some(state))
            }
            Err(e) => {
                tracing::debug!("Checkpoint not found or error loading: {}", e);
                Ok(None)
            }
        }
    }

    pub async fn save_checkpoint(
        &mut self,
        s3: &dyn factory_infrastructure::S3Storage,
        bucket: &str,
    ) -> anyhow::Result<()> {
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let key = Self::get_checkpoint_key(&self.session_id);
        let content = serde_json::to_string_pretty(self)?;
        s3.put_object(bucket, &key, content.into_bytes()).await?;
        Ok(())
    }
}
