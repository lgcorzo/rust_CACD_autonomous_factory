use async_trait::async_trait;
use serde_json::Value;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> String;
    async fn execute(&self, task_description: &str) -> anyhow::Result<Value>;
}

pub mod agents;
pub mod workflows;
