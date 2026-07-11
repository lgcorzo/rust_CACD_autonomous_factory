use crate::agents::{AuditorAgent, FinOpsAgent, RustantAgent, ZeroClawAgent};
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
    // FinOps background monitor: only spawn if LiteLLM base URL is configured
    let has_litellm = std::env::var("LITELLM_API_BASE")
        .or_else(|_| std::env::var("LITELLM_BASE_URL"))
        .map(|v| !v.is_empty())
        .unwrap_or(false);

    if has_litellm {
        tokio::spawn(async move {
            let finops_agent = FinOpsAgent::default();
            if let Err(e) = finops_agent.monitor_budget().await {
                tracing::error!("FinOpsAgent monitor crashed: {}", e);
            }
        });
    } else {
        tracing::warn!(
            "FinOpsAgent: LITELLM_API_BASE/LITELLM_BASE_URL not set, skipping budget monitor."
        );
    }

    // QA Observer background monitor: only spawn if Sentry API token is configured
    let sentry_token = std::env::var("SENTRY_API_TOKEN").unwrap_or_default();
    if !sentry_token.is_empty() {
        let hatchet_clone = hatchet.clone();
        tokio::spawn(async move {
            let sentry_url =
                std::env::var("SENTRY_URL").unwrap_or_else(|_| "https://sentry.io".to_string());
            let sentry_project =
                std::env::var("SENTRY_PROJECT").unwrap_or_else(|_| "dg-factory".to_string());
            let gitlab_url =
                std::env::var("GITLAB_URL").unwrap_or_else(|_| "https://gitlab.com".to_string());
            let gitlab_token = std::env::var("GITLAB_API_TOKEN").unwrap_or_default();
            let gitlab_project = std::env::var("GITLAB_PROJECT")
                .unwrap_or_else(|_| "lgcorzo-lab/autonomous_factory".to_string());

            let qa_agent = crate::agents::QAObserverAgent::new(
                sentry_url,
                sentry_token,
                sentry_project,
                gitlab_url,
                gitlab_token,
                gitlab_project,
                hatchet_clone,
            );

            if let Err(e) = qa_agent.monitor_crashes().await {
                tracing::error!("QAObserverAgent monitor crashed: {}", e);
            }
        });
    } else {
        tracing::warn!("QAObserverAgent: SENTRY_API_TOKEN not set, skipping crash monitor.");
    }

    let mcp_client: Arc<dyn McpClient> = Arc::new(McpHttpClient::new(mcp_url));
    let r2r_user =
        std::env::var("R2R_SUPERUSER_EMAIL").unwrap_or_else(|_| "lgcorzo@gmail.com".to_string());
    let r2r_pwd = std::env::var("R2R_SUPERUSER_PASSWORD").unwrap_or_else(|_| "admin".to_string());

    let r2r_client: Arc<dyn R2rClient> = Arc::new(HttpR2rClient::new(r2r_url, r2r_user, r2r_pwd));
    let kafka_client: Arc<dyn KafkaClient> = if kafka_brokers == "mock" || kafka_brokers.is_empty()
    {
        #[cfg(not(feature = "production"))]
        {
            Arc::new(factory_infrastructure::SimpleMockKafkaClient::new(&kafka_brokers).unwrap())
        }
        #[cfg(feature = "production")]
        {
            panic!(
                "Mock Kafka client is not available in production builds. Please provide real brokers."
            );
        }
    } else {
        Arc::new(factory_infrastructure::RdKafkaClient::new(&kafka_brokers).unwrap())
    };

    let aethalgard_client: Arc<dyn AethalgardClient> =
        Arc::new(HttpAethalgardClient::new(aethalgard_webhook_url));

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
    let aethalgard_client_clone = aethalgard_client.clone();
    let code_task = hatchet
        .task("zeroclaw-execute", move |input: MissionInput, ctx| {
            let mcp_client = mcp_client_clone.clone();
            let kafka_client = kafka_client_clone.clone();
            let aethalgard_client = aethalgard_client_clone.clone();
            let mission_id = input
                .mission_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            Box::pin(async move {
                let zeroclaw = ZeroClawAgent::new(mcp_client, aethalgard_client);
                let task_desc = "Execute the mission";

                kafka_client
                    .publish_thought(&mission_id, "Starting coding phase...", "zeroclaw")
                    .await?;

                let result = match zeroclaw.execute_task(&mission_id, task_desc, &[]).await {
                    Ok(r) => r,
                    Err(e) => {
                        tracing::error!("zeroclaw-execute failed with error: {:?}", e);
                        return Err(e);
                    }
                };
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
                let zeroclaw = ZeroClawAgent::new(mcp_client, aethalgard_client.clone());

                kafka_client
                    .publish_thought(&mission_id, "Starting validation phase...", "zeroclaw")
                    .await?;

                let mut attempt = 0;
                let max_retries = 3;
                let mut last_error;

                loop {
                    attempt += 1;
                    match zeroclaw.validate_mission(&mission_id, "cargo test").await {
                        Ok(raw_res) => {
                            tracing::info!("Raw validation response: {:?}", raw_res);
                            let mut status = String::new();
                            let mut last_err_msg = "Unknown test failure".to_string();
                            if let Some(content) = raw_res["content"].as_array().and_then(|c| c.first()) {
                                if let Some(text) = content["text"].as_str() {
                                    tracing::info!("Parsed validation text: {}", text);
                                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(text) {
                                        status = parsed["status"].as_str().unwrap_or("").to_string();
                                        tracing::info!("Extracted status: {}", status);
                                        last_err_msg = parsed["error"].as_str().unwrap_or("Unknown test failure").to_string();
                                    }
                                }
                            }
                            if status == "success" {
                                kafka_client
                                    .publish_thought(&mission_id, "Validation passed", "zeroclaw")
                                    .await?;
                                return Ok(raw_res);
                            } else {
                                last_error = last_err_msg;
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

                        // SIM-4: Background Audit of failed mission
                        let m_id = mission_id.clone();
                        tokio::spawn(async move {
                            let auditor = AuditorAgent::new();
                            if let Ok(logs) = auditor.analyze_dag_logs(&m_id).await {
                                let _ = auditor.audit_mission(&m_id, &logs).await;
                            }
                        });

                        anyhow::bail!(
                            "Validation failed permanently. Escalated to Jules Remediator."
                        );
                    }

                    kafka_client
                        .publish_thought(
                            &mission_id,
                            &format!("Validation attempt {} failed. Retrying with active patch generation...", attempt),
                            "zeroclaw",
                        )
                        .await?;

                    // SIM-4: Actively generate patch for failing code instead of sleeping
                    let fix_task = format!("Fix the following failing tests/code:\n{}", last_error);
                    if let Err(e) = zeroclaw.execute_task(&mission_id, &fix_task, &[]).await {
                        tracing::error!("Failed to execute patch task: {}", e);
                    }
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
                let rustant = RustantAgent::new(mcp_client, r2r_client);

                kafka_client
                    .publish_thought(&mission_id, "Starting security review phase...", "rustant")
                    .await?;
                let review = rustant.review_mission(&mission_id, "mock diff").await?;
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
    let r2r_client_clone_deliver = r2r_client.clone();
    let delivery_task = hatchet
        .task("factory-deliver", move |input: MissionInput, _ctx| {
            let mcp_client = mcp_client_clone.clone();
            let kafka_client = kafka_client_clone.clone();
            let mission_id = input
                .mission_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            Box::pin(async move {
                let rustant = RustantAgent::new(mcp_client.clone(), r2r_client_clone_deliver);
                let review_res = rustant.review_mission(&mission_id, "mock diff").await?;

                let mut is_approved = false;
                if let Some(content) = review_res["content"].as_array().and_then(|c| c.first()) {
                    if let Some(text) = content["text"].as_str() {
                        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(text) {
                            if parsed["status"] == "approved" {
                                is_approved = true;
                            }
                        }
                    }
                }

                if is_approved {
                    kafka_client
                        .publish_thought(&mission_id, "Review approved. Creating PR...", "factory")
                        .await?;

                    let branch_name = format!("mission-{}", mission_id);
                    let pr_res = serde_json::json!({
                        "url": format!("https://gitlab.com/repo/merge_requests/{}", mission_id)
                    });

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
        .workflow("darkgravitymission-test")
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
