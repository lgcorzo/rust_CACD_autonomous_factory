use crate::McpServer;
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

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

pub async fn handle_github_webhook(
    State(_server): State<Arc<McpServer>>,
    headers: HeaderMap,
    Json(payload): Json<GithubWebhookPayload>,
) -> impl IntoResponse {
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

            let goal = format!(
                "GitHub Issue #{}: {}\n\n{}",
                issue.number,
                issue.title,
                issue.body.unwrap_or_default()
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
    use axum::http::HeaderValue;

    #[tokio::test]
    async fn test_github_webhook_handler() {
        let server = Arc::new(McpServer::new());
        let mut headers = HeaderMap::new();
        headers.insert("x-github-event", HeaderValue::from_static("issues"));

        let payload = GithubWebhookPayload {
            action: Some("opened".to_string()),
            issue: Some(GithubWebhookIssue {
                number: 101,
                title: "Autonomous mission test from GitHub".to_string(),
                body: Some("Description of GitHub mission".to_string()),
                html_url: "https://github.com/my-org/my-repo/issues/101".to_string(),
                user: Some(GithubWebhookUser {
                    login: "test-user".to_string(),
                }),
            }),
            repository: Some(GithubWebhookRepository {
                full_name: "my-org/my-repo".to_string(),
                html_url: "https://github.com/my-org/my-repo".to_string(),
            }),
        };

        let response = handle_github_webhook(State(server), headers, Json(payload))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
