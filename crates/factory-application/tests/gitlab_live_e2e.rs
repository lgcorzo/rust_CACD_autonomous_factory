use factory_application::gitlab_verifier::GitlabVerifier;
use factory_infrastructure::gitlab::HttpGitlabClient;
use std::sync::Arc;

/// Live Integration Test for GitLab Communication
/// Automatically runs against real GitLab endpoints if `GITLAB_API_TOKEN` is present in the environment.
/// Gracefully skips when running in unauthenticated CI/offline environments.
#[tokio::test]
async fn test_live_gitlab_operational_verification() {
    let token = std::env::var("GITLAB_API_TOKEN").unwrap_or_default();
    let url = std::env::var("GITLAB_URL").unwrap_or_else(|_| "https://gitlab.com".to_string());
    let projects_raw = std::env::var("GITLAB_PROJECTS")
        .unwrap_or_else(|_| "lgcorzo/fastapi-autogen-team".to_string());

    if token.trim().is_empty() {
        eprintln!(
            "SKIPPED test_live_gitlab_operational_verification: GITLAB_API_TOKEN environment variable not set."
        );
        return;
    }

    let projects: Vec<String> = projects_raw
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let client = Arc::new(HttpGitlabClient::new(url.clone(), token));
    let verifier = GitlabVerifier::new(client, url);

    let report = verifier.verify_all(&projects).await;

    println!("=== Live GitLab Verification Report ===");
    println!("Overall Status: {:?}", report.overall_status);
    println!("Execution Duration: {}ms", report.execution_duration_ms);
    for check in &report.checks {
        println!(
            "  [{:?}] {}: {} ({})",
            check.status, check.check_name, check.message, check.target
        );
        if let Some(hint) = &check.remediation_hint {
            println!("       Remediation: {}", hint);
        }
    }

    assert!(
        !report.checks.is_empty(),
        "Expected at least one check to be run"
    );
    assert!(report.execution_duration_ms > 0);
}
