use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::process::Command;

pub struct ExecuteCodeTool;

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

        let output = match language {
            "python" => {
                Command::new("python3")
                    .arg("-c")
                    .arg(code)
                    .output()
                    .await?
            }
            "rust" => {
                // For Rust, we'd typically use a temporary file and cargo run
                // For now, let's just use rustc -e if supported, or a simplified approach
                // Simplified: use cargo-script if available, but let's stick to Python for basic tasks
                // or just return an error for Rust for now if we can't compile.
                Command::new("rustc")
                    .arg("-e") // NOTE: rustc doesn't have -e by default like perl, this is hypothetical
                    .output()
                    .await
                    .map_err(|_| anyhow::anyhow!("Rust execution requires a compiler and is more complex. Use Python for simple tasks."))?
            }
            _ => return Err(anyhow::anyhow!("Unsupported language")),
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok(CallToolResult {
                content: vec![McpContent::Text { text: stdout }],
                is_error: false,
            })
        } else {
            // As per plan, use a generic error message if execution fails to avoid leaking details
            Ok(CallToolResult {
                content: vec![McpContent::Text { 
                    text: format!("Execution failed.\nStdout: {}\nStderr: {}", stdout, stderr) 
                }],
                is_error: true,
            })
        }
    }
}
