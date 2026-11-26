use crate::agents::ZeroClawAgent;
use crate::bridge::{BridgeState, StepCheckpoint};
use crate::workflows::circuit_breaker::compute_diff_hash;
use crate::workspaces::worktree::WorktreeManager;
use factory_core::config::InferencePriorityClass;
use factory_infrastructure::{AethalgardClient, McpClient, McpHttpClient, S3Storage};
use hatchet_sdk::Hatchet;
use hatchet_sdk::runnables::Task;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

fn default_quarantined() -> bool {
    true
}

/// Backwards-compatible input payload for sequential `"code"` and isolated `"develop-task-lane"` tasks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskInput {
    pub task_id: String,
    pub description: String,
    pub relevant_files: Vec<String>,
    #[serde(default)]
    pub mission_id: Option<String>,
    #[serde(default)]
    pub lane_id: Option<String>,
    #[serde(default)]
    pub worktree_path: Option<String>,
    #[serde(default)]
    pub branch_name: Option<String>,
    #[serde(default)]
    pub inference_priority: Option<String>,
}

/// Output payload returned by a sequential or Swarm-lane coding task.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskOutput {
    pub result: serde_json::Value,
    #[serde(default)]
    pub lane_id: Option<String>,
    #[serde(default)]
    pub branch_name: Option<String>,
    #[serde(default = "default_quarantined")]
    pub quarantined: bool,
    #[serde(default)]
    pub diff_sha256: Option<String>,
}

/// Executes a single `TaskInput` (sequential or isolated Swarm lane) using injected clients.
pub async fn execute_develop_task_lane_with_clients(
    input: &TaskInput,
    mcp_client: Arc<dyn McpClient>,
    aethalgard_client: Arc<dyn AethalgardClient>,
    s3_client: &dyn S3Storage,
    bucket: &str,
    worktree_manager: Option<&WorktreeManager>,
) -> anyhow::Result<TaskOutput> {
    let is_lane_mode = input.lane_id.is_some() || input.mission_id.is_some();
    let mission_id = input
        .mission_id
        .as_deref()
        .unwrap_or(&input.task_id)
        .to_string();
    let lane_id = input
        .lane_id
        .as_deref()
        .unwrap_or(&input.task_id)
        .to_string();

    let mut state = if is_lane_mode {
        BridgeState::load_lane_checkpoint(&mission_id, &lane_id, s3_client, bucket)
            .await?
            .unwrap_or_else(|| BridgeState::new(mission_id.clone()))
    } else {
        BridgeState::load_checkpoint(&input.task_id, s3_client, bucket)
            .await?
            .unwrap_or_else(|| BridgeState::new(input.task_id.clone()))
    };

    let checkpoint_key = if is_lane_mode {
        lane_id.clone()
    } else {
        input.task_id.clone()
    };

    if state.should_skip_completed_step(&checkpoint_key)
        || state.should_skip_completed_step(&input.task_id)
    {
        tracing::info!(
            "Task {} (lane={}) already completed in checkpoint. Skipping.",
            input.task_id,
            lane_id
        );
        return Ok(TaskOutput {
            result: serde_json::json!({"status": "already_done"}),
            lane_id: input.lane_id.clone(),
            branch_name: input.branch_name.clone(),
            quarantined: true,
            diff_sha256: None,
        });
    }

    let mut resolved_branch = input.branch_name.clone();
    let mut resolved_worktree = input.worktree_path.as_ref().map(PathBuf::from);
    let mut provisioned_lane = None;

    if is_lane_mode
        && resolved_worktree.is_none()
        && let Some(manager) = worktree_manager
    {
        let lane = manager
            .provision_lane_worktree(&mission_id, &lane_id, None)
            .await?;
        resolved_branch = Some(lane.branch_name.clone());
        resolved_worktree = Some(lane.path.clone());
        provisioned_lane = Some(lane);
    }

    let zeroclaw = ZeroClawAgent::new(mcp_client, aethalgard_client);
    let result = zeroclaw
        .execute_task_in_worktree(
            &mission_id,
            &input.description,
            &input.relevant_files,
            resolved_worktree.as_deref(),
        )
        .await?;

    let commit_sha =
        if let (Some(manager), Some(lane)) = (worktree_manager, provisioned_lane.as_ref()) {
            manager
                .commit_lane_changes(
                    lane,
                    &format!("feat(swarm): {} ({})", input.task_id, lane_id),
                )
                .await
                .ok()
                .flatten()
        } else {
            None
        };

    let diff_sha256 = commit_sha.or_else(|| {
        let serialized = serde_json::to_string(&result).unwrap_or_default();
        Some(compute_diff_hash(&serialized))
    });

    let priority_tag = input.inference_priority.clone().unwrap_or_else(|| {
        InferencePriorityClass::StandardExecution
            .as_header_value()
            .to_string()
    });

    state.checkpoints.insert(
        checkpoint_key,
        StepCheckpoint {
            step_name: if is_lane_mode {
                "develop-task-lane".to_string()
            } else {
                "code".to_string()
            },
            input_snapshot: serde_json::json!({
                "description": input.description,
                "lane_id": lane_id,
                "branch_name": resolved_branch,
                "inference_priority": priority_tag,
            }),
            output_snapshot: Some(result.clone()),
            completed_at: Some(chrono::Utc::now()),
            quarantined: true,
            diff_sha256: diff_sha256.clone(),
        },
    );

    if is_lane_mode {
        state
            .save_lane_checkpoint(&mission_id, &lane_id, s3_client, bucket)
            .await?;
    } else {
        state.save_checkpoint(s3_client, bucket).await?;
    }

    Ok(TaskOutput {
        result,
        lane_id: input
            .lane_id
            .clone()
            .or(if is_lane_mode { Some(lane_id) } else { None }),
        branch_name: resolved_branch,
        quarantined: true,
        diff_sha256,
    })
}

