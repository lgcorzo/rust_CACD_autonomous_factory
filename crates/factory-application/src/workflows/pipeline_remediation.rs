//! Pipeline remediation workflow service.
//!
//! Orchestrates the use case of converting classified pipeline errors into
//! remediation missions via the existing Hatchet DAG workflow, or escalating
//! non-remediable errors to human review.

#[allow(unused_imports)]
use chrono::Utc;
use ed25519_dalek::SigningKey;
use factory_core::security::nhi::{AgentSubject, VerifiableCredential};
use factory_core::{
    ErrorCategory, ErrorClassification, PipelineFailureEvent, RemediationOutcome, RemediationStatus,
};
use factory_infrastructure::cursor_store::CursorStore;
use factory_infrastructure::github::GithubClient;
use factory_infrastructure::gitlab::GitlabClient;
use factory_infrastructure::kafka::KafkaClient;
use factory_infrastructure::pipeline_classifier::PipelineClassifier;
use rand::rngs::OsRng;
use std::sync::Arc;
use tokio::sync::Semaphore;
#[allow(unused_imports)]
use uuid::Uuid;

/// Number of repeated failures before forcing human escalation.
const RECURRING_FAILURE_ESCALATION_THRESHOLD: u32 = 3;

/// Default maximum concurrent remediation missions.
const DEFAULT_MAX_CONCURRENT_REMEDIATIONS: usize = 5;

/// Repository that requires human-in-the-loop approval for remediation.
const SELF_REFERENTIAL_REPO: &str = "lgcorzo/rust_CACD_autonomous_factory";

/// Pipeline remediation workflow service.
///
/// Classifies pipeline failures and routes them to either:
/// - Autonomous remediation via the `mission-input` Kafka topic
/// - Human escalation via GitHub/GitLab issue creation
pub struct PipelineRemediationService {
    kafka_client: Arc<dyn KafkaClient>,
    github_client: Option<Arc<dyn GithubClient>>,
    gitlab_client: Option<Arc<dyn GitlabClient>>,
    classifier: Arc<dyn PipelineClassifier>,
    semaphore: Arc<Semaphore>,
    /// Optional cursor store for recurring failure tracking.
    cursor_store: Option<Arc<dyn CursorStore>>,
    signing_key: SigningKey,
    key_id: String,
}

impl PipelineRemediationService {
    /// Create a new remediation service.
    pub fn new(
        kafka_client: Arc<dyn KafkaClient>,
        github_client: Option<Arc<dyn GithubClient>>,
        gitlab_client: Option<Arc<dyn GitlabClient>>,
        classifier: Arc<dyn PipelineClassifier>,
    ) -> Self {
        let max_concurrent = std::env::var("MAX_CONCURRENT_REMEDIATIONS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(DEFAULT_MAX_CONCURRENT_REMEDIATIONS);

        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);

        Self {
            kafka_client,
            github_client,
            gitlab_client,
            classifier,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            cursor_store: None,
            signing_key,
            key_id: "did:key:dark-gravity-remediation-nhi#1".to_string(),
        }
    }

    /// Attach a cursor store to enable recurring failure tracking (T063).
    pub fn with_cursor_store(mut self, cursor_store: Arc<dyn CursorStore>) -> Self {
        self.cursor_store = Some(cursor_store);
        self
    }

    /// Override the signing key for NHI credential generation.
    pub fn with_signing_key(mut self, signing_key: SigningKey, key_id: String) -> Self {
        self.signing_key = signing_key;
        self.key_id = key_id;
        self
    }

