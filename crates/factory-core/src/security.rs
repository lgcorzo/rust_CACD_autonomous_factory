use crate::error::Result;
use async_trait::async_trait;

/// Trait for validating requests or agent responses.
#[async_trait]
pub trait SecurityValidator {
    async fn validate_signature(&self, data: &[u8], signature: &str) -> Result<bool>;
    async fn audit_content(&self, content: &str) -> Result<AuditResult>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditResult {
    pub is_safe: bool,
    pub findings: Vec<String>,
}
