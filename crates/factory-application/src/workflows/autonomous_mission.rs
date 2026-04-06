use crate::agents::{RustantAgent, ZeroClawAgent};
use factory_infrastructure::{HttpR2rClient, KafkaClient, McpClient, McpHttpClient};
use hatchet_sdk::Hatchet;
use hatchet_sdk::runnables::Workflow;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MissionInput {
    pub mission_id: Option<String>,
    pub goal: String,
    pub repository_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MissionOutput {
    pub mission_id: String,
    pub status: String,
    pub summary: String,
    pub pr_url: Option<String>,
}

pub fn create_mission_workflow(
    hatchet: &Hatchet,
    mcp_url: String,
    r2r_url: String,
    kafka_brokers: String,
) -> Workflow<MissionInput, MissionOutput> {
    let mcp_url_clone = mcp_url.clone();
    let r2r_url_clone = r2r_url.clone();
    let kafka_brokers_clone = kafka_brokers.clone();

    // 1. Planning Phase (Rustant)
    let plan_task = hatchet
        .task("rustant:plan", move |input: MissionInput, _ctx| {
            let mcp_url = mcp_url_clone.clone();
            let r2r_url = r2r_url_clone.clone();
            let kafka_brokers = kafka_brokers_clone.clone();
            let mission_id = input
                .mission_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            Box::pin(async move {
                let mcp_client = Arc::new(McpHttpClient::new(mcp_url));
                let r2r_client = Arc::new(HttpR2rClient::new(
                    r2r_url,
                    "admin".to_string(),
                    "admin".to_string(),
                ));
                let kafka_client =
                    factory_infrastructure::SimpleMockKafkaClient::new(&kafka_brokers)?;

                let rustant = RustantAgent::new(mcp_client, r2r_client);

                kafka_client
                    .publish_thought(&mission_id, "Starting planning phase...", "rustant")
                    .await?;
                let plan = rustant.plan_mission(&mission_id, &input.goal).await?;
                kafka_client
                    .publish_thought(&mission_id, "Plan generated successfully", "rustant")
                    .await?;

                Ok(plan)
            })
        })
        .build()
        .unwrap();

    // 2. Coding Phase (ZeroClaw)
    let mcp_url_clone = mcp_url.clone();
    let kafka_brokers_clone = kafka_brokers.clone();
    let code_task = hatchet
        .task("zeroclaw:execute", move |input: MissionInput, ctx| {
            let mcp_url = mcp_url_clone.clone();
            let kafka_brokers = kafka_brokers_clone.clone();
            let mission_id = input
                .mission_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            Box::pin(async move {
                let plan: serde_json::Value = ctx.parent_output("rustant:plan").await?;
                let mcp_client = Arc::new(McpHttpClient::new(mcp_url));
                let kafka_client =
                    factory_infrastructure::SimpleMockKafkaClient::new(&kafka_brokers)?;

                let zeroclaw = ZeroClawAgent::new(mcp_client);

                kafka_client
                    .publish_thought(&mission_id, "Starting coding phase...", "zeroclaw")
                    .await?;
                let result = zeroclaw
                    .execute_task(&mission_id, plan["summary"].as_str().unwrap_or(""), &[])
                    .await?;
                kafka_client
                    .publish_thought(&mission_id, "Coding completed", "zeroclaw")
                    .await?;

                Ok(result)
            })
        })
        .build()
        .unwrap()
        .add_parent(&plan_task);

    // 3. Validation Phase (ZeroClaw)
    let mcp_url_clone = mcp_url.clone();
    let kafka_brokers_clone = kafka_brokers.clone();
    let validation_task = hatchet
        .task("zeroclaw:validate", move |input: MissionInput, _ctx| {
            let mcp_url = mcp_url_clone.clone();
            let kafka_brokers = kafka_brokers_clone.clone();
            let mission_id = input
                .mission_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            Box::pin(async move {
                let mcp_client = Arc::new(McpHttpClient::new(mcp_url));
                let kafka_client =
                    factory_infrastructure::SimpleMockKafkaClient::new(&kafka_brokers)?;

                let zeroclaw = ZeroClawAgent::new(mcp_client);

                kafka_client
                    .publish_thought(&mission_id, "Starting validation phase...", "zeroclaw")
                    .await?;
                let test_res = zeroclaw.validate_mission(&mission_id, "cargo test").await?;
                kafka_client
                    .publish_thought(
                        &mission_id,
                        &format!("Validation finished: {}", test_res["status"]),
                        "zeroclaw",
                    )
                    .await?;

                Ok(test_res)
            })
        })
        .build()
        .unwrap()
        .add_parent(&code_task);

    // 4. Review Phase (Rustant)
    let mcp_url_clone = mcp_url.clone();
    let r2r_url_clone = r2r_url.clone();
    let kafka_brokers_clone = kafka_brokers.clone();
    let review_task = hatchet
        .task("rustant:review", move |input: MissionInput, ctx| {
            let mcp_url = mcp_url_clone.clone();
            let r2r_url = r2r_url_clone.clone();
            let kafka_brokers = kafka_brokers_clone.clone();
            let mission_id = input
                .mission_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            Box::pin(async move {
                let code_res: serde_json::Value = ctx.parent_output("zeroclaw:execute").await?;
                let mcp_client = Arc::new(McpHttpClient::new(mcp_url));
                let r2r_client = Arc::new(HttpR2rClient::new(
                    r2r_url,
                    "admin".to_string(),
                    "admin".to_string(),
                ));
                let kafka_client =
                    factory_infrastructure::SimpleMockKafkaClient::new(&kafka_brokers)?;

                let rustant = RustantAgent::new(mcp_client, r2r_client);

                kafka_client
                    .publish_thought(&mission_id, "Starting security review phase...", "rustant")
                    .await?;
                let review = rustant.review_mission(&mission_id, &code_res.to_string()).await?;
                kafka_client
                    .publish_thought(&mission_id, "Review completed", "rustant")
                    .await?;

                Ok(review)
            })
        })
        .build()
        .unwrap()
        .add_parent(&validation_task);

    // 5. Delivery Phase (PR Creation)
    let mcp_url_clone = mcp_url.clone();
    let kafka_brokers_clone = kafka_brokers.clone();
    let delivery_task = hatchet
        .task("factory:deliver", move |input: MissionInput, ctx| {
            let mcp_url = mcp_url_clone.clone();
            let kafka_brokers = kafka_brokers_clone.clone();
            let mission_id = input
                .mission_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            Box::pin(async move {
                let review_res: serde_json::Value = ctx.parent_output("rustant:review").await?;
                let mcp_client = Arc::new(McpHttpClient::new(mcp_url));
                let kafka_client =
                    factory_infrastructure::SimpleMockKafkaClient::new(&kafka_brokers)?;

                if review_res["approved"].as_bool().unwrap_or(false) {
                    kafka_client
                        .publish_thought(&mission_id, "Review approved. Creating PR...", "factory")
                        .await?;

                    let branch_name = format!("mission-{}", mission_id);
                    let pr_res = mcp_client
                        .call_tool_json(
                            "create_pull_request",
                            serde_json::json!({
                                "branch_name": branch_name,
                                "title": format!("Mission Delivery: {}", mission_id),
                                "body": "Automatically generated by Dark Gravity Mission Factory"
                            }),
                        )
                        .await?;

                    Ok(MissionOutput {
                        mission_id: mission_id.clone(),
                        status: "completed".to_string(),
                        summary: "Mission successful and PR created".to_string(),
                        pr_url: Some(pr_res["url"].as_str().unwrap_or("").to_string()),
                    })
                } else {
                    kafka_client
                        .publish_thought(&mission_id, "Review REJECTED. Mission failed.", "factory")
                        .await?;
                    Ok(MissionOutput {
                        mission_id: mission_id.clone(),
                        status: "failed".to_string(),
                        summary: "Security review rejected".to_string(),
                        pr_url: None,
                    })
                }
            })
        })
        .build()
        .unwrap()
        .add_parent(&review_task);

    hatchet
        .workflow("DarkGravityMission")
        .build()
        .unwrap()
        .add_task(&plan_task)
        .add_task(&code_task)
        .add_task(&validation_task)
        .add_task(&review_task)
        .add_task(&delivery_task)
}