    /// Handle a detected pipeline failure.
    ///
    /// Classifies the error, checks safety guards, and routes to either
    /// autonomous remediation or human escalation.
    pub async fn handle_pipeline_failure(
        &self,
        event: &PipelineFailureEvent,
    ) -> anyhow::Result<RemediationStatus> {
        // 1. Classify the error
        let classification = self.classifier.classify(&event.error_log);

        // 2. Publish classification reasoning to agent-thought topic
        self.publish_classification_thought(event, &classification)
            .await?;

        // 3. Check self-referential safety guard
        if event.repository == SELF_REFERENTIAL_REPO {
            tracing::warn!(
                "Self-referential pipeline failure detected for {}. Escalating to human review.",
                event.repository
            );
            self.escalate_to_human(event, &classification).await?;
            return Ok(RemediationStatus::Escalated);
        }

        // 4. Check recurring failure threshold (T063)
        if classification.is_remediable
            && let Some(store) = &self.cursor_store
        {
            let count = store
                .increment_failure_count(&classification.error_fingerprint)
                .await
                .unwrap_or(1);
            if count >= RECURRING_FAILURE_ESCALATION_THRESHOLD {
                tracing::warn!(
                    "Recurring failure #{} for fingerprint {} in {}. Escalating.",
                    count,
                    classification.error_fingerprint,
                    event.repository
                );
                self.escalate_to_human(event, &classification).await?;
                return Ok(RemediationStatus::Escalated);
            }
        }

        // 5. Route based on classification
        if classification.is_remediable {
            // Acquire semaphore permit to limit concurrent remediations
            let _permit =
                self.semaphore.acquire().await.map_err(|e| {
                    anyhow::anyhow!("Failed to acquire remediation semaphore: {}", e)
                })?;

            self.trigger_remediation_mission(event, &classification)
                .await?;
            Ok(RemediationStatus::Pending)
        } else if classification.category == ErrorCategory::InfrastructureTransient {
            // Transient errors: log but don't escalate immediately
            tracing::info!(
                "Transient pipeline error in {}: {} — will retry on next poll cycle",
                event.repository,
                event.workflow_name
            );
            Ok(RemediationStatus::Pending)
        } else {
            // Non-remediable: escalate to human
            self.escalate_to_human(event, &classification).await?;
            Ok(RemediationStatus::Escalated)
        }
    }

    /// Trigger a remediation mission via the mission-input Kafka topic.
    async fn trigger_remediation_mission(
        &self,
        event: &PipelineFailureEvent,
        classification: &ErrorClassification,
    ) -> anyhow::Result<()> {
        let mission_id = format!(
            "remediation-{}-{}",
            event.repository.replace('/', "-"),
            event.run_id
        );

        // Create NHI Verifiable Credential
        let subject = AgentSubject {
            id: format!("nhi:pipeline:{}:{}", event.source_platform, event.run_id),
            roles: vec!["remediation_executor".to_string()],
            allowed_namespaces: vec!["agents".to_string(), "production".to_string()],
        };

        let mut vc = VerifiableCredential::new(
            format!("vc:remediation:{}:{}", event.repository, event.run_id),
            "did:factory:dark-gravity-euskadi".to_string(),
            subject,
        );

        vc.sign(&self.signing_key, &self.key_id)
            .map_err(|e| anyhow::anyhow!("NHI Ed25519 signing failed: {}", e))?;

        let error_summary = classification
            .rule_name
            .as_deref()
            .or(classification.test_name.as_deref())
            .unwrap_or("unknown");

        let goal = format!(
            "Fix {:?} error in {}: {} in {}",
            classification.category,
            event.repository,
            error_summary,
            classification
                .file_path
                .as_deref()
                .unwrap_or("unknown file")
        );

        let payload = serde_json::json!({
            "mission_id": mission_id,
            "goal": goal,
            "repository_path": event.repository,
            "source_platform": "pipeline_remediation",
            "pipeline_failure": {
                "source_platform": event.source_platform,
                "repository": event.repository,
                "run_id": event.run_id,
                "workflow_name": event.workflow_name,
                "failing_job": event.failing_job,
                "failing_step": event.failing_step,
                "run_url": event.run_url,
                "detected_at": event.detected_at.to_rfc3339(),
            },
            "error_classification": classification,
            "verifiable_credential": vc,
        });

        let payload_bytes = serde_json::to_vec(&payload)?;
        self.kafka_client
            .publish("mission-input", &mission_id, &payload_bytes)
            .await?;

        tracing::info!(
            "Remediation mission triggered: {} for {:?} in {}",
            mission_id,
            classification.category,
            event.repository
        );

        Ok(())
    }

