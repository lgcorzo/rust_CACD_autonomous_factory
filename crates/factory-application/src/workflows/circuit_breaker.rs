use factory_core::security::SastScanResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CircuitBreakerStatus {
    Passed,
    Retrying { attempt: u32, max_attempts: u32 },
    AgentStuck { reason: String },
}

/// Normalizes a diff by stripping volatile headers and collapsing whitespace.
pub fn normalize_diff(diff: &str) -> String {
    diff.lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty()
                && !trimmed.starts_with("diff --git")
                && !trimmed.starts_with("index ")
                && !trimmed.starts_with("--- ")
                && !trimmed.starts_with("+++ ")
                && !trimmed.starts_with("@@ ")
        })
        .map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            words.join(" ")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// Computes a deterministic SHA-256 digest hex string for a normalized code diff.
pub fn compute_diff_hash(diff: &str) -> String {
    use sha2::{Digest, Sha256};
    let normalized = normalize_diff(diff);
    let mut hasher = Sha256::new();
    hasher.update(normalized.as_bytes());
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

pub const DEFAULT_MAX_DIFF_HISTORY: usize = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerGuard {
    pub max_attempts: u32,
    pub current_attempt: u32,
    pub min_safety_score: f32,
    pub max_history_capacity: usize,
    #[serde(default)]
    pub diff_hash_history: Vec<String>,
    #[serde(default)]
    pub tripped: bool,
}

impl Default for CircuitBreakerGuard {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            current_attempt: 0,
            min_safety_score: 8.0,
            max_history_capacity: DEFAULT_MAX_DIFF_HISTORY,
            diff_hash_history: Vec::new(),
            tripped: false,
        }
    }
}

impl CircuitBreakerGuard {
    pub fn new(max_attempts: u32, min_safety_score: f32) -> Self {
        Self {
            max_attempts,
            current_attempt: 0,
            min_safety_score,
            max_history_capacity: DEFAULT_MAX_DIFF_HISTORY,
            diff_hash_history: Vec::new(),
            tripped: false,
        }
    }

    /// Appends a diff hash while respecting the bounded history capacity limit
    fn record_diff_hash(&mut self, hash: String) {
        if self.diff_hash_history.len() >= self.max_history_capacity {
            self.diff_hash_history.remove(0);
        }
        self.diff_hash_history.push(hash);
    }

    pub fn is_tripped(&self) -> bool {
        self.tripped
    }

    pub fn reset(&mut self) {
        self.tripped = false;
        self.current_attempt = 0;
        self.diff_hash_history.clear();
    }

    /// Evaluates the code diff against the security threshold, tracks attempts, and detects deadlock or oscillation.
    pub fn evaluate_diff(&mut self, diff: &str) -> (CircuitBreakerStatus, SastScanResult) {
        let scan_result = SastScanResult::inspect_diff(diff);

        if self.tripped {
            return (
                CircuitBreakerStatus::AgentStuck {
                    reason: "Circuit breaker is in tripped state; manual human reset is required before further evaluations".to_string(),
                },
                scan_result,
            );
        }

        let diff_hash = compute_diff_hash(diff);

        if scan_result.is_safe
            && scan_result.score >= self.min_safety_score
            && !scan_result.critical_vulnerabilities_detected
        {
            self.record_diff_hash(diff_hash);
            (CircuitBreakerStatus::Passed, scan_result)
        } else {
            self.current_attempt += 1;

            // Deadlock / Oscillation detection:
            // If diff hash matches ANY previous diff in this session, agent is trapped in a loop
            if self.diff_hash_history.contains(&diff_hash) {
                self.tripped = true;
                let reason = if self.diff_hash_history.last() == Some(&diff_hash) {
                    format!(
                        "Deadlock detected on attempt {}: agent generated identical diff hash {} without progressing remediation.",
                        self.current_attempt, diff_hash
                    )
                } else {
                    format!(
                        "Oscillation deadlock detected on attempt {}: agent regenerated previous diff hash {} without progressing remediation.",
                        self.current_attempt, diff_hash
                    )
                };
                self.record_diff_hash(diff_hash);
                return (CircuitBreakerStatus::AgentStuck { reason }, scan_result);
            }

            self.record_diff_hash(diff_hash);

            if self.current_attempt >= self.max_attempts {
                self.tripped = true;
                let reason = format!(
                    "Security gate failed score={:.1}/10.0 (required >={:.1}) after {} automatic remediation attempts. Findings: {:?}",
                    scan_result.score,
                    self.min_safety_score,
                    self.current_attempt,
                    scan_result.findings
                );
                (CircuitBreakerStatus::AgentStuck { reason }, scan_result)
            } else {
                (
                    CircuitBreakerStatus::Retrying {
                        attempt: self.current_attempt,
                        max_attempts: self.max_attempts,
                    },
                    scan_result,
                )
            }
        }
    }

