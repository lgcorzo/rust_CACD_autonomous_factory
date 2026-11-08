---
description: "Task list for 012-secure-gvisor-execution implementation"
---

# Tasks: Secure gVisor Execution & Evidence Governance

**Input**: Design documents from `specs/012-secure-gvisor-execution/`  
**Prerequisites**: `spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/sandbox-and-evidence-contracts.md`, `quickstart.md`  
**Tests**: Included per the audit verification mandate (TDD verification required for security controls).  
**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4, US5)
- Exact file paths included in each task description

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Baseline preparation and test harness setup

- [x] T001 Verify git branch state and clean working tree on `012-secure-gvisor-execution`
- [x] T002 Inspect existing driver tests in `crates/factory-mcp-server/tests/` and identify test fixtures
- [x] T003 [P] Add file locking crate (`fs2 = "0.4"`) to `crates/factory-application/Cargo.toml` if not already present

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core types, security violation errors, and execution attestation models

**⚠️ CRITICAL**: Must complete before user story implementation

- [x] T004 Define `SecurityPolicyViolation` error variant in `crates/factory-core/src/errors.rs` or `crates/factory-mcp-server/src/sandbox.rs`
- [x] T005 [P] Create `SandboxAttestation` struct and serialization in `crates/factory-mcp-server/src/sandbox.rs` per `data-model.md`
- [x] T006 [P] Update execution response payload to include `attestation` in `crates/factory-mcp-server/src/tools/execute_code.rs` and `run_tests.rs`

**Checkpoint**: Foundation ready - user story implementation can begin

---

## Phase 3: User Story 1 - Zero-Trust Untrusted Execution Enforcement (Priority: P1) 🎯 MVP

**Goal**: Prohibit implicit `SubprocessDriver` fallback; make `GvisorK8sDriver` the default and fail-closed across all environments.

**Independent Test**: Invoking test or code execution with `SANDBOX_MODE` unset or set to subprocess rejects untrusted payloads with `SecurityPolicyViolation`.

- [x] T007 [P] [US1] Write unit tests in `crates/factory-mcp-server/tests/sandbox_driver_tests.rs` asserting untrusted code fails closed when subprocess is requested without explicit override
- [x] T008 [US1] Update `crates/factory-mcp-server/src/lib.rs` to set `GvisorK8sDriver` as default when `SANDBOX_MODE` is unset or empty
- [x] T009 [US1] Modify `SubprocessDriver::execute` in `crates/factory-mcp-server/src/sandbox.rs` to reject untrusted code execution unconditionally unless an explicit testing flag (`ALLOW_INSECURE_HOST_EXECUTION=true`) is active
- [x] T010 [US1] Run `cargo test -p factory-mcp-server --test sandbox_driver_tests` and ensure all zero-trust enforcement tests pass

---

## Phase 4: User Story 2 - Containerd Runsc Shim & Node Runtime Verification (Priority: P1)

**Goal**: Verify and enforce MicroK8s containerd `runsc` shim configuration and successful sandbox pod execution with 512Mi RAM.

**Independent Test**: Launch a test pod specifying `runtimeClassName: gvisor` that compiles and runs a Rust test cleanly.

- [ ] T011 [US2] Verify `runsc` shim registration in `/var/snap/microk8s/current/args/containerd-template.toml` and ensure Kubernetes `RuntimeClass` `gvisor` exists (`kubectl get runtimeclass gvisor`)
- [ ] T012 [P] [US2] Audit `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs` to guarantee immutable image digests and 512Mi memory limits
- [ ] T013 [US2] Execute a verification container on MicroK8s using `runtimeClassName: gvisor` executing `cargo test` to validate real `runsc` containment and exit code 0

---

## Phase 5: User Story 3 - Multi-Process Atomic FinOps State Governance (Priority: P2)

**Goal**: Ensure FinOps budget tracking is shared and synchronized atomically across multiple worker instances.

