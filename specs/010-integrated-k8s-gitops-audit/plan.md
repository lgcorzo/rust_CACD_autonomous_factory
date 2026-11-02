# Implementation Plan - Dark Gravity Integrated Kubernetes & GitOps Audit & Remediation

**Feature ID**: `010-integrated-k8s-gitops-audit`  
**Status**: `Planned`  
**Created**: `2026-11-02`

---

## Technical Context

The Dark Gravity Autonomous Factory relies on isolated execution to run agent-generated code and mission workflows. While previous remediations secured the JIT credential memory lifecycle and Vault isolation, this plan establishes kernel-enforced execution isolation across the Rust application crates and GitOps manifests (`gitops_internal_lgcorzo`).

### Critical Risks to Remediate:
1. **Subprocess Driver Host Execution (P0)**: Eliminate host process fallback for untrusted code execution.
2. **Missing Executable Simulation in `run_tests` (P0)**: Eliminate fabricated test passes; require real execution inside sandbox.
3. **gVisor RuntimeClass Enforcement (P0)**: Make `runtimeClassName: gvisor` mandatory with fail-closed semantics.
4. **Kubernetes Job & Namespace Isolation (P1)**: Dedicate `factory-sandboxes` namespace with PodSecurity `restricted`, non-root, drop capabilities, and TTL cleanup.
5. **FinOps Admission Gate (P1)**: Wire FinOps budget hard-stop latch to mission admission.

---

## Execution Phases

### Phase 0: Baseline, Threat Model & Traceability Matrix (Read-Only)
- Produce `docs/audit/INTEGRATED_AUDIT.md`.
- Produce `docs/audit/TRACEABILITY_MATRIX.md`.
- Produce `docs/audit/THREAT_MODEL.md`.
- Produce `docs/audit/SECURITY_TEST_PLAN.md`.
- Produce `docs/architecture/DEPLOYMENT_ARCHITECTURE.md`.
- Produce `docs/operations/SECURITY_RUNBOOK.md`.

### Phase 1: Critical Execution Isolation (P0)
- Hardening `crates/factory-mcp-server/src/sandbox.rs`: Fail closed when `SubprocessDriver` is invoked under production profile.
- Hardening `crates/factory-mcp-server/src/tools/run_tests.rs`: Replace host `sh -c` with `SandboxDriver` invocation. Eliminate simulated pass logic. Introduce typed `TestExecutionReport`.
- Hardening `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs`: Require `runtimeClassName: gvisor` for production untrusted jobs; reject job creation if gVisor is missing.

### Phase 2: Kubernetes & GitOps Hardening (P0/P1)
- In `gitops_internal_lgcorzo`:
  - Create `infrastructure/agents/sandboxes-namespace.yaml` with Pod Security Standard `restricted`.
  - Update `sandbox-job.yaml` with full hardened `securityContext`, `ttlSecondsAfterFinished: 300`, `automountServiceAccountToken: false`.
  - Add default-deny `networkpolicy.yaml` for sandbox pods.
  - Scope RBAC to `factory-sandboxes` namespace instead of cluster-wide.

### Phase 3: FinOps Admission Hard-Stop (P1)
- Implement `MissionAdmissionGate` in `crates/factory-application/src/agents/finops.rs`.
- Wire admission check into workflow execution in `crates/factory-application/src/workflows/autonomous_mission.rs`.
- Write unit tests validating mission rejection upon budget breach.

### Phase 4: Verification, Quality Gates & Evidence Generation (P1/P2)
- Execute complete workspace tests (`cargo test --workspace`).
- Execute Clippy & formatting checks (`cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`).
- Update OKF documentation and knowledge graph (`okf_generator.py`, `graphify update .`).
- Present findings and Draft PR for user review (HITL).
