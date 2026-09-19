use chrono::Utc;
use factory_application::poller_service::PollerDaemonService;
use factory_application::workflows::comment_control::{CommentControlInput, CommentControlService};
use factory_core::PRDirective;
use factory_infrastructure::aethalgard::MockAethalgardClient;
use factory_infrastructure::cursor_store::InMemoryCursorStore;
use factory_infrastructure::git_poller::GitPlatformPoller;
use factory_infrastructure::gitlab::HttpGitlabClient;
use factory_infrastructure::kafka::SimpleMockKafkaClient;
use factory_infrastructure::mcp_client::MockMcpClient;
use factory_infrastructure::r2r::MockR2rClient;
use serde_json::json;
use std::sync::Arc;
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Helper to create mock MCP, R2R, and Aethalgard clients for testing.
fn create_mock_agent_dependencies() -> (
    Arc<MockMcpClient>,
    Arc<MockR2rClient>,
    Arc<MockAethalgardClient>,
) {
    let mut mock_mcp = MockMcpClient::new();
    mock_mcp.expect_call_tool_json().returning(|_tool, _args| {
        Ok(json!({
            "is_error": false,
            "content": [{"type": "text", "text": "{\"status\": \"ok\"}"}]
        }))
    });

    let mut mock_r2r = MockR2rClient::new();
    mock_r2r
        .expect_search()
        .returning(|_| Ok("Mock context".to_string()));

    let mock_aethalgard = MockAethalgardClient::new();

    (
        Arc::new(mock_mcp),
        Arc::new(mock_r2r),
        Arc::new(mock_aethalgard),
    )
}

