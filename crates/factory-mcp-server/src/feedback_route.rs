use crate::McpServer;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use factory_core::UserFeedbackPayload;
use factory_infrastructure::HttpGitlabClient;
use std::sync::Arc;

pub async fn handle_feedback(
    State(_server): State<Arc<McpServer>>,
    Json(payload): Json<UserFeedbackPayload>,
) -> impl IntoResponse {
    tracing::info!("Received feedback payload from user {}", payload.user_id);

    let gitlab_url =
        std::env::var("GITLAB_URL").unwrap_or_else(|_| "https://gitlab.com".to_string());
    let gitlab_token = std::env::var("GITLAB_TOKEN").unwrap_or_default();
    let gitlab_project_id = std::env::var("GITLAB_PROJECT_ID").unwrap_or_default();

    if gitlab_token.is_empty() || gitlab_project_id.is_empty() {
        tracing::error!("GitLab configuration missing. Cannot process feedback.");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Server misconfiguration").into_response();
    }

    let gitlab_client = Arc::new(HttpGitlabClient::new(gitlab_url, gitlab_token));

    if payload.sentiment.to_lowercase() == "bug" || payload.sentiment.to_lowercase() == "negative" {
        let title = format!(
            "[Feedback] {} from {}",
            payload.sentiment.to_uppercase(),
            payload.user_id
        );

        let description = format!(
            "## In-App User Feedback Report\n\n\
             **User ID:** `{}`\n\
             **Session ID:** {}\n\
             **Sentiment:** {}\n\n\
             ### Feedback Text\n\
             {}\n\n\
             ### Instructions for Agent\n\
             Please review this user feedback and address any underlying issues.\n\n\
             [RESOURCE_LIMIT: RAM <= 30Mi]",
            payload.user_id,
            payload.session_id.unwrap_or_else(|| "Unknown".to_string()),
            payload.sentiment,
            payload.feedback_text
        );

        use factory_infrastructure::GitlabClient;
        match gitlab_client
            .create_issue(&gitlab_project_id, &title, &description)
            .await
        {
            Ok(issue) => {
                tracing::info!(
                    "Successfully created GitLab issue for feedback: {}",
                    issue.web_url
                );
                (StatusCode::OK, "Feedback processed and issue created").into_response()
            }
            Err(e) => {
                tracing::error!("Failed to create GitLab issue for feedback: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to process feedback",
                )
                    .into_response()
            }
        }
    } else {
        tracing::info!(
            "Feedback sentiment '{}' does not require automatic issue generation.",
            payload.sentiment
        );
        (StatusCode::OK, "Feedback processed").into_response()
    }
}