    /// Generates human architect escalation message when stuck, sanitizing reason against markdown breakages.
    pub fn format_stuck_alert(&self, repo: &str, pr_number: u64, reason: &str) -> String {
        let sanitized_reason = reason.replace('`', "'");
        format!(
            "🚨 **Dark Gravity Autonomous Factory Alert: Agent-Stuck State Triggered** 🚨\n\n\
            - **Repository**: `{}`\n\
            - **Pull/Merge Request**: `#{}`\n\
            - **Status**: `Agent-Stuck` (Circuit Breaker Opened)\n\
            - **Escalation Reason**: {}\n\n\
            *Autonomous loop halted to prevent token budget depletion. Human architect review required before resuming.*",
            repo, pr_number, sanitized_reason
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker_pass() {
        let mut guard = CircuitBreakerGuard::default();
        let clean_diff = "+ pub fn calculate_total(a: u32, b: u32) -> u32 { a + b }";

        let (status, scan) = guard.evaluate_diff(clean_diff);
        assert_eq!(status, CircuitBreakerStatus::Passed);
        assert!(scan.score >= 8.0);
        assert!(!guard.is_tripped());
    }

    #[test]
    fn test_circuit_breaker_retries_and_stuck() {
        let mut guard = CircuitBreakerGuard::new(3, 8.0);
        let dangerous_diff1 = "+ let password = \"secret123\"; eval(user_input);";
        let dangerous_diff2 = "+ let password = \"secret456\"; eval(user_input);";
        let dangerous_diff3 = "+ let password = \"secret789\"; eval(user_input);";

        // Attempt 1: Retrying (1/3)
        let (status1, _) = guard.evaluate_diff(dangerous_diff1);
        assert_eq!(
            status1,
            CircuitBreakerStatus::Retrying {
                attempt: 1,
                max_attempts: 3
            }
        );
        assert!(!guard.is_tripped());

        // Attempt 2: Retrying (2/3) with different diff to avoid deadlock
        let (status2, _) = guard.evaluate_diff(dangerous_diff2);
        assert_eq!(
            status2,
            CircuitBreakerStatus::Retrying {
                attempt: 2,
                max_attempts: 3
            }
        );
        assert!(!guard.is_tripped());

        // Attempt 3: Exhausted -> AgentStuck and tripped
        let (status3, _) = guard.evaluate_diff(dangerous_diff3);
        match status3 {
            CircuitBreakerStatus::AgentStuck { reason } => {
                assert!(reason.contains("Security gate failed"));
            }
            _ => panic!("Expected AgentStuck status"),
        }
        assert!(guard.is_tripped());

        let alert = guard.format_stuck_alert("my-org/my-repo", 42, "Critical vulnerability");
        assert!(alert.contains("Agent-Stuck"));
        assert!(alert.contains("#42"));
    }

    #[test]
    fn test_deadlock_detection_trips_agent_stuck_on_attempt_2() {
        let mut guard = CircuitBreakerGuard::new(3, 8.0);
        let repetitive_diff = "+ let password = \"secret123\"; eval(user_input);";

        // Attempt 1: Retrying (1/3)
        let (status1, _) = guard.evaluate_diff(repetitive_diff);
        assert_eq!(
            status1,
            CircuitBreakerStatus::Retrying {
                attempt: 1,
                max_attempts: 3
            }
        );

        // Attempt 2 with exact duplicate diff: Deadlock detected -> AgentStuck
        let (status2, _) = guard.evaluate_diff(repetitive_diff);
        match status2 {
            CircuitBreakerStatus::AgentStuck { reason } => {
                assert!(reason.contains("Deadlock detected on attempt 2"));
            }
            _ => panic!("Expected AgentStuck due to duplicate diff hash"),
        }
        assert!(guard.is_tripped());
    }

    #[test]
    fn test_compute_diff_hash() {
        let diff1 = "+ fn foo() {}";
        let diff2 = "+ fn foo() {}";
        let diff3 = "+ fn bar() {}";

        let hash1 = compute_diff_hash(diff1);
        let hash2 = compute_diff_hash(diff2);
        let hash3 = compute_diff_hash(diff3);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_eq!(hash1.len(), 64);
    }
}

/// Reason why a swarm lane group was atomically frozen by the circuit breaker.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SwarmFreezeReason {
    PlanBaselineOverrun {
        spend_usd: f64,
        baseline_usd: f64,
        limit_usd: f64,
    },
    BurnVelocityExceeded {
        velocity_usd_per_min: f64,
        max_velocity_usd_per_min: f64,
    },
    DailyHardStopReached {
        daily_spend_usd: f64,
        hard_stop_usd: f64,
        daily_cap_usd: f64,
    },
    RetryWindowExceeded {
        retries_in_window: u32,
        max_retries: u32,
        window_secs: u64,
    },
    DiffOscillationHalt {
        lane_id: String,
        diff_hash: String,
    },
}

/// Persistent state for the swarm-level circuit breaker guard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmCircuitBreakerState {
    pub frozen: bool,
    pub freeze_reason: Option<SwarmFreezeReason>,
    pub frozen_lanes: Vec<String>,
    pub hitl_override_actor: Option<String>,
    pub hitl_override_reason: Option<String>,
    pub retry_timestamps_secs: Vec<u64>,
    pub lane_diff_history: std::collections::HashMap<String, Vec<String>>,
    pub plan_baseline_usd: f64,
    pub daily_cap_usd: f64,
    pub hard_stop_usd: f64,
    pub max_velocity_usd_per_min: f64,
    pub max_retries_in_window: u32,
    pub retry_window_secs: u64,
}

