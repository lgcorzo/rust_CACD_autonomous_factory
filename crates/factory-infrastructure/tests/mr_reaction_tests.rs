use factory_infrastructure::github::{GithubClient, HttpGithubClient};
use factory_infrastructure::gitlab::{GitlabClient, HttpGitlabClient};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_gitlab_add_merge_request_note_award_emoji() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 42,
        "name": "eyes",
        "user": {
            "username": "darkgravity-bot"
        }
    });

    Mock::given(method("POST"))
        .and(path(
            "/api/v4/projects/my-group%2Fmy-project/merge_requests/12/notes/34/award_emoji",
        ))
        .and(header("PRIVATE-TOKEN", "gl-token-123"))
        .respond_with(ResponseTemplate::new(201).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client = HttpGitlabClient::new(mock_server.uri(), "gl-token-123".to_string());
    let emoji = client
        .add_merge_request_note_award_emoji("my-group/my-project", 12, 34, "eyes")
        .await
        .expect("should add award emoji successfully");

    assert_eq!(emoji.id, 42);
    assert_eq!(emoji.name, "eyes");
    assert_eq!(emoji.user.username, "darkgravity-bot");
}

#[tokio::test]
async fn test_github_add_comment_reaction() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 99,
        "content": "eyes",
        "user": {
            "login": "darkgravity-bot"
        }
    });

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/issues/comments/567/reactions"))
        .and(header("Authorization", "Bearer gh-token-456"))
        .and(header("Accept", "application/vnd.github+json"))
        .respond_with(ResponseTemplate::new(201).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let client = HttpGithubClient::with_url(mock_server.uri(), "gh-token-456".to_string());
    let reaction = client
        .add_comment_reaction("owner/repo", 567, "eyes")
        .await
        .expect("should add github reaction successfully");

    assert_eq!(reaction.id, 99);
    assert_eq!(reaction.content, "eyes");
    assert_eq!(
        reaction.user.as_ref().map(|u| u.login.as_str()),
        Some("darkgravity-bot")
    );
}
