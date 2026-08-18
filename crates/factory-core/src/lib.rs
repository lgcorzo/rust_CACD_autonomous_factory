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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FinOpsTag {
    pub team: String,
    pub epic: String,
    pub microservice: String,
    pub environment: String,
    pub cost_center: String,
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
}

impl PRDirective {
    pub fn parse(text: &str) -> Option<Self> {
        let trimmed = text.trim();
        let command_str = if let Some(idx) = trimmed.find("@dark-gravity") {
            trimmed[idx + "@dark-gravity".len()..].trim()
        } else if trimmed.starts_with('/') {
            trimmed
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollerSyncCursor {
    pub source_key: String,
    pub last_polled_at: DateTime<Utc>,
    pub last_processed_id: u64,
    pub processed_hashes: Vec<String>,
}

pub mod proto {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/dark_gravity.factory.v1.rs"));
    }
}
