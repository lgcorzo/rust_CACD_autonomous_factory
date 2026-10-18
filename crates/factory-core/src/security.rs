use crate::error::Result;
use async_trait::async_trait;

pub mod nhi;

pub const DEFAULT_MAX_MEMORY_MIB: u32 = 30;
pub const DEFAULT_MAX_CPU_CORES: f32 = 0.25;
pub const SIDECAR_MAX_MEMORY_MIB: u32 = 20;
pub const SIDECAR_MAX_CPU_CORES: f32 = 0.10;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct SandboxConstraint {
    #[serde(alias = "max_memory_mb")]
    pub max_memory_mib: u32,
    pub max_cpu_cores: f32,
    pub network_egress_allowed: bool,
}

impl Default for SandboxConstraint {
    fn default() -> Self {
        Self {
            max_memory_mib: DEFAULT_MAX_MEMORY_MIB,
            max_cpu_cores: DEFAULT_MAX_CPU_CORES,
            network_egress_allowed: false,
        }
    }
}

impl SandboxConstraint {
    pub fn gvisor_default() -> Self {
        Self::default()
    }

    pub fn sidecar_default() -> Self {
        Self {
            max_memory_mib: SIDECAR_MAX_MEMORY_MIB,
            max_cpu_cores: SIDECAR_MAX_CPU_CORES,
            network_egress_allowed: false,
        }
    }

    pub fn validate_bounds(&self) -> bool {
        !self.network_egress_allowed
            && self.max_memory_mib > 0
            && self.max_memory_mib <= DEFAULT_MAX_MEMORY_MIB
            && self.max_cpu_cores > 0.0
            && self.max_cpu_cores <= DEFAULT_MAX_CPU_CORES
            && !self.max_cpu_cores.is_nan()
    }
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScanPolicy {
    pub min_safety_score: f32,
    pub allow_network: bool,
    pub allow_unsafe: bool,
}

impl Default for ScanPolicy {
    fn default() -> Self {
        Self {
            min_safety_score: 8.0,
            allow_network: false,
            allow_unsafe: false,
        }
    }
}

static SECRET_REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r#"(?i)\b(password|passwd|secret|api[_-]?key|private[_-]?key|token)\s*[:=]\s*["'][^"'\s]{6,}["']"#)
        .expect("valid secret regex")
});

static RCE_REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r#"\b(eval|exec|system)\s*\("#).expect("valid rce regex")
});

static CMD_INJECTION_REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(
        r#"(?i)(Command::new\s*\(\s*["'](sh|bash|cmd|powershell|zsh)["']|std::process::Command)"#,
    )
    .expect("valid cmd injection regex")
});

static NETWORK_REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r#"\b(reqwest::|TcpStream::|curl\b|wget\b)"#).expect("valid network regex")
});

static SQLI_REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r#"(?i)format!\s*\(\s*["']\s*(select|delete|update|insert)\b"#)
        .expect("valid sqli regex")
});

static DESTRUCTIVE_FS_REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r#"\b(remove_dir_all|fs::remove_file)\b"#)
        .expect("valid destructive fs regex")
});

