use crate::protocol::CallToolResult;
use crate::tools::Tool;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

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

#[async_trait]
pub trait SpecProvider: Send + Sync {
    async fn invoke(&self, command: SpecKitCommand, args: Vec<String>) -> anyhow::Result<String>;
}

#[derive(Debug, Clone)]
pub struct MockSpecProvider {
    specs_dir: std::path::PathBuf,
}

impl MockSpecProvider {
    pub fn new(specs_dir: std::path::PathBuf) -> Self {
        Self { specs_dir }
    }
}

impl Default for MockSpecProvider {
    fn default() -> Self {
        Self {
            specs_dir: std::path::PathBuf::from("specs"),
        }
    }
}

#[async_trait]
impl SpecProvider for MockSpecProvider {
    async fn invoke(&self, command: SpecKitCommand, _args: Vec<String>) -> anyhow::Result<String> {
        let command_str = serde_json::to_string(&command)?
            .trim_matches('"')
            .to_string();

        match command {
            SpecKitCommand::Init => {
                tokio::fs::create_dir_all(".specify").await?;
                let init_options = serde_json::json!({
                    "specs_dir": self.specs_dir.to_string_lossy()
                });
                tokio::fs::write(
                    ".specify/init-options.json",
                    serde_json::to_string_pretty(&init_options)?,
                )
                .await?;
            }
            SpecKitCommand::Specify => {
                tokio::fs::create_dir_all(&self.specs_dir).await?;
                tokio::fs::write(self.specs_dir.join("spec.md"), "# Mock Spec").await?;
            }
            SpecKitCommand::Plan => {
                tokio::fs::create_dir_all(&self.specs_dir).await?;
                tokio::fs::write(
                    self.specs_dir.join("plan.md"),
                    "{\"tasks\":[{\"description\":\"print('hello world from fix')\"}]}",
                )
                .await?;
            }
            SpecKitCommand::Execute => {
                tokio::fs::create_dir_all(&self.specs_dir).await?;
                tokio::fs::write(
                    self.specs_dir.join("tasks.md"),
                    "{\"tasks\":[{\"description\":\"print('hello world from fix')\"}]}",
                )
                .await?;
            }
            _ => {}
        }

        Ok(format!("Mock {} executed successfully", command_str))
    }
}

#[derive(Debug, Clone)]
pub struct CliSpecProvider {
    cli_path: String,
    fallback: MockSpecProvider,
}

impl CliSpecProvider {
    pub fn new(cli_path: String) -> Self {
        Self {
            cli_path,
            fallback: MockSpecProvider::default(),
        }
    }
}

#[async_trait]
impl SpecProvider for CliSpecProvider {
    async fn invoke(&self, command: SpecKitCommand, args: Vec<String>) -> anyhow::Result<String> {
        let command_str = serde_json::to_string(&command)?
            .trim_matches('"')
            .to_string();

        let mut cmd = tokio::process::Command::new(&self.cli_path);
        cmd.arg(&command_str);
        for arg in &args {
            cmd.arg(arg);
        }

        match cmd.output().await {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
                    Ok(stdout)
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
                    anyhow::bail!("SpecKit CLI exited with error: {}", stderr);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::warn!(
                    "SpecKit CLI not found at '{}'. Falling back to mock spec provider.",
                    self.cli_path
                );
                self.fallback.invoke(command, args).await
            }
            Err(e) => {
                anyhow::bail!("Failed to execute SpecKit CLI: {}", e);
            }
        }
    }
}

pub struct SpecKitTool {
    provider: Arc<dyn SpecProvider>,
}

impl SpecKitTool {
    pub fn new(specify_cli_path: String) -> Self {
        let provider: Arc<dyn SpecProvider> =
            if specify_cli_path == "mock" || specify_cli_path.is_empty() {
                Arc::new(MockSpecProvider::default())
            } else {
                Arc::new(CliSpecProvider::new(specify_cli_path))
            };
        Self { provider }
    }

    pub fn with_provider(provider: Arc<dyn SpecProvider>) -> Self {
        Self { provider }
    }

    pub async fn invoke_spec_kit(
        &self,
        command: SpecKitCommand,
        args: Vec<String>,
    ) -> anyhow::Result<String> {
        self.provider.invoke(command, args).await
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_spec_provider() {
        let unique_id = uuid::Uuid::new_v4().to_string();
        let specs_dir = std::path::PathBuf::from(format!("target/test-specs-{}", unique_id));

        let provider = MockSpecProvider::new(specs_dir.clone());

        // Test Init
        provider.invoke(SpecKitCommand::Init, vec![]).await.unwrap();
        assert!(std::path::Path::new(".specify/init-options.json").exists());
        let init_options_content = std::fs::read_to_string(".specify/init-options.json").unwrap();
        let init_options: serde_json::Value = serde_json::from_str(&init_options_content).unwrap();
        assert_eq!(
            init_options["specs_dir"].as_str().unwrap(),
            specs_dir.to_string_lossy()
        );

        // Test Specify
        provider
            .invoke(SpecKitCommand::Specify, vec![])
            .await
            .unwrap();
        assert!(specs_dir.join("spec.md").exists());
        assert_eq!(
            std::fs::read_to_string(specs_dir.join("spec.md")).unwrap(),
            "# Mock Spec"
        );

        // Test Plan
        provider.invoke(SpecKitCommand::Plan, vec![]).await.unwrap();
        assert!(specs_dir.join("plan.md").exists());

        // Test Execute
        provider
            .invoke(SpecKitCommand::Execute, vec![])
            .await
            .unwrap();
        assert!(specs_dir.join("tasks.md").exists());

        // Clean up .specify and specs_dir directories created by test
        let _ = std::fs::remove_dir_all(".specify");
        let _ = std::fs::remove_dir_all(&specs_dir);
    }

    #[tokio::test]
    async fn test_cli_spec_provider_fallback() {
        // Use a non-existent command to trigger fallback
        let provider = CliSpecProvider::new("non_existent_command_12345".to_string());

        // It should fallback to MockSpecProvider and succeed
        let unique_id = uuid::Uuid::new_v4().to_string();
        let specs_dir = std::path::PathBuf::from(format!("target/test-specs-{}", unique_id));

        // Replace the fallback provider with a custom specs_dir one
        let mut provider_with_custom_fallback = provider.clone();
        provider_with_custom_fallback.fallback = MockSpecProvider::new(specs_dir.clone());

        let res = provider_with_custom_fallback
            .invoke(SpecKitCommand::Specify, vec![])
            .await;
        assert!(res.is_ok());
        assert!(specs_dir.join("spec.md").exists());

        // Clean up specs_dir directory created by test
        let _ = std::fs::remove_dir_all(&specs_dir);
    }

    #[tokio::test]
    async fn test_spec_kit_tool_mock_mode() {
        let tool = SpecKitTool::new("mock".to_string());
        let res = tool
            .invoke_spec_kit(SpecKitCommand::Specify, vec![])
            .await
            .unwrap();
        assert!(res.contains("Mock specify executed successfully"));
    }
}
