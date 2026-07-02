use crate::agents::ZeroClawAgent;
use factory_infrastructure::{McpClient, McpHttpClient};
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
    let mcp_client: Arc<dyn McpClient> = Arc::new(McpHttpClient::new(mcp_url));

    let mcp_client_clone = mcp_client.clone();
    hatchet
        .task("code", move |input: TaskInput, _ctx| {
            let mcp_client = mcp_client_clone.clone();
            Box::pin(async move {
                tracing::info!("Workflow: executing task {}", input.task_id);

                let bucket = std::env::var("S3_CHECKPOINT_BUCKET").unwrap_or_else(|_| "dg-factory-checkpoints".to_string());
                let s3_client = factory_infrastructure::AwsS3Storage::new().await;

                // Crash Resilience: Load checkpoint if exists
                let mut state = crate::bridge::BridgeState::load_checkpoint(&input.task_id, &s3_client, &bucket).await?
                    .unwrap_or_else(|| crate::bridge::BridgeState::new(input.task_id.clone()));

                // Example: If task was already completed in the checkpoint, we skip execution
                if state.superpowers.completed_tasks.contains(&input.task_id) {
                    tracing::info!(
                        "Task {} already completed in checkpoint. Skipping.",
                        input.task_id
                    );
                    return Ok(TaskOutput {
                        result: serde_json::json!({"status": "already_done"}),
                    });
                }

                let zeroclaw = ZeroClawAgent::new(mcp_client);

                // Use the execute_task method from ZeroClawAgent
                let result = zeroclaw
                    .execute_task(&input.task_id, &input.description, &input.relevant_files)
                    .await?;

                // Crash Resilience: Update and save checkpoint after successful execution
                state
                    .superpowers
                    .completed_tasks
                    .push(input.task_id.clone());
                state.save_checkpoint(&s3_client, &bucket).await?;

                Ok(TaskOutput { result })
            })
        })
        .build()
        .unwrap()
}
