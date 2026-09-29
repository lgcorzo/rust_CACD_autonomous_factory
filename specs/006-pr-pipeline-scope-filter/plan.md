# Implementation Plan: PR Pipeline Scope Filtering for Autonomous Remediation

**Branch**: `006-pr-pipeline-scope-filter` | **Date**: 2026-09-29 | **Spec**: [specs/006-pr-pipeline-scope-filter/spec.md](spec.md)

**Input**: Feature specification from `/specs/006-pr-pipeline-scope-filter/spec.md`

## Summary

Restrict Dark Gravity's pipeline failure polling and autonomous remediation engine to exclusively detect, remediate, and escalate CI/CD failures occurring on active, tracked Pull Requests (GitHub Actions) and Merge Requests (GitLab CI). All failed workflow runs or CI pipelines that are not correlated with an active PR/MR will be skipped early without downloading error logs, recorded in the sync cursor to prevent re-evaluation, and prevented from triggering spurious escalation issues (resolving issues like #397–#402).

## Technical Context

**Language/Version**: Rust 1.80+ (stable 2021 edition)

**Primary Dependencies**:
- `reqwest` 0.12 (HTTP client for GitHub/GitLab REST APIs)
- `serde` / `serde_json` 1.0 (serialization of webhook events and API payloads)
- `chrono` 0.4 (UTC timestamping and cursor tracking)
- `tokio` 1.38 (async runtime)
- `tracing` 0.1 (structured logging)
- `async-trait` 0.1 (trait definition for async clients)

**Storage**:
- `CursorStore` (PostgreSQL via `sqlx` in production, `InMemoryCursorStore` in unit tests)

**Testing**:
- `cargo test --workspace` (unit and integration tests with `mockall` and `wiremock`)

**Target Platform**: Linux x86_64, MicroK8s Kubernetes container runtime (`dark-gravity-factory` container image)

**Project Type**: Autonomous agentic orchestrator / multi-crate workspace

**Performance Goals**:
- Pipeline poll cycle completes in < 5 seconds per repository
- 0 log bytes downloaded for non-PR pipeline runs

**Constraints**:
- Must not fail or panic on unexpected GitHub/GitLab API payload formats
- Strict Human-in-the-Loop policy: zero auto-merges or unauthorized branch alterations

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I: Domain-Driven Design (DDD)**: PASS. Pipeline scope filtering rules are encapsulated in `factory-core` as domain models and traits, with infrastructure adapters in `factory-infrastructure` and use-case coordination in `factory-application`.
- **Principle II: Type Safety & Quality**: PASS. Strict typing using Rust structs and enums (`PipelineScopeVerdict`, extended `PipelineFailureEvent`).
- **Principle III: Test-First (TDD)**: PASS. Unit tests written for `PipelineScopeFilter`, mocked GitHub/GitLab poller tests, and remediation service safety gates before production deployment.
- **Principle IV: Observability**: PASS. Structured `tracing` events for every in-scope and out-of-scope pipeline decision with run ID and branch.

## Project Structure

### Documentation (this feature)

```text
specs/006-pr-pipeline-scope-filter/
├── plan.md              # This file (/speckit-plan command output)
├── research.md          # Phase 0 output: Analysis of current failure ingestion and API payloads
├── data-model.md        # Phase 1 output: Extended PipelineFailureEvent and Github/Gitlab API schemas
├── quickstart.md        # Phase 1 output: Quick testing and verification guide
├── contracts/           # Phase 1 output: JSON Schema and interface contracts
│   ├── pipeline-failure-event.schema.json
│   └── poller-filter.md
├── checklists/
│   └── requirements.md
└── tasks.md             # Phase 2 output (/speckit-tasks command)
```

### Source Code Structure

```text
crates/
├── factory-core/
│   └── src/
│       └── lib.rs       # Extend PipelineFailureEvent with pr_number, head_branch, head_sha
├── factory-infrastructure/
│   └── src/
│       ├── github.rs     # Extend GithubWorkflowRun with pull_requests, head_branch, head_sha
│       ├── gitlab.rs     # Extend GitlabPipeline with ref_, sha, source; GitlabMergeRequest with source_branch, sha
│       └── git_poller.rs # Implement PR-scoped pipeline filtering in poll_github_pipeline_runs & poll_gitlab_pipeline_runs
├── factory-application/
│   └── src/
│       ├── poller_service.rs # Pass active PR context to pipeline poller
│       └── workflows/
│           └── pipeline_remediation.rs # Defense gate: refuse escalation when pr_number is None
└── factory-cli/
    └── src/
        └── main.rs       # CLI poller entrypoint
```

## Structure Decision

The multi-crate workspace follows the standard Dark Gravity DDD layout:
1. `factory-core`: Domain entity extension (`PipelineFailureEvent`).
2. `factory-infrastructure`: GitHub/GitLab API client model extensions and `GitPlatformPoller` query filtering.
3. `factory-application`: Workflow coordination and safety escalation guard.
4. `factory-cli`: Command-line driver for worker and poller daemons.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| *None*    | Clean extension of existing poller methods | Filtering inside remediation service was rejected due to excessive API/bandwidth overhead |
