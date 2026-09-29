# Contract: Git Platform Poller Pipeline Filtering Protocol

**Feature**: `006-pr-pipeline-scope-filter`
**Date**: 2026-09-29

## 1. Interface Signatures

### GitPlatformPoller Pipeline Ingestion Protocol

```rust
impl GitPlatformPoller {
    /// Polls GitHub Actions workflow runs, filtering strictly to active PRs.
    ///
    /// Preconditions:
    /// - Checks `list_active_pull_requests(repo)`.
    /// - If 0 active PRs, early-returns `Ok(vec![])`.
    ///
    /// Postconditions:
    /// - Emits `PipelineFailureEvent`s where `pr_number.is_some()`.
    /// - All inspected run IDs (in-scope and out-of-scope) are marked in `CursorStore`.
    pub async fn poll_github_pipeline_runs(
        &self,
        repo: &str,
    ) -> anyhow::Result<Vec<PipelineFailureEvent>>;

    /// Polls GitLab CI pipelines, filtering strictly to active MRs.
    ///
    /// Preconditions:
    /// - Checks `list_active_merge_requests(project_id)`.
    /// - If 0 active MRs, early-returns `Ok(vec![])`.
    ///
    /// Postconditions:
    /// - Emits `PipelineFailureEvent`s where `pr_number.is_some()`.
    /// - All inspected pipeline IDs (in-scope and out-of-scope) are marked in `CursorStore`.
    pub async fn poll_gitlab_pipeline_runs(
        &self,
        project: &str,
    ) -> anyhow::Result<Vec<PipelineFailureEvent>>;
}
```

### PipelineRemediationService Safety Gate

```rust
impl PipelineRemediationService {
    /// Evaluates whether an error should escalate to human review.
    ///
    /// Contract Enforcement:
    /// - If `event.pr_number.is_none()`, MUST NOT call `create_issue`.
    /// - Logs warning and returns `Ok(RemediationStatus::Skipped)`.
    async fn escalate_to_human(
        &self,
        event: &PipelineFailureEvent,
        classification: &ErrorClassification,
    ) -> anyhow::Result<RemediationStatus>;
}
```

## 2. Invariant Rules

1. **Active PR Precondition**: A pipeline failure without an active corresponding PR or MR must NEVER be passed to `remediation.handle_pipeline_failure()`.
2. **Cursor Completeness**: The poller cursor MUST advance over both in-scope and out-of-scope pipeline IDs. An out-of-scope run must never remain unacknowledged in the cursor store.
3. **Escalation Zero-Noise**: The remediation service must never open an issue on GitHub or GitLab unless the failure is attached to an identified, open PR/MR.
