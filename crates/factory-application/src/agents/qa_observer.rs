use crate::Agent;
use crate::workflows::autonomous_mission::MissionInput;
use async_trait::async_trait;
use factory_infrastructure::{GitlabClient, HttpGitlabClient, HttpSentryClient, SentryClient};
use hatchet_sdk::{Hatchet, Runnable};
use serde_json::Value;
use std::time::Duration;
use uuid::Uuid;

use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Circuit breaker threshold: after this many consecutive failures,
/// downgrade logging from ERROR to WARN and apply maximum backoff.
const CIRCUIT_BREAKER_THRESHOLD: u32 = 3;

pub struct QAObserverAgent {
    sentry_client: Box<dyn SentryClient>,
    gitlab_client: Box<dyn GitlabClient>,
    r2r_client: Option<Arc<dyn factory_infrastructure::R2rClient>>,
    sentry_project: String,
    gitlab_project: String,
    hatchet: Hatchet,
    processed_events: Arc<Mutex<HashSet<String>>>,
}

impl Default for QAObserverAgent {
    fn default() -> Self {
        panic!("QAObserverAgent should be constructed with ::new()");
    }
}

impl QAObserverAgent {
    pub fn new(
        sentry_url: String,
        sentry_token: String,
        sentry_project: String,
        gitlab_url: String,
        gitlab_token: String,
        gitlab_project: String,
        hatchet: Hatchet,
    ) -> Self {
        let r2r_url = std::env::var("R2R_BASE_URL").unwrap_or_else(|_| "http://r2r.llm-apps.svc.cluster.local:7272".to_string());
        let r2r_user = std::env::var("R2R_USER").unwrap_or_else(|_| "lgcorzo@gmail.com".to_string());
        let r2r_pwd = std::env::var("R2R_PWD").unwrap_or_else(|_| "admin".to_string());

        let r2r_client: Option<Arc<dyn factory_infrastructure::R2rClient>> = Some(Arc::new(
            factory_infrastructure::HttpR2rClient::new(r2r_url, r2r_user, r2r_pwd),
        ));

        Self {
            sentry_client: Box::new(HttpSentryClient::new(sentry_url, sentry_token)),
            gitlab_client: Box::new(HttpGitlabClient::new(gitlab_url, gitlab_token)),
            r2r_client,
            sentry_project,
            gitlab_project,
            hatchet,
            processed_events: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub async fn monitor_crashes(&self) -> anyhow::Result<()> {
        if self.sentry_project.is_empty() || self.gitlab_project.is_empty() {
            tracing::warn!("QAObserverAgent: Missing Sentry/GitLab config, monitoring disabled.");
            return Ok(());
        }

        let base_interval = Duration::from_secs(15 * 60); // 15 minutes
        let max_interval = Duration::from_secs(60 * 60); // 1 hour
        let mut current_interval = base_interval;
        let mut consecutive_failures: u32 = 0;

        loop {
            tracing::info!(
                "QAObserverAgent: Polling Sentry for recent crashes in {}...",
                self.sentry_project
            );

            // Poll for crashes in the last 15 minutes
            match self
                .sentry_client
                .fetch_recent_crashes(&self.sentry_project, 15)
                .await
            {
                Ok(crashes) => {
                    // Reset backoff on successful fetch
                    consecutive_failures = 0;
                    current_interval = base_interval;

                    for crash in crashes {
                        // In-memory deduplication across polling intervals
                        {
                            let mut seen = self.processed_events.lock().await;
                            if seen.contains(&crash.event_id) {
                                tracing::debug!("Skipping already processed Sentry event: {}", crash.event_id);
                                continue;
                            }
                            seen.insert(crash.event_id.clone());
                        }

                        tracing::info!("Detected crash: {} - {}", crash.event_id, crash.message);

                        // Contextual R2R GraphRAG Mapping
                        let mut ast_context = "No AST context available".to_string();
                        if let Some(ref r2r) = self.r2r_client {
                            let query = format!("{} {:?}", crash.message, crash.culprit);
                            if let Ok(ctx) = r2r.map_stacktrace_to_ast(&query).await {
                                ast_context = ctx;
                            }
                        }

                        // Create GitLab issue with auto-backlog labels
                        let title = format!("Crash Auto-Report: {}", crash.message);
                        let description = format!(
                            "### Sentry Crash Incident Report\n\n**Event ID:** `{}`\n**Level:** `{}`\n**Message:** `{}`\n**Culprit:** `{:?}`\n\n### R2R GraphRAG AST Context\n```\n{}\n```\n\n[RESOURCE_LIMIT: RAM <= 30Mi]",
                            crash.event_id, crash.level, crash.message, crash.culprit, ast_context
                        );

                        let labels = vec![
                            "autonomous-plan".to_string(),
                            "bug".to_string(),
                            "p0-hotfix".to_string(),
                        ];

                        match self
                            .gitlab_client
                            .create_issue_with_labels(&self.gitlab_project, &title, &description, &labels)
                            .await
                        {
                            Ok(issue) => {
                                tracing::info!("Created GitLab issue for crash: {}", issue.web_url);

                                // Trigger AutonomousMission Hotfix via Hatchet
                                let mission_input = MissionInput {
                                    mission_id: Some(Uuid::new_v4().to_string()),
                                    goal: format!(
                                        "Hotfix Crash: {}.\nAST Context:\n{}\nGitLab Issue: {}",
                                        title, ast_context, issue.web_url
                                    ),
                                    repository_path: String::new(),
                                };

                                let workflow = self.hatchet.workflow::<MissionInput, crate::workflows::autonomous_mission::MissionOutput>("AutonomousMission").build().unwrap();

                                if let Err(e) = workflow.run_no_wait(&mission_input, None).await {
                                    tracing::error!("Failed to trigger Hatchet mission: {}", e);
                                } else {
                                    tracing::info!(
                                        "Successfully triggered autonomous hotfix mission."
                                    );
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to create GitLab issue for crash: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    consecutive_failures += 1;
                    if consecutive_failures >= CIRCUIT_BREAKER_THRESHOLD {
                        // Circuit breaker open: downgrade to WARN to avoid log flooding
                        tracing::warn!(
                            "QAObserverAgent: Sentry unreachable ({} consecutive failures, backoff {}s): {}",
                            consecutive_failures,
                            current_interval.as_secs(),
                            e
                        );
                    } else {
                        tracing::error!("Failed to fetch Sentry crashes: {}", e);
                    }
                    // Exponential backoff: double interval on each failure, capped at max
                    current_interval = (current_interval * 2).min(max_interval);
                }
            }

            // Wait before polling again (adaptive interval with backoff)
            tokio::time::sleep(current_interval).await;
        }
    }
}

#[async_trait]
impl Agent for QAObserverAgent {
    fn name(&self) -> String {
        "QAObserverAgent".to_string()
    }

    async fn execute(&self, _task_description: &str) -> anyhow::Result<Value> {
        self.monitor_crashes().await?;
        Ok(serde_json::json!({ "status": "qa_monitoring_stopped" }))
    }
}

#[cfg(test)]
mod tests {
    // Testing the daemon loop is complex without mocking Hatchet client properly.
    // Unit tests will be limited to basic initialization.
}
