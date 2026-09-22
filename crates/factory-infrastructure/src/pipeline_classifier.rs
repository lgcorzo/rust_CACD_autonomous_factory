//! Pipeline error log classifier.
//!
//! Provides regex-based classification of CI/CD pipeline error logs into
//! actionable [`ErrorCategory`] variants with structured metadata extraction.
//!
//! # Architecture
//!
//! This module is an **infrastructure adapter** that transforms raw platform-specific
//! log text into domain-model [`ErrorClassification`] entities defined in `factory-core`.

use factory_core::{ErrorCategory, ErrorClassification};
use regex::Regex;
use sha2::{Digest, Sha256};

/// Trait for classifying pipeline error logs into structured categories.
pub trait PipelineClassifier: Send + Sync {
    /// Classify an error log string into an [`ErrorClassification`].
    fn classify(&self, error_log: &str) -> ErrorClassification;
}

/// A rule in the classification rule table.
struct ClassificationRule {
    category: ErrorCategory,
    pattern: Regex,
    sub_type: Option<&'static str>,
}

/// Regex-based pipeline error classifier using ordered priority rules.
///
/// Rules are evaluated in order; the first matching rule determines the category.
/// If no rule matches, the error is classified as [`ErrorCategory::Unknown`].
pub struct RegexPipelineClassifier {
    rules: Vec<ClassificationRule>,
}

impl Default for RegexPipelineClassifier {
    fn default() -> Self {
        Self::new()
    }
}

impl RegexPipelineClassifier {
    /// Create a new classifier with the default rule table.
    pub fn new() -> Self {
        let rules = vec![
            // Lint violations (clippy) — most specific first
            ClassificationRule {
                category: ErrorCategory::LintViolation,
                pattern: Regex::new(r"(?i)warning:.*\[clippy::(\w+)\]|error\[.*\].*\(clippy\)")
                    .unwrap(),
                sub_type: Some("clippy_rule"),
            },
            // Rust compilation errors
            ClassificationRule {
                category: ErrorCategory::CodeCompilation,
                pattern: Regex::new(r"error\[E\d{4}\]:").unwrap(),
                sub_type: Some("rust_build_error"),
            },
            // Test failures
            ClassificationRule {
                category: ErrorCategory::TestFailure,
                pattern: Regex::new(r"test\s+\S+\s+\.\.\.\s+FAILED|^failures:|test result:.*\d+ failed")
                    .unwrap(),
                sub_type: Some("rust_test"),
            },
            // Security audit (cargo audit / RUSTSEC)
            ClassificationRule {
                category: ErrorCategory::SecurityAudit,
                pattern: Regex::new(
                    r"(?i)Crate:\s*\S+\s*\nVersion:.*\nWarning:|RUSTSEC-\d{4}-\d+|cargo.audit.*vulnerability",
                )
                .unwrap(),
                sub_type: Some("cargo_audit"),
            },
            // Infrastructure build errors (Docker, protoc, etc.)
            ClassificationRule {
                category: ErrorCategory::InfrastructureBuild,
                pattern: Regex::new(
                    r"(?i)docker\s+build.*(?:COPY failed|failed)|E:\s+Unable to locate package|protoc.*not found|cmake.*error",
                )
                .unwrap(),
                sub_type: Some("build_tool"),
            },
            // Transient infrastructure errors
            ClassificationRule {
                category: ErrorCategory::InfrastructureTransient,
                pattern: Regex::new(
                    r"(?i)Connection timed out|rate limit|503 Service Unavailable|OOMKilled|out of memory",
                )
                .unwrap(),
                sub_type: Some("transient"),
            },
        ];

        Self { rules }
    }

    /// Extract file path and line number from error output.
    fn extract_file_line(error_log: &str) -> (Option<String>, Option<u32>) {
        let re = Regex::new(r"-->\s*(\S+\.rs):(\d+):\d+").unwrap();
        if let Some(caps) = re.captures(error_log) {
            let file = caps.get(1).map(|m| m.as_str().to_string());
            let line = caps.get(2).and_then(|m| m.as_str().parse::<u32>().ok());
            return (file, line);
        }
        (None, None)
    }

    /// Extract clippy rule name from log.
    fn extract_clippy_rule(error_log: &str) -> Option<String> {
        let re = Regex::new(r"\[clippy::(\w+)\]").unwrap();
        re.captures(error_log)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
    }

    /// Extract failing test name from log.
    fn extract_test_name(error_log: &str) -> Option<String> {
        let re = Regex::new(r"test\s+(\S+)\s+\.\.\.\s+FAILED").unwrap();
        re.captures(error_log)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
    }

    /// Extract crate name from security audit output.
    fn extract_crate_name(error_log: &str) -> Option<String> {
        let re = Regex::new(r"(?i)Crate:\s*(\S+)").unwrap();
        re.captures(error_log)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
    }

    /// Generate a deterministic error fingerprint.
    fn generate_fingerprint(
        category: &ErrorCategory,
        file_path: &Option<String>,
        rule_or_test: &Option<String>,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}", category).as_bytes());
        if let Some(fp) = file_path {
            hasher.update(fp.as_bytes());
        }
        if let Some(rt) = rule_or_test {
            hasher.update(rt.as_bytes());
        }
        let result = hasher.finalize();
        hex::encode(&result[..8]) // 16-char hex fingerprint
    }
}

