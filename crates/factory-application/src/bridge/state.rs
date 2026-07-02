use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpecKitState {
    pub spec_file_path: String,
    pub plan_file_path: String,
    pub active_task_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SuperpowersState {
    pub current_workspace: String,
    pub active_branch: String,
    pub completed_tasks: Vec<String>,
    pub pending_tasks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BridgeState {
    pub mission_id: String,
    pub spec_kit: SpecKitState,
    pub superpowers: SuperpowersState,
    pub last_updated: u64,
}

impl BridgeState {
    pub fn new(mission_id: String) -> Self {
        Self {
            mission_id,
            spec_kit: SpecKitState::default(),
            superpowers: SuperpowersState::default(),
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    fn get_checkpoint_key(mission_id: &str) -> String {
        format!("bridge_state_{}.json", mission_id)
    }

    pub async fn load_checkpoint(
        mission_id: &str,
        s3: &dyn factory_infrastructure::S3Storage,
        bucket: &str,
    ) -> anyhow::Result<Option<Self>> {
        let key = Self::get_checkpoint_key(mission_id);
        match s3.get_object(bucket, &key).await {
            Ok(data) => {
                let content = String::from_utf8(data)?;
                let state: Self = serde_json::from_str(&content)?;
                Ok(Some(state))
            }
            Err(e) => {
                // If the object doesn't exist, we return None.
                // In AWS SDK this is a specific error, but for simplicity we treat any fetch error as None
                // if it's "NoSuchKey", but we can just trace and return None for now.
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

        let key = Self::get_checkpoint_key(&self.mission_id);
        let content = serde_json::to_string_pretty(self)?;
        s3.put_object(bucket, &key, content.into_bytes()).await?;
        Ok(())
    }
}
