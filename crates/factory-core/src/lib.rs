use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub mod config;
pub mod error;
pub mod executor;
pub mod security;

pub use config::AgentModelConfig;

/// Metadata for tracing and versioning.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub timestamp: DateTime<Utc>,
    pub model_version: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Generic container for inputs.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inputs {
    pub input: String,
}

/// Structured response from an agent or mission.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Outputs {
    pub response: String,
    pub metadata: Metadata,
}

/// Target/Ground truth for training or evaluation.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Targets {
    pub input_target: String,
    pub response: String,
}

/// Representation of a mission in the factory.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mission {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub tasks: Vec<Task>,
    pub status: MissionStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MissionStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

/// Individual unit of work within a mission.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub mission_id: Uuid,
    pub description: String,
    pub assigned_agent: Option<String>,
    pub dependencies: Vec<Uuid>,
    pub status: TaskStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Queued,
    Active,
    Finished,
    Blocked,
}

/// Explanation results (SHAP).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SHAPValues {
    pub sample: String,
    pub explanation: String,
    pub shap_value: f32,
}

/// Feature importance rankings.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeatureImportances {
    pub feature: String,
    pub importance: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpecArtifact {
    pub id: Uuid,
    pub name: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OsrMetric {
    pub mission_id: String,
    pub osr_value: f32,
    pub wiki_commit_sha: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FinOpsTag {
    pub team: String,
    pub epic: String,
    pub microservice: String,
    pub environment: String,
    pub cost_center: String,
}

impl FinOpsTag {
    pub fn to_headers(&self) -> Vec<(String, String)> {
        vec![
            ("x-vtags-team".to_string(), self.team.clone()),
            ("x-vtags-epic".to_string(), self.epic.clone()),
            (
                "x-vtags-microservice".to_string(),
                self.microservice.clone(),
            ),
            ("x-vtags-environment".to_string(), self.environment.clone()),
            ("x-vtags-cost-center".to_string(), self.cost_center.clone()),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComplianceReport {
    pub report_id: Uuid,
    pub status: String,
    pub findings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserFeedbackPayload {
    pub user_id: String,
    pub session_id: Option<String>,
    pub feedback_text: String,
    pub sentiment: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyBudgetConfig {
    pub max_daily_budget_usd: f64,
    pub hardstop_threshold_ratio: f64,
    pub velocity_threshold_usd_per_min: f64,
}

impl Default for DailyBudgetConfig {
    fn default() -> Self {
        Self {
            max_daily_budget_usd: 50.0,
            hardstop_threshold_ratio: 0.90,
            velocity_threshold_usd_per_min: 1.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpendVelocityAlert {
    pub spend_velocity: f64,
    pub current_spend: f64,
    pub threshold: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct SentryCrashRecord {
    pub event_id: String,
    pub level: String,
    pub message: String,
    pub culprit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CausalProvenanceNode {
    pub node_id: String,
    pub issue_id: String,
    pub constitution_rule_id: String,
    pub spec_id: String,
    pub plan_id: String,
    pub ast_mutation_hash: String,
    pub test_result: String,
    pub is_valid: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum PRDirective {
    Spec { prompt: String },
    Refine { instruction: String },
    Retry,
    Status,
    Validate,
    Interact { prompt: String },
}

impl PRDirective {
    pub fn parse(text: &str) -> Option<Self> {
        let trimmed = text.trim();
        let lower = trimmed.to_lowercase();

        let tags = ["@darkgravity", "@dark-gravity", "@antigravity"];
        let mut matched_tag = None;
        for tag in tags {
            if let Some(pos) = lower.find(tag) {
                matched_tag = Some((pos, pos + tag.len()));
                break;
            }
        }

        let command_str = if let Some((start, end)) = matched_tag {
            let after = trimmed[end..].trim();
            if !after.is_empty() {
                after.to_string()
            } else {
                trimmed[..start].trim().to_string()
            }
        } else if trimmed.starts_with('/') {
            trimmed.to_string()
        } else {
            return None;
        };

        if let Some(prompt) = command_str.strip_prefix("/spec") {
            Some(PRDirective::Spec {
                prompt: prompt.trim().to_string(),
            })
        } else if let Some(instruction) = command_str.strip_prefix("/refine") {
            Some(PRDirective::Refine {
                instruction: instruction.trim().to_string(),
            })
        } else if command_str.starts_with("/retry") {
            Some(PRDirective::Retry)
        } else if command_str.starts_with("/status") {
            Some(PRDirective::Status)
        } else if command_str.starts_with("/validate") {
            Some(PRDirective::Validate)
        } else if !command_str.is_empty() {
            Some(PRDirective::Interact {
                prompt: command_str.trim().to_string(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PolledIssueEvent {
    pub source_platform: String,
    pub repository: String,
    pub issue_id: u64,
    pub issue_number: u64,
    pub title: String,
    pub body: String,
    pub labels: Vec<String>,
    pub resource_limits: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub html_url: String,
}

impl PolledIssueEvent {
    pub fn extract_resource_limits(body: &str) -> Option<String> {
        let marker = "[RESOURCE_LIMIT:";
        if let Some(start) = body.find(marker) {
            let rest = &body[start + marker.len()..];
            if let Some(end) = rest.find(']') {
                return Some(rest[..end].trim().to_string());
            }
        }
        None
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PRCommentEvent {
    pub source_platform: String,
    pub repository: String,
    pub pr_number: u64,
    pub comment_id: u64,
    pub author: String,
    pub body: String,
    pub directive: PRDirective,
    pub updated_at: DateTime<Utc>,
    pub html_url: String,
    #[serde(default)]
    pub thread_context: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollerSyncCursor {
    pub source_key: String,
    pub last_polled_at: DateTime<Utc>,
    pub last_processed_id: u64,
    pub processed_hashes: Vec<String>,
}

// ──────────────────────────────────────────────────────────────────────────────
// Pipeline Error Remediation Domain Entities
// ──────────────────────────────────────────────────────────────────────────────

/// Classification category for CI/CD pipeline errors.
///
/// Each variant maps to a `serde` tag used in Kafka event payloads.
/// The `is_remediable()` method determines whether autonomous remediation
/// should be attempted.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCategory {
    /// Rust compilation errors (e.g., `error[E0308]: mismatched types`)
    CodeCompilation,
    /// Clippy lint violations (e.g., `warning: unused variable [clippy::unused_variable]`)
    LintViolation,
    /// Test failures (e.g., `test core::test_parse ... FAILED`)
    TestFailure,
    /// Security audit findings (e.g., RUSTSEC advisories from `cargo audit`)
    SecurityAudit,
    /// Build infrastructure failures (e.g., Docker, protoc, cmake errors)
    InfrastructureBuild,
    /// Transient infrastructure failures (e.g., timeouts, rate limits, OOM)
    InfrastructureTransient,
    /// Unrecognized error pattern
    Unknown,
}

impl ErrorCategory {
    /// Returns `true` if this error category can be autonomously remediated.
    ///
    /// Only `CodeCompilation`, `LintViolation`, and `TestFailure` are considered
    /// safe for autonomous remediation. All other categories require human review
    /// or represent transient conditions.
    pub fn is_remediable(&self) -> bool {
        matches!(
            self,
            ErrorCategory::CodeCompilation
                | ErrorCategory::LintViolation
                | ErrorCategory::TestFailure
        )
    }
}

/// A detected CI/CD pipeline failure from GitHub Actions or GitLab CI.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PipelineFailureEvent {
    /// Source platform: `"github"` or `"gitlab"`
    pub source_platform: String,
    /// Repository in `owner/repo` format
    pub repository: String,
    /// Platform-specific run/pipeline ID
    pub run_id: u64,
    /// Name of the workflow or pipeline
    pub workflow_name: String,
    /// Name of the failing job
    pub failing_job: String,
    /// Name of the failing step (GitHub Actions only)
    pub failing_step: Option<String>,
    /// Truncated error log output (max 10KB)
    pub error_log: String,
    /// URL to the failed run
    pub run_url: String,
    /// When the failure was detected
    pub detected_at: DateTime<Utc>,
}

/// Result of classifying a pipeline error log.
///
/// Produced by the `PipelineClassifier` infrastructure adapter.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorClassification {
    /// Classified error category
    pub category: ErrorCategory,
    /// Sub-category (e.g., `"rust_build_error"`, `"clippy_rule"`)
    pub sub_type: Option<String>,
    /// Offending source file path
    pub file_path: Option<String>,
    /// Offending line number
    pub line_number: Option<u32>,
    /// Clippy rule name or security audit ID
    pub rule_name: Option<String>,
    /// Failing test name
    pub test_name: Option<String>,
    /// Deterministic hash of (category, file_path, rule/test name)
    pub error_fingerprint: String,
    /// Whether autonomous remediation should be attempted
    pub is_remediable: bool,
}

/// State machine for remediation missions.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RemediationStatus {
    /// Mission created, awaiting executor pickup
    Pending,
    /// Executor is working on the fix
    InProgress,
    /// Fix PR/MR created and passes CI
    Success,
    /// Fix attempt did not resolve the error
    Failed,
    /// Escalated to human review (issue created)
    Escalated,
}

/// Final result of a remediation attempt.
///
/// Published to the `mission-artifact` Kafka topic.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemediationOutcome {
    /// Reference to the remediation mission
    pub mission_id: Uuid,
    /// Final status
    pub result: RemediationStatus,
    /// URL of the fix PR/MR if created
    pub pr_url: Option<String>,
    /// Time from detection to successful fix (seconds)
    pub fix_duration_secs: Option<u64>,
    /// Whether the fix PR's pipeline passed
    pub pipeline_passed: bool,
    /// When the outcome was recorded
    pub completed_at: DateTime<Utc>,
}

pub mod proto {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/dark_gravity.factory.v1.rs"));
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Pipeline Error Remediation Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod pipeline_tests {
    use super::*;

    #[test]
    fn test_error_category_is_remediable() {
        assert!(ErrorCategory::CodeCompilation.is_remediable());
        assert!(ErrorCategory::LintViolation.is_remediable());
        assert!(ErrorCategory::TestFailure.is_remediable());
        assert!(!ErrorCategory::SecurityAudit.is_remediable());
        assert!(!ErrorCategory::InfrastructureBuild.is_remediable());
        assert!(!ErrorCategory::InfrastructureTransient.is_remediable());
        assert!(!ErrorCategory::Unknown.is_remediable());
    }

    #[test]
    fn test_pipeline_failure_event_serialization_roundtrip() {
        let event = PipelineFailureEvent {
            source_platform: "github".to_string(),
            repository: "lgcorzo/rust_CACD_autonomous_factory".to_string(),
            run_id: 12345,
            workflow_name: "CI/CD Pipeline".to_string(),
            failing_job: "Rust CI (Lint & Test)".to_string(),
            failing_step: Some("Lint with Clippy".to_string()),
            error_log: "error[E0308]: mismatched types".to_string(),
            run_url: "https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/runs/12345"
                .to_string(),
            detected_at: Utc::now(),
        };

        let json = serde_json::to_string(&event).unwrap();
        let deserialized: PipelineFailureEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.run_id, 12345);
        assert_eq!(deserialized.source_platform, "github");
        assert_eq!(
            deserialized.failing_step.as_deref(),
            Some("Lint with Clippy")
        );
    }

    #[test]
    fn test_error_classification_serialization_roundtrip() {
        let classification = ErrorClassification {
            category: ErrorCategory::LintViolation,
            sub_type: Some("clippy_rule".to_string()),
            file_path: Some("src/main.rs".to_string()),
            line_number: Some(42),
            rule_name: Some("unused_variable".to_string()),
            test_name: None,
            error_fingerprint: "a1b2c3d4".to_string(),
            is_remediable: true,
        };

        let json = serde_json::to_string(&classification).unwrap();
        let deserialized: ErrorClassification = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.category, ErrorCategory::LintViolation);
        assert_eq!(deserialized.file_path.as_deref(), Some("src/main.rs"));
        assert!(deserialized.is_remediable);
    }

    #[test]
    fn test_error_category_serde_tags() {
        let json = serde_json::to_string(&ErrorCategory::CodeCompilation).unwrap();
        assert_eq!(json, "\"code_compilation\"");

        let json = serde_json::to_string(&ErrorCategory::LintViolation).unwrap();
        assert_eq!(json, "\"lint_violation\"");

        let json = serde_json::to_string(&ErrorCategory::InfrastructureTransient).unwrap();
        assert_eq!(json, "\"infrastructure_transient\"");
    }

    #[test]
    fn test_remediation_status_transitions() {
        let status = RemediationStatus::Pending;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"pending\"");

        let status: RemediationStatus = serde_json::from_str("\"escalated\"").unwrap();
        assert_eq!(status, RemediationStatus::Escalated);
    }

    #[test]
    fn test_remediation_outcome_serialization() {
        let outcome = RemediationOutcome {
            mission_id: Uuid::new_v4(),
            result: RemediationStatus::Success,
            pr_url: Some("https://github.com/org/repo/pull/99".to_string()),
            fix_duration_secs: Some(180),
            pipeline_passed: true,
            completed_at: Utc::now(),
        };

        let json = serde_json::to_string(&outcome).unwrap();
        let deserialized: RemediationOutcome = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.result, RemediationStatus::Success);
        assert!(deserialized.pipeline_passed);
    }
}
