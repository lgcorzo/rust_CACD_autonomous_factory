# Implementation Plan: Pipeline Error Remediation

**Branch**: `005-pipeline-error-remediation` | **Date**: 2026-09-21 | **Spec**: [spec.md](file:///mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/specs/005-pipeline-error-remediation/spec.md)

**Input**: Feature specification from `specs/005-pipeline-error-remediation/spec.md`

## Summary

Dark Gravity currently polls GitHub and GitLab for **issues** and **PR/MR comments**, but has no awareness of CI/CD pipeline run outcomes. This feature adds an autonomous pipeline error remediation loop: the factory will poll GitHub Actions and GitLab CI pipeline runs, detect failures, classify errors using pattern-matching on log output, and trigger remediation missions through the existing Hatchet DAG workflow. The approach extends the proven `GitPlatformPoller` → `PollerDaemonService` → `KafkaClient` pipeline, adding a parallel polling path for workflow/pipeline runs alongside the existing issue/comment polling.

## Technical Context

**Language/Version**: Rust (stable 1.75+, Edition 2024)

**Primary Dependencies**: `reqwest` (HTTP), `serde`/`serde_json` (serialization), `tokio` (async runtime), `chrono` (time), `uuid` (identifiers), `thiserror`/`anyhow` (error handling), `tracing` (logging), `mockall` (test mocking), `wiremock` (HTTP test mocking), `rdkafka` (Kafka), `regex` (log parsing)

**Storage**: `CursorStore` trait (in-memory + PostgreSQL) for idempotent polling state

**Testing**: `cargo test --workspace` (unit + integration, `wiremock` HTTP mocking, `mockall` trait mocking)

**Target Platform**: Linux server (Kubernetes pods, Docker)

**Project Type**: Multi-crate Rust workspace (DDD: core → application → infrastructure → CLI)

**Performance Goals**: Pipeline failure detection within 5 minutes of occurrence; classification <1s per error; mission trigger <2 minutes after classification

**Constraints**: Zero duplicate remediation missions per failure event; self-referential safety guard on factory's own repository; maximum concurrent remediation missions configurable (default: 5)

**Scale/Scope**: 4 integrated repositories (GitHub + GitLab), ~50 pipeline runs/day, ~5-10 failures/week typical

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

The constitution template is unpopulated (placeholder values only), so no binding governance constraints exist. The following project-wide rules from AGENTS.md apply:

| Gate | Status | Evidence |
|------|--------|----------|
| DDD Layer Separation | ✅ PASS | New domain entities in `factory-core`, new use cases in `factory-application`, new API adapters in `factory-infrastructure` |
| Human-in-the-Loop (HITL) | ✅ PASS | Self-referential remediation (FR-010) requires human approval; all PRs require manual merge |
| Strict Typing | ✅ PASS | All new entities use Rust enums with `serde` tags; no `String`-typed categories |
| Test Coverage | ✅ PASS | Unit tests for classification logic, integration tests for poller, mock-based E2E test for full cycle |

## Project Structure

### Documentation (this feature)

```text
specs/005-pipeline-error-remediation/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit-tasks)
```

### Source Code (repository root)

```text
crates/
├── factory-core/src/
│   ├── lib.rs                          # [MODIFY] Add PipelineFailureEvent, ErrorClassification,
│   │                                   #   ErrorCategory, RemediationOutcome domain entities
│   └── error.rs                        # [EXISTING] Already has RemediationError variant
│
├── factory-infrastructure/src/
│   ├── github.rs                       # [MODIFY] Add workflow run + job log API methods
│   │                                   #   to GithubClient trait + HttpGithubClient impl
│   ├── gitlab.rs                       # [MODIFY] Add pipeline + job trace API methods
│   │                                   #   to GitlabClient trait + HttpGitlabClient impl
│   ├── git_poller.rs                   # [MODIFY] Add poll_github_pipeline_runs() and
│   │                                   #   poll_gitlab_pipeline_runs() methods
│   ├── pipeline_classifier.rs          # [NEW] Regex-based error log classifier
│   └── lib.rs                          # [MODIFY] Re-export new classifier module
│
├── factory-application/src/
│   ├── poller_service.rs               # [MODIFY] Add pipeline polling to poll_once() cycle
│   └── workflows/
│       └── pipeline_remediation.rs     # [NEW] Remediation mission orchestration service
│
└── factory-cli/src/
    └── main.rs                         # [MODIFY] Wire pipeline polling into worker daemon
```

**Structure Decision**: Follows existing DDD workspace layout. No new crates needed — all changes fit cleanly into existing layers. The `pipeline_classifier.rs` module is an infrastructure adapter because it transforms raw platform-specific log text into domain-model `ErrorClassification` entities. The `pipeline_remediation.rs` module is an application workflow because it orchestrates the use case of converting classified errors into missions.

## Complexity Tracking

No constitution violations. No complexity justifications needed.
