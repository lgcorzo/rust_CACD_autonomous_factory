# Implementation Plan: GitLab Communication End-to-End Verification & Testing

**Branch**: `003-gitlab-communication-e2e` | **Date**: 2026-09-19 | **Spec**: [specs/003-gitlab-communication-e2e/spec.md](spec.md)

**Input**: Feature specification from `specs/003-gitlab-communication-e2e/spec.md`

---

## Summary

Deliver a comprehensive End-to-End (E2E) testing harness and verification suite for all GitLab communication channels in the Dark Gravity Autonomous Factory.
1. **Hermetic E2E Wire-Level Test Suite (`gitlab_e2e_integration_test.rs`)**:
   - Tests the complete lifecycle using `wiremock` simulating GitLab REST v4 APIs.
   - Verifies issue polling, resource constraint extraction, NHI credential issuance, MR directive detection, instant `eyes` (`👀`) reaction award, agent execution, discussion reply posting, bot-author filtering, and cursor idempotency.
2. **Operational Verification Tool (`factory-cli gitlab-verify`)**:
   - Adds a native diagnostic subcommand to `factory-cli` to probe live GitLab instances, validate access tokens, check project visibility, verify MR/issue readability, and test write/reaction capabilities with actionable remediation reports.
3. **Live End-to-End Diagnostic Test (`gitlab_live_e2e.rs`)**:
   - Integrates an optional live integration test gated by `GITLAB_API_TOKEN` to validate real network and API contract compatibility against GitLab SaaS (`https://gitlab.com`) or self-hosted instances.

---

## Technical Context

**Language/Version**: Rust 1.80+ (2021 Edition)  
**Primary Dependencies**: `reqwest` (with `json`), `serde`, `serde_json`, `tokio`, `async-trait`, `wiremock`, `chrono`, `tracing`, `clap`  
**Storage**: Ephemeral state in test runners; `CursorStore` (`InMemoryCursorStore` / `PostgresCursorStore`) for synchronization tracking  
**Testing**: `cargo test --package factory-application --test gitlab_e2e_integration_test`, `cargo test --workspace`  
**Target Platform**: Linux containerized workloads (gVisor & Kubernetes) and developer workstations  
**Project Type**: Autonomous CA/CD Agent Factory (Multi-crate Cargo workspace)  
**Performance Goals**: Hermetic E2E test completes in $< 2$ seconds; live verification check finishes in $< 3$ seconds per project  
**Constraints**: Zero external network requirements for default CI runs (`wiremock` isolation); non-destructive checks for live operational verification; strict token security  
**Scale/Scope**: Covers all configured GitLab projects (`GITLAB_PROJECTS`) and active MR/Issue threads  

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Gate / Principle | Status | Evaluation & Compliance Notes |
| :--- | :--- | :--- |
| **I. Library-First** | PASS | Reuses existing `GitlabClient` and `GitPlatformPoller` domain models; diagnostic reporting models structured in clean modules. |
| **II. CLI Interface** | PASS | Exposes live verification capability via `factory-cli gitlab-verify`, supporting JSON output and human-readable tables. |
| **III. Test-First (NON-NEGOTIABLE)**| PASS | All communication flows verified via deterministic hermetic E2E tests before live cluster verification. |
| **IV. Integration Testing** | PASS | Directly targets inter-service and external REST API contracts between factory components and GitLab v4 APIs. |
| **V. Observability & Telemetry** | PASS | Produces structured diagnostic scorecards (`GitlabVerificationReport`) with actionable failure remediation. |

---

## Project Structure

### Documentation (this feature)

```text
specs/003-gitlab-communication-e2e/
├── spec.md              # Feature specification
├── plan.md              # This implementation plan
├── research.md          # Phase 0 research findings & decisions
├── data-model.md        # Phase 1 data models & diagnostic entities
├── quickstart.md        # Phase 1 runnable verification guide
├── contracts/
│   └── gitlab_e2e_contract.json # JSON Schema for verification report
└── checklists/
    └── requirements.md  # Specification quality checklist
```

