use crate::protocol::CallToolResult;
use crate::tools::Tool;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpecKitCommand {
    Brainstorming,
    DispatchingParallelAgents,
    ExecutingPlans,
    FinishingADevelopmentBranch,
    ReceivingCodeReview,
    RequestingCodeReview,
    SubagentDrivenDevelopment,
    SystematicDebugging,
    TestDrivenDevelopment,
    UsingGitWorktrees,
    UsingSuperpowers,
    VerificationBeforeCompletion,
    WritingPlans,
    WritingSkills,
    Init,
    Specify,
    Plan,
    Execute,
    Verify,
    GitCommit,
}

pub struct SpecKitTool {
    specify_cli_path: String,
}

impl SpecKitTool {
    pub fn new(specify_cli_path: String) -> Self {
        Self { specify_cli_path }
    }

    pub async fn invoke_spec_kit(
        &self,
        command: SpecKitCommand,
        args: Vec<String>,
    ) -> anyhow::Result<String> {
        let command_str = serde_json::to_string(&command)?
            .trim_matches('"')
            .to_string();

        let mut cmd = Command::new(&self.specify_cli_path);
        cmd.arg(&command_str);

        for arg in args {
            cmd.arg(arg);
        }

        let output = cmd.output().await?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let err = String::from_utf8_lossy(&output.stderr).to_string();
            anyhow::bail!("SpecKit command {} failed: {}", command_str, err)
        }
    }
}

#[async_trait]
impl Tool for SpecKitTool {
    fn name(&self) -> String {
        "invoke_spec_kit".to_string()
    }

    fn description(&self) -> String {
        "Invoke the Spec-Kit pipeline skills via the specify CLI".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The SpecKitCommand to execute"
                },
                "args": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "description": "Arguments to pass to the CLI"
                }
            },
            "required": ["command"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let command_str = params
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'command'"))?;

        let command: SpecKitCommand = serde_json::from_value(json!(command_str))?;

        let args = params
            .get("args")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        match self.invoke_spec_kit(command, args).await {
            Ok(output) => Ok(CallToolResult {
                content: vec![crate::protocol::McpContent::Text { text: output }],
                is_error: false,
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![crate::protocol::McpContent::Text {
                    text: e.to_string(),
                }],
                is_error: true,
            }),
        }
    }
}
