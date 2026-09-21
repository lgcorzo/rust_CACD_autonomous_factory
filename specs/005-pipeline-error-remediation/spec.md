# Feature Specification: Pipeline Error Remediation

**Feature Branch**: `005-pipeline-error-remediation`

**Created**: 2026-09-21

**Status**: Draft

**Input**: User description: "The system has to read the errors in the pipelines and solve them as part of the factory duties."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Autonomous Pipeline Failure Detection (Priority: P1)

As a DevOps operator of the CACD Autonomous Factory, I want the factory to automatically detect when a CI/CD pipeline run fails across any integrated repository so that pipeline failures are caught without manual monitoring.

**Why this priority**: Pipeline failure detection is the foundational capability. Without the ability to observe and detect failures, no remediation can occur. This is the entry point for the entire feature and delivers immediate visibility value.

**Independent Test**: Can be fully tested by triggering a known pipeline failure (e.g., introducing a compilation error) and verifying the factory captures the failure event, including the failing job name, step, and error logs. Delivers immediate value by providing a structured failure feed.

**Acceptance Scenarios**:

1. **Given** a GitHub Actions pipeline run fails on a monitored repository, **When** the factory polls for pipeline status, **Then** the factory captures the run ID, workflow name, failing job, failing step, and error log output within 5 minutes of the failure.
2. **Given** a GitLab CI pipeline fails on a monitored project, **When** the factory polls for pipeline status, **Then** the factory captures the pipeline ID, failing job name, stage, and job trace log within 5 minutes of the failure.
3. **Given** a pipeline run succeeds, **When** the factory polls for pipeline status, **Then** no failure event is generated and no remediation workflow is triggered.
4. **Given** a pipeline failure has already been detected and processed, **When** the factory polls again, **Then** the same failure is not re-ingested (idempotent cursor-based detection).

---

### User Story 2 - Error Classification and Triage (Priority: P2)

As the autonomous factory engine, I need to classify each detected pipeline error into actionable categories so that the appropriate remediation strategy can be selected.

**Why this priority**: Classification determines whether the factory can autonomously fix the error, escalate it, or log it for human review. Without classification, the factory cannot make intelligent remediation decisions.

**Independent Test**: Can be tested by feeding known pipeline error logs (e.g., "cargo clippy" warnings, test assertion failures, Docker build failures, missing dependencies) and verifying each is classified into the correct category. Delivers value by providing structured error triage.

**Acceptance Scenarios**:

1. **Given** a pipeline error log containing a Rust compilation error, **When** the error is classified, **Then** it is categorized as `code_compilation` with sub-type `rust_build_error` and the offending file/line is extracted.
2. **Given** a pipeline error log containing `cargo clippy` warnings treated as errors, **When** the error is classified, **Then** it is categorized as `lint_violation` with the specific clippy rule(s) extracted.
3. **Given** a pipeline error log containing a test failure, **When** the error is classified, **Then** it is categorized as `test_failure` with the failing test name and assertion details extracted.
4. **Given** a pipeline error log containing a Docker build failure (e.g., missing system dependency), **When** the error is classified, **Then** it is categorized as `infrastructure_build` and the root cause is extracted.
5. **Given** a pipeline error that does not match any known pattern, **When** the error is classified, **Then** it is categorized as `unknown` and flagged for human review with the full log attached.

---

### User Story 3 - Autonomous Remediation Mission Trigger (Priority: P3)

As the factory orchestrator, I want classified pipeline errors to automatically spawn remediation missions through the existing Hatchet DAG workflow so that fixes are applied without human intervention.

**Why this priority**: This is the core value proposition — closing the loop from detection to fix. It builds on P1 (detection) and P2 (classification) and leverages the existing mission execution infrastructure (Rustant/ZeroClaw).

**Independent Test**: Can be tested by injecting a classified `lint_violation` error event and verifying that a new mission is created in the Hatchet DAG with the correct remediation context, and that the Rustant planner receives the classified error as mission input. Delivers the autonomous remediation value.

**Acceptance Scenarios**:

1. **Given** a classified `lint_violation` error, **When** the remediation mission is triggered, **Then** a new mission is created with a description referencing the specific clippy rule, file, and line, and the Rustant planner generates a fix plan targeting only the affected code.
2. **Given** a classified `test_failure` error, **When** the remediation mission is triggered, **Then** a mission is created with the failing test context, and the ZeroClaw executor attempts to fix the code that broke the test.
3. **Given** a classified `code_compilation` error, **When** the remediation mission is triggered, **Then** a mission is created with the compiler error context, and the executor attempts to fix the build error.
4. **Given** a classified `unknown` error, **When** the remediation path is evaluated, **Then** no autonomous mission is triggered; instead, a GitHub issue or GitLab work item is created summarizing the unclassified failure for human review.
5. **Given** a remediation mission succeeds and produces a PR/MR, **When** the CI pipeline on the PR/MR passes, **Then** the factory marks the remediation as successful and logs the outcome to Kafka telemetry.

---

### User Story 4 - Remediation Feedback Loop (Priority: P4)

As the factory system, I want remediation outcomes (success or failure) to be tracked and fed back so that future remediations improve over time and operators have full observability.

**Why this priority**: This completes the observability and learning cycle. Without feedback, the factory operates blindly. This is lower priority because the core loop (detect → classify → fix) delivers value without it, but it is essential for production maturity.

**Independent Test**: Can be tested by running a successful remediation end-to-end, then verifying that the telemetry records (Kafka events, Sentry breadcrumbs) contain the correct remediation outcome metadata. Delivers observability and continuous improvement value.

**Acceptance Scenarios**:

