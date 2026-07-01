use crate::protocol::CallToolResult;
use async_trait::async_trait;
use serde_json::Value;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn input_schema(&self) -> Value;

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult>;
}

pub mod execute_code;
pub mod index_code;
pub mod launch_sandbox_pod;
pub mod plan_mission;
pub mod retrieve_context;
pub mod run_tests;
pub mod search_jira;
pub mod security_review;
pub mod spec_kit_tool;
pub mod update_mission_status;

pub use execute_code::ExecuteCodeTool;
pub use index_code::IndexCodeTool;
pub use retrieve_context::RetrieveContextTool;
pub use run_tests::RunTestsTool;
pub use search_jira::SearchJiraTool;
pub use spec_kit_tool::SpecKitTool;
pub use update_mission_status::UpdateMissionStatusTool;
