use crate::agents::{DocAgent, PlannerAgent, ReviewerAgent, TesterAgent};
use crate::workflows::develop_task::{TaskInput, create_develop_task_workflow};
use factory_infrastructure::McpHttpClient;
use futures::future::try_join_all;
use hatchet_sdk::Hatchet;
use hatchet_sdk::runnables::{Runnable, Workflow};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

pub fn create_mission_workflow(
    hatchet: &Hatchet,
    mcp_url: String,
) -> Workflow<MissionInput, MissionOutput> {
    let mcp_url_clone = mcp_url.clone();

    // 1. Planning Task
    let plan_task = hatchet
        .task("plan", move |input: MissionInput, _ctx| {
            let mcp_url = mcp_url_clone.clone();
            Box::pin(async move {
                let mcp_client = Arc::new(McpHttpClient::new(mcp_url));
                let planner = PlannerAgent::new(mcp_client);
                planner.create_plan(&input.goal).await
            })
        })
        .build()
        .unwrap();

    // 2. Fan-out Task (Dynamic)
    let mcp_url_clone = mcp_url.clone();
    let hatchet_clone = hatchet.clone();
    let fan_out_task = hatchet
        .task("fan_out", move |_input: MissionInput, ctx| {
            let mcp_url = mcp_url_clone.clone();
            let hatchet = hatchet_clone.clone();
            Box::pin(async move {
                let plan_val: serde_json::Value = ctx.parent_output("plan").await?;
                let plan: serde_json::Value = serde_json::from_value(plan_val)?;
                let tasks = plan["tasks"]
                    .as_array()
                    .ok_or(anyhow::anyhow!("No tasks in plan"))?;

                let dev_wf = create_develop_task_workflow(&hatchet, mcp_url);
                let mut futures = Vec::new();

                for task in tasks {
                    let child_input = TaskInput {
                        task_id: task["id"].as_str().unwrap_or("unknown").to_string(),
                        description: task["description"].as_str().unwrap_or("").to_string(),
                        relevant_files: vec![],
                    };
                    // In v0.2.7, run() returns a future that we can collect
                    futures.push(dev_wf.run(child_input, None));
                }

                let results = try_join_all(futures)
                    .await?
                    .into_iter()
                    .map(|res| res.result)
                    .collect();

                Ok(serde_json::Value::Array(results))
            })
        })
        .build()
        .unwrap()
        .add_parent(&plan_task);

    // 3. Review Task
    let mcp_url_clone = mcp_url.clone();
    let review_task = hatchet
        .task("review", move |_input: MissionInput, ctx| {
            let mcp_url = mcp_url_clone.clone();
            Box::pin(async move {
                let results_val: serde_json::Value = ctx.parent_output("fan_out").await?;
                let results: serde_json::Value = serde_json::from_value(results_val)?;

                let mcp_client = Arc::new(McpHttpClient::new(mcp_url));
                let tester = TesterAgent::new(mcp_client.clone());
                let reviewer = ReviewerAgent::new(mcp_client);

                let test_res = tester.run_tests("Run all tests").await?;
                let review_res = reviewer.review_changes(&results.to_string()).await?;

                let status = if test_res["status"] == "passed"
                    && review_res["approved"].as_bool().unwrap_or(false)
                {
                    "success"
                } else {
                    "failed"
                };

                Ok(MissionOutput {
                    status: status.to_string(),
                    summary: format!(
                        "Mission {}. Tests passed: {}. Review approved: {}",
                        status, test_res["status"], review_res["approved"]
                    ),
                })
            })
        })
        .build()
        .unwrap()
        .add_parent(&fan_out_task);

    // 4. Documentation Task
    let mcp_url_clone = mcp_url.clone();
    let doc_task = hatchet
        .task("document", move |_input: MissionInput, ctx| {
            let mcp_url = mcp_url_clone.clone();
            Box::pin(async move {
                let review_val: serde_json::Value = ctx.parent_output("review").await?;
                let review_output: MissionOutput = serde_json::from_value(review_val)?;

                let mcp_client = Arc::new(McpHttpClient::new(mcp_url));
                let doc_agent = DocAgent::new(mcp_client);

                let _ = doc_agent.generate_docs("Generate final report").await?;

                Ok(review_output)
            })
        })
        .build()
        .unwrap()
        .add_parent(&review_task);

    // Build the workflow
    hatchet
        .workflow("AutonomousMission")
        .build()
        .unwrap()
        .add_task(&plan_task)
        .add_task(&fan_out_task)
        .add_task(&review_task)
        .add_task(&doc_task)
}
