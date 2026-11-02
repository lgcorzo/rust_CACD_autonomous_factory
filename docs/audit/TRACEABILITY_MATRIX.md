# Dark Gravity Architecture Traceability Matrix

**Specification**: `010-integrated-k8s-gitops-audit`  
**Date**: `2026-11-02`  
**Evaluation Standard**:
- **PASS**: Verified and supported by active automated evidence.
- **PARTIAL**: Implemented in code/manifests but incompletely enforced at runtime.
- **FAIL**: Missing, bypassable, or insecure.
- **NOT_VERIFIED**: Requires live cluster verification run.

---

## Traceability Matrix

| Capability | Application Code (Rust) | GitOps Manifest (YAML) | Kubernetes Enforcement | Runtime / Test Evidence | Status |
|---|---|---|---|---|:---:|
| **Sandbox Execution Isolation** | `crates/factory-mcp-server/src/sandbox.rs` | N/A (Driver selection) | Fail-closed refusal of host execution | `tests/sandbox_driver_tests.rs` (Phase 1) | **FAIL** (Host Subprocess fallback exists) |
| **gVisor RuntimeClass** | `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs` | `infrastructure/agents/sandbox-job.yaml` | `RuntimeClass: gvisor` (Handler: `runsc`) | `kubectl get runtimeclass gvisor` -> active | **PARTIAL** (Optional toggle in code) |
| **Test Execution Integrity** | `crates/factory-mcp-server/src/tools/run_tests.rs` | N/A | Execution inside sandbox container | `tests/run_tests_security_tests.rs` (Phase 1) | **FAIL** (Host shell bypass & simulated pass) |
| **Pod Security Hardening** | `launch_sandbox_pod.rs` | `infrastructure/agents/sandbox-job.yaml` | PodSecurity Admission: `restricted` | Static YAML audit | **PARTIAL** (Lacks dropped caps, non-root, read-only) |
| **Namespace Isolation** | `launch_sandbox_pod.rs` | `infrastructure/agents/sandboxes-namespace.yaml` | Namespace boundary (`factory-sandboxes`) | K8s manifest validation | **PARTIAL** (Currently in `development` ns) |
| **Network Egress Isolation** | N/A | `infrastructure/agents/sandboxes-networkpolicy.yaml` | Calico / CNI default-deny NetworkPolicy | Negative curl reachability test | **FAIL** (Missing NetworkPolicy) |
| **RBAC Scoping** | N/A | `infrastructure/agents/rbac.yaml` | K8s RBAC authorizer | ClusterRoleBinding review | **PARTIAL** (ClusterRole broad scope) |
| **FinOps Admission Hard-Stop** | `factory-application/src/agents/finops.rs` | N/A | Application admission latch | Unit test admission rejection | **FAIL** (Hardstop flag not consumed by scheduler) |
| **NHI Credential Isolation** | `factory-core/src/security/nhi.rs` | N/A | Cryptographic Ed25519 & zeroize memory | `tests/mission_input_security_tests.rs` | **PASS** (Zeroize & non-serializable JIT tokens) |
| **Vault Repository ACLs** | `factory-infrastructure/src/vault.rs` | `infrastructure/security/` | Vault ACL path tokens | `tests/vault_isolation_tests.rs` | **PASS** (Strict 403 enforcement on foreign repos) |
| **Secret Encryption at Rest** | `factory-infrastructure` | `infrastructure/sealed-secrets/` | Bitnami SealedSecrets controller | Clean Gitleaks scan in Git | **PASS** (SealedSecrets encrypted in Git) |
| **Job Lifecycle & Cleanup** | `launch_sandbox_pod.rs` | `infrastructure/agents/sandbox-job.yaml` | `ttlSecondsAfterFinished: 300` | K8s Job controller | **FAIL** (No TTL configured in template) |
