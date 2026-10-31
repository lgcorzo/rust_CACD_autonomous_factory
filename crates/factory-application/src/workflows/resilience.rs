use crate::workspaces::checkpoints::{CheckpointHelper, StepCheckpoint};
use anyhow::Result;
use std::time::Duration;
use tracing::{info, warn};

/// Configuration for Hatchet DAG task step retry and fault tolerance
#[derive(Debug, Clone)]
pub struct StepRetryPolicy {
    pub max_retries: u32,
    pub backoff_initial_ms: u64,
    pub backoff_max_ms: u64,
    pub idempotent_key_prefix: String,
}

impl Default for StepRetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            backoff_initial_ms: 500,
            backoff_max_ms: 10_000,
            idempotent_key_prefix: "dag-step".to_string(),
        }
    }
}

pub struct DagResilienceHandler {
    policy: StepRetryPolicy,
}

impl DagResilienceHandler {
    pub fn new(policy: StepRetryPolicy) -> Self {
        Self { policy }
    }

    /// Evaluates if a DAG task step needs re-execution or can be safely recovered from checkpoint
    pub fn should_execute_step(
        &self,
        checkpoint: Option<&StepCheckpoint>,
        step_id: &str,
    ) -> Result<bool> {
        if let Some(chk) = checkpoint {
            if CheckpointHelper::verify_checkpoint(chk, step_id)? {
                info!(
                    step_id = %step_id,
                    execution_id = %chk.execution_id,
                    "Step verified in checkpoint; bypassing execution (at-least-once idempotency)"
                );
                return Ok(false);
            }
            warn!(
                step_id = %step_id,
                status = %chk.status,
                "Checkpoint step incomplete or failed; re-executing step"
            );
        }
        Ok(true)
    }

    /// Calculates exponential backoff duration for retry count
    pub fn calculate_backoff(&self, retry_count: u32) -> Duration {
        if retry_count == 0 {
            return Duration::from_millis(self.policy.backoff_initial_ms);
        }
        let exp = 2u64.saturating_pow(retry_count);
        let millis =
            (self.policy.backoff_initial_ms.saturating_mul(exp)).min(self.policy.backoff_max_ms);
        Duration::from_millis(millis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_dag_resilience_idempotency() {
        let handler = DagResilienceHandler::new(StepRetryPolicy::default());
        let step_id = "dag-step-codegen";

        // Without checkpoint -> must execute
        assert!(handler.should_execute_step(None, step_id).unwrap());

        // With completed checkpoint -> skip execution
        let chk = StepCheckpoint {
            step_id: step_id.to_string(),
            task_id: "task-1".to_string(),
            execution_id: "run-99".to_string(),
            step_index: 0,
            status: "COMPLETED".to_string(),
            output_payload_hash: CheckpointHelper::compute_payload_hash("result"),
            state_snapshot: HashMap::new(),
        };
        assert!(!handler.should_execute_step(Some(&chk), step_id).unwrap());

        // With failed status -> re-execute
        let failed_chk = StepCheckpoint {
            status: "FAILED".to_string(),
            ..chk.clone()
        };
        assert!(
            handler
                .should_execute_step(Some(&failed_chk), step_id)
                .unwrap()
        );
    }

    #[test]
    fn test_backoff_calculation() {
        let handler = DagResilienceHandler::new(StepRetryPolicy::default());
        let b0 = handler.calculate_backoff(0);
        let b1 = handler.calculate_backoff(1);
        let b2 = handler.calculate_backoff(2);
        assert_eq!(b0, Duration::from_millis(500));
        assert_eq!(b1, Duration::from_millis(1000));
        assert_eq!(b2, Duration::from_millis(2000));
    }
}
