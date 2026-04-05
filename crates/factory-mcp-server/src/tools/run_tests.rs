use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use crate::sandbox::SandboxDriver;
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
        let test_command = params["test_command"].as_str().unwrap_or("");
        let language = params["language"].as_str().unwrap_or("python");

        // We use the sandbox driver to execute the test command
        let result = self.driver.execute(test_command, language).await?;

        // Structured output for tests
        let mut output = format!("Test Execution Results ({})\n", language);
        output.push_str("------------------------------\n");
        output.push_str(&format!("Success: {}\n", result.is_success));
        output.push_str(&format!("Exit Code: {:?}\n\n", result.exit_code));
        output.push_str("Output:\n");
        output.push_str(&result.stdout);
        if !result.stderr.is_empty() {
            output.push_str("\n\nErrors:\n");
            output.push_str(&result.stderr);
        }

        Ok(CallToolResult {
            content: vec![McpContent::Text { text: output }],
            is_error: !result.is_success,
        })
    }
}
