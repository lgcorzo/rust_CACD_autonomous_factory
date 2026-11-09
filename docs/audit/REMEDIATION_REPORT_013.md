# DG-RVP-013: Verified Autonomous Execution Closure — Remediation Report

**Target Repository**: `lgcorzo/rust_CACD_autonomous_factory` (`v0.3.1`)  
**Specification**: [`specs/013-verified-autonomous-execution-closure/`](../../specs/013-verified-autonomous-execution-closure/spec.md)  
**Evaluated At (UTC)**: `2026-11-09T18:55:00Z`  
**Cluster**: `server.internal.lgcorzo` (MicroK8s `v1.31.14`, Ubuntu 24.04.5 LTS, kernel `6.17.0-14-generic`)

---

## 1. Inspected Repository Revisions (`T001`, `T002`)

| Repository | Branch | Latest Commit SHA | Release / Status |
|---|---|---|---|
| `lgcorzo/rust_CACD_autonomous_factory` | `main` | `45a508e990af5080b99c0772c62794a4bca5576c` | Release [`v0.3.1`](https://github.com/lgcorzo/rust_CACD_autonomous_factory/releases/tag/v0.3.1) (`4cdec64698d77647a83279126c8c3a6e485ac68c`), CI/CD green (`42673675574`, `42674766659`) |
| `lgcorzo/gitops_internal_lgcorzo` | `main` | `4728557c0424234a535ab2021a73a2cf876e5a7f` | Merged [PR #200](https://github.com/lgcorzo/gitops_internal_lgcorzo/pull/200) & [PR #201](https://github.com/lgcorzo/gitops_internal_lgcorzo/pull/201) |
| `lgcorzo/lgcorzo.github.io` | `main` | `94f4307234d9215499d04be03df9db74e0b8c584` | Merged [PR #51](https://github.com/lgcorzo/lgcorzo.github.io/pull/51) (footer & canonical claims updated to `v0.3.1`) |
| `lgcorzo/fastapi-autogen-team` | `main` | `6b9c4513eea630a04255a68412f2042cd61dbea1` | Synced & clean |

---

## 2. P0 — Unified Fail-Closed Execution Security Policy (`US1` / `T004`–`T010`)

- **Implemented**: [`crates/factory-core/src/security_policy.rs`](../../crates/factory-core/src/security_policy.rs) defines `ExecutionSecurityPolicy`, `ExecutionEnvironment` (`Production`, `Staging`, `Development`, `Unknown`), `WorkloadTrustLevel` (`UntrustedMissionPayload`, `TrustedInternalTest`), `SandboxIsolationType`, and `SecurityPolicyViolation`.
- **Fail-Closed Enforcement**:
  - `SubprocessDriver::execute` ([`crates/factory-mcp-server/src/sandbox.rs`](../../crates/factory-mcp-server/src/sandbox.rs)) validates `ExecutionSecurityPolicy::from_env` and unconditionally rejects `UntrustedMissionPayload` across all environments (`production`, `staging`, `development`, and `unknown`).
  - `build_sandbox_job_spec` ([`crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs`](../../crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs)) rejects `require_gvisor = false` for untrusted workloads in every environment:
    ```text
    Untrusted sandbox execution requires gVisor runtime across all environments. Fallback to standard runc/host is forbidden.
    ```
- **Live Cluster Verification**:
  - When `factory-mcp-server` initially rolled out with legacy `USE_GVISOR=false` in `deployment.yaml`, the new `v0.3.1` binary immediately failed closed in production:
    ```text
    2026-11-09T18:47:34.548225Z ERROR factory_mcp_server: Tool execution error for launch_sandbox_pod: Untrusted sandbox execution requires gVisor runtime across all environments. Fallback to standard runc/host is forbidden.
    ```
  - After updating `USE_GVISOR=true` and `FACTORY_ENV=production` in [PR #201](https://github.com/lgcorzo/gitops_internal_lgcorzo/pull/201), `factory-mcp-server` admitted and scheduled gVisor-isolated Kubernetes Jobs in `factory-sandboxes`.

---

## 3. P0 — Real MicroK8s gVisor Runtime Investigation & Repair (`US2` / `T011`–`T014`)

- **Host Configuration Remediated on Disk**:
  - Updated `/var/snap/microk8s/current/args/containerd-template.toml` and `/var/snap/microk8s/current/args/containerd.toml` (`[plugins."io.containerd.grpc.v1.cri".containerd.runtimes.runsc]`) from `runtime_type = "io.containerd.runc.v2"` to `runtime_type = "io.containerd.runsc.v1"`.
- **Live Runtime Observation (`Job: sandbox-job-818ed351` / `Pod: sandbox-job-818ed351-fksrx`)**:
  - Triggered via `real-mission-trigger` (`real-mission-e7-001-1794250359`, Hatchet workflow run `c08fedda-e484-4246-9903-98e87545f99e`).
  - **Observed Pod Telemetry**:
    - Namespace: `factory-sandboxes`
    - Job Name / UID: `sandbox-job-818ed351` / `6432b493-a95c-4be8-accd-bdac32dde84b`
    - Pod Name / UID: `sandbox-job-818ed351-fksrx` / `b89380f3-532e-4db4-9c4c-f97bee0ab656`
    - Node: `server.internal.lgcorzo` (`192.168.2.45`), Pod IP: `10.1.34.174`
    - Container ID: `containerd://e4c4f265420310891ae2989f02b2221f4218c778f2d3880f81fd108a1e5450bb`
    - Image Digest: `docker.io/library/python@sha256:5e2bdb1b0d542d5039cffb8181c65930093949194a65c365dfb1b0fe13a4fa5b`
    - RuntimeClass: `gvisor`
    - Host Process Verification: `runsc-sandbox` (`PID 669625`), `runsc-gofer` (`PID 669620`), `runsc-fd-parking` (`PID 669626`).
  - **Honest Operational Blocker (`T013`, `T014`)**:
    - Although `/var/snap/microk8s/current/args/containerd.toml` is fixed on disk, restarting the root-owned `microk8s.daemon-containerd` systemd service (`PID 36839`) requires interactive `sudo` credentials unavailable to the autonomous agent shell. Until a human operator runs `sudo snap restart microk8s.daemon-containerd`, the running containerd daemon retains its in-memory `runc.v2` shim mapping and `runsc-sandbox` containers time out at `activeDeadlineSeconds: 60` (failing closed). Therefore, `T013` and `T014` remain explicitly marked **`BLOCKED`** rather than `VERIFIED`.

---

## 4. P0 — Observed Fact-Based Runtime Attestation (`US3` / `T015`–`T018`)

- Extended `SandboxAttestation` in [`crates/factory-mcp-server/src/sandbox.rs`](../../crates/factory-mcp-server/src/sandbox.rs) to record independently observed Kubernetes Pod status fields: `cluster_id`, `node_name`, `job_name`, `pod_name`, `pod_uid`, `container_id`, `container_sha`, `exit_code`, `command_hash` (SHA-256 hex digest of executed command), and `observed_at` (RFC 3339 timestamp).
- Updated `LaunchSandboxPodTool::call` in [`crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs`](../../crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs) to query `kube::Api<Pod>` after job execution and populate `SandboxAttestation` from real `PodStatus` / `ContainerStatus` rather than configuration constants.
- Verified that `RecordingSandboxDriver` sets `attestation: None` and `ExecutionOutcome::Mocked`, preventing mock runs from passing `live_runtime` verification gates.

---

## 5. P1 — FinOps Concurrency & Public Claims Alignment (`US4`, `US5` / `T019`–`T027`)

- **FinOps Concurrency**: Verified unified `.lock` file acquisition (`lock_shared()` / `lock_exclusive()`), atomic `check_and_reserve_budget`, and atomic temp-file rename in [`crates/factory-infrastructure/src/finops_stop_store.rs`](../../crates/factory-infrastructure/src/finops_stop_store.rs) (`cargo test -p factory-application --test finops_admission_tests` — 3/3 passed).
- **Public Claims & Release Footer**: Updated `lgcorzo.github.io` (`data/canonical-claims.json`, `index.html`, `architecture.html`, `lifecycle.html`, `notebooks.html`, `fork-policy.html`) to `v0.3.1` (`013-verified-autonomous-execution-closure`). Verified via `python3 scripts/check_public_claims.py` (`OK: Verified 5 canonical facts`) and `pytest tests/` (`5 passed`).
