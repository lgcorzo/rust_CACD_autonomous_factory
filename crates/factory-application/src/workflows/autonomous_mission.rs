use crate::agents::{RustantAgent, ZeroClawAgent};
use factory_infrastructure::{
    HttpR2rClient, KafkaClient, McpClient, McpHttpClient, R2rClient,
    aethalgard::{AethalgardClient, HttpAethalgardClient},
};
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

impl MissionInput {
    pub fn from_protobuf(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        use factory_core::proto::v1::MissionInput as ProtoInput;
        use prost::Message;

        let proto = ProtoInput::decode(bytes)?;
        Ok(MissionInput {
            mission_id: if proto.mission_id.is_empty() {
                None
            } else {
                Some(proto.mission_id)
            },
            goal: format!(
                "Title: {}\nDescription: {}\nLabels: {:?}",
                proto.epic_title, proto.epic_description, proto.labels
            ),
            repository_path: String::new(),
        })
    }
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
    aethalgard_webhook_url: String,
) -> Workflow<MissionInput, MissionOutput> {
    let mcp_client: Arc<dyn McpClient> = Arc::new(McpHttpClient::new(mcp_url));
    let r2r_client: Arc<dyn R2rClient> = Arc::new(HttpR2rClient::new(
        r2r_url,
        "admin".to_string(),
        "admin".to_string(),
    ));
    let kafka_client: Arc<dyn KafkaClient> = if kafka_brokers == "mock" || kafka_brokers.is_empty()
    {
        Arc::new(factory_infrastructure::SimpleMockKafkaClient::new(&kafka_brokers).unwrap())
    } else {
        Arc::new(factory_infrastructure::RdKafkaClient::new(&kafka_brokers).unwrap())
    };

    // 1. Planning Phase (Rustant)
    let mcp_client_clone = mcp_client.clone();
    let r2r_client_clone = r2r_client.clone();
    let kafka_client_clone = kafka_client.clone();
    let plan_task = hatchet
        .task("rustant-plan", move |input: MissionInput, _ctx| {
            let mcp_client = mcp_client_clone.clone();
            let r2r_client = r2r_client_clone.clone();
            let kafka_client = kafka_client_clone.clone();
            let mission_id = input
                .mission_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            Box::pin(async move {
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
    let mcp_client_clone = mcp_client.clone();
    let kafka_client_clone = kafka_client.clone();
    let code_task = hatchet
        .task("zeroclaw-execute", move |input: MissionInput, ctx| {
            let mcp_client = mcp_client_clone.clone();
            let kafka_client = kafka_client_clone.clone();
            let mission_id = input
                .mission_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            Box::pin(async move {
                let plan: serde_json::Value = ctx.parent_output("rustant-plan").await?;
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

    let aethalgard_client: Arc<dyn AethalgardClient> =
        Arc::new(HttpAethalgardClient::new(aethalgard_webhook_url));

    // 3. Validation Phase (ZeroClaw)
    let mcp_client_clone = mcp_client.clone();
    let kafka_client_clone = kafka_client.clone();
    let aethalgard_client_clone = aethalgard_client.clone();
    let validation_task = hatchet
        .task("zeroclaw-validate", move |input: MissionInput, _ctx| {
            let mcp_client = mcp_client_clone.clone();
            let kafka_client = kafka_client_clone.clone();
            let aethalgard_client = aethalgard_client_clone.clone();
            let mission_id = input
                .mission_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            Box::pin(async move {
                let zeroclaw = ZeroClawAgent::new(mcp_client);

                kafka_client
                    .publish_thought(&mission_id, "Starting validation phase...", "zeroclaw")
                    .await?;

                let mut attempt = 0;
                let max_retries = 3;
                let mut last_error;

                loop {
                    attempt += 1;
                    match zeroclaw.validate_mission(&mission_id, "cargo test").await {
                        Ok(test_res) => {
                            if test_res["status"].as_str().unwrap_or("") == "success" {
                                kafka_client
                                    .publish_thought(&mission_id, "Validation passed", "zeroclaw")
                                    .await?;
                                return Ok(test_res);
                            } else {
                                last_error = test_res["error"]
                                    .as_str()
                                    .unwrap_or("Unknown test failure")
                                    .to_string();
                            }
                        }
                        Err(e) => {
                            last_error = e.to_string();
                        }
                    }

                    if attempt >= max_retries {
                        kafka_client
                            .publish_thought(
                                &mission_id,
                                "Validation failed after 3 attempts. Escalating to Aethalgard...",
                                "zeroclaw",
                            )
                            .await?;
                        aethalgard_client
                            .notify_remediation(&mission_id, &last_error)
                            .await?;
                        anyhow::bail!(
                            "Validation failed permanently. Escalated to Jules Remediator."
                        );
                    }

                    kafka_client
                        .publish_thought(
                            &mission_id,
                            &format!("Validation attempt {} failed. Retrying...", attempt),
                            "zeroclaw",
                        )
                        .await?;
                    // Simulate fixing code before retry
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                }
            })
        })
        .build()
        .unwrap()
        .add_parent(&code_task);

    // 4. Review Phase (Rustant)
    let mcp_client_clone = mcp_client.clone();
    let r2r_client_clone = r2r_client.clone();
    let kafka_client_clone = kafka_client.clone();
    let review_task = hatchet
        .task("rustant-review", move |input: MissionInput, ctx| {
            let mcp_client = mcp_client_clone.clone();
            let r2r_client = r2r_client_clone.clone();
            let kafka_client = kafka_client_clone.clone();
            let mission_id = input
                .mission_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            Box::pin(async move {
                let code_res: serde_json::Value = ctx.parent_output("zeroclaw-execute").await?;
                let rustant = RustantAgent::new(mcp_client, r2r_client);

                kafka_client
                    .publish_thought(&mission_id, "Starting security review phase...", "rustant")
                    .await?;
                let review = rustant
                    .review_mission(&mission_id, &code_res.to_string())
                    .await?;
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
    let mcp_client_clone = mcp_client.clone();
    let kafka_client_clone = kafka_client.clone();
    let delivery_task = hatchet
        .task("factory-deliver", move |input: MissionInput, ctx| {
            let mcp_client = mcp_client_clone.clone();
            let kafka_client = kafka_client_clone.clone();
            let mission_id = input
                .mission_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            Box::pin(async move {
                let review_res: serde_json::Value = ctx.parent_output("rustant-review").await?;

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
        .workflow("darkgravitymission")
        .build()
        .unwrap()
        .add_task(&plan_task)
        .add_task(&code_task)
        .add_task(&validation_task)
        .add_task(&review_task)
        .add_task(&delivery_task)
}

#[cfg(test)]
mod tests {
    use super::*;
    use factory_core::proto::v1::MissionInput as ProtoInput;
    use prost::Message;

    #[test]
    fn test_mission_input_from_protobuf() {
        let proto = ProtoInput {
            mission_id: "test-uuid".to_string(),
            epic_title: "Implement Kafka".to_string(),
            epic_description: "Real Kafka Client adapter".to_string(),
            labels: vec!["p0".to_string(), "sprint1".to_string()],
        };

        let mut bytes = Vec::new();
        proto.encode(&mut bytes).unwrap();

        let input = MissionInput::from_protobuf(&bytes).unwrap();
        assert_eq!(input.mission_id, Some("test-uuid".to_string()));
        assert!(input.goal.contains("Implement Kafka"));
        assert!(input.goal.contains("Real Kafka Client adapter"));
        assert!(input.goal.contains("p0"));
    }
}