    /// Escalate a non-remediable error to human review by creating a GitHub/GitLab issue.
    async fn escalate_to_human(
        &self,
        event: &PipelineFailureEvent,
        classification: &ErrorClassification,
    ) -> anyhow::Result<()> {
        let escalation_key = format!(
            "{}:{}:{:?}:{}",
            event.source_platform,
            event.repository,
            classification.category,
            classification.error_fingerprint
        );
        let escalation_marker = format!("<!-- dark-gravity-escalation-key: {} -->", escalation_key);
        let title = format!(
            "[Pipeline Remediation] {:?} failure in {}",
            classification.category, event.repository
        );

        let body = format!(
            "## Pipeline Failure Detected\n\n\
            **Repository**: {}\n\
            **Workflow**: {}\n\
            **Failing Job**: {}\n\
            **Error Category**: {:?}\n\
            **Remediable**: {}\n\
            **Run URL**: {}\n\
            **Detected At**: {}\n\n\
            ### Error Log (truncated)\n\n\
            ```\n{}\n```\n\n\
            ### Classification Details\n\n\
            - **Sub-type**: {}\n\
            - **File**: {}\n\
            - **Line**: {}\n\
            - **Rule/Test**: {}\n\
            - **Fingerprint**: `{}`\n\n\
            ---\n\
            *This issue was created automatically by Dark Gravity Pipeline Remediation.*\n\n{}",
            event.repository,
            event.workflow_name,
            event.failing_job,
            classification.category,
            classification.is_remediable,
            event.run_url,
            event.detected_at.to_rfc3339(),
            &event.error_log[..event.error_log.len().min(2000)],
            classification.sub_type.as_deref().unwrap_or("N/A"),
            classification.file_path.as_deref().unwrap_or("N/A"),
            classification
                .line_number
                .map_or("N/A".to_string(), |n| n.to_string()),
            classification
                .rule_name
                .as_deref()
                .or(classification.test_name.as_deref())
                .unwrap_or("N/A"),
            classification.error_fingerprint,
            escalation_marker,
        );

        let label = if classification.category == ErrorCategory::Unknown {
            "pipeline-failure-unknown"
        } else {
            "pipeline-remediation-pending"
        };

        match event.source_platform.as_str() {
            "github" => {
                if let Some(gh) = &self.github_client {
                    let existing = gh
                        .list_open_issues(&event.repository, Some(label.to_string()))
                        .await?;
                    if existing.iter().any(|issue| {
                        issue
                            .body
                            .as_deref()
                            .is_some_and(|body| body.contains(&escalation_marker))
                    }) {
                        tracing::info!(
                            "Escalation already exists for {} in {}",
                            escalation_key,
                            event.repository
                        );
                        return Ok(());
                    }
                    gh.create_issue(&event.repository, &title, &body, &[label.to_string()])
                        .await?;
                }
            }
            "gitlab" => {
                if let Some(gl) = &self.gitlab_client {
                    let existing = gl
                        .list_open_issues(&event.repository, Some(label.to_string()))
                        .await?;
                    if existing.iter().any(|issue| {
                        issue
                            .description
                            .as_deref()
                            .is_some_and(|description| description.contains(&escalation_marker))
                    }) {
                        tracing::info!(
                            "Escalation already exists for {} in {}",
                            escalation_key,
                            event.repository
                        );
                        return Ok(());
                    }
                    gl.create_issue_with_labels(
                        &event.repository,
                        &title,
                        &body,
                        &[label.to_string()],
                    )
                    .await?;
                }
            }
            _ => {
                tracing::warn!(
                    "Unknown platform {} for escalation of run #{}",
                    event.source_platform,
                    event.run_id
                );
            }
        }

        tracing::info!(
            "Escalated pipeline failure to human review: {} run #{} ({:?})",
            event.repository,
            event.run_id,
            classification.category
        );

        Ok(())
    }

    /// Publish classification reasoning to the agent-thought Kafka topic.
    async fn publish_classification_thought(
        &self,
        event: &PipelineFailureEvent,
        classification: &ErrorClassification,
    ) -> anyhow::Result<()> {
        let mission_id = format!(
            "remediation-{}-{}",
            event.repository.replace('/', "-"),
            event.run_id
        );

        let thought = format!(
            "Classified as {:?} ({}) with confidence: regex_match. File: {}:{}. Remediable: {}.",
            classification.category,
            classification
                .rule_name
                .as_deref()
                .or(classification.test_name.as_deref())
                .unwrap_or("N/A"),
            classification.file_path.as_deref().unwrap_or("N/A"),
            classification
                .line_number
                .map_or("N/A".to_string(), |n| n.to_string()),
            classification.is_remediable,
        );

        self.kafka_client
            .publish_thought(&mission_id, &thought, "pipeline-classifier")
            .await?;

        Ok(())
    }

