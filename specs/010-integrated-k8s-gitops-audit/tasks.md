# Tasks: Dark Gravity Integrated Kubernetes & GitOps Audit & Remediation

**Feature**: `010-integrated-k8s-gitops-audit`  
**Status**: `Ready for Execution`  
**Date**: `2026-11-02`

---

## Phase 0: Baseline Audit & Traceability Matrix (Read-Only)

- [x] T001: Generate full repository inventory and commit baseline for `rust_CACD_autonomous_factory` and `gitops_internal_lgcorzo` in `docs/audit/INTEGRATED_AUDIT.md`.
- [x] T002: Construct full traceability matrix mapping Rust capabilities to GitOps manifests, Kubernetes enforcement, and cluster verification in `docs/audit/TRACEABILITY_MATRIX.md`.
- [x] T003: Document comprehensive threat model with trust boundaries and attack paths in `docs/audit/THREAT_MODEL.md`.
- [x] T004: Document reproducible security test plan and verification procedures in `docs/audit/SECURITY_TEST_PLAN.md`.
- [x] T005: Create system architecture diagrams for control, execution, and security planes in `docs/architecture/DEPLOYMENT_ARCHITECTURE.md`.
- [x] T006: Create operational and incident response runbook in `docs/operations/SECURITY_RUNBOOK.md`.

---

## Phase 1: Critical Execution Isolation (P0)

- [ ] T007: Write failing test in `crates/factory-mcp-server/tests/sandbox_driver_tests.rs` asserting `SubprocessDriver` fails closed in production mode.
- [ ] T008: Implement fail-closed production check in `SubprocessDriver::execute` in `crates/factory-mcp-server/src/sandbox.rs`.
- [ ] T009: Write failing test in `crates/factory-mcp-server/tests/run_tests_security_tests.rs` verifying missing test runners return `TestExecutionOutcome::InfrastructureError` instead of simulated success.
- [ ] T010: Refactor `crates/factory-mcp-server/src/tools/run_tests.rs` to route execution through `SandboxDriver` and return typed `TestExecutionReport`.
- [ ] T011: Write failing test in `crates/factory-mcp-server/tests/launch_sandbox_pod_tests.rs` asserting missing or unsupported runtime class fails closed.
- [ ] T012: Update `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs` to require `runtimeClassName: gvisor` and reject unisolated pods.

---

## Phase 2: Kubernetes Workload & GitOps Hardening (P0/P1)

- [ ] T013: Create dedicated sandbox namespace manifest `infrastructure/agents/sandboxes-namespace.yaml` with Pod Security Standard `restricted` in `gitops_internal_lgcorzo`.
- [ ] T014: Harden `infrastructure/agents/sandbox-job.yaml` with non-root UID, `allowPrivilegeEscalation: false`, `capabilities.drop: ["ALL"]`, `readOnlyRootFilesystem: true`, and `ttlSecondsAfterFinished: 300` in `gitops_internal_lgcorzo`.
- [ ] T015: Create default-deny ingress and egress NetworkPolicy in `infrastructure/agents/sandboxes-networkpolicy.yaml` blocking API server and metadata endpoints in `gitops_internal_lgcorzo`.
- [ ] T016: Scope RBAC permissions in `infrastructure/agents/rbac.yaml` to restrict job creation strictly to the sandbox namespace in `gitops_internal_lgcorzo`.
- [ ] T017: Validate GitOps manifests using `kubectl kustomize` without errors.

---

## Phase 3: FinOps Admission Hard-Stop (P1)

- [ ] T018: Write failing unit test in `crates/factory-application/tests/finops_admission_tests.rs` asserting missions are rejected when the FinOps hard-stop latch is active.
- [ ] T019: Implement `MissionAdmissionGate` in `crates/factory-application/src/agents/finops.rs`.
- [ ] T020: Integrate `MissionAdmissionGate` into mission dispatching in `crates/factory-application/src/workflows/autonomous_mission.rs`.

---

## Phase 4: Verification, Quality Gates & Deliverables (P1/P2)

- [ ] T021: Run full workspace unit and integration test suite (`cargo test --workspace`).
- [ ] T022: Verify static formatting and zero clippy warnings (`cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D warnings`).
- [ ] T023: Update OKF documentation (`python3 .agents/tools/okf_generator.py`) and knowledge graph (`graphify update .`).
- [ ] T024: Validate CI/CD pipeline on Draft PR, compile evidence, and present to user for manual merge (HITL).

