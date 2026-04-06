use crate::agents::ZeroClawAgent;
use factory_infrastructure::McpHttpClient;
use hatchet_sdk::Hatchet;
use hatchet_sdk::runnables::Task;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

pub fn create_develop_task_workflow(
    hatchet: &Hatchet,
    mcp_url: String,
) -> Task<TaskInput, TaskOutput> {
    let mcp_url_clone = mcp_url.clone();

    hatchet
        .task(
            "zeroclaw:execute_coding_task",
            move |input: TaskInput, _ctx| {
                let mcp_url = mcp_url_clone.clone();
                Box::pin(async move {
                    tracing::info!("Workflow: executing task {}", input.task_id);

                    let mcp_client = Arc::new(McpHttpClient::new(mcp_url));
                    let zeroclaw = ZeroClawAgent::new(mcp_client);

                    // Use the execute_task method from ZeroClawAgent
                    let result = zeroclaw
                        .execute_task(&input.task_id, &input.description, &input.relevant_files)
                        .await?;

                    Ok(TaskOutput { result })
                })
            },
        )
        .build()
        .unwrap()
}
