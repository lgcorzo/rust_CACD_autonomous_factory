use crate::Agent;
use crate::workflows::autonomous_mission::MissionInput;
use async_trait::async_trait;
use factory_infrastructure::{GitlabClient, HttpGitlabClient, HttpSentryClient, SentryClient};
use hatchet_sdk::{Hatchet, Runnable};
use serde_json::Value;
use std::time::Duration;
use uuid::Uuid;

pub struct QAObserverAgent {
    sentry_client: Box<dyn SentryClient>,
    gitlab_client: Box<dyn GitlabClient>,
    sentry_project: String,
    gitlab_project: String,
    hatchet: Hatchet,
}

impl Default for QAObserverAgent {
    fn default() -> Self {
        // We will construct this using the environment variables if provided.
        // It requires a Hatchet client instance.
        // For default initialization, we will initialize a mock Hatchet if possible,
        // but typically this should be constructed explicitly.
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
        Self {
            sentry_client: Box::new(HttpSentryClient::new(sentry_url, sentry_token)),
            gitlab_client: Box::new(HttpGitlabClient::new(gitlab_url, gitlab_token)),
            sentry_project,
            gitlab_project,
            hatchet,
        }
    }

    pub async fn monitor_crashes(&self) -> anyhow::Result<()> {
        if self.sentry_project.is_empty() || self.gitlab_project.is_empty() {
            tracing::warn!("QAObserverAgent: Missing Sentry/GitLab config, monitoring disabled.");
            return Ok(());
        }

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
                    for crash in crashes {
                        tracing::info!("Detected crash: {} - {}", crash.event_id, crash.message);

                        // Create GitLab issue
                        let title = format!("Crash Auto-Report: {}", crash.message);
                        let description = format!(
                            "A crash was detected by Sentry.\n\nEvent ID: {}\nLevel: {}\nMessage: {}\nCulprit: {:?}\n\n[RESOURCE_LIMIT: RAM <= 30Mi]",
                            crash.event_id, crash.level, crash.message, crash.culprit
                        );

                        match self
                            .gitlab_client
                            .create_issue(&self.gitlab_project, &title, &description)
                            .await
                        {
                            Ok(issue) => {
                                tracing::info!("Created GitLab issue for crash: {}", issue.web_url);

                                // Trigger AutonomousMission Hotfix via Hatchet
                                let mission_input = MissionInput {
                                    mission_id: Some(Uuid::new_v4().to_string()),
                                    goal: format!(
                                        "Hotfix Crash: {}. Context: {}",
                                        title, issue.web_url
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
                    tracing::error!("Failed to fetch Sentry crashes: {}", e);
                }
            }

            // Wait 15 minutes before polling again
            tokio::time::sleep(Duration::from_secs(15 * 60)).await;
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