static UNSAFE_BLOCK_REGEX: std::sync::LazyLock<regex::Regex> =
    std::sync::LazyLock::new(|| regex::Regex::new(r#"\bunsafe\s*\{"#).expect("valid unsafe regex"));

/// Heuristic tripwire diff scanner.
///
/// Note: This is an automated heuristic tripwire designed to catch obvious command injections,
/// hardcoded secrets, and destructive primitives in generated code diffs.
/// The deterministic gating remains rustfmt, clippy, cargo-audit, and human review.
impl SastScanResult {
    pub fn passes_gate(&self) -> bool {
        self.passes_gate_with(8.0)
    }

    pub fn passes_gate_with(&self, min_score: f32) -> bool {
        self.is_safe && self.score >= min_score && !self.critical_vulnerabilities_detected
    }

    pub fn inspect_diff(diff: &str) -> Self {
        Self::inspect_diff_with(diff, &ScanPolicy::default())
    }

    pub fn inspect_diff_with(diff: &str, policy: &ScanPolicy) -> Self {
        let mut findings = Vec::new();
        let mut critical = false;
        let mut score: f32 = 10.0;

        // Scan only added lines, ignoring diff header lines (+++)
        let added_lines: Vec<&str> = diff
            .lines()
            .filter(|l| l.starts_with('+') && !l.starts_with("+++"))
            .map(|l| &l[1..])
            .collect();

        if added_lines.is_empty() {
            return Self {
                is_safe: true,
                score: 10.0,
                findings,
                critical_vulnerabilities_detected: false,
            };
        }

        let combined_added = added_lines.join("\n");

        // 1. Hardcoded Secrets Check
        if SECRET_REGEX.is_match(&combined_added) {
            findings
                .push("Hardcoded secret or credential detected in added diff lines".to_string());
            critical = true;
            score -= 4.0;
        }

        // 2. Command Injection / Arbitrary Code Execution
        if RCE_REGEX.is_match(&combined_added) || CMD_INJECTION_REGEX.is_match(&combined_added) {
            findings.push(
                "Potential Command Injection / Arbitrary Code Execution vector detected"
                    .to_string(),
            );
            critical = true;
            score -= 5.0;
        }

        // 3. Outbound Network Calls
        if !policy.allow_network && NETWORK_REGEX.is_match(&combined_added) {
            findings.push("Outbound network primitive detected in generated code".to_string());
            critical = true;
            score -= 4.0;
        }

        // 4. Raw SQL Injection check
        if SQLI_REGEX.is_match(&combined_added) {
            findings.push("Unparameterized SQL string formatting detected (SQLi risk)".to_string());
            critical = true;
            score -= 4.0;
        }

        // 5. Destructive File System operations
        if DESTRUCTIVE_FS_REGEX.is_match(&combined_added) {
            findings.push("Destructive filesystem operation detected (remove_dir_all)".to_string());
            critical = true;
            score -= 4.0;
        }

        // 6. Unsafe blocks
        if !policy.allow_unsafe && UNSAFE_BLOCK_REGEX.is_match(&combined_added) {
            findings.push("Unsafe block introduced in generated code".to_string());
            score -= 3.0;
        }

        // 7. Unbounded loop check
        for line in &added_lines {
            if line.contains("loop {")
                && !combined_added.contains("break")
                && !combined_added.contains("return")
            {
                findings
                    .push("Potential unbounded loop without break condition detected".to_string());
                score -= 2.5;
                break;
            }
        }

        let clamped_score = score.clamp(0.0, 10.0);
        let is_safe = !critical && clamped_score >= policy.min_safety_score;

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
        Err(crate::error::FactoryError::Security(
            "Default audit_content is not implemented; security validators must fail closed"
                .to_string(),
        ))
    }
}

#[derive(serde::Deserialize, zeroize::Zeroize, zeroize::ZeroizeOnDrop)]
pub struct JitToken {
    token: String,
}

impl JitToken {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }

    pub fn expose_token(&self) -> &str {
        &self.token
    }

    pub fn is_empty(&self) -> bool {
        self.token.is_empty()
    }
}

impl std::fmt::Debug for JitToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JitToken")
            .field("token", &"[REDACTED]")
            .finish()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeroize() {
        let mut token = JitToken::new("super-secret-vault-token-xyz");
        assert_eq!(token.expose_token(), "super-secret-vault-token-xyz");
        assert!(format!("{:?}", token).contains("[REDACTED]"));

        use zeroize::Zeroize;
        token.zeroize();
        assert!(token.expose_token().is_empty() || token.expose_token().chars().all(|c| c == '\0'));
    }

    #[test]
    fn test_sandbox_constraint_validation() {
        let default_constraint = SandboxConstraint::gvisor_default();
        assert_eq!(default_constraint.max_memory_mib, 30);
        assert!((default_constraint.max_cpu_cores - 0.25).abs() < f32::EPSILON);
        assert!(default_constraint.validate_bounds());

        let sidecar_constraint = SandboxConstraint::sidecar_default();
        assert_eq!(sidecar_constraint.max_memory_mib, 20);
        assert!((sidecar_constraint.max_cpu_cores - 0.10).abs() < f32::EPSILON);
        assert!(sidecar_constraint.validate_bounds());

        let over_limit = SandboxConstraint {
            max_memory_mib: 64,
            max_cpu_cores: 0.50,
            network_egress_allowed: true,
        };
        assert!(!over_limit.validate_bounds());

        // Strict egress rejection
        let mut egress_violation = SandboxConstraint::gvisor_default();
        egress_violation.network_egress_allowed = true;
        assert!(
            !egress_violation.validate_bounds(),
            "Egress allowed must fail bounds check"
        );

        // Zero or negative/NaN checks
        let zero_mem = SandboxConstraint {
            max_memory_mib: 0,
            max_cpu_cores: 0.25,
            network_egress_allowed: false,
        };
        assert!(
            !zero_mem.validate_bounds(),
            "0 MiB memory must fail bounds check"
        );

        let zero_cpu = SandboxConstraint {
            max_memory_mib: 30,
            max_cpu_cores: 0.0,
            network_egress_allowed: false,
        };
        assert!(
            !zero_cpu.validate_bounds(),
            "0.0 CPU must fail bounds check"
        );

        let nan_cpu = SandboxConstraint {
            max_memory_mib: 30,
            max_cpu_cores: f32::NAN,
            network_egress_allowed: false,
        };
        assert!(!nan_cpu.validate_bounds(), "NaN CPU must fail bounds check");
    }
}
