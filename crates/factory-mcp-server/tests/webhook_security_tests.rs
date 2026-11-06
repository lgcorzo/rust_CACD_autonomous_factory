use async_trait::async_trait;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use factory_mcp_server::github_webhook::{
    verify_github_signature, MissionDispatchReceipt, WebhookMissionDispatcher,
};
use factory_mcp_server::{build_mcp_router, McpServer};
use hmac::{Hmac, KeyInit, Mac};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use sha2::Sha256;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tower::ServiceExt;

type HmacSha256 = Hmac<Sha256>;

struct RecordingMissionDispatcher {
    dispatched_count: Arc<AtomicUsize>,
    should_fail: bool,
}

#[async_trait]
impl WebhookMissionDispatcher for RecordingMissionDispatcher {
    async fn dispatch_webhook_mission(
        &self,
        delivery_id: &str,
        mission_id: &str,
        repository: &str,
        goal: &str,
    ) -> Result<MissionDispatchReceipt, String> {
        if self.should_fail {
            return Err("Hatchet workflow engine unreachable".to_string());
        }
        self.dispatched_count.fetch_add(1, Ordering::SeqCst);
        Ok(MissionDispatchReceipt {
            dispatch_id: format!("receipt-{}-{}", delivery_id, mission_id),
            mission_id: mission_id.to_string(),
            repository: repository.to_string(),
            goal_digest: format!("len:{}", goal.len()),
        })
    }
}

fn sign_payload(secret: &str, body: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(body);
    format!("sha256={}", hex::encode(mac.finalize().into_bytes()))
}

#[tokio::test]
async fn test_webhook_rejects_missing_secret_in_protected_mode() {
    let server = Arc::new(
        McpServer::new()
            .with_protected_mode(true)
            .with_webhook_secret(String::new()),
    );
    let app = build_mcp_router(server);

    let body = serde_json::to_vec(&json!({
        "action": "opened",
        "issue": { "number": 1, "title": "Test", "html_url": "https://github.com/org/repo/issues/1" },
        "repository": { "full_name": "org/repo", "html_url": "https://github.com/org/repo" }
    }))
    .unwrap();

    let req = Request::builder()
        .method("POST")
        .uri("/webhooks/github")
        .header("content-type", "application/json")
        .header("x-github-event", "issues")
        .header("x-github-delivery", "deliv-missing-secret-01")
        .body(Body::from(body))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert!(
        resp.status() == StatusCode::UNAUTHORIZED
            || resp.status() == StatusCode::SERVICE_UNAVAILABLE,
        "Missing webhook secret in protected mode MUST reject delivery, got {}",
        resp.status()
    );
}

#[tokio::test]
async fn test_webhook_rejects_invalid_signature_malformed_and_oversized_payload() {
    let secret = "test-webhook-hmac-secret-011";
    let dispatched = Arc::new(AtomicUsize::new(0));
    let server = Arc::new(
        McpServer::new()
            .with_protected_mode(true)
            .with_webhook_secret(secret.to_string())
            .with_max_webhook_payload_bytes(1024)
            .with_webhook_dispatcher(Arc::new(RecordingMissionDispatcher {
                dispatched_count: dispatched.clone(),
                should_fail: false,
            })),
    );
    let app = build_mcp_router(server);

    let valid_body = serde_json::to_vec(&json!({
        "action": "opened",
        "issue": { "number": 2, "title": "Bug", "html_url": "https://github.com/org/repo/issues/2" },
        "repository": { "full_name": "org/repo", "html_url": "https://github.com/org/repo" }
    }))
    .unwrap();

    // 1. Invalid signature -> 401
    let req_bad_sig = Request::builder()
        .method("POST")
        .uri("/webhooks/github")
        .header("x-github-event", "issues")
        .header("x-github-delivery", "deliv-bad-sig")
        .header("x-hub-signature-256", "sha256=0000000000000000000000000000000000000000000000000000000000000000")
        .body(Body::from(valid_body.clone()))
        .unwrap();
    let resp_bad_sig = app.clone().oneshot(req_bad_sig).await.unwrap();
    assert_eq!(resp_bad_sig.status(), StatusCode::UNAUTHORIZED);

    // 2. Malformed JSON with valid signature -> 400
    let malformed = b"{not-valid-json";
    let sig_malformed = sign_payload(secret, malformed);
    let req_malformed = Request::builder()
        .method("POST")
        .uri("/webhooks/github")
        .header("x-github-event", "issues")
        .header("x-github-delivery", "deliv-malformed")
        .header("x-hub-signature-256", sig_malformed)
        .body(Body::from(malformed.to_vec()))
        .unwrap();
    let resp_malformed = app.clone().oneshot(req_malformed).await.unwrap();
    assert_eq!(resp_malformed.status(), StatusCode::BAD_REQUEST);

    // 3. Oversized payload (> 1024 bytes) -> 413 Payload Too Large
    let oversized = vec![b'a'; 2048];
    let sig_oversized = sign_payload(secret, &oversized);
    let req_oversized = Request::builder()
        .method("POST")
        .uri("/webhooks/github")
        .header("x-github-event", "issues")
        .header("x-github-delivery", "deliv-oversized")
        .header("x-hub-signature-256", sig_oversized)
        .body(Body::from(oversized))
        .unwrap();
    let resp_oversized = app.oneshot(req_oversized).await.unwrap();
    assert_eq!(resp_oversized.status(), StatusCode::PAYLOAD_TOO_LARGE);

    assert_eq!(
        dispatched.load(Ordering::SeqCst),
        0,
        "No mission may be dispatched for invalid, malformed, or oversized webhooks"
    );
}

