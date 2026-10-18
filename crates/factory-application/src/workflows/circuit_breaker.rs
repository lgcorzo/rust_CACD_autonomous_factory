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
    format!("{:x}", hasher.finalize())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerGuard {
    pub max_attempts: u32,
    pub current_attempt: u32,
    pub min_safety_score: f32,
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
            diff_hash_history: Vec::new(),
            tripped: false,
        }
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
            self.diff_hash_history.push(diff_hash);
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
                self.diff_hash_history.push(diff_hash);
                return (CircuitBreakerStatus::AgentStuck { reason }, scan_result);
            }

            self.diff_hash_history.push(diff_hash);

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
