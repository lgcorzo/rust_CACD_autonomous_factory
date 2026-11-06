use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use factory_mcp_server::{build_mcp_router, McpServer};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tower::ServiceExt;

#[tokio::test]
async fn test_privileged_routes_reject_unauthenticated_callers_and_cors_only_headers() {
    let server = Arc::new(
        McpServer::new()
            .with_protected_mode(true)
            .with_api_bearer_token("secret-mcp-token-011".to_string()),
    );
    let app = build_mcp_router(server);

    // 1. /mcp without Authorization header -> 401, even with CORS Origin header
    let req_mcp_unauth = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .header("origin", "https://trusted.example.com")
        .body(Body::from(
            serde_json::to_vec(&json!({
                "jsonrpc": "2.0",
                "method": "list_tools",
                "id": 1
            }))
            .unwrap(),
        ))
        .unwrap();
    let resp_mcp_unauth = app.clone().oneshot(req_mcp_unauth).await.unwrap();
    assert_eq!(
        resp_mcp_unauth.status(),
        StatusCode::UNAUTHORIZED,
        "CORS Origin header alone MUST NOT grant access to /mcp"
    );

    // 2. /sse without Authorization header -> 401
    let req_sse_unauth = Request::builder()
        .method("GET")
        .uri("/sse")
        .header("origin", "https://trusted.example.com")
        .body(Body::empty())
        .unwrap();
    let resp_sse_unauth = app.clone().oneshot(req_sse_unauth).await.unwrap();
    assert_eq!(resp_sse_unauth.status(), StatusCode::UNAUTHORIZED);

    // 3. /feedback without Authorization header -> 401
    let req_fb_unauth = Request::builder()
        .method("POST")
        .uri("/feedback")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_vec(&json!({"mission_id": "m1", "feedback": "hi"})).unwrap(),
        ))
        .unwrap();
    let resp_fb_unauth = app.clone().oneshot(req_fb_unauth).await.unwrap();
    assert_eq!(resp_fb_unauth.status(), StatusCode::UNAUTHORIZED);

    // 4. /mcp WITH valid Bearer token -> 200 OK
    let req_mcp_auth = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .header("authorization", "Bearer secret-mcp-token-011")
        .body(Body::from(
            serde_json::to_vec(&json!({
                "jsonrpc": "2.0",
                "method": "list_tools",
                "id": 1
            }))
            .unwrap(),
        ))
        .unwrap();
    let resp_mcp_auth = app.oneshot(req_mcp_auth).await.unwrap();
    assert_eq!(resp_mcp_auth.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_sse_session_bounds_ownership_disconnect_cleanup_and_expiry() {
    let server = Arc::new(
        McpServer::new()
            .with_protected_mode(true)
            .with_api_bearer_token("token-owner-a".to_string())
            .with_max_sessions(2)
            .with_session_ttl(Duration::from_millis(120)),
    );
    let app = build_mcp_router(server.clone());

    // Open session 1
    let req1 = Request::builder()
        .method("GET")
        .uri("/sse")
        .header("authorization", "Bearer token-owner-a")
        .header("x-mcp-client-id", "client-a")
        .body(Body::empty())
        .unwrap();
    let resp1 = app.clone().oneshot(req1).await.unwrap();
    assert_eq!(resp1.status(), StatusCode::OK);
    let sid1 = resp1
        .headers()
        .get("x-mcp-session-id")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // Open session 2
    let req2 = Request::builder()
        .method("GET")
        .uri("/sse")
        .header("authorization", "Bearer token-owner-a")
        .header("x-mcp-client-id", "client-a")
        .body(Body::empty())
        .unwrap();
    let resp2 = app.clone().oneshot(req2).await.unwrap();
    assert_eq!(resp2.status(), StatusCode::OK);
    assert_eq!(server.active_session_count().await, 2);

    // 3rd concurrent session exceeds max_sessions (2) -> 429 Too Many Requests
    let req3 = Request::builder()
        .method("GET")
        .uri("/sse")
        .header("authorization", "Bearer token-owner-a")
        .header("x-mcp-client-id", "client-a")
        .body(Body::empty())
        .unwrap();
    let resp3 = app.clone().oneshot(req3).await.unwrap();
    assert_eq!(
        resp3.status(),
        StatusCode::TOO_MANY_REQUESTS,
        "Exceeding bounded SSE session limit must return 429"
    );

    // Different client owner ("client-b") attempting to post to sid1 -> 403 Forbidden
    let req_wrong_owner = Request::builder()
        .method("POST")
        .uri(format!("/mcp?session_id={}", sid1))
        .header("content-type", "application/json")
        .header("authorization", "Bearer token-owner-a")
        .header("x-mcp-client-id", "client-b")
        .body(Body::from(
            serde_json::to_vec(&json!({
                "jsonrpc": "2.0",
                "method": "ping",
                "id": 1
            }))
            .unwrap(),
        ))
        .unwrap();
    let resp_wrong_owner = app.clone().oneshot(req_wrong_owner).await.unwrap();
    assert_eq!(
        resp_wrong_owner.status(),
        StatusCode::FORBIDDEN,
        "Session owned by client-a must reject messages from client-b"
    );

    // Drop response stream 1 -> disconnect guard releases session slot
    drop(resp1);
    tokio::time::sleep(Duration::from_millis(30)).await;
    server.prune_expired_and_closed_sessions().await;
    assert_eq!(
        server.active_session_count().await,
        1,
        "Dropping SSE response body must clean up disconnected session"
    );

    // Wait for TTL expiry (120ms) -> remaining session expires and is cleaned up
    tokio::time::sleep(Duration::from_millis(150)).await;
    server.prune_expired_and_closed_sessions().await;
    assert_eq!(
        server.active_session_count().await,
        0,
        "Expired SSE sessions must be pruned after TTL"
    );
}
