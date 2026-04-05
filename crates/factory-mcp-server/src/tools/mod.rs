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
pub mod plan_mission;
pub mod retrieve_context;
pub mod run_tests;
pub mod security_review;