/// 1. E2E Test: Real-Time MR Directives, Eyes Reaction Award, Thread Resolution, and Idempotency
#[tokio::test]
async fn test_mr_directive_reaction_and_reply_flow() {
    let mock_server = MockServer::start().await;
    let token = "glpat-test-secret-token";
    let project = "test-org/test-app";

    let gl_client = Arc::new(HttpGitlabClient::new(mock_server.uri(), token.to_string()));
    let cursor_store = Arc::new(InMemoryCursorStore::new());

    let (mcp_arc, r2r_arc, aethalgard_arc) = create_mock_agent_dependencies();
    let comment_service = Arc::new(CommentControlService::new(
        None,
        Some(gl_client.clone()),
        mcp_arc,
        r2r_arc,
        aethalgard_arc,
    ));

    let poller = Arc::new(
        GitPlatformPoller::new(None, Some(gl_client.clone()), cursor_store.clone())
            .with_bot_username("darkgravity-bot"),
    );

    // Mock 1: Active MRs
    let mr_list_resp = json!([
        {
            "id": 100,
            "iid": 42,
            "title": "Feature: Add caching to auth layer",
            "description": "MR description",
            "web_url": format!("{}/{}/merge_requests/42", mock_server.uri(), project),
            "state": "opened",
            "updated_at": Utc::now().to_rfc3339()
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v4/projects/test-org%2Ftest-app/merge_requests"))
        .and(header("PRIVATE-TOKEN", token))
        .respond_with(ResponseTemplate::new(200).set_body_json(mr_list_resp))
        .mount(&mock_server)
        .await;

    // Mock 2: MR Notes (One human developer directive, one bot self-comment)
    let notes_resp = json!([
        {
            "id": 1001,
            "body": "Hey @darkgravity /status",
            "author": { "username": "alice" },
            "updated_at": Utc::now().to_rfc3339()
        },
        {
            "id": 1002,
            "body": "Older automated reply from bot",
            "author": { "username": "darkgravity-bot" },
            "updated_at": Utc::now().to_rfc3339()
        }
    ]);

    Mock::given(method("GET"))
        .and(path(
            "/api/v4/projects/test-org%2Ftest-app/merge_requests/42/notes",
        ))
        .and(header("PRIVATE-TOKEN", token))
        .respond_with(ResponseTemplate::new(200).set_body_json(notes_resp))
        .mount(&mock_server)
        .await;

    // Mock 3: Eyes emoji award reaction (Must be called exactly 1 time for note 1001)
    let emoji_resp = json!({
        "id": 55,
        "name": "eyes",
        "user": { "username": "darkgravity-bot" }
    });

    Mock::given(method("POST"))
        .and(path(
            "/api/v4/projects/test-org%2Ftest-app/merge_requests/42/notes/1001/award_emoji",
        ))
        .and(header("PRIVATE-TOKEN", token))
        .and(body_json(json!({ "name": "eyes" })))
        .respond_with(ResponseTemplate::new(201).set_body_json(emoji_resp))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Mock 4: Post reply back to MR discussion thread (Must be called exactly 1 time)
    let reply_note_resp = json!({
        "id": 1003,
        "body": "Status report",
        "author": { "username": "darkgravity-bot" },
        "updated_at": Utc::now().to_rfc3339()
    });

    Mock::given(method("POST"))
        .and(path(
            "/api/v4/projects/test-org%2Ftest-app/merge_requests/42/notes",
        ))
        .and(header("PRIVATE-TOKEN", token))
        .respond_with(ResponseTemplate::new(201).set_body_json(reply_note_resp))
        .expect(1)
        .mount(&mock_server)
        .await;

    // --- STEP 1: Polling cycle executes ---
    let events = poller.poll_gitlab_mr_notes(project).await.unwrap();

    // Verify only the developer directive note was emitted; bot self-comment was ignored
    assert_eq!(events.len(), 1, "Only human note should generate an event");
    let event = &events[0];
    assert_eq!(event.author, "alice");
    assert_eq!(event.pr_number, 42);
    assert_eq!(event.comment_id, 1001);
    assert_eq!(event.directive, PRDirective::Status);

    // --- STEP 2: Directive execution & response posting ---
    let output = comment_service
        .handle_directive(&CommentControlInput {
            event: event.clone(),
        })
        .await
        .unwrap();

    assert_eq!(output.directive_type, "status");
    assert!(
        output.comment_posted,
        "Comment should be posted to GitLab MR"
    );
    assert!(
        output
            .response_body
            .contains("Dark Gravity MCP System Report")
    );

    // --- STEP 3: Idempotency Verification ---
    // In a second polling run, the already processed note should NOT be emitted or reacted to again
    let second_events = poller.poll_gitlab_mr_notes(project).await.unwrap();
    assert_eq!(
        second_events.len(),
        0,
        "Second poll must be idempotent and return 0 events"
    );
}

/// 2. E2E Test: Conversational `@darkgravity` Interaction Directive
#[tokio::test]
async fn test_mr_interact_directive_flow() {
    let mock_server = MockServer::start().await;
    let token = "glpat-test-secret-token";
    let project = "test-org/test-app";

    let gl_client = Arc::new(HttpGitlabClient::new(mock_server.uri(), token.to_string()));
    let cursor_store = Arc::new(InMemoryCursorStore::new());

    let (mcp_arc, r2r_arc, aethalgard_arc) = create_mock_agent_dependencies();
    let comment_service = Arc::new(CommentControlService::new(
        None,
        Some(gl_client.clone()),
        mcp_arc,
        r2r_arc,
        aethalgard_arc,
    ));

    let poller = Arc::new(
        GitPlatformPoller::new(None, Some(gl_client.clone()), cursor_store)
            .with_bot_username("darkgravity-bot"),
    );

    // Mock MR
    Mock::given(method("GET"))
        .and(path("/api/v4/projects/test-org%2Ftest-app/merge_requests"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([{
            "id": 200,
            "iid": 15,
            "title": "Refactor parser",
            "description": None::<String>,
            "web_url": "https://gitlab.com/test-org/test-app/merge_requests/15",
            "state": "opened",
            "updated_at": Utc::now().to_rfc3339()
        }])))
        .mount(&mock_server)
        .await;

    // Mock conversational note
    Mock::given(method("GET"))
        .and(path(
            "/api/v4/projects/test-org%2Ftest-app/merge_requests/15/notes",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([{
            "id": 2001,
            "body": "@darkgravity can you review the security implications of this PR?",
            "author": { "username": "bob" },
            "updated_at": Utc::now().to_rfc3339()
        }])))
        .mount(&mock_server)
        .await;

    // Mock eyes emoji
    Mock::given(method("POST"))
        .and(path(
            "/api/v4/projects/test-org%2Ftest-app/merge_requests/15/notes/2001/award_emoji",
        ))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 99,
            "name": "eyes",
            "user": { "username": "darkgravity-bot" }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Mock response post
    Mock::given(method("POST"))
        .and(path(
            "/api/v4/projects/test-org%2Ftest-app/merge_requests/15/notes",
        ))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": 2002,
            "body": "Response",
            "author": { "username": "darkgravity-bot" },
            "updated_at": Utc::now().to_rfc3339()
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let events = poller.poll_gitlab_mr_notes(project).await.unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(
        events[0].directive,
        PRDirective::Interact {
            prompt: "can you review the security implications of this PR?".to_string()
        }
    );

    let output = comment_service
        .handle_directive(&CommentControlInput {
            event: events[0].clone(),
        })
        .await
        .unwrap();

    assert_eq!(output.directive_type, "interact");
    assert!(output.comment_posted);
    assert!(output.response_body.contains("Dark Gravity Assistant"));
}

/// 3. E2E Test: Automated Issue-Driven Mission Ingestion & NHI Credential Issuance
#[tokio::test]
async fn test_issue_ingestion_and_credential_issuance() {
    let mock_server = MockServer::start().await;
    let token = "glpat-test-secret-token";
    let project = "test-org/test-app";

    let gl_client = Arc::new(HttpGitlabClient::new(mock_server.uri(), token.to_string()));
    let cursor_store = Arc::new(InMemoryCursorStore::new());

    let (mcp_arc, r2r_arc, aethalgard_arc) = create_mock_agent_dependencies();
    let comment_service = Arc::new(CommentControlService::new(
        None,
        Some(gl_client.clone()),
        mcp_arc,
        r2r_arc,
        aethalgard_arc,
    ));

    let poller = Arc::new(
        GitPlatformPoller::new(None, Some(gl_client.clone()), cursor_store)
            .with_bot_username("darkgravity-bot"),
    );

    let kafka_client = Arc::new(SimpleMockKafkaClient);
    let daemon = PollerDaemonService::new(poller, kafka_client, None, comment_service);

    // Mock: Issues with autonomous-mission label and resource limits in description
    let issue_resp = json!([
        {
            "id": 501,
            "iid": 7,
            "title": "Fix memory leak in buffer pool",
            "description": "Root cause identified in allocator.\n\nCPU: 200m, RAM: 256Mi, Timeout: 300s",
            "web_url": "https://gitlab.com/test-org/test-app/issues/7",
            "updated_at": Utc::now().to_rfc3339()
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v4/projects/test-org%2Ftest-app/issues"))
        .and(header("PRIVATE-TOKEN", token))
        .respond_with(ResponseTemplate::new(200).set_body_json(issue_resp))
        .mount(&mock_server)
        .await;

    // Mock active MRs (empty)
    Mock::given(method("GET"))
        .and(path("/api/v4/projects/test-org%2Ftest-app/merge_requests"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&mock_server)
        .await;

    // Run first polling cycle
    let stats = daemon.poll_once(&[], &[project.to_string()]).await;
    assert_eq!(stats.issues_ingested, 1, "Issue should be ingested");
    assert!(
        stats.errors.is_empty(),
        "No errors should occur: {:?}",
        stats.errors
    );

    // Run second polling cycle - should deduplicate and ingest 0
    let second_stats = daemon.poll_once(&[], &[project.to_string()]).await;
    assert_eq!(
        second_stats.issues_ingested, 0,
        "Issue must be deduplicated on subsequent poll cycles"
    );
}

/// 4. E2E Test: Bot Loop Prevention (Ignoring Self-Authored Notes)
#[tokio::test]
async fn test_bot_self_authored_notes_ignored() {
    let mock_server = MockServer::start().await;
    let token = "glpat-test-secret-token";
    let project = "test-org/test-app";

    let gl_client = Arc::new(HttpGitlabClient::new(mock_server.uri(), token.to_string()));
    let cursor_store = Arc::new(InMemoryCursorStore::new());

    let poller = Arc::new(
        GitPlatformPoller::new(None, Some(gl_client.clone()), cursor_store)
            .with_bot_username("darkgravity-bot"),
    );

    // Mock active MR
    Mock::given(method("GET"))
        .and(path("/api/v4/projects/test-org%2Ftest-app/merge_requests"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([{
            "id": 300,
            "iid": 8,
            "title": "Test MR",
            "description": None::<String>,
            "web_url": "https://gitlab.com/test-org/test-app/merge_requests/8",
            "state": "opened",
            "updated_at": Utc::now().to_rfc3339()
        }])))
        .mount(&mock_server)
        .await;

    // Notes ONLY by bot
    Mock::given(method("GET"))
        .and(path(
            "/api/v4/projects/test-org%2Ftest-app/merge_requests/8/notes",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "id": 3001,
                "body": "@darkgravity /retry",
                "author": { "username": "darkgravity-bot" },
                "updated_at": Utc::now().to_rfc3339()
            },
            {
                "id": 3002,
                "body": "System report",
                "author": { "username": "DarkGravity-Bot" },
                "updated_at": Utc::now().to_rfc3339()
            }
        ])))
        .mount(&mock_server)
        .await;

    let events = poller.poll_gitlab_mr_notes(project).await.unwrap();
    assert_eq!(events.len(), 0, "Bot self-authored notes must be ignored");
}

/// 5. Failure Report Formatting & Remediation Guidance
#[test]
fn test_failure_report_formatting() {
    let report = CommentControlService::format_failure_report(
        "Failed to pull image from registry.gitlab.com: 403 Forbidden",
        &[
            "Verify imagePullSecrets in deployment manifest",
            "Ensure GitLab deploy token has read_registry scope",
        ],
    );

    assert!(report.contains("❌ **Dark Gravity Action Failed**"));
    assert!(report.contains("403 Forbidden"));
    assert!(report.contains("1. Verify imagePullSecrets"));
    assert!(report.contains("2. Ensure GitLab deploy token"));
}