fn build_develop_task_with_name(
    hatchet: &Hatchet,
    task_name: &str,
    mcp_url: String,
) -> Task<TaskInput, TaskOutput> {
    let mcp_client: Arc<dyn McpClient> = Arc::new(McpHttpClient::new(mcp_url));

    let mcp_client_clone = mcp_client.clone();
    hatchet
        .task(task_name, move |input: TaskInput, _ctx| {
            let mcp_client = mcp_client_clone.clone();
            Box::pin(async move {
                tracing::info!("Workflow: executing task {}", input.task_id);

                let bucket = std::env::var("S3_CHECKPOINT_BUCKET")
                    .unwrap_or_else(|_| "dg-factory-checkpoints".to_string());
                let s3_client = factory_infrastructure::AwsS3Storage::new().await;
                let aethalgard_client =
                    Arc::new(factory_infrastructure::HttpAethalgardClient::new(
                        std::env::var("AETHALGARD_WEBHOOK_URL")
                            .unwrap_or_else(|_| "http://localhost:8080/webhook".to_string()),
                    ));
                let wt_manager = if input.lane_id.is_some() && input.worktree_path.is_none() {
                    Some(WorktreeManager::from_env())
                } else {
                    None
                };

                execute_develop_task_lane_with_clients(
                    &input,
                    mcp_client,
                    aethalgard_client,
                    &s3_client,
                    &bucket,
                    wt_manager.as_ref(),
                )
                .await
            })
        })
        .build()
        .unwrap()
}

/// Registers the backwards-compatible standalone `"code"` Hatchet task.
pub fn create_develop_task_workflow(
    hatchet: &Hatchet,
    mcp_url: String,
) -> Task<TaskInput, TaskOutput> {
    build_develop_task_with_name(hatchet, "code", mcp_url)
}

/// Registers the isolated Swarm `"develop-task-lane"` Hatchet task.
pub fn create_develop_task_lane_workflow(
    hatchet: &Hatchet,
    mcp_url: String,
) -> Task<TaskInput, TaskOutput> {
    build_develop_task_with_name(hatchet, "develop-task-lane", mcp_url)
}
