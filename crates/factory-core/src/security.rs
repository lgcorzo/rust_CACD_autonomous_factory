use crate::error::Result;
use async_trait::async_trait;

pub mod nhi;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SandboxConstraint {
    pub max_memory_mb: u32,
    pub max_cpu_cores: f32,
    pub network_egress_allowed: bool,
}

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

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, zeroize::Zeroize, zeroize::ZeroizeOnDrop,
)]
pub struct JitToken {
    pub token: String,
}

#[async_trait]
pub trait SecurityBounds: Send + Sync {
    async fn validate_token(&self, token: &JitToken) -> Result<bool>;
    async fn issue_jit_token(&self, audience: &str) -> Result<JitToken>;
    fn wipe_token_from_memory(&self, token: &mut JitToken) {
        use zeroize::Zeroize;
        token.zeroize();
    }
}
