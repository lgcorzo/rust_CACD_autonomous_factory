use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub mod error;
pub mod security;

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

pub mod proto {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/dark_gravity.factory.v1.rs"));
    }
}
