# Research: DG-RVP-013: Verified Autonomous Execution Closure

## 1. Unified Fail-Closed Execution Policy (`ExecutionSecurityPolicy`)

### Decision
Create a dedicated `ExecutionSecurityPolicy` domain type in `crates/factory-core/src/security_policy.rs` (and integrate into `crates/factory-mcp-server/src/sandbox.rs` and `launch_sandbox_pod.rs`).
The policy evaluates:
1. `ExecutionEnvironment`: Production, Staging, Development, Test.
2. `WorkloadTrustLevel`: `Untrusted` (all autonomous missions, external code, agent surgical patches) vs `Trusted` (internal unit tests).
3. `SandboxDriver`: `KubernetesGVisor` only for untrusted workloads.
4. `AdmissionRule`:
   - Any untrusted workload MUST require gVisor runtime (`runtimeClassName: "gvisor"`).
   - `USE_GVISOR=false` cannot disable isolation for `Untrusted` code; if specified, it raises a typed `SecurityPolicyViolation::ForbiddenIsolationDowngrade`.
   - `SANDBOX_MODE=subprocess` is rejected for `Untrusted` code across all environments.
   - `allow_insecure_host_execution` is strictly limited to explicitly classified trusted development unit tests, never permitted for mission payloads or production.
   - Missing or unknown configuration fails closed (`SecurityPolicyViolation::FailClosed`).

### Rationale
Consolidates scattered environment variables (`SANDBOX_MODE`, `USE_GVISOR`, `FACTORY_ENV`, `ALLOW_INSECURE_HOST_EXECUTION`) into a single deterministic evaluator before any execution can be scheduled.

### Alternatives Considered
- *Per-driver ad-hoc checks*: Kept in `SubprocessDriver` and `launch_sandbox_pod.rs`. Rejected because configuration drift allowed `USE_GVISOR=false` outside production mode.

---

## 2. MicroK8s containerd Runtime Handler for gVisor

### Decision
Fix the containerd configuration template in MicroK8s:
- File: `/var/snap/microk8s/current/args/containerd-template.toml` and `/var/snap/microk8s/current/args/containerd.toml`.
- Defect identified:
  ```toml
  [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.runsc]
    runtime_type = "io.containerd.runc.v2"
  ```
- Correct runtime handler:
  ```toml
  [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.runsc]
    runtime_type = "io.containerd.runsc.v1"
    [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.runsc.options]
      BinaryName = "/usr/bin/runsc"
  ```
- Ensure the RuntimeClass `gvisor` with `handler: runsc` maps cleanly to `io.containerd.runsc.v1`.
- Restart `snap.microk8s.daemon-containerd` with safeguards to maintain cluster stability.

### Rationale
The previous containerd template was improperly specifying `io.containerd.runc.v2` for the `runsc` handler, causing containerd to attempt standard runc shim invocation with gVisor flags or misinterpreting the shim interface.

### Alternatives Considered
- Direct runc invocation: Rejected as it completely violates gVisor kernel emulation requirements.

---

## 3. Truthful Runtime Attestation Based on Observed Cluster Facts

### Decision
Refactor `SandboxAttestation` in `crates/factory-mcp-server/src/sandbox.rs` and `launch_sandbox_pod.rs`:
- Stop echoing static strings (`"io.containerd.runsc.v1"`, `"RuntimeDefault"`, `null`).
- Directly query the Kubernetes Pod status via `kube::Client`:
  - `observed_pod_uid`: `pod.metadata.uid`
  - `observed_node_name`: `pod.spec.node_name`
  - `observed_container_id`: `pod.status.container_statuses[0].container_id`
  - `observed_image_digest`: `pod.status.container_statuses[0].image_id`
  - `observed_runtime_class`: `pod.spec.runtime_class_name`
  - `observed_exit_code`: `state.terminated.exit_code`
  - `observed_at`: timestamp of Pod completion.
- If the Pod cannot be queried or observations are missing, fields are set to `None` and `is_verified` evaluates to `false`. Mock drivers cannot generate observed Kubernetes Pod UIDs.

### Rationale
Ensures zero-trust integrity by distinguishing between requested security policy and independently observed cluster reality.

---

## 4. FinOps Cross-Process & Distributed Consistency

### Decision
Audit and fix `FileFinOpsStopStore` in `crates/factory-infrastructure/src/finops_stop_store.rs`:
1. Align lock targets: Both `read_state()` and `write_state()` must use the exact same lockfile (`.lock`).
   - In `read_state()`, open and lock the `.lock` file with `lock_shared()` before reading the JSON state file.
   - In `write_state()`, open and lock the `.lock` file with `lock_exclusive()` before writing atomically via a temporary file and atomic rename.
2. Atomic admission method: Introduce `check_and_reserve_budget(mission_id, estimated_cost)` into `FinOpsStopStore` to perform check-and-latch under exclusive lock in a single transaction.
3. Fail-closed on missing, corrupted, or unreachable storage.

### Rationale
Previously, `read_state()` locked the JSON state file directly while `write_state()` locked a `.lock` file, creating a race condition where writers could mutate state while readers observed partial or stale data.

---

## 5. Verification Evidence Gate & Public Claims Integrity

### Decision
1. Update `evidence/` records and enforce exact commit SHA binding.
2. Update `data/canonical-claims.json` and `scripts/check_public_claims.py` in `lgcorzo.github.io` to require actual observed runtime evidence before admitting `runtime_verified`.
3. Distinguish test classifications: `UNIT`, `COMPONENT`, `INTEGRATION`, `KUBERNETES_ADMISSION`, `LIVE_RUNTIME`, `SECURITY_NEGATIVE`.

