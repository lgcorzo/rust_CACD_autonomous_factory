# Feature Specification: PR Pipeline Scope Filtering for Autonomous Remediation

**Feature Branch**: `006-pr-pipeline-scope-filter`

**Created**: 2026-09-29

**Status**: Draft

**Input**: User description: "the agentic factory is detecting pipelines failures from pipelines that are not in the PR that is followed by the agentic system https://github.com/lgcorzo/rust_CACD_autonomous_factory/issues the system has to take into account only the pipelines that are in the PR not others"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Scope Pipeline Failure Detection Exclusively to Active PRs/MRs (Priority: P1)

As a repository owner and DevOps engineer, I want the autonomous agentic factory poller to exclusively detect and process pipeline failures originating from active Pull Requests (GitHub) and Merge Requests (GitLab) being tracked by the agentic system, so that failures occurring on default branches, scheduled jobs, manual triggers, or unmanaged branches do not spawn unwanted remediation missions or pollute the issue tracker.

**Why this priority**: Currently, the poller indiscriminately ingests all failed workflow runs and pipelines across entire repositories, causing high volume noise, false positive escalation issues (such as #397-#402 in `rust_CACD_autonomous_factory`), and wasted LLM token spend. Restricting ingestion to PR-scoped pipelines is the primary requirement.

**Independent Test**: Can be independently verified by simulating failed workflow runs across two categories—(a) a workflow run associated with an active tracked PR, and (b) a workflow run on a non-PR branch or default branch—and validating that only event (a) is emitted as a `PipelineFailureEvent` while event (b) is ignored and recorded in the sync cursor without remediation or escalation.

**Acceptance Scenarios**:

1. **Given** a repository with open Pull Requests and workflow runs, **When** a workflow run fails on a branch or commit belonging to an active tracked Pull Request, **Then** the poller identifies the association, captures the failure, attaches the PR reference, and forwards it to the remediation service.
2. **Given** a workflow run that fails on the default branch (`main`) or on a standalone branch without an open PR, **When** the poller inspects repository workflow runs, **Then** the failure is filtered out and no remediation mission or escalation issue is generated.
3. **Given** a GitLab CI pipeline failure on a branch without an open Merge Request, **When** the GitLab pipeline poller executes, **Then** the pipeline is discarded from remediation processing.

---

### User Story 2 - Contextual PR Metadata Enrichment on Failure Events (Priority: P2)

As an autonomous developer agent (Rustant/ZeroClaw), when a pipeline failure occurs on an active PR, I want the failure event to explicitly carry the PR number, head branch, and commit SHA, so that the remediation workflow can inspect the PR context, target git commits to the correct branch, and communicate status back into the PR thread.

**Why this priority**: When a pipeline failure is verified to belong to a PR, the remediation service needs direct access to the PR context (PR number and branch) rather than treating the failure as an isolated, detached workflow run.

**Independent Test**: Can be tested by verifying that generated `PipelineFailureEvent` objects contain populated `pr_number` and `head_branch` fields matching the active PR metadata, and that downstream event handlers can resolve the PR without secondary lookups.

**Acceptance Scenarios**:

1. **Given** an active GitHub PR #403 with head branch `fix/factory-mcp-server-kafka-fallback`, **When** a CI/CD workflow run for that PR fails, **Then** the emitted `PipelineFailureEvent` contains `pr_number: Some(403)` and `head_branch: Some("fix/factory-mcp-server-kafka-fallback")`.
2. **Given** an active GitLab MR with source branch `feature-branch`, **When** the pipeline for that MR fails, **Then** the emitted `PipelineFailureEvent` includes the MR IID and source branch.

---

### User Story 3 - Suppression of False-Positive Human Escalations (Priority: P3)

As a project maintainer, I want the pipeline remediation service to enforce a strict precondition check before creating any human escalation issues, ensuring issues are only opened when an active PR tracked by the system experiences an irremediable failure, preventing noisy issues from accumulating in repository backlogs.

**Why this priority**: Non-remediable errors (e.g. unknown errors or infrastructure failures) trigger automated issue creation on GitHub/GitLab. Ensuring that only failures with valid active PR context can escalate protects the issue tracker from noise.

**Independent Test**: Can be tested by feeding an irremediable failure event with no PR association to the remediation service and confirming that zero issues are created on the host platform.

**Acceptance Scenarios**:

1. **Given** a pipeline failure that does not correlate to an active tracked PR, **When** the remediation service evaluates whether to escalate the failure, **Then** the escalation is skipped, logged as out-of-scope, and no issue is opened.
2. **Given** an irremediable failure on an active tracked PR, **When** the remediation service evaluates escalation, **Then** an escalation issue or PR comment is created referencing the specific PR.

---

### Edge Cases

- **PR Closed While Workflow Running**: A workflow run starts for an active PR, but the PR is closed or merged before the failure is polled. The system MUST re-verify that the PR is still open before triggering remediation.
- **Multiple PRs Sharing a Commit or Branch**: If multiple PRs target different base branches from the same head, the system MUST attribute the failure to the active tracked PR with matching head SHA.
- **Push vs. Pull Request Event Types**: GitHub Actions workflows may trigger on `push` to the PR branch or on `pull_request` event. The system MUST support matching both: via explicit `run.pull_requests` list and via `run.head_branch` / `run.head_sha` correlation with active PRs.
- **Draft PRs**: The system SHOULD allow configuring whether draft PR pipelines are eligible for autonomous remediation or ignored until marked ready for review.
- **Cursor Advancement for Skipped Runs**: Skipped non-PR pipeline runs MUST still be recorded in the sync cursor to avoid re-evaluating old historical failures on every polling cycle.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST discover the list of active, tracked Pull Requests (GitHub) and Merge Requests (GitLab) in each monitored repository.
- **FR-002**: The system MUST filter pipeline failure polling results so that only workflow runs or CI pipelines associated with an active tracked PR/MR are ingested.
- **FR-003**: The system MUST verify PR association through at least one of:
  - Direct workflow event metadata (e.g., GitHub `pull_requests` attribute in workflow run).
  - Branch and commit SHA matching against the active PR's head branch and head commit.
  - GitLab pipeline source metadata (`merge_request_event`) or MR source branch matching.
- **FR-004**: The system MUST silently discard pipeline runs on default branches (e.g., `main`, `master`) unless an active PR is explicitly open for that reference.
- **FR-005**: The system MUST record processed non-PR pipeline IDs into the cursor store so that skipped runs are not repeatedly evaluated across polling cycles.
- **FR-006**: The system MUST enrich `PipelineFailureEvent` with optional PR context (`pr_number`, `head_branch`, and `head_sha`).
- **FR-007**: The remediation service MUST NOT create human escalation issues on GitHub or GitLab for pipeline failures that lack active PR correlation.
- **FR-008**: The system MUST log the reason for ignoring any pipeline failure (e.g., "Ignored pipeline run {id}: branch '{ref}' does not match any active PR").

### Key Entities *(include if feature involves data)*

- **ActivePRObject**: Represents an open pull/merge request under observation, including repository identifier, PR/MR number, head branch name, head commit SHA, and author.
- **PipelineFailureEvent**: Represents a failed CI/CD execution, extended to include `pr_number: Option<u64>`, `head_branch: Option<String>`, and `head_sha: Option<String>`.
- **PipelineScopeFilter**: Domain rule that evaluates a candidate pipeline run against the current set of active tracked PRs, returning whether the pipeline is in-scope or out-of-scope with an explanation.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of pipeline failures polled and dispatched to autonomous remediation or human escalation are attributable to an active open PR/MR.
- **SC-002**: Zero automated escalation issues created in repository issue trackers for non-PR pipeline runs (reducing spurious issues from 6+ per cycle to 0).
- **SC-003**: Poller cycle duration remains under 10 seconds for repositories with up to 50 concurrent pipeline runs by filtering before fetching full job logs.
- **SC-004**: Cursor store correctly tracks skipped pipeline IDs, resulting in 0 redundant evaluations of historical runs on subsequent poll cycles.

## Assumptions

- Target repositories use standard GitHub Actions workflow events (`pull_request`, `push`) or GitLab CI merge request pipelines.
- Polling user/token possesses permissions to list pull requests, workflow runs, and pipeline jobs for the configured repositories.
- Only open/active PRs are considered in-scope; pipeline failures on closed or merged PRs are discarded.
- Remediation fixes will continue to be delivered to the PR's head branch in compliance with repository branching and Human-in-the-Loop policies.
