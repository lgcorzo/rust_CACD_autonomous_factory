use chrono::{DateTime, Utc};
use factory_infrastructure::gitlab::GitlabClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum CheckStatus {
    Success,
    Warning,
    Failure,
    Skipped,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum VerificationStatus {
    #[serde(rename = "PASSED")]
    Passed,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "PARTIAL")]
    Partial,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitlabCheckResult {
    pub check_name: String,
    pub target: String,
    pub status: CheckStatus,
    pub latency_ms: u64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_hint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitlabVerificationReport {
    pub timestamp: DateTime<Utc>,
    pub gitlab_url: String,
    pub target_projects: Vec<String>,
    pub overall_status: VerificationStatus,
    pub checks: Vec<GitlabCheckResult>,
    pub execution_duration_ms: u64,
}

pub struct GitlabVerifier {
    gitlab_client: Arc<dyn GitlabClient>,
    gitlab_url: String,
}

impl GitlabVerifier {
    pub fn new(gitlab_client: Arc<dyn GitlabClient>, gitlab_url: String) -> Self {
        Self {
            gitlab_client,
            gitlab_url,
        }
    }

    pub async fn verify_all(&self, projects: &[String]) -> GitlabVerificationReport {
        let start_time = Instant::now();
        let mut checks = Vec::new();

        // 1. Authentication Handshake Check
        let auth_start = Instant::now();
        match self.gitlab_client.check_current_user().await {
            Ok(user) => {
                checks.push(GitlabCheckResult {
                    check_name: "GitLab Authentication".to_string(),
                    target: self.gitlab_url.clone(),
                    status: CheckStatus::Success,
                    latency_ms: auth_start.elapsed().as_millis() as u64,
                    message: format!("Authenticated successfully as user '{}'", user.username),
                    remediation_hint: None,
                });
            }
            Err(e) => {
                checks.push(GitlabCheckResult {
                    check_name: "GitLab Authentication".to_string(),
                    target: self.gitlab_url.clone(),
                    status: CheckStatus::Failure,
                    latency_ms: auth_start.elapsed().as_millis() as u64,
                    message: format!("Authentication failed: {}", e),
                    remediation_hint: Some(
                        "Verify GITLAB_API_TOKEN is set, valid, and has 'api' or 'read_api' scopes."
                            .to_string(),
                    ),
                });
            }
        }

        // 2. Per-Project Verification Checks
        for project in projects {
            let project_trimmed = project.trim();
            if project_trimmed.is_empty() {
                continue;
            }

            // A. Project Metadata & Access Check
            let proj_start = Instant::now();
            match self.gitlab_client.get_project(project_trimmed).await {
                Ok(true) => {
                    checks.push(GitlabCheckResult {
                        check_name: "Project Metadata Access".to_string(),
                        target: project_trimmed.to_string(),
                        status: CheckStatus::Success,
                        latency_ms: proj_start.elapsed().as_millis() as u64,
                        message: "Project metadata accessible".to_string(),
                        remediation_hint: None,
                    });
                }
                Ok(false) => {
                    checks.push(GitlabCheckResult {
                        check_name: "Project Metadata Access".to_string(),
                        target: project_trimmed.to_string(),
                        status: CheckStatus::Failure,
                        latency_ms: proj_start.elapsed().as_millis() as u64,
                        message: "Project not found (404)".to_string(),
                        remediation_hint: Some(
                            "Check repository path and verify bot user has project membership."
                                .to_string(),
                        ),
                    });
                }
                Err(e) => {
                    checks.push(GitlabCheckResult {
                        check_name: "Project Metadata Access".to_string(),
                        target: project_trimmed.to_string(),
                        status: CheckStatus::Failure,
                        latency_ms: proj_start.elapsed().as_millis() as u64,
                        message: format!("Failed to retrieve project: {}", e),
                        remediation_hint: Some(
                            "Verify network connectivity and project permissions.".to_string(),
                        ),
                    });
                }
            }

            // B. Issues Polling Check
            let issue_start = Instant::now();
            match self
                .gitlab_client
                .list_open_issues(project_trimmed, Some("autonomous-mission".to_string()))
                .await
            {
                Ok(issues) => {
                    checks.push(GitlabCheckResult {
                        check_name: "Open Issues Polling".to_string(),
                        target: project_trimmed.to_string(),
                        status: CheckStatus::Success,
                        latency_ms: issue_start.elapsed().as_millis() as u64,
                        message: format!(
                            "Successfully polled issues (found {} labeled 'autonomous-mission')",
                            issues.len()
                        ),
                        remediation_hint: None,
                    });
                }
                Err(e) => {
                    checks.push(GitlabCheckResult {
                        check_name: "Open Issues Polling".to_string(),
                        target: project_trimmed.to_string(),
                        status: CheckStatus::Failure,
                        latency_ms: issue_start.elapsed().as_millis() as u64,
                        message: format!("Failed to query issues: {}", e),
                        remediation_hint: Some(
                            "Verify API token has 'read_api' permission on issues.".to_string(),
                        ),
                    });
                }
            }

            // C. Active Merge Requests Polling Check
            let mr_start = Instant::now();
            match self
                .gitlab_client
                .list_active_merge_requests(project_trimmed)
                .await
            {
                Ok(mrs) => {
                    checks.push(GitlabCheckResult {
                        check_name: "Active Merge Requests Polling".to_string(),
                        target: project_trimmed.to_string(),
                        status: CheckStatus::Success,
                        latency_ms: mr_start.elapsed().as_millis() as u64,
                        message: format!(
                            "Successfully queried active MRs (found {} opened)",
                            mrs.len()
                        ),
                        remediation_hint: None,
                    });

                    // D. MR Notes Capability Check (using first active MR if any)
                    let note_start = Instant::now();
                    if let Some(first_mr) = mrs.first() {
                        match self
                            .gitlab_client
                            .list_merge_request_notes(project_trimmed, first_mr.iid, None)
                            .await
                        {
                            Ok(notes) => {
                                checks.push(GitlabCheckResult {
                                    check_name: "MR Discussion Notes Access".to_string(),
                                    target: format!("{}!{}", project_trimmed, first_mr.iid),
                                    status: CheckStatus::Success,
                                    latency_ms: note_start.elapsed().as_millis() as u64,
                                    message: format!(
                                        "Successfully read MR discussion notes (found {})",
                                        notes.len()
                                    ),
                                    remediation_hint: None,
                                });
                            }
                            Err(e) => {
                                checks.push(GitlabCheckResult {
                                    check_name: "MR Discussion Notes Access".to_string(),
                                    target: format!("{}!{}", project_trimmed, first_mr.iid),
                                    status: CheckStatus::Warning,
                                    latency_ms: note_start.elapsed().as_millis() as u64,
                                    message: format!("Failed to read MR notes: {}", e),
                                    remediation_hint: Some(
                                        "Verify bot user has Developer or Reporter role on MR discussions."
                                            .to_string(),
                                    ),
                                });
                            }
                        }
                    } else {
                        checks.push(GitlabCheckResult {
                            check_name: "MR Discussion Notes Access".to_string(),
                            target: project_trimmed.to_string(),
                            status: CheckStatus::Skipped,
                            latency_ms: 0,
                            message: "No active merge requests to probe discussion notes"
                                .to_string(),
                            remediation_hint: None,
                        });
                    }
                }
                Err(e) => {
                    checks.push(GitlabCheckResult {
                        check_name: "Active Merge Requests Polling".to_string(),
                        target: project_trimmed.to_string(),
                        status: CheckStatus::Failure,
                        latency_ms: mr_start.elapsed().as_millis() as u64,
                        message: format!("Failed to list merge requests: {}", e),
                        remediation_hint: Some(
                            "Verify API token has 'read_api' permission for merge requests."
                                .to_string(),
                        ),
                    });
                }
            }
        }

        // Determine overall status
        let has_failure = checks.iter().any(|c| c.status == CheckStatus::Failure);
        let has_warning = checks.iter().any(|c| c.status == CheckStatus::Warning);

        let overall_status = if has_failure {
            VerificationStatus::Failed
        } else if has_warning {
            VerificationStatus::Partial
        } else {
            VerificationStatus::Passed
        };

        GitlabVerificationReport {
            timestamp: Utc::now(),
            gitlab_url: self.gitlab_url.clone(),
            target_projects: projects.to_vec(),
            overall_status,
            checks,
            execution_duration_ms: start_time.elapsed().as_millis() as u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use factory_infrastructure::gitlab::{
        GitlabAuthor, GitlabIssue, GitlabMergeRequest, MockGitlabClient,
    };

    #[tokio::test]
    async fn test_gitlab_verifier_all_success() {
        let mut mock = MockGitlabClient::new();

        mock.expect_check_current_user().returning(|| {
            Ok(GitlabAuthor {
                username: "test-bot".to_string(),
            })
        });

        mock.expect_get_project()
            .with(mockall::predicate::eq("org/project"))
            .returning(|_| Ok(true));

        mock.expect_list_open_issues()
            .with(
                mockall::predicate::eq("org/project"),
                mockall::predicate::eq(Some("autonomous-mission".to_string())),
            )
            .returning(|_, _| {
                Ok(vec![GitlabIssue {
                    id: 1,
                    iid: 10,
                    title: "Test Issue".to_string(),
                    description: None,
                    web_url: "https://gitlab.com/org/project/issues/10".to_string(),
                    updated_at: None,
                }])
            });

        mock.expect_list_active_merge_requests()
            .with(mockall::predicate::eq("org/project"))
            .returning(|_| {
                Ok(vec![GitlabMergeRequest {
                    id: 2,
                    iid: 5,
                    title: "Test MR".to_string(),
                    description: None,
                    web_url: "https://gitlab.com/org/project/merge_requests/5".to_string(),
                    state: "opened".to_string(),
                    updated_at: None,
                }])
            });

        mock.expect_list_merge_request_notes()
            .with(
                mockall::predicate::eq("org/project"),
                mockall::predicate::eq(5),
                mockall::predicate::eq(None),
            )
            .returning(|_, _, _| Ok(vec![]));

        let verifier = GitlabVerifier::new(Arc::new(mock), "https://gitlab.com".to_string());
        let report = verifier.verify_all(&["org/project".to_string()]).await;

        assert_eq!(report.overall_status, VerificationStatus::Passed);
        assert_eq!(report.checks.len(), 5);
        assert!(
            report
                .checks
                .iter()
                .all(|c| c.status == CheckStatus::Success)
        );
    }

    #[tokio::test]
    async fn test_gitlab_verifier_auth_failure() {
        let mut mock = MockGitlabClient::new();

        mock.expect_check_current_user()
            .returning(|| Err(anyhow::anyhow!("HTTP 401 Unauthorized")));

        mock.expect_get_project()
            .returning(|_| Err(anyhow::anyhow!("HTTP 401 Unauthorized")));

        mock.expect_list_open_issues()
            .returning(|_, _| Err(anyhow::anyhow!("HTTP 401 Unauthorized")));

        mock.expect_list_active_merge_requests()
            .returning(|_| Err(anyhow::anyhow!("HTTP 401 Unauthorized")));

        let verifier = GitlabVerifier::new(Arc::new(mock), "https://gitlab.com".to_string());
        let report = verifier.verify_all(&["org/project".to_string()]).await;

        assert_eq!(report.overall_status, VerificationStatus::Failed);
        assert!(
            report
                .checks
                .iter()
                .any(|c| c.status == CheckStatus::Failure)
        );
        let auth_check = &report.checks[0];
        assert!(auth_check.remediation_hint.is_some());
    }
}
