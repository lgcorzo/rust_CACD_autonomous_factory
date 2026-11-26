use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fmt::Write;

fn default_quarantined() -> bool {
    true
}

/// Step checkpoint stored in object storage (MinIO S3)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StepCheckpoint {
    pub step_id: String,
    pub task_id: String,
    pub execution_id: String,
    pub step_index: u32,
    pub status: String,
    pub output_payload_hash: String,
    pub state_snapshot: HashMap<String, String>,
    #[serde(default = "default_quarantined")]
    pub quarantined: bool,
}

/// Pre-merge quarantine guard rejecting promotion of unverified lane artifacts to R2R.
pub fn assert_promotable_to_r2r(checkpoint: &StepCheckpoint) -> Result<()> {
    if checkpoint.quarantined {
        return Err(anyhow!(
            "Checkpoint '{}' is still quarantined and cannot trigger R2R ingestion",
            checkpoint.step_id
        ));
    }
    if checkpoint.status != "VERIFIED" || checkpoint.output_payload_hash.is_empty() {
        return Err(anyhow!(
            "Checkpoint '{}' must be Stage 5 VERIFIED with a non-empty hash before R2R promotion",
            checkpoint.step_id
        ));
    }
    Ok(())
}

/// Marks a completed checkpoint as Stage 5 verified and clears its quarantine flag.
pub fn mark_stage5_verified(checkpoint: &mut StepCheckpoint) -> Result<()> {
    if checkpoint.status != "COMPLETED" && checkpoint.status != "VERIFIED" {
        return Err(anyhow!(
            "Checkpoint '{}' cannot clear quarantine from status '{}'",
            checkpoint.step_id,
            checkpoint.status
        ));
    }
    if checkpoint.output_payload_hash.is_empty() {
        return Err(anyhow!(
            "Checkpoint '{}' has empty output_payload_hash and cannot be verified",
            checkpoint.step_id
        ));
    }
    checkpoint.status = "VERIFIED".to_string();
    checkpoint.quarantined = false;
    Ok(())
}

pub struct CheckpointHelper;

impl CheckpointHelper {
    /// Computes SHA256 hash of output payload
    pub fn compute_payload_hash(payload: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        let result = hasher.finalize();
        let mut s = String::with_capacity(result.len() * 2);
        for byte in result {
            let _ = write!(&mut s, "{:02x}", byte);
        }
        s
    }

    /// Verifies if a checkpoint is valid and complete before resuming a DAG step
    pub fn verify_checkpoint(checkpoint: &StepCheckpoint, expected_step_id: &str) -> Result<bool> {
        if checkpoint.step_id != expected_step_id {
            return Err(anyhow!(
                "Checkpoint step_id mismatch: expected {}, found {}",
                expected_step_id,
                checkpoint.step_id
            ));
        }

        if checkpoint.status != "COMPLETED" && checkpoint.status != "VERIFIED" {
            return Ok(false);
        }

        if checkpoint.output_payload_hash.is_empty() {
            return Err(anyhow!("Invalid checkpoint: output_payload_hash is empty"));
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkpoint_verification() {
        let mut snapshot = HashMap::new();
        snapshot.insert("file.rs".to_string(), "hash123".to_string());

        let valid_chk = StepCheckpoint {
            step_id: "step-1".to_string(),
            task_id: "task-abc".to_string(),
            execution_id: "run-001".to_string(),
            step_index: 1,
            status: "COMPLETED".to_string(),
            output_payload_hash: CheckpointHelper::compute_payload_hash("result data"),
            state_snapshot: snapshot,
            quarantined: true,
        };

        assert!(CheckpointHelper::verify_checkpoint(&valid_chk, "step-1").unwrap());
        assert!(
            !CheckpointHelper::verify_checkpoint(
                &StepCheckpoint {
                    status: "FAILED".to_string(),
                    ..valid_chk.clone()
                },
                "step-1"
            )
            .unwrap()
        );
        assert!(CheckpointHelper::verify_checkpoint(&valid_chk, "step-2").is_err());
    }
}
