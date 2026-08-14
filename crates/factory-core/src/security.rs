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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SastScanResult {
    pub is_safe: bool,
    pub score: f32, // 0.0 to 10.0
    pub findings: Vec<String>,
    pub critical_vulnerabilities_detected: bool,
}

impl SastScanResult {
    pub fn passes_gate(&self) -> bool {
        self.is_safe && self.score >= 8.0 && !self.critical_vulnerabilities_detected
    }

    pub fn inspect_diff(diff: &str) -> Self {
        let mut findings = Vec::new();
        let mut critical = false;
        let mut score: f32 = 10.0;

        let lower = diff.to_lowercase();

        // 1. Hardcoded Secrets Check
        if lower.contains("password = \"")
            || lower.contains("api_key = \"")
            || lower.contains("secret = \"")
            || lower.contains("private_key = \"")
        {
            findings.push("Hardcoded secret or credential detected in diff".to_string());
            critical = true;
            score -= 4.0;
        }

        // 2. Command Injection / RCE check
        if lower.contains("system(")
            || lower.contains("std::process::command::new(\"sh\")")
            || lower.contains("exec(")
            || lower.contains("eval(")
        {
            findings.push(
                "Potential Command Injection / Arbitrary Code Execution vector detected"
                    .to_string(),
            );
            critical = true;
            score -= 5.0;
        }

        // 3. Raw SQL Injection check
        if lower.contains("format!(\"select") || lower.contains("format!(\"delete") {
            findings.push("Unparameterized SQL string formatting detected (SQLi risk)".to_string());
            critical = true;
            score -= 4.0;
        }

        // 4. Unbounded recursion check
        if lower.contains("loop {") && !lower.contains("break") && !lower.contains("return") {
            findings.push("Potential unbounded loop without break condition detected".to_string());
            score -= 2.5;
        }

        let clamped_score = score.clamp(0.0, 10.0);
        let is_safe = !critical && clamped_score >= 8.0;

        Self {
            is_safe,
            score: clamped_score,
            findings,
            critical_vulnerabilities_detected: critical,
        }
    }
}

pub struct Ed25519SecurityValidator {
    pub public_key: ed25519_dalek::VerifyingKey,
}

#[async_trait]
impl SecurityValidator for Ed25519SecurityValidator {
    async fn validate_signature(&self, data: &[u8], signature: &str) -> Result<bool> {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
        use ed25519_dalek::{Signature, Verifier};

        let decoded_sig = URL_SAFE_NO_PAD.decode(signature).map_err(|e| {
            crate::error::FactoryError::Security(format!("Base64 decode error: {}", e))
        })?;

        let sig = Signature::from_slice(&decoded_sig).map_err(|e| {
            crate::error::FactoryError::Security(format!("Signature format error: {}", e))
        })?;

        Ok(self.public_key.verify(data, &sig).is_ok())
    }

    async fn audit_content(&self, _content: &str) -> Result<AuditResult> {
        // Mock audit for now, meant to be overridden or implemented by an LLM agent
        Ok(AuditResult {
            is_safe: true,
            findings: vec![],
        })
    }
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
