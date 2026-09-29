# Research: PR Pipeline Scope Filtering for Autonomous Remediation

**Feature**: `006-pr-pipeline-scope-filter`
**Date**: 2026-09-29

## 1. Problem Space & Architecture Analysis

### Current Ingestion Bottleneck
In Dark Gravity V7 Architecture, `PollerDaemonService` invokes `poll_github_pipeline_runs` and `poll_gitlab_pipeline_runs` every 30 seconds.
- GitHub implementation: `GithubClient::list_failed_workflow_runs(repo, since)` returns all failed workflow runs across the repository without branch or event discrimination.
- GitLab implementation: `GitlabClient::list_failed_pipelines(project, since)` returns all failed pipelines in the project.

### Root Cause of False Positive Escalation
1. Repositories have continuous integration runs on default branches (`main`, `master`), release branches, scheduled cron tests, and historical commits.
2. When a legacy or detached pipeline failure occurs, `PipelineClassifier` analyzes the error log.
3. If classified as `ErrorCategory::Unknown` or non-remediable, `PipelineRemediationService::escalate_to_human` creates a new GitHub/GitLab issue.
4. In `lgcorzo/rust_CACD_autonomous_factory`, issues #397–#402 were created from historical runs from weeks prior on unrelated commits because the poller lacked PR-level scoping.
5. In GitLab (`lince-rs`), 78 legacy pipeline failures were repeatedly evaluated and escalated in a single poll cycle.

## 2. Technical Approaches Evaluated

### Approach A: Filter inside `PipelineRemediationService` (Consumer Side)
- **Concept**: The poller continues fetching all workflow runs, jobs, and error logs across the repository. The remediation service inspects whether each event belongs to an active PR and drops it if not.
- **Trade-offs**:
  - *Cons*: Wastes network bandwidth and GitHub/GitLab API rate limits downloading job logs (up to 10KB each) and job lists for runs that will ultimately be discarded.
  - *Cons*: High latency during poll cycles (hundreds of runs inspected).
- **Verdict**: Rejected as inefficient and resource-heavy.

### Approach B: Upstream Active PR Correlation in `GitPlatformPoller` (Recommended)
- **Concept**:
  1. At the start of each pipeline poll cycle for a repo/project, retrieve the set of active, open PRs/MRs (already cached or readily available via `list_active_pull_requests` / `list_active_merge_requests`).
  2. If there are zero active PRs in the repository, immediately skip pipeline polling for that repository.
  3. For candidate failed runs:
     - Check `run.pull_requests` list from GitHub workflow run JSON payload.
     - Match `run.head_branch` and `run.head_sha` against active PR head branches and commit SHAs.
     - For GitLab, match `pipeline.ref` against active MR `source_branch` and `pipeline.sha` against MR `sha`.
  4. If in-scope: extract failing job, download error log, attach `pr_number` / `head_branch` / `head_sha` to `PipelineFailureEvent`, and dispatch.
  5. If out-of-scope: immediately record the run ID in the `CursorStore` as processed to prevent re-querying, but do NOT fetch jobs or error logs.
- **Trade-offs**:
  - *Pros*: Minimal API calls. Zero log downloads for non-PR failures.
  - *Pros*: Advances cursor idempotently so historical runs are never re-evaluated.
  - *Pros*: Downstream remediation service receives clean, contextual PR metadata.
- **Verdict**: Selected as the optimal, high-performance architecture.

### Approach C: Precondition Guard in `PipelineRemediationService`
- **Concept**: Add a secondary safety gate in `PipelineRemediationService::handle_pipeline_failure`. If `event.pr_number.is_none()`, reject execution and refuse to open human escalation issues.
- **Verdict**: Selected as Defense-in-Depth alongside Approach B.

## 3. GitHub & GitLab REST API Payloads

### GitHub Actions Workflow Run Payload
`GET /repos/{owner}/{repo}/actions/runs` returns:
```json
{
  "id": 35138683447,
  "name": "CI/CD Pipeline",
  "head_branch": "fix/factory-mcp-server-kafka-fallback",
  "head_sha": "f5fdf87b40974b7c...",
  "event": "pull_request",
  "status": "completed",
  "conclusion": "failure",
  "pull_requests": [
    {
      "id": 123456,
      "number": 403,
      "head": {
        "ref": "fix/factory-mcp-server-kafka-fallback",
        "sha": "f5fdf87b40974b7c..."
      },
      "base": {
        "ref": "main",
        "sha": "088b523951f7..."
      }
    }
  ]
}
```

### GitLab CI Pipeline Payload
`GET /projects/:id/pipelines` returns:
```json
{
  "id": 987654,
  "status": "failed",
  "ref": "mission-lgcorzo-lince-rs-79",
  "sha": "b764cdc0eab71374...",
  "source": "merge_request_event",
  "web_url": "https://gitlab.com/lgcorzo/lince-rs/-/pipelines/987654"
}
```

## 4. Architectural Decisions (ADRs)

- **ADR-001: Early Exit when Active PR Set is Empty**: If a target repository has no active tracked PRs, the poller skips pipeline querying completely for that cycle, saving API budget.
- **ADR-002: Dual Correlation Vectors**: A workflow run is deemed in-scope if `run.pull_requests` explicitly contains an active PR number, OR if `run.head_branch` matches an active PR's branch name. This handles both `on: pull_request` and `on: push` workflow triggers.
- **ADR-003: Idempotent Skip Caching**: Non-PR pipeline IDs MUST be saved into `CursorStore` with their event hashes so that non-PR runs are evaluated at most once.
- **ADR-004: Escalation Defense Gate**: `PipelineRemediationService` will refuse to call `create_issue` if `pr_number` is missing, logging an explicit warning instead of generating untracked issues.