impl Default for SwarmCircuitBreakerState {
    fn default() -> Self {
        Self {
            frozen: false,
            freeze_reason: None,
            frozen_lanes: Vec::new(),
            hitl_override_actor: None,
            hitl_override_reason: None,
            retry_timestamps_secs: Vec::new(),
            lane_diff_history: std::collections::HashMap::new(),
            plan_baseline_usd: 0.0,
            daily_cap_usd: 50.0,
            hard_stop_usd: 45.0,
            max_velocity_usd_per_min: 1.0,
            max_retries_in_window: 6,
            retry_window_secs: 60,
        }
    }
}

/// Enforces swarm-wide cost, velocity, retry, and diff oscillation circuit breakers
/// with atomic group freeze and HITL Vertex 3 override support.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SwarmCircuitBreakerGuard {
    pub state: SwarmCircuitBreakerState,
}

impl SwarmCircuitBreakerGuard {
    pub fn new(plan_baseline_usd: f64) -> Self {
        Self {
            state: SwarmCircuitBreakerState {
                plan_baseline_usd,
                ..Default::default()
            },
        }
    }

    pub fn is_frozen(&self) -> bool {
        self.state.frozen
    }

    /// Atomically freezes all active lanes in the swarm group.
    pub fn freeze_group(&mut self, reason: SwarmFreezeReason, active_lanes: &[String]) {
        self.state.frozen = true;
        self.state.freeze_reason = Some(reason);
        self.state.frozen_lanes = active_lanes.to_vec();
    }

