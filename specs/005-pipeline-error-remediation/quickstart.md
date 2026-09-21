# Quickstart: Pipeline Error Remediation

**Feature**: `005-pipeline-error-remediation`

## Prerequisites

- Rust toolchain (stable 1.75+)
- System packages: `protobuf-compiler`, `libprotobuf-dev`, `libcurl4-openssl-dev`
- Environment variables configured in `.env`:
  - `GITHUB_TOKEN` — GitHub API token with `actions:read`, `repo` scopes
  - `GITLAB_API_TOKEN` — GitLab API token with `read_api` scope
  - `KAFKA_BROKERS` — Kafka bootstrap servers
- Repositories configured in `config/repos.yaml` or env vars:
  - `GITHUB_REPOS` — Comma-separated list (e.g., `lgcorzo/rust_CACD_autonomous_factory,lgcorzo/lince-rs`)
  - `GITLAB_PROJECTS` — Comma-separated list (e.g., `lgcorzo/fastapi-autogen-team`)

## Build & Test

```bash
# 1. Build the entire workspace
cargo build --workspace

# 2. Run all tests (including new pipeline remediation tests)
cargo test --workspace -- --skip smoke

# 3. Run only pipeline remediation unit tests
cargo test --workspace -- pipeline

# 4. Run classifier-specific tests
cargo test -p factory-infrastructure -- pipeline_classifier

# 5. Run pipeline poller integration tests
cargo test -p factory-infrastructure -- git_poller::test_poll_github_pipeline
```

## Validation Scenarios

### Scenario 1: Error Classification (Unit)

Verify that known error log patterns are classified correctly.

```bash
cargo test -p factory-infrastructure -- test_classify_clippy_error
cargo test -p factory-infrastructure -- test_classify_compilation_error
cargo test -p factory-infrastructure -- test_classify_test_failure
cargo test -p factory-infrastructure -- test_classify_unknown_error
```

**Expected**: Each test passes, confirming the regex classifier maps log patterns to the correct `ErrorCategory` variant with correct metadata extraction.

### Scenario 2: Pipeline Failure Detection (Integration, Mocked)

Verify the `GitPlatformPoller` detects failed GitHub Actions runs using `wiremock`.

```bash
cargo test -p factory-infrastructure -- test_poll_github_pipeline_runs
```

**Expected**: The poller calls the GitHub Actions API, detects the mock failure, constructs a `PipelineFailureEvent`, and marks it as processed in the `CursorStore`.

### Scenario 3: Remediation Mission Trigger (Integration, Mocked)

Verify the `PollerDaemonService` ingests pipeline failures and publishes to Kafka.

```bash
cargo test -p factory-application -- test_pipeline_remediation_full_cycle
```

**Expected**: A mock pipeline failure is detected → classified → remediation mission published to `mission-input` topic → outcome tracked.

### Scenario 4: Self-Referential Safety Guard

Verify that pipeline failures on `lgcorzo/rust_CACD_autonomous_factory` do NOT trigger autonomous remediation.

```bash
cargo test -p factory-application -- test_self_referential_safety_guard
```

**Expected**: Instead of publishing to `mission-input`, the service creates a GitHub issue tagged `pipeline-remediation-pending`.

### Scenario 5: Idempotency

Verify the same pipeline failure is not processed twice.

```bash
cargo test -p factory-infrastructure -- test_pipeline_cursor_idempotency
```

**Expected**: First call produces a `PipelineFailureEvent`; second call with the same run ID returns an empty result.

## Live Smoke Test (Manual)

> ⚠️ Requires live GitHub API access and a failing pipeline run.

1. Push a commit with a known clippy violation to a monitored repository.
2. Wait for the CI pipeline to fail.
3. Start the factory worker:
   ```bash
   cargo run -p factory-cli -- worker --mcp-url http://localhost:8100
   ```
4. Within 5 minutes, the factory should:
   - Log: `"Pipeline failure detected: {repo} run #{id}"`
   - Log: `"Classified as lint_violation: clippy::{rule}"`
   - Publish a remediation mission to Kafka `mission-input`
5. Verify in Kafka (or mock logs) that the mission payload contains the correct error classification.