1. **Given** a remediation mission completes successfully, **When** the PR/MR pipeline passes, **Then** a `remediation-success` event is published to the `mission-artifact` Kafka topic with mission ID, error category, and time-to-fix.
2. **Given** a remediation mission fails (the fix does not resolve the pipeline error), **When** the failure is detected, **Then** a `remediation-failure` event is published to Kafka with the failure reason, and the original error is re-classified for potential human escalation.
3. **Given** multiple remediation outcomes over time, **When** an operator queries remediation metrics, **Then** they can see success rate per error category, average time-to-fix, and most common failure patterns.

---

### Edge Cases

- What happens when a pipeline fails with a transient infrastructure error (e.g., network timeout, runner OOM)? → The error is classified as `infrastructure_transient` and a retry is attempted before triggering a remediation mission.
- What happens when the same pipeline error keeps recurring after a successful remediation? → The factory detects the recurrence, escalates to human review after 3 consecutive identical failures, and creates a GitHub issue tagged `recurring-failure`.
- What happens when multiple pipelines fail simultaneously across different repositories? → Each failure is processed independently with its own mission. A concurrency limit prevents overwhelming the Hatchet DAG.
- What happens when the factory itself has a pipeline failure? → Self-referential errors on the `rust_CACD_autonomous_factory` repository are detected but require explicit human approval before autonomous remediation is attempted (safety guard).
- How does the system handle rate limits on the GitHub/GitLab API when polling pipeline runs? → Exponential backoff with jitter is applied, and the cursor store tracks the last successfully polled timestamp to avoid data loss.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST periodically poll CI/CD pipeline run statuses for all configured repositories via GitHub Actions API and GitLab CI Pipelines API.
- **FR-002**: System MUST detect failed pipeline runs and extract the workflow/pipeline name, failing job name, failing step name, and the relevant error log output.
- **FR-003**: System MUST classify each detected error into one of the following categories: `code_compilation`, `lint_violation`, `test_failure`, `infrastructure_build`, `infrastructure_transient`, `security_audit`, or `unknown`.
- **FR-004**: System MUST extract structured error metadata (file path, line number, rule name, test name, dependency name) from classified errors when available.
- **FR-005**: System MUST trigger a new remediation mission via the existing Hatchet DAG workflow for errors classified as remediable (`code_compilation`, `lint_violation`, `test_failure`).
- **FR-006**: System MUST create a GitHub issue or GitLab work item for errors classified as `unknown` or `infrastructure_build` for human triage.
- **FR-007**: System MUST track pipeline failure detection using cursor-based idempotency to prevent duplicate processing of the same failure.
- **FR-008**: System MUST publish remediation outcomes (success/failure) to the Kafka telemetry topic for observability.
- **FR-009**: System MUST apply a concurrency limit on simultaneous remediation missions to prevent resource exhaustion.
- **FR-010**: System MUST implement a safety guard that prevents autonomous self-remediation on the factory's own repository without explicit human approval.
- **FR-011**: System MUST handle transient infrastructure errors with configurable retry logic before escalating to a remediation mission.
- **FR-012**: System MUST detect recurring identical failures and escalate to human review after a configurable threshold (default: 3 consecutive identical failures).

### Key Entities

- **PipelineFailureEvent**: Represents a single detected pipeline failure. Contains the source platform, repository, run/pipeline ID, workflow name, failing job, failing step, error log, timestamp, and the URL to the failed run.
- **ErrorClassification**: The result of analyzing a pipeline failure. Contains the error category, sub-type, structured metadata (file, line, rule, test name), confidence score, and whether the error is considered autonomously remediable.
- **RemediationMission**: A mission spawned to fix a classified error. Links to the originating PipelineFailureEvent and ErrorClassification, and tracks the remediation outcome (pending, success, failure, escalated).
- **RemediationOutcome**: The final result of a remediation attempt. Contains the mission ID, result status, time-to-fix, the PR/MR URL if created, and whether the subsequent pipeline run passed.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Pipeline failures across all configured repositories are detected within 5 minutes of occurrence, with zero missed failures during a 24-hour observation period.
- **SC-002**: At least 80% of detected pipeline errors are correctly classified into an actionable category (not `unknown`) on standard Rust CI/CD pipelines.
- **SC-003**: Autonomous remediation missions are triggered for 100% of errors classified as remediable, within 2 minutes of classification.
- **SC-004**: At least 50% of autonomously triggered remediation missions for `lint_violation` and `test_failure` categories produce a successful fix (PR/MR that passes CI) within 30 minutes.
- **SC-005**: Zero duplicate remediation missions are triggered for the same pipeline failure event.
- **SC-006**: All remediation outcomes are logged to the telemetry system with 100% completeness.
- **SC-007**: Operators can query remediation success rates and average fix times through the existing Kafka/telemetry interface.

## Assumptions

- The factory already has authenticated API access to GitHub Actions and GitLab CI through existing `GithubClient` and `GitlabClient` infrastructure adapters.
- The existing Hatchet DAG mission orchestration (Rustant/ZeroClaw) can be reused for remediation missions with appropriately formatted mission inputs.
- Pipeline error logs from GitHub Actions and GitLab CI are accessible via their respective APIs and contain sufficient detail for error classification.
- The existing Kafka telemetry infrastructure (`mission-input`, `agent-thought`, `mission-artifact` topics) can be extended to carry remediation-specific events.
- The existing `CursorStore` mechanism can be extended to track pipeline run polling state alongside the current issue/comment polling state.
- Polling interval for pipeline runs defaults to 5 minutes, consistent with the existing issue polling cadence.
- The factory's self-referential safety guard (FR-010) is a hard requirement from day one to prevent recursive autonomous modification loops.
