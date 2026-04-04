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
