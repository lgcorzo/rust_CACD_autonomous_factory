# Runtime Attestation Integrity Report

**Component**: `crates/factory-mcp-server/src/sandbox.rs`, `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs`  
**Standard**: DG-RVP-014 §4  
**Evaluated At**: 2026-11-09T23:40:00Z  
**Status**: `VERIFIED`

---

## 1. Architectural Separation

`SandboxAttestation` has been completely restructured to eliminate static configuration echoes and uncorroborated assumptions:

1. **`RequestedSandboxSpec`**:
   - `runtime_class_name`: e.g. `"gvisor"`
   - `runtime_handler`: e.g. `"runsc"`
   - `seccomp_profile`: e.g. `"RuntimeDefault"`
   - `run_as_non_root`: `true`
   - `run_as_user`: `65534`
   - `read_only_root_filesystem`: `true`
   - `capabilities_dropped`: `["ALL"]`
   - `image_digest`: e.g. `docker.io/library/rust@sha256:...`
   - Resource limits (memory, CPU)

2. **`ObservedRuntimeFacts`**:
   - `job_uid`, `job_name`
   - `pod_uid`, `pod_name`
   - `node_name`
   - `container_id`
   - `container_image_digest`
   - `pod_phase`
   - `exit_code`, `termination_reason`
   - `observed_runtime_class_name`, `observed_run_as_non_root`, `observed_run_as_user`, `observed_seccomp_profile`, `observed_read_only_root_filesystem`, `observed_capabilities_dropped`
   - `host_runtime_handler_observed`: `Option<String>` (populated ONLY when verified by direct CRI/host inspection).

3. **`VerifiedSandboxChecks`**:
   - Evaluated by `SandboxAttestation::evaluate_verification`:
     - Compares requested vs. observed values.
     - Corroborates Job name and Pod UID.
     - Verifies non-root UID, read-only rootfs, dropped capabilities, and expected image digest.
     - If `host_runtime_handler_observed` is `None`, status is set to `UNVERIFIED` with failure:
       `"Runtime observation unavailable: host_runtime_handler_observed is None (K8s Pod API alone cannot prove host runsc handler)"`.
     - Only when all checks pass is `verification_status` set to `VERIFIED`.

---

## 2. Attestation Verification Suite

Automated regression tests in `crates/factory-mcp-server/tests/launch_sandbox_pod_tests.rs`:
- `test_attestation_fully_corroborated_is_verified`: Confirms that a fully corroborated attestation yields `VERIFIED`.
- `test_sandbox_attestation_distinguishes_requested_spec_from_observed_runtime`: Proves that requesting `gvisor` alone leaves `observed.host_runtime_handler_observed` as `None`, yielding `UNVERIFIED`.
- `test_attestation_negative_cases_prevent_verified`: Proves fail-closed `UNVERIFIED` on missing Pod UID, missing container ID, image digest mismatch, runtime class mismatch, termination failure, stale observation, job correlation mismatch, and read-only rootfs mismatch.
