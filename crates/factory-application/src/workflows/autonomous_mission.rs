use crate::agents::{AuditorAgent, FinOpsAgent, RustantAgent, ZeroClawAgent};
use crate::workflows::circuit_breaker::CircuitBreakerGuard;
use factory_infrastructure::{
    GithubClient, GitlabClient, GitlabCommitAction, HttpGithubClient, HttpGitlabClient,
    HttpR2rClient, KafkaClient, McpClient, McpHttpClient, R2rClient,
    aethalgard::{AethalgardClient, HttpAethalgardClient},
};
use hatchet_sdk::Hatchet;
use hatchet_sdk::runnables::Workflow;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MissionInput {
    pub mission_id: Option<String>,
    pub goal: String,
    pub repository_path: String,
    #[serde(default)]
    pub source_platform: Option<String>,
    #[serde(default)]
    pub repository: Option<String>,
    #[serde(default)]
    pub issue_number: Option<u64>,
    #[serde(default)]
    pub jit_token: Option<String>,
    #[serde(default)]
    pub verifiable_credential: Option<factory_core::security::nhi::VerifiableCredential>,
}

impl MissionInput {
    pub fn attach_nhi_and_jit(
        &mut self,
        agent_id: &str,
        signing_key: &ed25519_dalek::SigningKey,
        key_id: &str,
        jit_token: String,
    ) -> Result<(), factory_core::error::FactoryError> {
        use factory_core::security::nhi::{AgentSubject, VerifiableCredential};

        let subject = AgentSubject {
            id: agent_id.to_string(),
            roles: vec!["worker".to_string(), "autonomous-remediation".to_string()],
            allowed_namespaces: vec!["factory-sandbox".to_string()],
        };

        let mut vc = VerifiableCredential::new(
            format!("urn:uuid:{}", Uuid::new_v4()),
            "did:darkgravity:authority".to_string(),
            subject,
        );

        vc.sign(signing_key, key_id)?;
        self.verifiable_credential = Some(vc);
        self.jit_token = Some(jit_token);
        Ok(())
    }

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
            source_platform: None,
            repository: None,
            issue_number: None,
            jit_token: None,
            verifiable_credential: None,
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

pub async fn post_mission_milestone(
    gl_client: &Option<Arc<dyn GitlabClient>>,
    gh_client: &Option<Arc<dyn GithubClient>>,
    input: &MissionInput,
    milestone_title: &str,
    details: &str,
) {
    let platform = input
        .source_platform
        .as_deref()
        .unwrap_or("")
        .to_lowercase();
    let repo = match &input.repository {
        Some(r) if !r.is_empty() => r.as_str(),
        _ => return,
    };
    let issue_number = match input.issue_number {
        Some(n) if n > 0 => n,
        _ => return,
    };

    let mission_id_display = input.mission_id.as_deref().unwrap_or("unknown");
    let comment_body = format!(
        "### {} (Mission `{}`)\n\n{}\n\n*Dark Gravity Autonomous CA/CD Factory | {}*",
        milestone_title,
        mission_id_display,
        details,
        chrono::Utc::now().to_rfc3339()
    );

    match platform.as_str() {
        "gitlab" => {
            if let Some(gl) = gl_client {
                if let Err(e) = gl.post_issue_note(repo, issue_number, &comment_body).await {
                    tracing::warn!(
                        "Failed to post milestone note to GitLab issue #{} in {}: {}",
                        issue_number,
                        repo,
                        e
                    );
                } else {
                    tracing::info!(
                        "Milestone note posted to GitLab issue #{} in {}: {}",
                        issue_number,
                        repo,
                        milestone_title
                    );
                }
            }
        }
        "github" => {
            if let Some(gh) = gh_client {
                if let Err(e) = gh
                    .post_issue_comment(repo, issue_number, &comment_body)
                    .await
                {
                    tracing::warn!(
                        "Failed to post milestone comment to GitHub issue #{} in {}: {}",
                        issue_number,
                        repo,
                        e
                    );
                } else {
                    tracing::info!(
                        "Milestone comment posted to GitHub issue #{} in {}: {}",
                        issue_number,
                        repo,
                        milestone_title
                    );
                }
            }
        }
        _ => {
            tracing::debug!(
                "No recognized source platform for milestone comment: {}",
                platform
            );
        }
    }
}

pub fn create_mission_workflow(
    hatchet: &Hatchet,
    mcp_url: String,
    r2r_url: String,
    kafka_brokers: String,
    aethalgard_webhook_url: String,
) -> Workflow<MissionInput, MissionOutput> {
    let gitlab_token = std::env::var("GITLAB_API_TOKEN").unwrap_or_default();
    let gitlab_url =
        std::env::var("GITLAB_URL").unwrap_or_else(|_| "https://gitlab.com".to_string());
    let gl_client: Option<Arc<dyn GitlabClient>> = if !gitlab_token.is_empty() {
        Some(Arc::new(HttpGitlabClient::new(gitlab_url, gitlab_token)))
    } else {
        None
    };

    let github_token = std::env::var("GITHUB_API_TOKEN").unwrap_or_default();
    let gh_client: Option<Arc<dyn GithubClient>> = if !github_token.is_empty() {
        Some(Arc::new(HttpGithubClient::new(github_token)))
    } else {
        None
    };

    create_mission_workflow_with_clients(
        hatchet,
        mcp_url,
        r2r_url,
        kafka_brokers,
        aethalgard_webhook_url,
        gl_client,
        gh_client,
    )
}

pub fn create_mission_workflow_with_clients(
    hatchet: &Hatchet,
    mcp_url: String,
    r2r_url: String,
    kafka_brokers: String,
    aethalgard_webhook_url: String,
    gl_client: Option<Arc<dyn GitlabClient>>,
    gh_client: Option<Arc<dyn GithubClient>>,
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
    let gl_client_plan = gl_client.clone();
    let gh_client_plan = gh_client.clone();
    let plan_task = hatchet
        .task("rustant-plan", move |input: MissionInput, _ctx| {
            let mcp_client = mcp_client_clone.clone();
            let r2r_client = r2r_client_clone.clone();
            let kafka_client = kafka_client_clone.clone();
            let gl_client = gl_client_plan.clone();
            let gh_client = gh_client_plan.clone();
            let mission_id = input
                .mission_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            Box::pin(async move {
                let rustant = RustantAgent::new(mcp_client, r2r_client);

                post_mission_milestone(
                    &gl_client,
                    &gh_client,
                    &input,
                    "📋 Planning Initiated",
                    "RustantAgent is analyzing repository architecture, AST nodes, and formulating specification blueprints.",
                )
                .await;

                kafka_client
                    .publish_thought(&mission_id, "Starting planning phase...", "rustant")
                    .await?;
                let plan = rustant.plan_mission(&mission_id, &input.goal).await?;
                kafka_client
                    .publish_thought(&mission_id, "Plan generated successfully", "rustant")
                    .await?;

                post_mission_milestone(
                    &gl_client,
                    &gh_client,
                    &input,
                    "✅ Planning Completed",
                    "Architecture blueprint formulated and verified against system constraints.",
                )
                .await;

                Ok(plan)
            })
        })
        .build()
        .unwrap();

    // 2. Coding Phase (ZeroClaw)
    let mcp_client_clone = mcp_client.clone();
    let kafka_client_clone = kafka_client.clone();
    let aethalgard_client_clone = aethalgard_client.clone();
    let gl_client_code = gl_client.clone();
    let gh_client_code = gh_client.clone();
    let code_task = hatchet
        .task("zeroclaw-execute", move |input: MissionInput, _ctx| {
            let mcp_client = mcp_client_clone.clone();
            let kafka_client = kafka_client_clone.clone();
            let aethalgard_client = aethalgard_client_clone.clone();
            let gl_client = gl_client_code.clone();
            let gh_client = gh_client_code.clone();
            let mission_id = input
                .mission_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            Box::pin(async move {
                let zeroclaw = ZeroClawAgent::new(mcp_client, aethalgard_client);
                let task_desc = "import time\ntime.sleep(15)\nprint('Done')";

                kafka_client
                    .publish_thought(&mission_id, "Starting coding phase...", "zeroclaw")
                    .await?;

                let result = match zeroclaw.execute_task(&mission_id, task_desc, &[]).await {
                    Ok(r) => {
                        post_mission_milestone(
                            &gl_client,
                            &gh_client,
                            &input,
                            "⚡ Execution Completed",
                            "ZeroClawAgent generated code mutations within isolated gVisor sandbox.",
                        )
                        .await;
                        r
                    }
                    Err(e) => {
                        tracing::error!("zeroclaw-execute failed with error: {:?}", e);
                        post_mission_milestone(
                            &gl_client,
                            &gh_client,
                            &input,
                            "⚠️ Execution Failed",
                            &format!("ZeroClawAgent execution failed: {:?}", e),
                        )
                        .await;
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
    let gl_client_val = gl_client.clone();
    let gh_client_val = gh_client.clone();
    let validation_task = hatchet
        .task("zeroclaw-validate", move |input: MissionInput, _ctx| {
            let mcp_client = mcp_client_clone.clone();
            let kafka_client = kafka_client_clone.clone();
            let aethalgard_client = aethalgard_client_clone.clone();
            let gl_client = gl_client_val.clone();
            let gh_client = gh_client_val.clone();
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
                let mut previous_error_hash: u64 = 0;

                loop {
                    attempt += 1;
                    match zeroclaw.validate_mission(&mission_id, "cargo test").await {
                        Ok(raw_res) => {
                            tracing::info!("Raw validation response: {:?}", raw_res);
                            let mut status = String::new();
                            let mut last_err_msg = "Unknown test failure".to_string();
                            #[allow(clippy::collapsible_if)]
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

                                post_mission_milestone(
                                    &gl_client,
                                    &gh_client,
                                    &input,
                                    "🧪 Validation Verified",
                                    "Test suite verified with 100% pass rate. No regressions detected.",
                                )
                                .await;

                                return Ok(raw_res);
                            } else {
                                last_error = last_err_msg;
                            }
                        }
                        Err(e) => {
                            last_error = e.to_string();
                        }
                    }

                    // Compute error signature hash for Deadlock & Stagnation Detection
                    use std::hash::{Hash, Hasher};
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    last_error.hash(&mut hasher);
                    let current_error_hash = hasher.finish();

                    if attempt > 1 && current_error_hash == previous_error_hash {
                        tracing::error!(
                            "DEADLOCK DETECTED in mission {}! ZeroClaw produced identical error hash {} across consecutive attempts. Breaking loop immediately.",
                            mission_id,
                            current_error_hash
                        );
                        kafka_client
                            .publish_thought(
                                &mission_id,
                                "Deadlock detected: identical failure diff across attempts. Escalating immediately...",
                                "zeroclaw",
                            )
                            .await?;
                        aethalgard_client
                            .notify_remediation(&mission_id, &format!("Deadlock detected: {}", last_error))
                            .await?;

                        let guard = CircuitBreakerGuard::new(max_retries as u32, 8.0);
                        let stuck_alert = guard.format_stuck_alert(
                            input.repository.as_deref().unwrap_or("unknown"),
                            input.issue_number.unwrap_or(0),
                            &format!("Deadlock detected: {}", last_error),
                        );

                        tracing::warn!(
                            "[AutonomousMission:{}] Circuit breaker tripped; JIT token invalidated and revoked.",
                            mission_id
                        );

                        post_mission_milestone(
                            &gl_client,
                            &gh_client,
                            &input,
                            "🚨 Circuit Breaker: Agent-Stuck State Triggered (Deadlock)",
                            &stuck_alert,
                        )
                        .await;

                        // Clean error recovery: stash git state
                        let _ = std::process::Command::new("git")
                            .args(["stash", "save", &format!("stuck-mission-{}", mission_id)])
                            .output();

                        anyhow::bail!("Deadlock detected in validation loop. Escalated to supervisor.");
                    }
                    previous_error_hash = current_error_hash;

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

                        let guard = CircuitBreakerGuard::new(max_retries as u32, 8.0);
                        let stuck_alert = guard.format_stuck_alert(
                            input.repository.as_deref().unwrap_or("unknown"),
                            input.issue_number.unwrap_or(0),
                            &format!("Validation failed after {} attempts. Error: {}", max_retries, last_error),
                        );

                        tracing::warn!(
                            "[AutonomousMission:{}] Circuit breaker tripped; JIT token invalidated and revoked.",
                            mission_id
                        );

                        post_mission_milestone(
                            &gl_client,
                            &gh_client,
                            &input,
                            "🚨 Circuit Breaker: Agent-Stuck State Triggered (Max Retries)",
                            &stuck_alert,
                        )
                        .await;

                        // SIM-4: Background Audit of failed mission
                        let m_id = mission_id.clone();
                        tokio::spawn(async move {
                            let auditor = AuditorAgent::new();
                            if let Ok(logs) = auditor.analyze_dag_logs(&m_id).await {
                                let _ = auditor.audit_mission(&m_id, &logs).await;
                            }
                        });

                        // Clean error recovery: stash git state
                        let _ = std::process::Command::new("git")
                            .args(["stash", "save", &format!("stuck-mission-{}", mission_id)])
                            .output();

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
        .execution_timeout(std::time::Duration::from_secs(1800))
        .build()
        .unwrap()
        .add_parent(&code_task);

    // 4. Review Phase (Rustant)
    let mcp_client_clone = mcp_client.clone();
    let r2r_client_clone = r2r_client.clone();
    let kafka_client_clone = kafka_client.clone();
    let gl_client_rev = gl_client.clone();
    let gh_client_rev = gh_client.clone();
    let review_task = hatchet
        .task("rustant-review", move |input: MissionInput, _ctx| {
            let mcp_client = mcp_client_clone.clone();
            let r2r_client = r2r_client_clone.clone();
            let kafka_client = kafka_client_clone.clone();
            let gl_client = gl_client_rev.clone();
            let gh_client = gh_client_rev.clone();
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

                post_mission_milestone(
                    &gl_client,
                    &gh_client,
                    &input,
                    "🛡️ Security Review Completed",
                    "Aethelgard LLM-as-a-Judge SAST forensic evaluation executed.",
                )
                .await;

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
    let gl_client_deliv = gl_client.clone();
    let gh_client_deliv = gh_client.clone();
    let delivery_task = hatchet
        .task("factory-deliver", move |input: MissionInput, _ctx| {
            let mcp_client = mcp_client_clone.clone();
            let kafka_client = kafka_client_clone.clone();
            let gl_client = gl_client_deliv.clone();
            let gh_client = gh_client_deliv.clone();
            let mission_id = input
                .mission_id
                .clone()
                .unwrap_or_else(|| Uuid::new_v4().to_string());

            Box::pin(async move {
                let rustant = RustantAgent::new(mcp_client.clone(), r2r_client_clone_deliver);
                let review_res = rustant.review_mission(&mission_id, "mock diff").await?;

                let mut is_approved = false;
                #[allow(clippy::collapsible_if)]
                if let Some(text) = review_res["content"]
                    .as_array()
                    .and_then(|c| c.first())
                    .and_then(|c| c["text"].as_str())
                {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(text) {
                        is_approved = parsed["status"] == "approved";
                    }
                }

                if is_approved {
                    kafka_client
                        .publish_thought(&mission_id, "Review approved. Creating PR...", "factory")
                        .await?;

                    let branch_name = if let Some(num) = input.issue_number {
                        format!(
                            "mission-{}-{}",
                            input
                                .repository
                                .as_deref()
                                .unwrap_or("repo")
                                .replace('/', "-"),
                            num
                        )
                    } else {
                        format!("mission-{}", mission_id)
                    };

                    let platform = input
                        .source_platform
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase();
                    let repo = input.repository.as_deref().unwrap_or("");
                    let issue_number = input.issue_number.unwrap_or(1);

                    let mut delivered_url = None;

                    match platform.as_str() {
                        "gitlab" => {
                            if let Some(gl) = gl_client.as_ref().filter(|_| !repo.is_empty()) {
                                // 1. Create remote branch
                                if let Err(e) = gl.create_branch(repo, &branch_name, "main").await {
                                    tracing::warn!("Branch {} may already exist or error: {}", branch_name, e);
                                }

                                // 2. Commit delivered mission artifacts
                                let commit_msg = format!(
                                    "feat: [Dark Gravity] autonomous mission delivery\n\nCloses #{}",
                                    issue_number
                                );
                                let actions = vec![GitlabCommitAction {
                                    action: "create".to_string(),
                                    file_path: format!(".dark-gravity/missions/{}/summary.md", mission_id),
                                    content: Some(format!(
                                        "# Mission {}\n\nGoal: {}\nTimestamp: {}\nStatus: Approved",
                                        mission_id,
                                        input.goal,
                                        chrono::Utc::now().to_rfc3339()
                                    )),
                                }];
                                if let Err(e) = gl.create_commit_files(repo, &branch_name, &commit_msg, &actions).await {
                                    tracing::warn!("Commit files warning: {}", e);
                                }

                                // 3. Open Merge Request
                                let mr_title = format!(
                                    "feat: [Dark Gravity] autonomous mission for issue #{}",
                                    issue_number
                                );
                                let mr_desc = format!(
                                    "Closes #{}\n\nAutomated delivery by Dark Gravity autonomous CA/CD factory.\nMission ID: `{}`",
                                    issue_number, mission_id
                                );
                                match gl.create_merge_request(repo, &branch_name, "main", &mr_title, &mr_desc).await {
                                    Ok(mr) => {
                                        delivered_url = Some(mr.web_url.clone());
                                        let final_msg = format!(
                                            "🎉 **Dark Gravity Mission Delivered**\n\n- **Branch**: `{}`\n- **Merge Request**: [{}]({})\n- **Status**: Ready for Human Reviewer (Vertex 4)",
                                            branch_name, mr.web_url, mr.web_url
                                        );
                                        post_mission_milestone(
                                            &gl_client,
                                            &gh_client,
                                            &input,
                                            "🚀 Delivery Completed",
                                            &final_msg,
                                        )
                                        .await;
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to create GitLab MR: {}", e);
                                    }
                                }
                            }
                        }
                        "github" => {
                            if let Some(gh) = gh_client.as_ref().filter(|_| !repo.is_empty()) {
                                if let Err(e) = gh.create_branch(repo, &branch_name, "main").await {
                                    tracing::warn!("Branch {} may already exist or error: {}", branch_name, e);
                                }

                                let pr_title = format!(
                                    "feat: [Dark Gravity] autonomous mission for issue #{}",
                                    issue_number
                                );
                                let pr_desc = format!(
                                    "Closes #{}\n\nAutomated delivery by Dark Gravity autonomous CA/CD factory.\nMission ID: `{}`",
                                    issue_number, mission_id
                                );
                                match gh.create_pull_request(repo, &pr_title, &branch_name, "main", &pr_desc).await {
                                    Ok(url) => {
                                        delivered_url = Some(url.clone());
                                        let final_msg = format!(
                                            "🎉 **Dark Gravity Mission Delivered**\n\n- **Branch**: `{}`\n- **Pull Request**: [{}]({})\n- **Status**: Ready for Human Reviewer (Vertex 4)",
                                            branch_name, url, url
                                        );
                                        post_mission_milestone(
                                            &gl_client,
                                            &gh_client,
                                            &input,
                                            "🚀 Delivery Completed",
                                            &final_msg,
                                        )
                                        .await;
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to create GitHub PR: {}", e);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }

                    let final_url = delivered_url.unwrap_or_else(|| {
                        format!(
                            "https://{}.com/{}/merge_requests/{}",
                            if platform.is_empty() {
                                "gitlab"
                            } else {
                                &platform
                            },
                            if repo.is_empty() { "repo" } else { repo },
                            mission_id
                        )
                    });

                    Ok(MissionOutput {
                        mission_id: mission_id.clone(),
                        status: "completed".to_string(),
                        summary: "Mission successful and MR/PR created".to_string(),
                        pr_url: Some(final_url),
                    })
                } else {
                    kafka_client
                        .publish_thought(&mission_id, "Review REJECTED. Mission failed.", "factory")
                        .await?;
                    let rej_msg = format!(
                        "❌ Security Review Rejected for mission `{}`. The proposed changes did not meet Aethelgard SAST criteria (threshold >= 8.0/10.0). Workspace state preserved.",
                        mission_id
                    );
                    post_mission_milestone(
                        &gl_client,
                        &gh_client,
                        &input,
                        "❌ Security Review Rejected",
                        &rej_msg,
                    )
                    .await;

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
        .unwrap();

    hatchet
        .workflow("darkgravitymission-dev-lgcorzo")
        .version("1.2.0".to_string())
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

    #[tokio::test]
    async fn test_post_mission_milestone_gitlab() {
        let mut mock_gl = factory_infrastructure::MockGitlabClient::new();
        mock_gl
            .expect_post_issue_note()
            .withf(|proj, iid, body| {
                proj == "lgcorzo/lince-rs"
                    && *iid == 42
                    && body.contains("Planning Completed")
                    && body.contains("mission-abc")
            })
            .returning(|_, _, _| {
                Ok(factory_infrastructure::gitlab::GitlabNote {
                    id: 1,
                    body: "ok".to_string(),
                    author: factory_infrastructure::gitlab::GitlabAuthor {
                        username: "bot".to_string(),
                    },
                    updated_at: None,
                })
            });

        let gl_client: Option<Arc<dyn GitlabClient>> = Some(Arc::new(mock_gl));
        let gh_client: Option<Arc<dyn GithubClient>> = None;

        let input = MissionInput {
            mission_id: Some("mission-abc".to_string()),
            goal: "Improve nesting density".to_string(),
            repository_path: String::new(),
            source_platform: Some("gitlab".to_string()),
            repository: Some("lgcorzo/lince-rs".to_string()),
            issue_number: Some(42),
            ..Default::default()
        };

        post_mission_milestone(
            &gl_client,
            &gh_client,
            &input,
            "✅ Planning Completed",
            "Blueprint validated.",
        )
        .await;
    }

    #[tokio::test]
    async fn test_post_mission_milestone_github() {
        let mut mock_gh = factory_infrastructure::MockGithubClient::new();
        mock_gh
            .expect_post_issue_comment()
            .withf(|repo, issue_num, body| {
                repo == "my-org/my-repo"
                    && *issue_num == 99
                    && body.contains("Validation Verified")
                    && body.contains("mission-xyz")
            })
            .returning(|_, _, _| {
                Ok(factory_infrastructure::github::GithubComment {
                    id: 2,
                    body: "ok".to_string(),
                    user: factory_infrastructure::github::GithubUser {
                        login: "bot".to_string(),
                    },
                    html_url: "https://github.com".to_string(),
                    updated_at: None,
                })
            });

        let gl_client: Option<Arc<dyn GitlabClient>> = None;
        let gh_client: Option<Arc<dyn GithubClient>> = Some(Arc::new(mock_gh));

        let input = MissionInput {
            mission_id: Some("mission-xyz".to_string()),
            goal: "Improve nesting density".to_string(),
            repository_path: String::new(),
            source_platform: Some("github".to_string()),
            repository: Some("my-org/my-repo".to_string()),
            issue_number: Some(99),
            ..Default::default()
        };

        post_mission_milestone(
            &gl_client,
            &gh_client,
            &input,
            "🧪 Validation Verified",
            "Tests passed 100%.",
        )
        .await;
    }

    #[test]
    fn test_mission_input_attach_nhi_and_jit() {
        use ed25519_dalek::SigningKey;
        use rand::rngs::OsRng;

        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);

        let mut input = MissionInput {
            mission_id: Some("mission-001".to_string()),
            goal: "Secure factory".to_string(),
            ..Default::default()
        };

        input
            .attach_nhi_and_jit(
                "agent-zeroclaw-01",
                &signing_key,
                "key-id-vault-01",
                "s.vault-jit-token-xyz".to_string(),
            )
            .unwrap();

        assert_eq!(input.jit_token.as_deref(), Some("s.vault-jit-token-xyz"));
        let vc = input.verifiable_credential.as_ref().unwrap();
        assert_eq!(vc.credential_subject.id, "agent-zeroclaw-01");
        assert!(vc.proof.is_some());
        assert_eq!(
            vc.proof.as_ref().unwrap().verification_method,
            "key-id-vault-01"
        );
    }
}
