# Security Test Plan & Verification Procedures

**Specification**: `010-integrated-k8s-gitops-audit`  
**Date**: `2026-11-02`

---

## 1. Test Categories & Verification Requirements

### STP-1: Host Subprocess Denial Test
- **Objective**: Ensure that untrusted code execution fails closed and refuses to execute on the host machine in production.
- **Test File**: `crates/factory-mcp-server/tests/sandbox_driver_tests.rs`
- **Assertion**:
  - In `production` mode, `SubprocessDriver::execute(...)` returns an `Err` containing `"Host subprocess execution is forbidden in production mode"`.

### STP-2: Test Execution Integrity & Non-Simulation Test
- **Objective**: Verify that `run_tests` does not simulate successful test passes when a test runner is missing or fails.
- **Test File**: `crates/factory-mcp-server/tests/run_tests_security_tests.rs`
- **Assertion**:
  - When given an invalid or missing command, `run_tests` returns a structured report with `outcome: TestExecutionOutcome::InfrastructureError` or `Failed`.
  - It NEVER returns `exit_code: 0` or `"status": "success"` for non-executed commands.

### STP-3: gVisor RuntimeClass Mandatory Selection Test
- **Objective**: Ensure that a sandbox Job cannot be scheduled without `runtimeClassName: gvisor`.
- **Test File**: `crates/factory-mcp-server/tests/launch_sandbox_pod_tests.rs`
- **Assertion**:
  - Attempting to launch an untrusted sandbox job when gVisor is disabled or unavailable triggers an immediate fail-closed error.

### STP-4: FinOps Budget Admission Hard-Stop Test
- **Objective**: Ensure that missions are rejected when the FinOps hard-stop latch is active.
- **Test File**: `crates/factory-application/tests/finops_admission_tests.rs`
- **Assertion**:
  - `MissionAdmissionGate::can_admit_mission` returns `Err(FactoryError::FinOps(...))` when `agent.is_halted()` is `true`.

### STP-5: GitOps Manifest Schema & Policy Validation
- **Objective**: Verify that GitOps manifests conform to Kubernetes v1.31 schemas and enforce restricted Pod Security Standards.
- **Command**:
  ```bash
  cd /mnt/F024B17C24B145FE/Repos/gitops_internal_lgcorzo
  kubectl kustomize infrastructure/agents/
  ```
