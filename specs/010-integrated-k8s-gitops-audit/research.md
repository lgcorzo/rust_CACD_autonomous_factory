# Phase 0: Research & Security Baseline

**Feature**: `010-integrated-k8s-gitops-audit`  
**Date**: `2026-11-02`  
**Environment Audited**:
- Application Repo: `rust_CACD_autonomous_factory` (Commit: `b32f461d`, Branch: `010-integrated-k8s-gitops-audit`)
- GitOps Repo: `gitops_internal_lgcorzo` (Commit: `c24b52c8`, Branch: `main`)
- Cluster: `server.internal.lgcorzo` (Kubernetes v1.31.14 / containerd 1.6.28 / Ubuntu 24.04.5 LTS / RuntimeClass `gvisor` with handler `runsc`)

---

## 1. Discovered Findings & Vulnerabilities

### Finding 1: Host Subprocess Execution Fallback (P0 - Execution Isolation)
- **Location**: `crates/factory-mcp-server/src/sandbox.rs:49-88`
- **Mechanism**: `SubprocessDriver` executes `tokio::process::Command::new("python3")`, `rustc`, `go` directly on the host operating system with no container boundary or kernel namespace isolation.
- **Impact**: Any untrusted code submitted to this driver runs with the host user privileges, accessing host filesystem, environment variables, network interfaces, and Unix sockets.
- **Remediation**:
  - In production (`FACTORY_ENV=production` or `cfg!(feature = "production")`), `SubprocessDriver` must fail closed with an explicit error refusing host execution.
  - Subprocess execution is restricted strictly to local dev/mock test environments.

### Finding 2: False Positive Simulation & Host Shell Bypass in `run_tests` (P0 - Test Integrity)
- **Location**: `crates/factory-mcp-server/src/tools/run_tests.rs:56-98`
- **Mechanism**:
  1. `run_tests` spawns `tokio::process::Command::new("sh").arg("-c").arg(test_cmd)` on the host machine, bypassing the `SandboxDriver` completely.
  2. If the executable is missing (exit code 127 or "not found" in stderr) or fails to spawn, the tool returns `{"status": "success", "exit_code": 0, "output": "Simulated test execution..."}`!
- **Impact**: A mission can report that all tests passed even when no test runner was installed or tests were never run, completely undermining verification gates.
- **Remediation**:
  - Route test execution exclusively through the configured `SandboxDriver` (inside gVisor Kubernetes Job).
  - Eliminate all simulated pass logic for missing runners or spawn failures.
  - Return typed execution outcomes: `passed`, `failed`, `not_executed`, `infrastructure_error`, `timeout`.

### Finding 3: Non-Enforced gVisor Selection & Contradictory Configuration (P0 - Container Isolation)
- **Location**:
  - `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs:128, 168-177`
  - `gitops_internal_lgcorzo/infrastructure/agents/sandbox-job.yaml`
- **Mechanism**: In `launch_sandbox_pod.rs`, `use_gvisor` is checked via `std::env::var("USE_GVISOR") == "true"`. If unset or false, `runtimeClassName` is omitted, causing Kubernetes to schedule the Job using default `runc` without gVisor sandbox protection.
- **Impact**: Untrusted code runs in a standard container that shares the host Linux kernel directly.
- **Remediation**:
  - Make `runtimeClassName: gvisor` mandatory for production untrusted workloads.
  - If gVisor is unavailable or fails admission, fail closed immediately.
  - Unify configuration to eliminate conflicting toggles.

### Finding 4: Inadequate Kubernetes Job SecurityContext & Namespace Isolation (P1 - Workload Hardening)
- **Location**:
  - `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs:130-166`
  - `gitops_internal_lgcorzo/infrastructure/agents/sandbox-job.yaml`
- **Mechanism**:
  1. Job is scheduled in the `development` namespace instead of an isolated sandbox-dedicated namespace.
  2. No Pod `securityContext`: lacks `allowPrivilegeEscalation: false`, `capabilities.drop: ["ALL"]`, `seccompProfile: RuntimeDefault`, `readOnlyRootFilesystem: true`.
  3. No explicit `automountServiceAccountToken: false`.
  4. Lacks `ttlSecondsAfterFinished` for automatic cleanup of finished sandbox Jobs.
- **Impact**: Potential container breakout escalation paths, residual pods consuming memory/etcd, and unwanted cluster token mounting.
- **Remediation**:
  - Define dedicated `factory-sandboxes` namespace with Pod Security Standard `restricted`.
  - Add full hardened `securityContext` and `ttlSecondsAfterFinished: 300`.
  - Add explicit default-deny NetworkPolicy.

### Finding 5: FinOps Hard-Stop Latch Disconnected from Mission Admission (P1 - Operational Control)
- **Location**: `crates/factory-application/src/agents/finops.rs:140` (`self.set_halted(true)`)
- **Mechanism**: When `current_spend >= hardstop_threshold` (90% of budget), `FinOpsAgent` sets its internal `halted` flag to `true` and logs an alert, but no scheduler or mission admission check consumes `is_halted()` to block incoming missions.
- **Impact**: Autonomous agents can continue running missions and incurring LLM spend even after the hard-stop budget limit is triggered.
- **Remediation**:
  - Introduce `MissionAdmissionController` or hook into mission entry points (`Workflow::run`, `MissionInput::validate`) that queries the FinOps budget status.
  - If budget is halted, reject mission admission with `FactoryError::QuotaExceeded`.

---

## 2. Target Traceability Mapping

| Capability | Application Code (Rust) | GitOps Manifest (YAML) | Cluster Enforcement | Evidence / Test | Status |
|---|---|---|---|---|:---:|
| **Sandbox Driver Isolation** | `crates/factory-mcp-server/src/sandbox.rs` | N/A (Client config) | Host OS process isolation | TDD fail-closed tests | P0 Fix Required |
| **gVisor RuntimeClass** | `tools/launch_sandbox_pod.rs` | `infrastructure/agents/sandbox-job.yaml` | `RuntimeClass: gvisor` (`runsc`) | Pod execution in live cluster | P0 Fix Required |
| **Test Execution Integrity** | `tools/run_tests.rs` | N/A | Kubernetes Sandbox Pod | TDD no-simulated-pass tests | P0 Fix Required |
| **Pod Security Context** | `launch_sandbox_pod.rs` | `infrastructure/agents/sandbox-job.yaml` | PodSecurity admission (restricted) | K8s manifest validation | P1 Fix Required |
| **Network Isolation** | N/A | `infrastructure/agents/networkpolicy.yaml` | CNI NetworkPolicy (default-deny) | Egress curl test to 169.254.169.254 | P1 Fix Required |
| **RBAC Least Privilege** | N/A | `infrastructure/agents/rbac.yaml` | K8s RBAC authorizer | ClusterRole review | P1 Fix Required |
| **FinOps Admission Latch** | `factory-application/src/agents/finops.rs` | N/A | Application scheduler admission | Unit test budget hardstop admission | P1 Fix Required |
| **Secret Protection** | `factory-core`, `factory-infrastructure` | `infrastructure/sealed-secrets/` | SOPS / SealedSecrets | Gitleaks + K8s sealed secrets | Pass / Verified |
