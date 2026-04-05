use crate::protocol::{CallToolResult, McpContent};
use crate::sandbox::SandboxDriver;
use crate::tools::Tool;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Arc;

pub struct ExecuteCodeTool {
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
        let code = params["code"].as_str().unwrap_or("");
        let language = params["language"].as_str().unwrap_or("python");

        let result = self.driver.execute(code, language).await?;

        if result.is_success {
            Ok(CallToolResult {
                content: vec![McpContent::Text {
                    text: result.stdout,
                }],
                is_error: false,
            })
        } else {
            Ok(CallToolResult {
                content: vec![McpContent::Text {
                    text: format!(
                        "Execution failed.\nStdout: {}\nStderr: {}",
                        result.stdout, result.stderr
                    ),
                }],
                is_error: true,
            })
        }
    }
}