#[tokio::test]
async fn test_webhook_replay_rejection_and_verifiable_mission_dispatch() {
    let secret = "test-webhook-hmac-secret-replay";
    let dispatched = Arc::new(AtomicUsize::new(0));
    let server = Arc::new(
        McpServer::new()
            .with_protected_mode(true)
            .with_webhook_secret(secret.to_string())
            .with_webhook_dispatcher(Arc::new(RecordingMissionDispatcher {
                dispatched_count: dispatched.clone(),
                should_fail: false,
            })),
    );
    let app = build_mcp_router(server);

    let body = serde_json::to_vec(&json!({
        "action": "opened",
        "issue": {
            "number": 638,
            "title": "Verify webhook dispatch",
            "body": "Ensure receipt is returned",
            "html_url": "https://github.com/lgcorzo/rust_CACD_autonomous_factory/issues/638"
        },
        "repository": {
            "full_name": "lgcorzo/rust_CACD_autonomous_factory",
            "html_url": "https://github.com/lgcorzo/rust_CACD_autonomous_factory"
        }
    }))
    .unwrap();
    let sig = sign_payload(secret, &body);

    // First delivery succeeds and returns verifiable dispatch receipt
    let req1 = Request::builder()
        .method("POST")
        .uri("/webhooks/github")
        .header("x-github-event", "issues")
        .header("x-github-delivery", "delivery-unique-638")
        .header("x-hub-signature-256", sig.clone())
        .body(Body::from(body.clone()))
        .unwrap();

    let resp1 = app.clone().oneshot(req1).await.unwrap();
    assert_eq!(resp1.status(), StatusCode::OK);
    let bytes1 = resp1.into_body().collect().await.unwrap().to_bytes();
    let json1: Value = serde_json::from_slice(&bytes1).unwrap();
    assert_eq!(json1["status"], "mission_triggered");
    assert!(
        json1["dispatch_receipt"]["dispatch_id"]
            .as_str()
            .unwrap_or("")
            .starts_with("receipt-delivery-unique-638-"),
        "Response must include verifiable dispatch_receipt, got: {}",
        json1
    );
    assert_eq!(dispatched.load(Ordering::SeqCst), 1);

    // Replayed delivery with identical X-GitHub-Delivery ID is rejected with 409 Conflict
    let req2 = Request::builder()
        .method("POST")
        .uri("/webhooks/github")
        .header("x-github-event", "issues")
        .header("x-github-delivery", "delivery-unique-638")
        .header("x-hub-signature-256", sig)
        .body(Body::from(body))
        .unwrap();

    let resp2 = app.oneshot(req2).await.unwrap();
    assert_eq!(resp2.status(), StatusCode::CONFLICT);
    assert_eq!(
        dispatched.load(Ordering::SeqCst),
        1,
        "Replayed webhook must not dispatch a second mission"
    );
}

#[tokio::test]
async fn test_webhook_returns_error_when_mission_dispatch_fails() {
    let secret = "test-webhook-hmac-secret-fail";
    let dispatched = Arc::new(AtomicUsize::new(0));
    let server = Arc::new(
        McpServer::new()
            .with_protected_mode(true)
            .with_webhook_secret(secret.to_string())
            .with_webhook_dispatcher(Arc::new(RecordingMissionDispatcher {
                dispatched_count: dispatched.clone(),
                should_fail: true,
            })),
    );
    let app = build_mcp_router(server);

    let body = serde_json::to_vec(&json!({
        "action": "opened",
        "issue": { "number": 640, "title": "Dispatch failure", "html_url": "https://github.com/org/repo/issues/640" },
        "repository": { "full_name": "org/repo", "html_url": "https://github.com/org/repo" }
    }))
    .unwrap();
    let sig = sign_payload(secret, &body);

    let req = Request::builder()
        .method("POST")
        .uri("/webhooks/github")
        .header("x-github-event", "issues")
        .header("x-github-delivery", "delivery-dispatch-fail-01")
        .header("x-hub-signature-256", sig)
        .body(Body::from(body))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(
        resp.status(),
        StatusCode::BAD_GATEWAY,
        "Webhook must fail with 502 Bad Gateway when mission submission fails rather than returning false 200 OK"
    );
    let _ = verify_github_signature(secret, "sha256=00", b"");
}
