use hatchet_sdk::workflow::Workflow;
use hatchet_sdk::Context;
use serde::{Deserialize, Serialize};
use crate::agents::CoderAgent;
use crate::Agent;
use factory_infrastructure::{McpHttpClient, McpClient};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskInput {
    pub task_id: String,
    pub description: String,
    pub relevant_files: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskOutput {
    pub result: serde_json::Value,
}

pub struct DevelopTaskWorkflow {
    mcp_url: String,
}

impl DevelopTaskWorkflow {
    pub fn new(mcp_url: String) -> Self {
        Self { mcp_url }
    }
}

#[hatchet_sdk::workflow(name = "DevelopTaskWorkflow")]
impl DevelopTaskWorkflow {
    #[hatchet_sdk::step(name = "execute_coding_task", timeout = "15m")]
    pub async fn execute_coding_task(&self, ctx: Context<TaskInput>) -> anyhow::Result<TaskOutput> {
        let input = &ctx.workflow_input;
        tracing::info!("Workflow: executing task {}", input.task_id);
        
        let mcp_client = std::sync::Arc::new(McpHttpClient::new(self.mcp_url.clone()));
        let coder = CoderAgent::new(mcp_client);
        
        let result = coder.execute(&input.description).await?;
        
        Ok(TaskOutput { result })
    }
}