### Source Code Impact (repository crates)

```text
crates/
├── factory-infrastructure/
│   ├── src/
│   │   ├── gitlab.rs           # Expose project verification probe helpers
│   │   └── lib.rs
│   └── tests/                  # Contract assertions
├── factory-application/
│   ├── src/
│   │   ├── gitlab_verifier.rs  # Core verification logic and scorecard generation
│   │   └── lib.rs
│   └── tests/
│       ├── gitlab_e2e_integration_test.rs # Hermetic Wiremock E2E test
│       └── gitlab_live_e2e.rs             # Optional live API verification test
└── factory-cli/
    └── src/
        └── main.rs             # Subcommand `gitlab-verify`
```

---

## Proposed Changes by Component

### 1. `factory-application`

#### [NEW] `crates/factory-application/src/gitlab_verifier.rs`
- Implement `GitlabVerifier` service:
  - `verify_project(&self, project_id: &str) -> GitlabVerificationReport`
  - Runs 5 checks:
    1. Auth Handshake (`GET /api/v4/user` or project get)
    2. Project Metadata (`GET /api/v4/projects/:id`)
    3. Issues Polling (`GET /api/v4/projects/:id/issues?labels=autonomous-mission`)
    4. Merge Requests Polling (`GET /api/v4/projects/:id/merge_requests?state=opened`)
    5. Emoji & Note Capabilities
- Generates `GitlabVerificationReport` conforming to `gitlab_e2e_contract.json`.

#### [NEW] `crates/factory-application/tests/gitlab_e2e_integration_test.rs`
- Stand up `wiremock::MockServer`.
- Configure `HttpGitlabClient` pointing to mock server.
- Instantiate `GitPlatformPoller`, `CommentControlService`, and `PollerDaemonService`.
- Simulate complete flow:
  1. Polling issues: returns issue #42 labeled `autonomous-mission` with description `CPU: 100m, RAM: 128Mi`. Asserts ingested event.
  2. Polling MR notes: returns note #101 on MR !5 with body `@darkgravity /status`.
  3. Asserts mock server receives `POST /projects/.../merge_requests/5/notes/101/award_emoji` with `name: "eyes"`.
  4. Asserts mock server receives `POST /projects/.../merge_requests/5/notes` containing the status report.
  5. Asserts second poll cycle performs 0 redundant reactions or comments (idempotency).
  6. Asserts bot's own notes are ignored.

#### [NEW] `crates/factory-application/tests/gitlab_live_e2e.rs`
- Live integration test marked `#[tokio::test]`.
- Reads `GITLAB_API_TOKEN` and `GITLAB_PROJECTS`.
- If missing, skips cleanly with informative message (`eprintln!`).
- If present, executes real API queries and validates response parsing without mutating active MRs.

### 2. `factory-cli`

#### [MODIFY] `crates/factory-cli/src/main.rs`
- Add `GitlabVerify` subcommand to `Commands` enum:
  ```rust
  /// Verify GitLab connectivity, authentication, and communication channels
  GitlabVerify {
      #[arg(long, env = "GITLAB_URL", default_value = "https://gitlab.com")]
      gitlab_url: String,
      #[arg(long, env = "GITLAB_API_TOKEN")]
      gitlab_token: Option<String>,
      #[arg(long, env = "GITLAB_PROJECTS")]
      gitlab_projects: String,
      #[arg(long)]
      json: bool,
  }
  ```
- Print formatted ASCII health table or JSON report.

---

## Verification Plan

### Automated Tests
```bash
# 1. Hermetic E2E Wiremock Test
cargo test --package factory-application --test gitlab_e2e_integration_test -- --nocapture

# 2. Workspace regression testing
cargo test --workspace

# 3. Linter & formatting checks
cargo fmt -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

### Manual / Cluster Verification
```bash
# Verify CLI diagnostic command locally or in live pod
cargo run --bin factory-cli -- gitlab-verify --gitlab-projects "lgcorzo/fastapi-autogen-team"
```
