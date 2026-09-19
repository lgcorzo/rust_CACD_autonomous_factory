use chrono::Utc;
use factory_application::workflows::comment_control::{CommentControlInput, CommentControlService};
use factory_core::{PRCommentEvent, PRDirective};
use factory_infrastructure::aethalgard::MockAethalgardClient;
use factory_infrastructure::github::{GithubComment, GithubUser, MockGithubClient};
use factory_infrastructure::gitlab::{GitlabAuthor, GitlabNote, MockGitlabClient};
use factory_infrastructure::mcp_client::MockMcpClient;
use factory_infrastructure::r2r::MockR2rClient;
use std::sync::Arc;

#[tokio::test]
async fn test_comment_control_interact_and_validate_directives() {
    let mut mock_gh = MockGithubClient::new();
    mock_gh
        .expect_post_pull_request_comment()
        .with(
            mockall::predicate::eq("my-org/my-repo"),
            mockall::predicate::eq(42),
            mockall::predicate::always(),
        )
        .returning(|_repo, _pr, body| {
            Ok(GithubComment {
                id: 1001,
                body: body.to_string(),
                user: GithubUser {
                    login: "darkgravity-bot".to_string(),
                },
                html_url: "https://github.com/my-org/my-repo/pull/42#issuecomment-1001".to_string(),
                updated_at: Some(Utc::now()),
            })
        });

    let mut mock_gl = MockGitlabClient::new();
    mock_gl
        .expect_post_merge_request_note()
        .with(
            mockall::predicate::eq("gitlab-org/gl-repo"),
            mockall::predicate::eq(88),
            mockall::predicate::always(),
        )
        .returning(|_repo, _mr, body| {
            Ok(GitlabNote {
                id: 2002,
                body: body.to_string(),
                author: GitlabAuthor {
                    username: "darkgravity-bot".to_string(),
                },
                updated_at: Some(Utc::now()),
            })
        });

    let mut mock_mcp = MockMcpClient::new();
    mock_mcp.expect_call_tool_json().returning(|_tool, _args| {
        Ok(serde_json::json!({
            "is_error": false,
            "content": [{"type": "text", "text": "{\"status\": \"ok\"}"}]
        }))
    });

    let mut mock_r2r = MockR2rClient::new();
    mock_r2r
        .expect_search()
        .returning(|_| Ok("Retrieved architecture context".to_string()));

    let service = CommentControlService::new(
        Some(Arc::new(mock_gh)),
        Some(Arc::new(mock_gl)),
        Arc::new(mock_mcp),
        Arc::new(mock_r2r),
        Arc::new(MockAethalgardClient::new()),
    );

    // Test 1: GitHub Interact Directive
    let input_interact = CommentControlInput {
        event: PRCommentEvent {
            source_platform: "github".to_string(),
            repository: "my-org/my-repo".to_string(),
            pr_number: 42,
            comment_id: 501,
            author: "carol".to_string(),
            body: "@darkgravity can you explain the architectural impact?".to_string(),
            directive: PRDirective::Interact {
                prompt: "can you explain the architectural impact?".to_string(),
            },
            updated_at: Utc::now(),
            html_url: "https://github.com/my-org/my-repo/pull/42#501".to_string(),
            thread_context: vec![],
        },
    };

    let output_interact = service.handle_directive(&input_interact).await.unwrap();
    assert_eq!(output_interact.directive_type, "interact");
    assert!(output_interact.comment_posted);
    assert!(
        output_interact
            .response_body
            .contains("can you explain the architectural impact?")
    );

    // Test 2: GitLab Validate Directive
    let input_validate = CommentControlInput {
        event: PRCommentEvent {
            source_platform: "gitlab".to_string(),
            repository: "gitlab-org/gl-repo".to_string(),
            pr_number: 88,
            comment_id: 502,
            author: "dan".to_string(),
            body: "@darkgravity /validate".to_string(),
            directive: PRDirective::Validate,
            updated_at: Utc::now(),
            html_url: "https://gitlab.com/gitlab-org/gl-repo/-/merge_requests/88#note_502"
                .to_string(),
            thread_context: vec![],
        },
    };

    let output_validate = service.handle_directive(&input_validate).await.unwrap();
    assert_eq!(output_validate.directive_type, "validate");
    assert!(output_validate.comment_posted);
    assert!(output_validate.response_body.contains("validation"));
}
