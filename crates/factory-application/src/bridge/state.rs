use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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

    fn get_checkpoint_path(mission_id: &str) -> PathBuf {
        PathBuf::from(format!("/tmp/bridge_state_{}.json", mission_id))
    }

    pub fn load_checkpoint(mission_id: &str) -> anyhow::Result<Option<Self>> {
        let path = Self::get_checkpoint_path(mission_id);
        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&path)?;
        let state: Self = serde_json::from_str(&content)?;
        Ok(Some(state))
    }

    pub fn save_checkpoint(&mut self) -> anyhow::Result<()> {
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let path = Self::get_checkpoint_path(&self.mission_id);
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }
}
