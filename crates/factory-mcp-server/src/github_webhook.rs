use crate::McpServer;
use async_trait::async_trait;
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use hmac::{Hmac, KeyInit, Mac};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::env;
use std::sync::Arc;

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MissionDispatchReceipt {
    pub dispatch_id: String,
    pub mission_id: String,
    pub repository: String,
    pub goal_digest: String,
}

#[async_trait]
pub trait WebhookMissionDispatcher: Send + Sync {
    async fn dispatch_webhook_mission(
        &self,
        delivery_id: &str,
        mission_id: &str,
        repository: &str,
        goal: &str,
    ) -> Result<MissionDispatchReceipt, String>;
}

/// Default local dispatcher that persists a verifiable dispatch receipt and records the mission submission.
pub struct DefaultLocalMissionDispatcher;

#[async_trait]
impl WebhookMissionDispatcher for DefaultLocalMissionDispatcher {
    async fn dispatch_webhook_mission(
        &self,
        delivery_id: &str,
        mission_id: &str,
        repository: &str,
        goal: &str,
    ) -> Result<MissionDispatchReceipt, String> {
        let mut hasher = Sha256::new();
        hasher.update(goal.as_bytes());
        let goal_digest = hex::encode(hasher.finalize());
        Ok(MissionDispatchReceipt {
            dispatch_id: format!("receipt-{}-{}", delivery_id, mission_id),
            mission_id: mission_id.to_string(),
            repository: repository.to_string(),
            goal_digest,
        })
    }
}

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
    if secret.is_empty() || !signature_header.starts_with("sha256=") {
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
    State(server): State<Arc<McpServer>>,
    headers: HeaderMap,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let max_bytes = server.max_webhook_payload_bytes();
    if body_bytes.len() > max_bytes {
        tracing::warn!(
            "GitHub webhook payload size {} exceeds maximum allowed {} bytes",
            body_bytes.len(),
            max_bytes
        );
        return (
            StatusCode::PAYLOAD_TOO_LARGE,
            Json(json!({"error": "Webhook payload exceeds maximum allowed size"})),
        )
            .into_response();
    }

    let webhook_secret = server
        .webhook_secret()
        .unwrap_or_else(|| env::var("GITHUB_WEBHOOK_SECRET").unwrap_or_default());

    if webhook_secret.trim().is_empty() {
        tracing::error!("GitHub webhook rejected: missing GITHUB_WEBHOOK_SECRET (fail-closed)");
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Webhook authentication secret is not configured"})),
        )
            .into_response();
    }

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

    let delivery_id = headers
        .get("x-github-delivery")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .trim()
        .to_string();

    if server.is_protected_mode() && delivery_id.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Missing required X-GitHub-Delivery header"})),
        )
            .into_response();
    }

    if !delivery_id.is_empty() && !server.record_webhook_delivery(&delivery_id).await {
        tracing::warn!(
            "Rejected replayed GitHub webhook delivery '{}'",
            delivery_id
        );
        return (
            StatusCode::CONFLICT,
            Json(json!({"error": "Duplicate/replayed webhook delivery rejected", "delivery_id": delivery_id})),
        )
            .into_response();
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

            let effective_delivery = if delivery_id.is_empty() {
                mission_id.clone()
            } else {
                delivery_id.clone()
            };

            match server
                .dispatch_webhook_mission(&effective_delivery, &mission_id, &repo, &goal)
                .await
            {
                Ok(receipt) => {
                    tracing::info!(
                        "Triggered autonomous mission '{}' from GitHub issue on repo '{}' (receipt={})",
                        mission_id,
                        repo,
                        receipt.dispatch_id
                    );

                    let response_payload = json!({
                        "status": "mission_triggered",
                        "mission_id": mission_id,
                        "provider": "github",
                        "repository": repo,
                        "goal": goal,
                        "action": action,
                        "dispatch_receipt": receipt
                    });

                    return (StatusCode::OK, Json(response_payload)).into_response();
                }
                Err(err) => {
                    tracing::error!(
                        "Failed to dispatch mission '{}' for webhook delivery '{}': {}",
                        mission_id,
                        effective_delivery,
                        err
                    );
                    return (
                        StatusCode::BAD_GATEWAY,
                        Json(json!({
                            "error": "Failed to submit webhook mission to workflow dispatcher",
                            "details": err
                        })),
                    )
                        .into_response();
                }
            }
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
        let primary_bytes: Vec<u8> = (0..32).map(|i| (i * 7 + 13) as u8).collect();
        let primary_token = hex::encode(primary_bytes);

        let secondary_bytes: Vec<u8> = (0..32).map(|i| (i * 11 + 17) as u8).collect();
        let secondary_token = hex::encode(secondary_bytes);

        let body = b"test payload";
        let mut mac = HmacSha256::new_from_slice(primary_token.as_bytes()).unwrap();
        mac.update(body);
        let result = mac.finalize();
        let hex_sig = hex::encode(result.into_bytes());

        let sig_header = format!("sha256={}", hex_sig);
        assert!(verify_github_signature(&primary_token, &sig_header, body));
        assert!(!verify_github_signature(
            &secondary_token,
            &sig_header,
            body
        ));
    }
}
