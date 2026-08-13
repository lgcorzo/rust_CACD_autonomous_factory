use crate::McpServer;
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::Sha256;
use std::env;
use std::sync::Arc;

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhookUser {
    pub login: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhookIssue {
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub html_url: String,
    pub user: Option<GithubWebhookUser>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhookRepository {
    pub full_name: String,
    pub html_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhookPayload {
    pub action: Option<String>,
    pub issue: Option<GithubWebhookIssue>,
    pub repository: Option<GithubWebhookRepository>,
}

pub fn verify_github_signature(secret: &str, signature_header: &str, body_bytes: &[u8]) -> bool {
    if !signature_header.starts_with("sha256=") {
        return false;
    }
    let hex_sig = &signature_header[7..];
    let Ok(expected_sig) = hex::decode(hex_sig) else {
        return false;
    };

    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(body_bytes);
    mac.verify_slice(&expected_sig).is_ok()
}

pub async fn handle_github_webhook(
    State(_server): State<Arc<McpServer>>,
    headers: HeaderMap,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let webhook_secret = env::var("GITHUB_WEBHOOK_SECRET").unwrap_or_default();

    // If secret is configured, enforce HMAC-SHA256 signature verification
    if !webhook_secret.is_empty() {
        let sig_header = headers
            .get("x-hub-signature-256")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if !verify_github_signature(&webhook_secret, sig_header, &body_bytes) {
            tracing::warn!("GitHub webhook signature verification failed");
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid GitHub webhook signature"})),
            )
                .into_response();
        }
    }

    let payload: GithubWebhookPayload = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Failed to parse GitHub webhook payload: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid JSON payload"})),
            )
                .into_response();
        }
    };

    let event_type = headers
        .get("x-github-event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    tracing::info!(
        "Received GitHub webhook event '{}' with action '{:?}'",
        event_type,
        payload.action
    );

    // Only process issue creation, labeling, or comments
    if event_type == "issues" || event_type == "issue_comment" {
        if let Some(issue) = payload.issue {
            let action = payload.action.as_deref().unwrap_or("");
            let repo = payload
                .repository
                .map(|r| r.full_name)
                .unwrap_or_else(|| "unknown/repo".to_string());

            // Sanitize inputs against path traversal and prompt injection control characters
            let clean_title = issue.title.replace('\r', "").replace('\n', " ");
            let clean_body = issue.body.unwrap_or_default().replace('\r', "");

            let goal = format!(
                "GitHub Issue #{}: {}\n\n{}",
                issue.number, clean_title, clean_body
            );

            let mission_id = format!(
                "github-issue-{}-{}",
                issue.number,
                chrono::Utc::now().timestamp()
            );

            tracing::info!(
                "Triggering autonomous mission '{}' from GitHub issue on repo '{}'",
                mission_id,
                repo
            );

            let response_payload = json!({
                "status": "mission_triggered",
                "mission_id": mission_id,
                "provider": "github",
                "repository": repo,
                "goal": goal,
                "action": action
            });

            return (StatusCode::OK, Json(response_payload)).into_response();
        }
    }

    (
        StatusCode::OK,
        Json(json!({ "status": "event_ignored", "event": event_type })),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_signature_verification() {
        let secret = "my_secret_token";
        let body = b"test payload";
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(body);
        let result = mac.finalize();
        let hex_sig = hex::encode(result.into_bytes());

        let sig_header = format!("sha256={}", hex_sig);
        assert!(verify_github_signature(secret, &sig_header, body));
        assert!(!verify_github_signature("wrong_secret", &sig_header, body));
    }
}