**Independent Test**: Concurrent budget check and admission operations across multiple simulated worker processes serialize correctly without budget overrun.

- [ ] T014 [P] [US3] Write concurrent multi-process admission tests in `crates/factory-application/tests/finops_admission_tests.rs`
- [ ] T015 [US3] Implement cross-process file-locking (`fs2` / advisory locks) in `FileFinOpsStopStore` in `crates/factory-application/src/agents/finops.rs`
- [ ] T016 [US3] Ensure fail-closed behavior if the shared lock cannot be acquired within timeout
- [ ] T017 [US3] Execute `cargo test -p factory-application --test finops_admission_tests` to verify atomic synchronization

---

## Phase 6: User Story 4 - Evidence Gate Freshness & Non-Mock Cryptographic Validation (Priority: P2)

**Goal**: Update `factory-cli validate-evidence` to enforce target commit SHA matching, TTL freshness, and rejection of mock attestations.

**Independent Test**: Stale evidence or mock attestation is rejected with non-zero exit code.

- [ ] T018 [P] [US4] Write unit tests in `crates/factory-cli/tests/evidence_tests.rs` verifying stale commit SHA and mock attestation rejections
- [ ] T019 [US4] Update `crates/factory-cli/src/evidence.rs` to validate `commit_sha == target_sha`, check timestamp freshness, and reject `MockInterfaceOnly`
- [ ] T020 [US4] Run `cargo test -p factory-cli --test evidence_tests`
- [ ] T021 [US4] Regenerate `docs/audit/VERIFICATION_EVIDENCE.json` with current commit SHA and run `factory-cli validate-evidence`

---

## Phase 7: User Story 5 - Public Claims & Documentation Synchronization (Priority: P3)

**Goal**: Reconcile `canonical-claims.json`, READMEs, and website documentation with demonstrated system state.

**Independent Test**: Run `check_public_claims.py` and confirm 100% test pass.

- [ ] T022 [P] [US5] Update audit review dates to 2026-11-08 in `docs/audit/` and `canonical-claims.json`
- [ ] T023 [P] [US5] Clarify gVisor isolation claim to reflect zero-trust policy enforcement
- [ ] T024 [US5] Run `python3 tests/check_public_claims.py` (or repository claim validator) to verify claims consistency

---

## Phase 8: Polish & Cross-Cutting Integration

**Purpose**: End-to-end integration and verification before PR completion

- [ ] T025 Run full workspace test suite `cargo test --workspace` with `cargo -j 1`
- [ ] T026 Execute `gitleaks protect -v --staged` to guarantee zero secrets committed
- [ ] T027 Trigger end-to-end verification mission in cluster to generate final verified execution evidence
- [ ] T028 Update OKF architectural documentation using `.agents/tools/okf_generator.py`

---

## Dependencies & Completion Order

```
[Phase 1: Setup] ──> [Phase 2: Foundational]
                             │
                             ├──────────────────────────┐
                             ▼                          ▼
                   [Phase 3: US1 (P1)] 🎯      [Phase 4: US2 (P1)]
                             │                          │
                             └───────────┬──────────────┘
                                         ▼
                               [Phase 5: US3 (P2)]
                                         │
                                         ▼
                               [Phase 6: US4 (P2)]
                                         │
                                         ▼
                               [Phase 7: US5 (P3)]
                                         │
                                         ▼
                               [Phase 8: Polish]
```

## Parallel Execution Examples

- **Batch 1 (Foundational)**: T005, T006 can run in parallel after T004.
- **Batch 2 (US1 & US2 Prep)**: T007 (US1 unit tests) and T012 (US2 pod spec audit) can run in parallel.
- **Batch 3 (US3 & US4 Tests)**: T014 (FinOps concurrency test) and T018 (Evidence Gate unit test) can run in parallel.
- **Batch 4 (US5 Documentation)**: T022 and T023 can run in parallel.

