# Implementation Plan: Work Item Progress Comments & Remote Branch GitOps Delivery

**Branch**: `001-workitem-status-and` | **Date**: 2026-09-18 | **Spec**: [specs/001-workitem-status-and/spec.md](file:///mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/specs/001-workitem-status-and/spec.md)

**Input**: Feature specification from `specs/001-workitem-status-and/spec.md`

---

## Summary

Implement real-time work item milestone commenting and remote GitOps branch/MR delivery for the Dark Gravity autonomous factory.
1. **Milestone Comments**: The Hatchet DAG tasks (`rustant-plan`, `zeroclaw-execute`, `zeroclaw-validate`, `rustant-review`, `factory-deliver`) emit structured Markdown progress comments back to the originating GitLab / GitHub issue thread using authenticated platform API clients (`HttpGitlabClient` / `HttpGithubClient`).
2. **Real Branch & MR Delivery**: Replace the Phase 5 mock URL stub (`https://gitlab.com/repo/merge_requests/{}`) with real remote branch creation via platform REST API (`POST /repository/branches`), atomic multi-file commit generation (`POST /repository/commits`), and Merge Request opening (`POST /merge_requests`) targeting `main` with link back to the originating issue.

---

## Technical Context

**Language/Version**: Rust 1.80+ (2021 Edition)  
**Primary Dependencies**: `reqwest` (with `json`), `serde`, `serde_json`, `tokio`, `async-trait`, `hatchet-sdk`, `tracing`, `mockall` (test-utils)  
**Storage**: Ephemeral / stateless execution in worker pods; High-watermark persistence via `PostgresCursorStore` / `InMemoryCursorStore`  
**Testing**: `cargo test --workspace`, unit tests with Mock clients (`MockGitlabClient`, `MockGithubClient`), integration test suites  
**Target Platform**: Linux (gVisor containerized workloads in Kubernetes)  
**Project Type**: Autonomous CA/CD Agent Factory (Multi-crate Cargo workspace)  
**Performance Goals**: Comment latency < 1.5s per phase transition; Branch & MR creation < 3.0s in Phase 5  
**Constraints**: Zero-trust air-gapped outbound-only communication; RAM <= 30Mi per pod clamp; Zero credential exposure in shell processes or logs  
**Scale/Scope**: Handles continuous polling and multi-agent DAG execution across multiple GitLab projects and GitHub repositories  

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Gate / Principle | Status | Evaluation & Compliance Notes |
| :--- | :--- | :--- |
| **I. Outbound-Only / Zero-Trust** | PASS | Uses outbound HTTPS REST requests via `HttpGitlabClient` and `HttpGithubClient`. Cero open inbound ports. |
| **II. Verification Triad** | PASS | Maintains 100% test pass requirement, strict clippy warning-free compilation, and SAST >= 8.0 review score before delivery. |
| **III. Credential Security** | PASS | Uses platform API tokens (`GITLAB_API_TOKEN` / `GITHUB_API_TOKEN`) passed via Authorization headers. No raw passwords or shell credential leaks. |
| **IV. Non-Blocking Telemetry** | PASS | Comment posting errors are logged as warnings and do not crash or block the underlying execution DAG. |
| **V. Real GitOps Delivery** | PASS | Replaces mock stub URLs with authentic remote branches and Merge Requests linked to originating issues. |

---

## Project Structure

### Documentation (this feature)

```text
specs/001-workitem-status-and/
├── spec.md              # Feature specification with Q1 & Q2 clarifications
├── plan.md              # This implementation plan
├── research.md          # Phase 0 research findings & API designs
├── data-model.md        # Phase 1 data models & trait definitions
├── quickstart.md        # Phase 1 runnable verification guide
├── contracts/
│   └── git_platform_delivery.json # Interface contract schema
└── checklists/
    └── requirements.md  # Specification quality checklist
```

### Source Code Impact (repository crates)

```text
crates/
├── factory-core/
│   └── src/
│       └── models/      # WorkItemContext models & event structures
├── factory-infrastructure/
│   ├── src/
│   │   ├── gitlab.rs    # Post issue note, create branch, create commit, create MR
│   │   ├── github.rs    # Post issue comment, create branch, create PR
│   │   └── lib.rs
│   └── tests/           # Unit and mock tests for extended platform clients
├── factory-application/
│   ├── src/
│   │   ├── workflows/
│   │   │   └── autonomous_mission.rs # Milestone comments emission + Phase 5 real delivery
│   │   └── poller_service.rs         # Propagate source_platform, repository, issue_number in MissionInput
│   └── tests/           # Workflow end-to-end integration tests
└── factory-cli/
    └── src/
        └── main.rs      # Pass platform clients into workflow runner
```

---

## Proposed Changes by Component

### 1. `factory-infrastructure`

#### `crates/factory-infrastructure/src/gitlab.rs`
- Add to `GitlabClient` trait:
  - `post_issue_note(project_id: &str, issue_iid: u64, body: &str) -> anyhow::Result<GitlabNote>`
  - `create_branch(project_id: &str, branch: &str, ref_branch: &str) -> anyhow::Result<String>`
  - `create_commit_files(project_id: &str, branch: &str, commit_message: &str, actions: &[GitlabCommitAction]) -> anyhow::Result<String>`
  - `create_merge_request(project_id: &str, source_branch: &str, target_branch: &str, title: &str, description: &str) -> anyhow::Result<GitlabMergeRequest>`
- Implement methods on `HttpGitlabClient` using `reqwest`:
  - `POST /api/v4/projects/{id}/issues/{iid}/notes`
  - `POST /api/v4/projects/{id}/repository/branches`
  - `POST /api/v4/projects/{id}/repository/commits`
  - `POST /api/v4/projects/{id}/merge_requests`
- Add mock unit tests verifying HTTP request format, JSON payloads, and error handling.

#### `crates/factory-infrastructure/src/github.rs`
- Add to `GithubClient` trait:
  - `post_issue_comment(repo: &str, issue_number: u64, body: &str) -> anyhow::Result<GithubComment>`
  - `create_branch(repo: &str, branch: &str, base_branch: &str) -> anyhow::Result<String>`
- Implement methods on `HttpGithubClient`.
- Add mock unit tests.

### 2. `factory-application`

#### `crates/factory-application/src/workflows/autonomous_mission.rs`
- Update `MissionInput` to include:
  - `source_platform: Option<String>`
  - `repository: Option<String>`
  - `issue_number: Option<u64>`
- Inject platform clients (`Arc<dyn GitlabClient>` / `Arc<dyn GithubClient>`) into `create_mission_workflow`.
- Add helper function `post_mission_comment(platform, repo, issue_number, message)`.
- Emit milestone comments at:
  - Workflow start / planning initiation
  - Planning completion
  - Code generation completion
  - Validation completion (with test suite metrics)
  - Security review approval (with SAST score)
- Refactor Phase 5 (`factory-deliver`):
  - Check `is_approved`. If approved:
    - Generate branch name: `specs/001-mission-{id}` or `mission-{id}`.
    - Call platform client to create the remote branch.
    - Commit changed files via `create_commit_files`.
    - Call `create_merge_request` (GitLab) or `create_pull_request` (GitHub).
    - Post completion comment on originating issue with the real MR/PR link.
    - Return `MissionOutput` containing genuine `pr_url`.

#### `crates/factory-application/src/poller_service.rs`
- Ensure `ingest_issue` serializes `source_platform`, `repository`, and `issue_number` into the `MissionInput` JSON payload for Kafka and Hatchet.

---

## Verification Plan

### Automated Tests
- `cargo test -p factory-infrastructure` — unit tests for new API methods on `HttpGitlabClient` and `HttpGithubClient`.
- `cargo test -p factory-application` — workflow tests verifying milestone comments and delivery logic.
- `cargo clippy --workspace -- -D warnings` — ensure zero lint warnings.
- `cargo fmt --all -- --check` — ensure clean formatting.

### Manual / Live Verification
- Launch factory poller against GitLab `lgcorzo/lince-rs` Issue #1.
- Confirm milestone comments appear on the issue.
- Confirm delivery branch is pushed to GitLab and Merge Request is opened targeting `main`.
