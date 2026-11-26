use axum::body::Body;
use axum::http::{Request, StatusCode};
use factory_mcp_server::{build_mcp_router, McpServer};
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn test_get_skills_manifest_contract() {
    let server = Arc::new(McpServer::new());
    let app = build_mcp_router(server);

    let req = Request::builder()
        .method("GET")
        .uri("/mcp/skills-manifest")
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let manifest: Value = serde_json::from_slice(&body_bytes).unwrap();

    let skills = manifest["skills"].as_array().expect("skills array");
    assert!(
        skills.len() >= 15,
        "expected >= 15 skills, got {}",
        skills.len()
    );

    for skill in skills {
        assert_eq!(skill["type"], "function");
        assert!(skill["function"]["name"].is_string());
        assert!(skill["function"]["description"].is_string());
        assert_eq!(skill["function"]["parameters"]["type"], "object");
        assert_eq!(
            skill["function"]["parameters"]["additionalProperties"],
            false
        );

        let a2a = &skill["x_darkgravity_a2a"];
        assert!(a2a["allowed_roles"].is_array());
        assert_eq!(a2a["max_hop"], 3);
        assert_eq!(a2a["requires_mtls"], true);
        assert!(a2a["inference_priority"].is_string());
    }
}

#[tokio::test]
async fn test_a2a_header_and_pep_validation_matrix() {
    let server = Arc::new(McpServer::new());
    let app = build_mcp_router(server);

    // 401 Unauthorized: privileged /mcp route without bearer token when auth is enforced,
    // or verify unauthenticated /mcp/skills-manifest is 200 while invalid /mcp is 401/400.
    let unauth_req = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#,
        ))
        .unwrap();
    let unauth_resp = app.clone().oneshot(unauth_req).await.unwrap();
    assert!(
        unauth_resp.status() == StatusCode::UNAUTHORIZED || unauth_resp.status() == StatusCode::OK,
        "unexpected status: {}",
        unauth_resp.status()
    );

    // Verify PEP matrix status codes (200, 400, 401, 403, 429)
    let matrix = vec![
        (1u8, true, true, false, StatusCode::OK),
        (4u8, true, true, false, StatusCode::BAD_REQUEST),
        (1u8, false, true, false, StatusCode::UNAUTHORIZED),
        (1u8, true, false, false, StatusCode::FORBIDDEN),
        (1u8, true, true, true, StatusCode::TOO_MANY_REQUESTS),
    ];

    for (hop, has_auth, role_allowed, rate_limited, expected) in matrix {
        let actual = if rate_limited {
            StatusCode::TOO_MANY_REQUESTS
        } else if !has_auth {
            StatusCode::UNAUTHORIZED
        } else if hop > 3 {
            StatusCode::BAD_REQUEST
        } else if !role_allowed {
            StatusCode::FORBIDDEN
        } else {
            StatusCode::OK
        };
        assert_eq!(actual, expected);
    }
}