    /// Record a remediation outcome to the mission-artifact Kafka topic.
    pub async fn record_remediation_outcome(
        &self,
        outcome: &RemediationOutcome,
        event: &PipelineFailureEvent,
        classification: &ErrorClassification,
    ) -> anyhow::Result<()> {
        let mission_id = format!(
            "remediation-{}-{}",
            event.repository.replace('/', "-"),
            event.run_id
        );

        let payload = serde_json::json!({
            "mission_id": mission_id,
            "event_type": "remediation_outcome",
            "result": outcome.result,
            "error_category": classification.category,
            "repository": event.repository,
            "pr_url": outcome.pr_url,
            "fix_duration_secs": outcome.fix_duration_secs,
            "pipeline_passed": outcome.pipeline_passed,
            "completed_at": outcome.completed_at.to_rfc3339(),
        });

        let payload_bytes = serde_json::to_vec(&payload)?;
        self.kafka_client
            .publish("mission-artifact", &mission_id, &payload_bytes)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use factory_infrastructure::github::{GithubIssue, MockGithubClient};
    use factory_infrastructure::kafka::SimpleMockKafkaClient;
    use factory_infrastructure::pipeline_classifier::RegexPipelineClassifier;

    fn make_event(repo: &str, error_log: &str) -> PipelineFailureEvent {
        PipelineFailureEvent {
            source_platform: "github".to_string(),
            repository: repo.to_string(),
            run_id: 12345,
            workflow_name: "CI/CD Pipeline".to_string(),
            failing_job: "Rust CI".to_string(),
            failing_step: Some("Lint".to_string()),
            error_log: error_log.to_string(),
            run_url: format!("https://github.com/{}/actions/runs/12345", repo),
            detected_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_self_referential_safety_guard() {
        let mut mock_gh = MockGithubClient::new();
        mock_gh
            .expect_list_open_issues()
            .returning(|_repo, _labels| Ok(Vec::new()));
        mock_gh
            .expect_create_issue()
            .returning(|_repo, _title, _body, _labels| {
                Ok(GithubIssue {
                    id: 999,
                    number: 100,
                    title: "test".to_string(),
                    body: Some("test".to_string()),
                    html_url: "https://github.com/test".to_string(),
                    updated_at: Some(Utc::now()),
                })
            });

        let kafka = Arc::new(SimpleMockKafkaClient::new("mock").unwrap());
        let classifier = Arc::new(RegexPipelineClassifier::new());

        let service =
            PipelineRemediationService::new(kafka, Some(Arc::new(mock_gh)), None, classifier);

        let event = make_event(
            SELF_REFERENTIAL_REPO,
            "warning: unused [clippy::dead_code]\n  --> src/main.rs:10:1",
        );

        let result = service.handle_pipeline_failure(&event).await.unwrap();
        assert_eq!(result, RemediationStatus::Escalated);
    }

    #[tokio::test]
    async fn test_remediable_error_triggers_mission() {
        let kafka = Arc::new(SimpleMockKafkaClient::new("mock").unwrap());
        let classifier = Arc::new(RegexPipelineClassifier::new());

        let service = PipelineRemediationService::new(kafka, None, None, classifier);

        let event = make_event(
            "lgcorzo/lince-rs",
            "error[E0308]: mismatched types\n  --> src/lib.rs:15:5",
        );

        let result = service.handle_pipeline_failure(&event).await.unwrap();
        assert_eq!(result, RemediationStatus::Pending);
    }

    #[tokio::test]
    async fn test_unknown_error_creates_issue() {
        let mut mock_gh = MockGithubClient::new();
        mock_gh
            .expect_list_open_issues()
            .returning(|_repo, _labels| Ok(Vec::new()));
        mock_gh
            .expect_create_issue()
            .returning(|_repo, _title, _body, _labels| {
                Ok(GithubIssue {
                    id: 999,
                    number: 100,
                    title: "test".to_string(),
                    body: Some("test".to_string()),
                    html_url: "https://github.com/test".to_string(),
                    updated_at: Some(Utc::now()),
                })
            });

        let kafka = Arc::new(SimpleMockKafkaClient::new("mock").unwrap());
        let classifier = Arc::new(RegexPipelineClassifier::new());

        let service =
            PipelineRemediationService::new(kafka, Some(Arc::new(mock_gh)), None, classifier);

        let event = make_event(
            "lgcorzo/lince-rs",
            "Something unexpected happened with exit code 137",
        );

        let result = service.handle_pipeline_failure(&event).await.unwrap();
        assert_eq!(result, RemediationStatus::Escalated);
    }

    #[tokio::test]
    async fn test_transient_error_does_not_escalate() {
        let kafka = Arc::new(SimpleMockKafkaClient::new("mock").unwrap());
        let classifier = Arc::new(RegexPipelineClassifier::new());

        let service = PipelineRemediationService::new(kafka, None, None, classifier);

        let event = make_event(
            "lgcorzo/lince-rs",
            "fatal: unable to access: Connection timed out",
        );

        let result = service.handle_pipeline_failure(&event).await.unwrap();
        assert_eq!(result, RemediationStatus::Pending);
    }

    #[tokio::test]
    async fn test_remediation_outcome_telemetry() {
        let kafka = Arc::new(SimpleMockKafkaClient::new("mock").unwrap());
        let classifier = Arc::new(RegexPipelineClassifier::new());

        let service = PipelineRemediationService::new(kafka, None, None, classifier);

        let event = make_event(
            "lgcorzo/lince-rs",
            "error[E0308]: mismatched types\n  --> src/lib.rs:15:5",
        );

        let classification = service.classifier.classify(&event.error_log);

        let outcome = RemediationOutcome {
            mission_id: Uuid::new_v4(),
            result: RemediationStatus::Success,
            pr_url: Some("https://github.com/lgcorzo/lince-rs/pull/99".to_string()),
            fix_duration_secs: Some(180),
            pipeline_passed: true,
            completed_at: Utc::now(),
        };

        let result = service
            .record_remediation_outcome(&outcome, &event, &classification)
            .await;
        assert!(result.is_ok());
    }

    // ── T044: Full cycle integration test ──

    #[tokio::test]
    async fn test_pipeline_remediation_full_cycle() {
        let kafka = Arc::new(SimpleMockKafkaClient::new("mock").unwrap());
        let classifier = Arc::new(RegexPipelineClassifier::new());
        let service = PipelineRemediationService::new(kafka.clone(), None, None, classifier);

        // Simulate a compilable error from a non-self-referential repo
        let event = make_event(
            "lgcorzo/lince-rs",
            "error[E0308]: mismatched types\n  --> src/lib.rs:15:5",
        );

        // Step 1: classify and trigger mission
        let status = service.handle_pipeline_failure(&event).await.unwrap();
        assert_eq!(status, RemediationStatus::Pending);

        // Step 2: record the outcome (simulating mission success)
        let classification = service.classifier.classify(&event.error_log);
        let outcome = RemediationOutcome {
            mission_id: Uuid::new_v4(),
            result: RemediationStatus::Success,
            pr_url: Some("https://github.com/lgcorzo/lince-rs/pull/42".to_string()),
            fix_duration_secs: Some(120),
            pipeline_passed: true,
            completed_at: Utc::now(),
        };
        let outcome_result = service
            .record_remediation_outcome(&outcome, &event, &classification)
            .await;
        assert!(outcome_result.is_ok(), "Outcome recording should succeed");
    }

    // ── T047: Concurrency limiter test ──

    #[tokio::test]
    async fn test_concurrency_limiter() {
        // Set max to 2 concurrent remediations
        // SAFETY: Test environment, no other threads reading this var concurrently.
        unsafe {
            std::env::set_var("MAX_CONCURRENT_REMEDIATIONS", "2");
        }

        let kafka = Arc::new(SimpleMockKafkaClient::new("mock").unwrap());
        let classifier = Arc::new(RegexPipelineClassifier::new());
        let service = Arc::new(PipelineRemediationService::new(
            kafka, None, None, classifier,
        ));

        // Verify the semaphore allows up to 2 concurrent permits
        let _permit1 = service.semaphore.clone().acquire_owned().await.unwrap();
        let _permit2 = service.semaphore.clone().acquire_owned().await.unwrap();

        // A third acquire should block (semaphore exhausted) — we just verify it's possible
        // without actually blocking by checking available permits
        assert_eq!(service.semaphore.available_permits(), 0);

        // When permits are dropped, they should be released
        drop(_permit1);
        assert_eq!(service.semaphore.available_permits(), 1);

        // SAFETY: Test environment
        unsafe {
            std::env::remove_var("MAX_CONCURRENT_REMEDIATIONS");
        }
    }

    // ── T048: Transient error does not escalate test ──

    #[tokio::test]
    async fn test_transient_error_retry_behavior() {
        let kafka = Arc::new(SimpleMockKafkaClient::new("mock").unwrap());
        let classifier = Arc::new(RegexPipelineClassifier::new());
        let service = PipelineRemediationService::new(kafka, None, None, classifier);

        // Transient errors should return Pending (not Escalated), signaling retry on next poll
        let transient_event = make_event(
            "lgcorzo/lince-rs",
            "Connection timed out after 30s waiting for registry",
        );
        let result = service
            .handle_pipeline_failure(&transient_event)
            .await
            .unwrap();
        assert_eq!(
            result,
            RemediationStatus::Pending,
            "Transient errors should return Pending for retry"
        );

        // Separate from OOMKilled
        let oom_event = make_event("lgcorzo/lince-rs", "OOMKilled: container ran out of memory");
        let oom_result = service.handle_pipeline_failure(&oom_event).await.unwrap();
        assert_eq!(
            oom_result,
            RemediationStatus::Pending,
            "OOM errors should also return Pending"
        );
    }

    // ── T058-T059: Remediation telemetry tests ──

    #[tokio::test]
    async fn test_remediation_success_telemetry() {
        let kafka = Arc::new(SimpleMockKafkaClient::new("mock").unwrap());
        let classifier = Arc::new(RegexPipelineClassifier::new());
        let service = PipelineRemediationService::new(kafka, None, None, classifier);
        let event = make_event("lgcorzo/lince-rs", "error[E0308]: mismatched types");
        let classification = service.classifier.classify(&event.error_log);

        let outcome = RemediationOutcome {
            mission_id: Uuid::new_v4(),
            result: RemediationStatus::Success,
            pr_url: Some("https://github.com/lgcorzo/lince-rs/pull/88".to_string()),
            fix_duration_secs: Some(90),
            pipeline_passed: true,
            completed_at: Utc::now(),
        };

        let result = service
            .record_remediation_outcome(&outcome, &event, &classification)
            .await;
        assert!(
            result.is_ok(),
            "Success telemetry should publish without error"
        );
    }

    #[tokio::test]
    async fn test_remediation_failure_telemetry() {
        let kafka = Arc::new(SimpleMockKafkaClient::new("mock").unwrap());
        let classifier = Arc::new(RegexPipelineClassifier::new());
        let service = PipelineRemediationService::new(kafka, None, None, classifier);
        let event = make_event("lgcorzo/lince-rs", "error[E0308]: mismatched types");
        let classification = service.classifier.classify(&event.error_log);

        let outcome = RemediationOutcome {
            mission_id: Uuid::new_v4(),
            result: RemediationStatus::Failed,
            pr_url: None,
            fix_duration_secs: None,
            pipeline_passed: false,
            completed_at: Utc::now(),
        };

        let result = service
            .record_remediation_outcome(&outcome, &event, &classification)
            .await;
        assert!(
            result.is_ok(),
            "Failure telemetry should publish without error"
        );
    }

    // ── T060: Recurring failure escalation test ──

    #[tokio::test]
    async fn test_recurring_failure_escalation() {
        use factory_infrastructure::cursor_store::InMemoryCursorStore;

        let mut mock_gh = MockGithubClient::new();
        mock_gh
            .expect_list_open_issues()
            .returning(|_repo, _labels| Ok(Vec::new()));
        mock_gh
            .expect_create_issue()
            .returning(|_repo, _title, _body, _labels| {
                Ok(GithubIssue {
                    id: 999,
                    number: 100,
                    title: "test".to_string(),
                    body: Some("test".to_string()),
                    html_url: "https://github.com/test".to_string(),
                    updated_at: Some(Utc::now()),
                })
            });

        let kafka = Arc::new(SimpleMockKafkaClient::new("mock").unwrap());
        let classifier = Arc::new(RegexPipelineClassifier::new());
        let cursor_store = Arc::new(InMemoryCursorStore::new());

        let service =
            PipelineRemediationService::new(kafka, Some(Arc::new(mock_gh)), None, classifier)
                .with_cursor_store(cursor_store.clone());

        let error_log = "error[E0308]: mismatched types\n  --> src/lib.rs:15:5";
        let event = make_event("lgcorzo/lince-rs", error_log);

        // First 2 failures: should return Pending (trigger mission)
        let r1 = service.handle_pipeline_failure(&event).await.unwrap();
        assert_eq!(
            r1,
            RemediationStatus::Pending,
            "First failure: mission triggered"
        );

        let r2 = service.handle_pipeline_failure(&event).await.unwrap();
        assert_eq!(
            r2,
            RemediationStatus::Pending,
            "Second failure: mission triggered"
        );

        // Third failure: exceeds threshold → escalate
        let r3 = service.handle_pipeline_failure(&event).await.unwrap();
        assert_eq!(
            r3,
            RemediationStatus::Escalated,
            "Third failure: should escalate to human"
        );
    }
}
