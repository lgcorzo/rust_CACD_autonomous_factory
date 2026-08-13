use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionResponseFormat, ChatCompletionResponseFormatType,
    CreateChatCompletionRequestArgs,
};
use async_openai::Client;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;

pub struct SecurityReviewTool {
    client: Client<OpenAIConfig>,
    model: String,
}

impl SecurityReviewTool {
    pub fn new() -> Self {
        let api_base = env::var("LITELLM_API_BASE")
            .unwrap_or_else(|_| "http://litellm.llm-apps.svc.cluster.local:4000".to_string());
        let api_key = env::var("LITELLM_API_KEY").unwrap_or_else(|_| "sk-dummy".to_string());
        let model = env::var("LITELLM_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());

        let config = OpenAIConfig::new()
            .with_api_base(api_base)
            .with_api_key(api_key);

        Self {
            client: Client::with_config(config),
            model,
        }
    }

    fn heuristic_scan(diff: &str) -> (f64, Vec<String>) {
        let mut findings = Vec::new();
        let mut penalty = 0.0;

        let diff_lower = diff.to_lowercase();
        if diff_lower.contains("eval(")
            || diff_lower.contains("exec(")
            || diff_lower.contains("system(")
        {
            findings.push("Potential Code Execution (eval/exec/system detected)".to_string());
            penalty += 4.0;
        }
        if diff_lower.contains("password =")
            || diff_lower.contains("secret =")
            || diff_lower.contains("api_key =")
        {
            findings.push("Potential hardcoded secret or API key detected".to_string());
            penalty += 3.0;
        }
        if diff_lower.contains("drop table") || diff_lower.contains("rm -rf /") {
            findings.push("Dangerous command or destructive SQL detected".to_string());
            penalty += 5.0;
        }

        let score = f64::max(10.0 - penalty, 0.0);
        (score, findings)
    }
}

impl Default for SecurityReviewTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for SecurityReviewTool {
    fn name(&self) -> String {
        "security_review".to_string()
    }

    fn description(&self) -> String {
        "Analyzes code diffs for security vulnerabilities using LLM SAST and enforces a strict 8.0/10.0 score threshold.".to_string()
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
        let (h_score, h_findings) = Self::heuristic_scan(diff);

        let system_prompt = "You are a SAST Security Judge (Aethelgard). Analyze the code diff for security vulnerabilities (injection, hardcoded secrets, unsafe code, memory leaks). Return a JSON object with keys: score (float 0.0-10.0), status ('approved' if score >= 8.0 else 'rejected'), and findings (array of strings).";

        let mut final_score = h_score;
        let mut final_status = if final_score >= 8.0 {
            "approved"
        } else {
            "rejected"
        };
        let mut final_findings = h_findings;

        // Intentar consulta al gateway LLM LiteLLM si está disponible
        if let Ok(request) = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt)
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(diff)
                    .build()?
                    .into(),
            ])
            .response_format(ChatCompletionResponseFormat {
                r#type: ChatCompletionResponseFormatType::JsonObject,
            })
            .build()
        {
            if let Ok(response) = self.client.chat().create(request).await {
                if let Some(choice) = response.choices.first() {
                    if let Some(ref text) = choice.message.content {
                        if let Ok(parsed) = serde_json::from_str::<Value>(text) {
                            if let Some(s) = parsed["score"].as_f64() {
                                final_score = s;
                            }
                            if let Some(f) = parsed["findings"].as_array() {
                                for item in f {
                                    if let Some(st) = item.as_str() {
                                        if !final_findings.contains(&st.to_string()) {
                                            final_findings.push(st.to_string());
                                        }
                                    }
                                }
                            }
                            final_status = if final_score >= 8.0 {
                                "approved"
                            } else {
                                "rejected"
                            };
                        }
                    }
                }
            }
        }

        let output_json = json!({
            "status": final_status,
            "score": final_score,
            "threshold": 8.0,
            "findings": final_findings
        });

        Ok(CallToolResult {
            content: vec![McpContent::Text {
                text: output_json.to_string(),
            }],
            is_error: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_review_heuristic_pass() {
        let tool = SecurityReviewTool::new();
        let params = json!({ "diff": "fn safe_function() { println!(\"Hello World\"); }" });
        let result = tool.call(params).await.unwrap();
        let text = match &result.content[0] {
            McpContent::Text { text } => text,
            _ => panic!("Expected text content"),
        };
        let val: Value = serde_json::from_str(text).unwrap();
        assert_eq!(val["status"], "approved");
        assert!(val["score"].as_f64().unwrap() >= 8.0);
    }

    #[tokio::test]
    async fn test_security_review_heuristic_fail() {
        let tool = SecurityReviewTool::new();
        let params = json!({ "diff": "let password = \"123456\"; eval(\"dangerous_input\");" });
        let result = tool.call(params).await.unwrap();
        let text = match &result.content[0] {
            McpContent::Text { text } => text,
            _ => panic!("Expected text content"),
        };
        let val: Value = serde_json::from_str(text).unwrap();
        assert_eq!(val["status"], "rejected");
        assert!(val["score"].as_f64().unwrap() < 8.0);
        assert!(!val["findings"].as_array().unwrap().is_empty());
    }
}
