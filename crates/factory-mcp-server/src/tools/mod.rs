use async_trait::async_trait;
use serde_json::Value;
use crate::protocol::CallToolResult;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn input_schema(&self) -> Value;
    
    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult>;
}

pub mod retrieve_context;
pub mod plan_mission;
pub mod execute_code;
pub mod security_review;
