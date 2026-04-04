use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct ExecuteCodeTool;

#[async_trait]
impl Tool for ExecuteCodeTool {
    fn name(&self) -> String {
        "execute_code".to_string()
    }

    fn description(&self) -> String {
        "Executes code in a sandbox.".to_string()
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

    async fn call(&self, _params: Value) -> anyhow::Result<CallToolResult> {
        // TODO: Implement Firecracker/E2B logic
        Ok(CallToolResult {
            content: vec![McpContent::Text {
                text: "Code execution not yet implemented in Rust.".to_string(),
            }],
            is_error: true,
        })
    }
}
