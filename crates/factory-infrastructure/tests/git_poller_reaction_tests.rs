use chrono::Utc;
use factory_core::PRDirective;
use factory_infrastructure::cursor_store::InMemoryCursorStore;
use factory_infrastructure::git_poller::GitPlatformPoller;
use factory_infrastructure::github::{GithubComment, GithubPullRequest, GithubReaction, GithubUser, MockGithubClient};
use factory_infrastructure::gitlab::{
    GitlabAuthor, GitlabAwardEmoji, GitlabMergeRequest, GitlabNote, MockGitlabClient,
};
use std::sync::Arc;

#[tokio::test]
async fn test_gitlab_poller_applies_eyes_reaction_and_ignores_bot() {
    let mut mock_gitlab = MockGitlabClient::new();

    let mr = GitlabMergeRequest {
        id: 1,
        iid: 10,
        title: "Feature branch MR".to_string(),
        description: None,
        web_url: "https://gitlab.com/repo/mrs/10".to_string(),
        state: "opened".to_string(),
        updated_at: Some(Utc::now()),
    };

    let user_note = GitlabNote {
        id: 101,
        body: "Hey @darkgravity can you check this code?".to_string(),
        author: GitlabAuthor {
            username: "developer".to_string(),
        },
        updated_at: Some(Utc::now()),
    };

    let bot_note = GitlabNote {
        id: 102,
        body: "Here is your analysis @darkgravity".to_string(),
        author: GitlabAuthor {
            username: "darkgravity-bot".to_string(),
        },
        updated_at: Some(Utc::now()),
    };

    mock_gitlab
        .expect_list_active_merge_requests()
        .with(mockall::predicate::eq("my-proj"))
        .returning(move |_| Ok(vec![mr.clone()]));

    mock_gitlab
        .expect_list_merge_request_notes()
        .with(
            mockall::predicate::eq("my-proj"),
            mockall::predicate::eq(10),
            mockall::predicate::always(),
        )
        .returning(move |_, _, _| Ok(vec![user_note.clone(), bot_note.clone()]));

    // Expect reaction ONLY for user_note, NOT bot_note!
    mock_gitlab
        .expect_add_merge_request_note_award_emoji()
        .with(
            mockall::predicate::eq("my-proj"),
            mockall::predicate::eq(10),
            mockall::predicate::eq(101),
            mockall::predicate::eq("eyes"),
        )
        .times(1)
        .returning(|_, _, _, _| {
            Ok(GitlabAwardEmoji {
                id: 1,
                name: "eyes".to_string(),
                user: GitlabAuthor {
                    username: "darkgravity-bot".to_string(),
                },
            })
        });

    let cursor_store = Arc::new(InMemoryCursorStore::new());
    let poller = GitPlatformPoller::new(None, Some(Arc::new(mock_gitlab)), cursor_store)
        .with_bot_username("darkgravity-bot");

    let events = poller
        .poll_gitlab_mr_notes("my-proj")
        .await
        .expect("polling should succeed");

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].comment_id, 101);
    assert_eq!(events[0].author, "developer");
    assert_eq!(
        events[0].directive,
        PRDirective::Interact {
            prompt: "can you check this code?".to_string()
        }
    );
}

#[tokio::test]
async fn test_github_poller_applies_eyes_reaction_and_ignores_bot() {
    let mut mock_github = MockGithubClient::new();

    let pr = GithubPullRequest {
        id: 5,
        number: 20,
        title: "Add awesome feature".to_string(),
        body: None,
        state: "open".to_string(),
        html_url: "https://github.com/owner/repo/pull/20".to_string(),
        updated_at: Some(Utc::now()),
    };

    let user_comment = GithubComment {
        id: 201,
        body: "@antigravity /validate".to_string(),
        user: GithubUser {
            login: "developer".to_string(),
        },
        html_url: "https://github.com/owner/repo/pull/20#issuecomment-201".to_string(),
        updated_at: Some(Utc::now()),
    };

    let bot_comment = GithubComment {
        id: 202,
        body: "@darkgravity summary posted".to_string(),
        user: GithubUser {
            login: "darkgravity-bot".to_string(),
        },
        html_url: "https://github.com/owner/repo/pull/20#issuecomment-202".to_string(),
        updated_at: Some(Utc::now()),
    };

    mock_github
        .expect_list_active_pull_requests()
        .with(mockall::predicate::eq("owner/repo"))
        .returning(move |_| Ok(vec![pr.clone()]));

    mock_github
        .expect_list_pull_request_comments()
        .with(
            mockall::predicate::eq("owner/repo"),
            mockall::predicate::eq(20),
            mockall::predicate::always(),
        )
        .returning(move |_, _, _| Ok(vec![user_comment.clone(), bot_comment.clone()]));

    // Expect reaction ONLY for user_comment, NOT bot_comment!
    mock_github
        .expect_add_comment_reaction()
        .with(
            mockall::predicate::eq("owner/repo"),
            mockall::predicate::eq(201),
            mockall::predicate::eq("eyes"),
        )
        .times(1)
        .returning(|_, _, _| {
            Ok(GithubReaction {
                id: 1,
                content: "eyes".to_string(),
                user: Some(GithubUser {
                    login: "darkgravity-bot".to_string(),
                }),
            })
        });

    let cursor_store = Arc::new(InMemoryCursorStore::new());
    let poller = GitPlatformPoller::new(Some(Arc::new(mock_github)), None, cursor_store)
        .with_bot_username("darkgravity-bot");

    let events = poller
        .poll_github_pr_comments("owner/repo")
        .await
        .expect("polling should succeed");

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].comment_id, 201);
    assert_eq!(events[0].author, "developer");
    assert_eq!(events[0].directive, PRDirective::Validate);
}
