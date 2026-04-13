use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
use regex::Regex;
use serde_json::{json, Value};
use std::sync::LazyLock;

pub struct SecurityReviewTool;

struct SecurityRule {
    rule: &'static str,
    re: Regex,
    severity: &'static str,
    description: &'static str,
}

static SECURITY_RULES: LazyLock<Vec<SecurityRule>> = LazyLock::new(|| {
    vec![
        SecurityRule {
            rule: "A03:Injection - SQL",
            re: Regex::new(
                r#"(?i)(?:execute|cursor\.execute|raw\s*\()(?s:.*?)(?:%s|format\(|f["']|\+\s*[a-zA-Z_]|\,\s*[a-zA-Z_])"#,
            )
            .unwrap(),
            severity: "high",
            description: "Potential SQL injection.",
        },
        SecurityRule {
            rule: "A03:Injection - Command",
            re: Regex::new(r#"(?i)(?:os\.system|subprocess\.call|subprocess\.Popen)\s*\(\s*[f"']"#)
                .unwrap(),
            severity: "high",
            description: "Potential command injection.",
        },
        SecurityRule {
            rule: "A02:Crypto - Hardcoded Secret",
            re: Regex::new(r#"(?i)(?:password|secret|api_key|token)\s*=\s*["'][^"']{8,}["']"#)
                .unwrap(),
            severity: "medium",
            description: "Possible hardcoded secret.",
        },
    ]
});

#[async_trait]
impl Tool for SecurityReviewTool {
    fn name(&self) -> String {
        "security_review".to_string()
    }

    fn description(&self) -> String {
        "Analyzes code diffs for OWASP Top 10 security vulnerabilities.".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "diff": { "type": "string" }
            },
            "required": ["diff"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let diff = params["diff"].as_str().unwrap_or("");
        let findings = self.scan_owasp_patterns(diff);

        let status = if findings.iter().any(|f| f["severity"] == "high") {
            "rejected"
        } else {
            "approved"
        };

        Ok(CallToolResult {
            content: vec![McpContent::Text {
                text: json!({
                    "status": status,
                    "findings": findings
                })
                .to_string(),
            }],
            is_error: false,
        })
    }
}

impl SecurityReviewTool {
    fn scan_owasp_patterns(&self, diff: &str) -> Vec<Value> {
        let mut findings = Vec::new();
        for rule in SECURITY_RULES.iter() {
            if rule.re.is_match(diff) {
                findings.push(json!({
                    "rule": rule.rule,
                    "severity": rule.severity,
                    "description": rule.description
                }));
            }
        }
        findings
    }
}
