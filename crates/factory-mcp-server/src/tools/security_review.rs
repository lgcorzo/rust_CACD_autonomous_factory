use async_trait::async_trait;
use serde_json::{json, Value};
use regex::Regex;
use crate::tools::Tool;
use crate::protocol::{CallToolResult, McpContent};

pub struct SecurityReviewTool;

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
                }).to_string()
            }],
            is_error: false,
        })
    }
}

impl SecurityReviewTool {
    fn scan_owasp_patterns(&self, diff: &str) -> Vec<Value> {
        let patterns = vec![
            ("A03:Injection - SQL", r#"(?i)(?:execute|cursor\.execute|raw\s*\()\s*\(?.*?["'].*?%s|format\(|f["']"# , "high", "Potential SQL injection."),
            ("A03:Injection - Command", r#"(?i)(?:os\.system|subprocess\.call|subprocess\.Popen)\s*\(\s*[f"']"# , "high", "Potential command injection."),
            ("A02:Crypto - Hardcoded Secret", r#"(?i)(?:password|secret|api_key|token)\s*=\s*["'][^"']{8,}["']"# , "medium", "Possible hardcoded secret."),
        ];

        let mut findings = Vec::new();
        for (rule, pattern, severity, description) in patterns {
            let re = Regex::new(pattern).unwrap();
            if re.is_match(diff) {
                findings.push(json!({
                    "rule": rule,
                    "severity": severity,
                    "description": description
                }));
            }
        }
        findings
    }
}