    /// Evaluates mission spend against the +15% plan baseline, >$1.00/min burn velocity,
    /// and $45/$50 daily HardStop thresholds.
    pub fn evaluate_spend(
        &mut self,
        mission_spend_usd: f64,
        daily_spend_usd: f64,
        burn_velocity_usd_per_min: f64,
        active_lanes: &[String],
    ) -> Option<SwarmFreezeReason> {
        if self.state.frozen {
            return self.state.freeze_reason.clone();
        }

        let baseline_limit = self.state.plan_baseline_usd * 1.15;
        if self.state.plan_baseline_usd > 0.0 && mission_spend_usd > baseline_limit {
            let reason = SwarmFreezeReason::PlanBaselineOverrun {
                spend_usd: mission_spend_usd,
                baseline_usd: self.state.plan_baseline_usd,
                limit_usd: baseline_limit,
            };
            self.freeze_group(reason.clone(), active_lanes);
            return Some(reason);
        }

        if burn_velocity_usd_per_min > self.state.max_velocity_usd_per_min {
            let reason = SwarmFreezeReason::BurnVelocityExceeded {
                velocity_usd_per_min: burn_velocity_usd_per_min,
                max_velocity_usd_per_min: self.state.max_velocity_usd_per_min,
            };
            self.freeze_group(reason.clone(), active_lanes);
            return Some(reason);
        }

        if daily_spend_usd >= self.state.hard_stop_usd {
            let reason = SwarmFreezeReason::DailyHardStopReached {
                daily_spend_usd,
                hard_stop_usd: self.state.hard_stop_usd,
                daily_cap_usd: self.state.daily_cap_usd,
            };
            self.freeze_group(reason.clone(), active_lanes);
            return Some(reason);
        }

        None
    }

    /// Records a retry attempt at `timestamp_secs` and freezes the group if >6 retries
    /// occur within the 60s sliding window.
    pub fn record_retry_attempt(
        &mut self,
        timestamp_secs: u64,
        active_lanes: &[String],
    ) -> Option<SwarmFreezeReason> {
        if self.state.frozen {
            return self.state.freeze_reason.clone();
        }

        let window_start = timestamp_secs.saturating_sub(self.state.retry_window_secs);
        self.state
            .retry_timestamps_secs
            .retain(|&ts| ts >= window_start && ts <= timestamp_secs);
        self.state.retry_timestamps_secs.push(timestamp_secs);

        let retries_in_window = self.state.retry_timestamps_secs.len() as u32;
        if retries_in_window > self.state.max_retries_in_window {
            let reason = SwarmFreezeReason::RetryWindowExceeded {
                retries_in_window,
                max_retries: self.state.max_retries_in_window,
                window_secs: self.state.retry_window_secs,
            };
            self.freeze_group(reason.clone(), active_lanes);
            return Some(reason);
        }

        None
    }

    /// Records a lane diff and halts the group if an A-B-A-B oscillation pattern is detected.
    pub fn record_lane_diff(
        &mut self,
        lane_id: &str,
        diff: &str,
        active_lanes: &[String],
    ) -> Option<SwarmFreezeReason> {
        if self.state.frozen {
            return self.state.freeze_reason.clone();
        }

        let diff_hash = compute_diff_hash(diff);
        let history = self
            .state
            .lane_diff_history
            .entry(lane_id.to_string())
            .or_default();
        history.push(diff_hash.clone());

        let len = history.len();
        if len >= 4
            && history[len - 4] == history[len - 2]
            && history[len - 3] == history[len - 1]
            && history[len - 4] != history[len - 3]
        {
            let reason = SwarmFreezeReason::DiffOscillationHalt {
                lane_id: lane_id.to_string(),
                diff_hash,
            };
            self.freeze_group(reason.clone(), active_lanes);
            return Some(reason);
        }

        None
    }

    /// Applies a HITL Vertex 3 override, unfreezing the group and recording audit metadata.
    pub fn apply_hitl_override(&mut self, actor: &str, justification: &str) -> bool {
        let was_frozen = self.state.frozen;
        self.state.frozen = false;
        self.state.freeze_reason = None;
        self.state.frozen_lanes.clear();
        self.state.retry_timestamps_secs.clear();
        self.state.lane_diff_history.clear();
        self.state.hitl_override_actor = Some(actor.to_string());
        self.state.hitl_override_reason = Some(justification.to_string());
        was_frozen
    }
}
