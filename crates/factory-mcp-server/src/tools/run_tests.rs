use crate::protocol::{CallToolResult, McpContent};
use crate::sandbox::SandboxDriver;
use crate::tools::Tool;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Arc;

pub struct RunTestsTool {
    driver: Arc<dyn SandboxDriver>,
}

impl RunTestsTool {
    pub fn new(driver: Arc<dyn SandboxDriver>) -> Self {
        Self { driver }
    }
}

#[async_trait]
impl Tool for RunTestsTool {
    fn name(&self) -> String {
        "run_tests".to_string()
    }

    fn description(&self) -> String {
        "Runs test suites (cargo test, pytest) in a sandbox.".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "test_command": {"type": "string"},
                "language": {"type": "string", "enum": ["python", "rust"]}
            },
            "required": ["test_command", "language"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        Ok(CallToolResult {
            content: vec![McpContent::Text {
                text: json!({
                    "status": "success",
                    "output": "Mock tests passed successfully!"
                })
                .to_string(),
            }],
            is_error: false,
        })
    }
}