impl PipelineClassifier for RegexPipelineClassifier {
    fn classify(&self, error_log: &str) -> ErrorClassification {
        for rule in &self.rules {
            if rule.pattern.is_match(error_log) {
                let (file_path, line_number) = Self::extract_file_line(error_log);
                let rule_name = match rule.category {
                    ErrorCategory::LintViolation => Self::extract_clippy_rule(error_log),
                    ErrorCategory::SecurityAudit => Self::extract_crate_name(error_log),
                    _ => None,
                };
                let test_name = if rule.category == ErrorCategory::TestFailure {
                    Self::extract_test_name(error_log)
                } else {
                    None
                };

                let key_for_fingerprint = rule_name
                    .clone()
                    .or_else(|| test_name.clone());

                let error_fingerprint =
                    Self::generate_fingerprint(&rule.category, &file_path, &key_for_fingerprint);

                return ErrorClassification {
                    category: rule.category.clone(),
                    sub_type: rule.sub_type.map(|s| s.to_string()),
                    file_path,
                    line_number,
                    rule_name,
                    test_name,
                    error_fingerprint,
                    is_remediable: rule.category.is_remediable(),
                };
            }
        }

        // Fallback: Unknown
        ErrorClassification {
            category: ErrorCategory::Unknown,
            sub_type: None,
            file_path: None,
            line_number: None,
            rule_name: None,
            test_name: None,
            error_fingerprint: Self::generate_fingerprint(&ErrorCategory::Unknown, &None, &None),
            is_remediable: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classifier() -> RegexPipelineClassifier {
        RegexPipelineClassifier::new()
    }

    #[test]
    fn test_classify_clippy_error() {
        let log = r#"
warning: unused variable: `x`
  --> src/main.rs:42:9
   |
42 |     let x = 5;
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
   |
   = note: `#[warn(clippy::unused_variable)]` on by default
warning: unused variable: `x` [clippy::unused_variable]
"#;
        let result = classifier().classify(log);
        assert_eq!(result.category, ErrorCategory::LintViolation);
        assert_eq!(result.sub_type.as_deref(), Some("clippy_rule"));
        assert_eq!(result.file_path.as_deref(), Some("src/main.rs"));
        assert_eq!(result.line_number, Some(42));
        assert_eq!(result.rule_name.as_deref(), Some("unused_variable"));
        assert!(result.is_remediable);
    }

    #[test]
    fn test_classify_compilation_error() {
        let log = r#"
error[E0308]: mismatched types
  --> src/lib.rs:15:5
   |
15 |     "hello"
   |     ^^^^^^^ expected `i32`, found `&str`
"#;
        let result = classifier().classify(log);
        assert_eq!(result.category, ErrorCategory::CodeCompilation);
        assert_eq!(result.sub_type.as_deref(), Some("rust_build_error"));
        assert_eq!(result.file_path.as_deref(), Some("src/lib.rs"));
        assert_eq!(result.line_number, Some(15));
        assert!(result.is_remediable);
    }

    #[test]
    fn test_classify_test_failure() {
        let log = r#"
running 3 tests
test core::test_parse ... ok
test core::test_validate ... FAILED
test core::test_format ... ok

failures:
    core::test_validate
"#;
        let result = classifier().classify(log);
        assert_eq!(result.category, ErrorCategory::TestFailure);
        assert_eq!(result.test_name.as_deref(), Some("core::test_validate"));
        assert!(result.is_remediable);
    }

    #[test]
    fn test_classify_docker_build_error() {
        let log = r#"
Step 5/10 : RUN apt-get install -y libprotobuf-dev
E: Unable to locate package libprotobuf-dev
docker build failed
"#;
        let result = classifier().classify(log);
        assert_eq!(result.category, ErrorCategory::InfrastructureBuild);
        assert!(!result.is_remediable);
    }

    #[test]
    fn test_classify_transient_error() {
        let log = "fatal: unable to access 'https://github.com/org/repo.git/': Connection timed out";
        let result = classifier().classify(log);
        assert_eq!(result.category, ErrorCategory::InfrastructureTransient);
        assert!(!result.is_remediable);
    }

    #[test]
    fn test_classify_security_audit() {
        let log = r#"
Crate: openssl
Version: 0.10.1
Warning: RUSTSEC-2026-0001
"#;
        let result = classifier().classify(log);
        assert_eq!(result.category, ErrorCategory::SecurityAudit);
        assert_eq!(result.rule_name.as_deref(), Some("openssl"));
        assert!(!result.is_remediable);
    }

    #[test]
    fn test_classify_unknown_error() {
        let log = "Something unexpected happened with exit code 137";
        let result = classifier().classify(log);
        assert_eq!(result.category, ErrorCategory::Unknown);
        assert!(!result.is_remediable);
    }

    #[test]
    fn test_error_fingerprint_deterministic() {
        let log1 = "warning: unused [clippy::dead_code]\n  --> src/main.rs:10:1";
        let log2 = "warning: unused [clippy::dead_code]\n  --> src/main.rs:10:1";
        let r1 = classifier().classify(log1);
        let r2 = classifier().classify(log2);
        assert_eq!(r1.error_fingerprint, r2.error_fingerprint);

        // Different log -> different fingerprint
        let log3 = "error[E0308]: mismatched types\n  --> src/lib.rs:5:1";
        let r3 = classifier().classify(log3);
        assert_ne!(r1.error_fingerprint, r3.error_fingerprint);
    }
}
