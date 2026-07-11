use crate::protocol::{CallToolResult, McpContent};
use crate::sandbox::SandboxDriver;
use crate::tools::Tool;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Arc;

pub struct ExecuteCodeTool {
    #[allow(dead_code)]
    driver: Arc<dyn SandboxDriver>,
}

impl ExecuteCodeTool {
    pub fn new(driver: Arc<dyn SandboxDriver>) -> Self {
        Self { driver }
    }
}

#[async_trait]
impl Tool for ExecuteCodeTool {
    fn name(&self) -> String {
        "execute_code".to_string()
    }

    fn description(&self) -> String {
        "Executes code in a sandbox (Python or Rust).".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "code": {"type": "string"},
                "language": {"type": "string", "enum": ["python", "rust"]}
            },
            "required": ["code", "language"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let _code = params["code"].as_str().unwrap_or("");
        let _language = params["language"].as_str().unwrap_or("python");

        Ok(CallToolResult {
            content: vec![McpContent::Text {
                text: "Mock execution successful.".to_string(),
            }],
            is_error: false,
        })
    }
}
