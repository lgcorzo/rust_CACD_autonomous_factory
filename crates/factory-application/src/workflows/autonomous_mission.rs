use hatchet_sdk::workflow::Workflow;
use hatchet_sdk::Context;
use serde::{Deserialize, Serialize};
use crate::agents::{PlannerAgent, ReviewerAgent, TesterAgent, DocAgent};
use crate::Agent;
use factory_infrastructure::{McpHttpClient, McpClient};
use crate::workflows::develop_task::TaskInput;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MissionInput {
    pub goal: String,
    pub repository_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MissionOutput {
    pub status: String,
    pub summary: String,
}

pub struct AutonomousMissionWorkflow {
    mcp_url: String,
}

impl AutonomousMissionWorkflow {
    pub fn new(mcp_url: String) -> Self {
        Self { mcp_url }
    }
}

#[hatchet_sdk::workflow(name = "AutonomousMissionWorkflow", on_events = ["autonomous_mission"])]
impl AutonomousMissionWorkflow {
    #[hatchet_sdk::step(name = "plan", timeout = "5m")]
    pub async fn plan(&self, ctx: Context<MissionInput>) -> anyhow::Result<serde_json::Value> {
        let input = &ctx.workflow_input;
        tracing::info!("Workflow: planning mission for {}", input.goal);
        
        let mcp_client = std::sync::Arc::new(McpHttpClient::new(self.mcp_url.clone()));
        let planner = PlannerAgent::new(mcp_client);
        
        let plan = planner.execute(&input.goal).await?;
        Ok(plan)
    }

    #[hatchet_sdk::step(name = "fan_out_tasks", parents = ["plan"], timeout = "30m")]
    pub async fn fan_out_tasks(&self, ctx: Context<MissionInput>) -> anyhow::Result<serde_json::Value> {
        let input = &ctx.workflow_input;
        let plan: serde_json::Value = ctx.step_output("plan")?;
        let tasks = plan["tasks"].as_array().ok_or(anyhow::anyhow!("No tasks in plan"))?;
        
        tracing::info!("Workflow: fanning out {} tasks", tasks.len());
        
        let mut child_runs: Vec<hatchet_sdk::workflow_run::ChildRunHandle<serde_json::Value>> = Vec::new();
        for task in tasks {
            let child_input = TaskInput {
                task_id: task["id"].as_str().unwrap_or("unknown").to_string(),
                description: task["description"].as_str().unwrap_or("").to_string(),
                relevant_files: vec![], // Simplified
            };
            
            let child_run = ctx.spawn_child::<TaskInput, serde_json::Value>(
                "DevelopTaskWorkflow",
                child_input,
                None,
            ).await?;
            
            child_runs.push(child_run);
        }
        
        let mut results = Vec::new();
        for mut run in child_runs {
            let result: serde_json::Value = run.result().await?;
            results.push(result);
        }
        
        Ok(serde_json::Value::Array(results))
    }

    #[hatchet_sdk::step(name = "aggregate_and_review", parents = ["fan_out_tasks"], timeout = "15m")]
    pub async fn aggregate_and_review(&self, ctx: Context<MissionInput>) -> anyhow::Result<MissionOutput> {
        let input = &ctx.workflow_input;
        let results: serde_json::Value = ctx.step_output("fan_out_tasks")?;
        tracing::info!("Workflow: aggregating results and reviewing");
        
        let mcp_client = std::sync::Arc::new(McpHttpClient::new(self.mcp_url.clone()));
        let tester = TesterAgent::new(mcp_client.clone());
        let reviewer = ReviewerAgent::new(mcp_client);
        
        let test_res = tester.execute("Run all tests").await?;
        let review_res = reviewer.review_changes(&results.to_string()).await?;
        
        let status = if test_res["status"] == "passed" && review_res["approved"].as_bool().unwrap_or(false) {
            "success"
        } else {
            "failed"
        };
        
        Ok(MissionOutput {
            status: status.to_string(),
            summary: format!("Mission {}. Tests passed: {}. Review approved: {}", status, test_res["status"], review_res["approved"]),
        })
    }

    #[hatchet_sdk::step(name = "document_mission", parents = ["aggregate_and_review"], timeout = "10m")]
    pub async fn document_mission(&self, ctx: Context<MissionInput>) -> anyhow::Result<MissionOutput> {
        let input = &ctx.workflow_input;
        let review_output: MissionOutput = ctx.step_output("aggregate_and_review")?;
        tracing::info!("Workflow: documenting mission");
        
        let mcp_client = std::sync::Arc::new(McpHttpClient::new(self.mcp_url.clone()));
        let doc_agent = DocAgent::new(mcp_client);
        
        let _ = doc_agent.execute("Generate final report").await?;
        
        Ok(review_output)
    }
}
