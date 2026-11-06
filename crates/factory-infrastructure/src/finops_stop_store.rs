use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinOpsStopState {
    pub halted: bool,
    pub state_version: u64,
    pub reason: Option<String>,
    pub current_spend: f64,
    pub max_daily_budget: f64,
    pub updated_at: DateTime<Utc>,
}

impl Default for FinOpsStopState {
    fn default() -> Self {
        Self {
            halted: false,
            state_version: 0,
            reason: None,
            current_spend: 0.0,
            max_daily_budget: 50.0,
            updated_at: Utc::now(),
        }
    }
}

pub trait FinOpsStopStore: Send + Sync {
    /// Reads the current authoritative stop state. Implementations MUST fail closed on read error.
    fn read_state(&self) -> Result<FinOpsStopState, String>;

    /// Atomically updates the stop state, incrementing `state_version`.
    fn write_state(
        &self,
        halted: bool,
        reason: Option<String>,
        current_spend: f64,
        max_daily_budget: f64,
    ) -> Result<FinOpsStopState, String>;
}

/// Durable file-backed FinOps stop store enabling cross-worker visibility across OS processes
/// or threads sharing the same state path.
pub struct FileFinOpsStopStore {
    path: PathBuf,
    lock: Mutex<()>,
}

impl FileFinOpsStopStore {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            lock: Mutex::new(()),
        }
    }

    pub fn default_shared() -> Arc<dyn FinOpsStopStore> {
        static GLOBAL_STORE: OnceLock<Arc<dyn FinOpsStopStore>> = OnceLock::new();
        GLOBAL_STORE
            .get_or_init(|| {
                let path = std::env::var("FINOPS_STOP_STORE_PATH")
                    .map(PathBuf::from)
                    .unwrap_or_else(|_| {
                        std::env::temp_dir().join(format!(
                            "dark_gravity_finops_stop_state_{}.json",
                            std::process::id()
                        ))
                    });
                Arc::new(FileFinOpsStopStore::new(path))
            })
            .clone()
    }
}

impl FinOpsStopStore for FileFinOpsStopStore {
    fn read_state(&self) -> Result<FinOpsStopState, String> {
        let _guard = self
            .lock
            .lock()
            .map_err(|e| format!("FinOpsStopStore lock poisoned: {}", e))?;

        if !self.path.exists() {
            return Ok(FinOpsStopState::default());
        }

        let raw = std::fs::read_to_string(&self.path).map_err(|e| {
            format!(
                "Failed to read FinOps stop store at '{}': {}",
                self.path.display(),
                e
            )
        })?;

        serde_json::from_str::<FinOpsStopState>(&raw).map_err(|e| {
            format!(
                "Corrupted FinOps stop store JSON at '{}': {}",
                self.path.display(),
                e
            )
        })
    }

    fn write_state(
        &self,
        halted: bool,
        reason: Option<String>,
        current_spend: f64,
        max_daily_budget: f64,
    ) -> Result<FinOpsStopState, String> {
        let _guard = self
            .lock
            .lock()
            .map_err(|e| format!("FinOpsStopStore lock poisoned: {}", e))?;

        let current_version = if self.path.exists() {
            let raw = std::fs::read_to_string(&self.path).map_err(|e| {
                format!(
                    "Failed to read existing FinOps stop store at '{}': {}",
                    self.path.display(),
                    e
                )
            })?;
            let parsed: FinOpsStopState = serde_json::from_str(&raw).map_err(|e| {
                format!(
                    "Corrupted FinOps stop store JSON at '{}': {}",
                    self.path.display(),
                    e
                )
            })?;
            parsed.state_version
        } else {
            0
        };

        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                format!(
                    "Failed to create parent directory '{}': {}",
                    parent.display(),
                    e
                )
            })?;
        }

        let next = FinOpsStopState {
            halted,
            state_version: current_version + 1,
            reason,
            current_spend,
            max_daily_budget,
            updated_at: Utc::now(),
        };

        let tmp_path = self.path.with_extension("json.tmp");
        let serialized = serde_json::to_string_pretty(&next)
            .map_err(|e| format!("Failed to serialize FinOpsStopState: {}", e))?;
        std::fs::write(&tmp_path, serialized).map_err(|e| {
            format!(
                "Failed to write temporary FinOps stop state '{}': {}",
                tmp_path.display(),
                e
            )
        })?;
        std::fs::rename(&tmp_path, &self.path).map_err(|e| {
            format!(
                "Failed to atomically commit FinOps stop state '{}': {}",
                self.path.display(),
                e
            )
        })?;

        Ok(next)
    }
}

/// Deterministic failing store used to verify fail-closed admission on storage read/write errors.
pub struct FailingFinOpsStopStore {
    error_message: String,
}

impl FailingFinOpsStopStore {
    pub fn new(error_message: impl Into<String>) -> Self {
        Self {
            error_message: error_message.into(),
        }
    }
}

impl FinOpsStopStore for FailingFinOpsStopStore {
    fn read_state(&self) -> Result<FinOpsStopState, String> {
        Err(self.error_message.clone())
    }

    fn write_state(
        &self,
        _halted: bool,
        _reason: Option<String>,
        _current_spend: f64,
        _max_daily_budget: f64,
    ) -> Result<FinOpsStopState, String> {
        Err(self.error_message.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_finops_stop_store_cross_worker_visibility_and_versioning() {
        let path =
            std::env::temp_dir().join(format!("unit_finops_stop_{}.json", uuid::Uuid::new_v4()));
        let worker_a = FileFinOpsStopStore::new(path.clone());
        let worker_b = FileFinOpsStopStore::new(path.clone());

        let s0 = worker_b.read_state().unwrap();
        assert!(!s0.halted);
        assert_eq!(s0.state_version, 0);

        let s1 = worker_a
            .write_state(true, Some("Budget reached 90%".to_string()), 45.0, 50.0)
            .unwrap();
        assert!(s1.halted);
        assert_eq!(s1.state_version, 1);

        let observed_by_b = worker_b.read_state().unwrap();
        assert!(observed_by_b.halted);
        assert_eq!(observed_by_b.state_version, 1);
        assert_eq!(observed_by_b.reason.as_deref(), Some("Budget reached 90%"));

        let _ = std::fs::remove_file(&path);
    }
}
