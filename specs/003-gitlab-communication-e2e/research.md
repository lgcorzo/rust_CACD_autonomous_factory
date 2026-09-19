# Phase 0 Research: GitLab Communication End-to-End Verification & Testing

**Branch**: `003-gitlab-communication-e2e` | **Date**: 2026-09-19

## Overview

This research document evaluates the technical strategy, design patterns, and testing harnesses required to validate all GitLab communication flows with the Dark Gravity Autonomous Factory end-to-end.

---

## Research Topics & Decisions

### Decision 1: Dual-Tier E2E Test Architecture (Hermetic Mock + Live Platform Verification)

- **Context**: The user requires testing all features related to GitLab communication and verifying that real operations work as expected.
- **Alternatives Considered**:
  1. *Live-only testing*: Requires real GitLab tokens and projects in every CI build. Flaky, rate-limited, and pollutes production issue/MR threads.
  2. *Unit mocks only*: Tests only isolated functions (`GitlabClient` trait mocks) without testing HTTP serialization, real wire protocols, query parameters, or deserialization bugs.
  3. *Dual-tier architecture (Recommended)*:
     - **Tier 1 (Hermetic Mock E2E)**: A full end-to-end integration test using `wiremock` simulating GitLab REST v4 wire protocols (`/api/v4/projects/:id/issues`, `/merge_requests/:iid/notes`, `/award_emoji`, `/commits`, etc.). Runs unconditionally in `cargo test --workspace` without network dependencies.
     - **Tier 2 (Live Platform Diagnostic Runner)**: A dedicated diagnostic capability (accessible via CLI `factory-cli gitlab-verify` and ignored-by-default integration test `tests/gitlab_live_e2e.rs`) that executes live verification when `GITLAB_API_TOKEN` and `GITLAB_PROJECTS` are configured.
- **Decision**: Adopt Dual-Tier Architecture. This gives 100% CI coverage while enabling real-world verification against GitLab.com or self-hosted GitLab instances.

---

### Decision 2: Wiremock vs Custom HTTP Mock for Wire Protocol Testing

- **Context**: `factory-infrastructure` uses `reqwest` inside `HttpGitlabClient`. We need to verify headers (`PRIVATE-TOKEN`), URL encoding (`/projects/lgcorzo%2Ffastapi-autogen-team/merge_requests/...`), pagination, and JSON payload serialization.
- **Alternatives Considered**:
  1. `mockall` trait mocks: Only tests Rust trait boundary; misses HTTP wire defects, header injection, and URL encoding bugs.
  2. `wiremock`: Spins up a local HTTP mock server, validates incoming HTTP requests, matches headers, returns real JSON, and asserts call counts. Already standard in Rust asynchronous ecosystem.
- **Decision**: Use `wiremock` for full wire-level verification in `tests/gitlab_e2e_integration_test.rs`.

---

### Decision 3: CLI Operational Verification Command (`factory-cli gitlab-verify`)

- **Context**: Operators and Kubernetes pods (e.g. `factory-poller` in `agents` namespace) need a simple, single-command check to verify connectivity, token permissions, and project access before or during daemon operation.
- **Decision**: Add a `gitlab-verify` subcommand to `factory-cli`. It reads `GITLAB_URL`, `GITLAB_API_TOKEN`, and target projects, executes a non-destructive health probe across all supported endpoints (authentication, project access, issue listing, MR listing, emoji capability test), and outputs a structured diagnostic health table.

---

### Decision 4: Event Stream Coverage & Edge Case Validation

- **Context**: Ensure all 6 key communication interfaces are exercised:
  1. **Issue Detection**: Ingestion of issues tagged `autonomous-mission`, extracting resource bounds (`CPU: 200m, RAM: 256Mi`), credential issuance (`VerifiableCredential`).
  2. **MR Discussion Ingestion**: Detection of active MRs and retrieval of updated notes.
  3. **Acknowledgement Award**: Calling `add_merge_request_note_award_emoji` with `eyes`.
  4. **Directive Parsing & Execution**: Handling `/status`, `/spec`, `/refine`, `/retry`, `/validate`, and `/interact`.
  5. **Threaded Discussion Reply**: Calling `post_merge_request_note` with formatted markdown and remediation guidance on failure.
  6. **Loop Prevention & Idempotency**: Ignoring self-notes and skipping already-processed notes via `CursorStore`.
- **Decision**: Formalize these 6 interfaces into standardized contract tests and integration assertions.
