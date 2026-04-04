use async_trait::async_trait;
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        ChatCompletionResponseFormat, ChatCompletionResponseFormatType,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use serde_json::{json, Value};
use crate::tools::Tool;
use crate::protocol::{CallToolResult, McpContent};

pub struct PlanMissionTool {
    client: Client<async_openai::config::OpenAIConfig>,
    model: String,
}

impl PlanMissionTool {
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        let config = async_openai::config::OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(base_url);
        Self {
            client: Client::with_config(config),
            model,
        }
    }
}

#[async_trait]
impl Tool for PlanMissionTool {
    fn name(&self) -> String {
        "plan_mission".to_string()
    }

    fn description(&self) -> String {
        "Generates a multi-task mission plan from a description.".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "mission_description": {"type": "string"}
            },
            "required": ["mission_description"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let description = params["mission_description"].as_str()
            .ok_or_else(|| anyhow::anyhow!("mission_description is required"))?;

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("You are a mission planner. Break down the user's request into a JSON list of tasks. Each task should have: id (uuid), description, assigned_agent (coder, reviewer, or tester), and dependencies (list of ids).")
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(description)
                    .build()?
                    .into(),
            ])
            .response_format(ChatCompletionResponseFormat {
                r#type: ChatCompletionResponseFormatType::JsonObject,
            })
            .build()?;

        let response = self.client.chat().create(request).await?;
        let content = response.choices[0].message.content.clone().unwrap_or_default();

        Ok(CallToolResult {
            content: vec![McpContent::Text { text: content }],
            is_error: false,
        })
    }
}
