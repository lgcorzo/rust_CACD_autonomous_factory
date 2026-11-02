use factory_core::security::SecurityBounds;
use factory_infrastructure::vault::VaultSecurityBounds;
use factory_infrastructure::vault_test_fixtures::MockVaultServer;
use serde_json::json;

#[tokio::test]
async fn test_cross_repository_acl_isolation_enforcement() {
    let mock_vault = MockVaultServer::start().await;
    let bounds = VaultSecurityBounds::new(mock_vault.uri(), "valid-role-token".to_string());

    let repo_a = "org/repo-a";
    let _repo_b = "org/repo-b";
    let token_a = "s.token-for-repo-a";

    // 1. Mock token issuance for repo-a
    mock_vault.mock_token_create(repo_a, "agent-01", token_a).await;

    // 2. Mock token lookup for token_a (valid, policy repo-org-repo-a attached)
    mock_vault
        .mock_token_lookup(token_a, repo_a, "agent-01", false, 300, 200)
        .await;

    // 3. Mock secret reading:
    // Path inside repo-a: secret/data/repos/org/repo-a/deploy-key -> ALLOWED (200)
    // Path inside repo-b: secret/data/repos/org/repo-b/deploy-key -> DENIED (403 Forbidden by ACL)
    mock_vault
        .mock_secret_read(
            token_a,
            "secret/data/repos/org/repo-a/deploy-key",
            "secret/data/repos/org/repo-a/",
            json!({"key": "repo-a-private-key"}),
        )
        .await;

    mock_vault
        .mock_secret_read(
            token_a,
            "secret/data/repos/org/repo-b/deploy-key",
            "secret/data/repos/org/repo-a/",
            json!({"key": "repo-b-private-key"}),
        )
        .await;

    // Issue token for repo-a
    let jit_token = bounds
        .issue_jit_token_for_repo(repo_a, "agent-01")
        .await
        .expect("token creation should succeed");

    // Validate token against expected audience/repo policy
    let is_valid = bounds.validate_token(&jit_token).await.expect("validate ok");
    assert!(is_valid, "Token should be valid for repo-a");

    // Attempt to read secret under repo-a path (should succeed)
    let secret_a = bounds
        .read_secret(&jit_token, "secret/data/repos/org/repo-a/deploy-key")
        .await;
    assert!(secret_a.is_ok(), "Access to own repository secrets must succeed");

    // Attempt cross-repository access under repo-b path (MUST fail with 403 / ACL denial)
    let secret_b = bounds
        .read_secret(&jit_token, "secret/data/repos/org/repo-b/deploy-key")
        .await;
    assert!(
        secret_b.is_err(),
        "Cross-repository secret access must fail by Vault ACL policy"
    );
}

