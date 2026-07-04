use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_openai::Client;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;

pub struct SecurityReviewTool {
    client: Client<OpenAIConfig>,
}

impl SecurityReviewTool {
    pub fn new() -> Self {
        let api_base = env::var("LITELLM_API_BASE")
            .unwrap_or_else(|_| "http://litellm.llm-apps.svc.cluster.local:4000".to_string());

        let config = OpenAIConfig::new()
            .with_api_base(api_base)
            .with_api_key("sk-dummy"); // LiteLLM might not require a real key for internal traffic, but it requires the format

        Self {
            client: Client::with_config(config),
        }
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
        "Analyzes code diffs for security vulnerabilities using LLM SAST and generates a score out of 10.0.".to_string()
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

        let system_msg = ChatCompletionRequestSystemMessageArgs::default()
            .content("You are a strict security auditor. Review the provided code diff for vulnerabilities (e.g. injection, hardcoded secrets, memory safety). You must output ONLY a valid JSON object with two fields: 'score' (a float from 0.0 to 10.0, where 10.0 is perfectly secure, and anything below 8.0 means it has high/medium vulnerabilities), and 'findings' (an array of strings describing the vulnerabilities found).")
            .build()?;

        let user_msg = ChatCompletionRequestUserMessageArgs::default()
            .content(format!("Review this code:\n{}", diff))
            .build()?;

        let request = CreateChatCompletionRequestArgs::default()
            .model("qwen2.5")
            .messages([system_msg.into(), user_msg.into()])
            .build()?;

        let response = match self.client.chat().create(request).await {
            Ok(resp) => resp,
            Err(e) => {
                return Ok(CallToolResult {
                    content: vec![McpContent::Text {
                        text: format!("SAST LLM error: {}", e),
                    }],
                    is_error: true,
                });
            }
        };

        let content = response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_else(|| "{}".to_string());

        // Try to parse the JSON response
        let parsed: Value = serde_json::from_str(&content).unwrap_or_else(|_| {
            // Fallback if the LLM didn't return perfect JSON
            json!({
                "score": 0.0,
                "findings": ["Failed to parse SAST JSON from LLM"]
            })
        });

        let score = parsed["score"].as_f64().unwrap_or(0.0);
        let status = if score < 8.0 { "rejected" } else { "approved" };

        Ok(CallToolResult {
            content: vec![McpContent::Text {
                text: json!({
                    "status": status,
                    "score": score,
                    "findings": parsed["findings"]
                })
                .to_string(),
            }],
            is_error: false,
        })
    }
}
