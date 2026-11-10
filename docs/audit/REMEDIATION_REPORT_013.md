# DG-RVP-013: Verified Autonomous Execution Closure — Remediation Report

**Target Repository**: `lgcorzo/rust_CACD_autonomous_factory` (`v0.3.3`)  
**Specification**: [`specs/013-verified-autonomous-execution-closure/`](../../specs/013-verified-autonomous-execution-closure/spec.md)  
**Evaluated At (UTC)**: `2026-11-09T21:12:30Z`  
**Cluster**: `server.internal.lgcorzo` (MicroK8s `v1.31.14`, Ubuntu 24.04.5 LTS, kernel `6.17.0-14-generic`)

---

## 1. Inspected & Deployed Repository Revisions (`T001`, `T002`, `T024`, `T025`)

| Repository | Branch | Latest Commit SHA | Release / Status |
|---|---|---|---|
| `lgcorzo/rust_CACD_autonomous_factory` | `main` | Code/Image Release: `7ab32fa66b770761d559a55eb4ed01b3752eac16` (`v0.3.3`)<br>Audit & Evidence Closure: `ec07add0` (+ `tasks.md` sync) | Release [`v0.3.3`](https://github.com/lgcorzo/rust_CACD_autonomous_factory/releases/tag/v0.3.3), CI/CD green (`42689562611`), CodeQL green (`42689562946`) |
| `lgcorzo/gitops_internal_lgcorzo` | `main` | `2716e548fa5bd7563ebfce232ee68a6a24bb11b2` | Merged [PR #202](https://github.com/lgcorzo/gitops_internal_lgcorzo/pull/202), Reconciled by Flux CD (`Ready=True`) |
| `lgcorzo/lgcorzo.github.io` | `main` | `e3f4b13f2f8b1534049cb01f140754cc308573b7` (merged in [PR #53](https://github.com/lgcorzo/lgcorzo.github.io/pull/53)) | Merged [PR #52](https://github.com/lgcorzo/lgcorzo.github.io/pull/52) & [PR #53](https://github.com/lgcorzo/lgcorzo.github.io/pull/53) (all 6 pages & canonical claims synchronized to `v0.3.3`) |
| `lgcorzo/fastapi-autogen-team` | `main` | `07a14f64e58b8498a38d0fa9866468eb5a24a459` | Synced & clean |

---

## 2. P0 — Unified Fail-Closed Execution Security Policy (`US1` / `T004`–`T010`)

- **Implemented**: [`crates/factory-core/src/security_policy.rs`](../../crates/factory-core/src/security_policy.rs) defines `ExecutionSecurityPolicy`, `ExecutionEnvironment` (`Production`, `Staging`, `Development`, `Unknown`), `WorkloadTrustLevel` (`Untrusted`, `TrustedInternal`), `SandboxIsolationType`, and `SecurityPolicyViolation`.
- **Fail-Closed Enforcement**:
  - `SubprocessDriver::execute` ([`crates/factory-mcp-server/src/sandbox.rs`](../../crates/factory-mcp-server/src/sandbox.rs)) validates `ExecutionSecurityPolicy::from_env()` and unconditionally rejects `WorkloadTrustLevel::Untrusted` across all environments (`production`, `staging`, `development`, and `unknown`).
  - `LaunchSandboxPodTool::call` and `build_sandbox_job_spec` ([`crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs`](../../crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs)) derive `require_gvisor` directly from `ExecutionSecurityPolicy` and reject any attempt to disable gVisor for untrusted workloads.

---

## 3. P0 — Real MicroK8s gVisor Runtime Repair & Live Verification (`US2` / `T011`–`T014`)

- **Host Configuration & Shim Protocol Remediated**:
  - Updated `/var/snap/microk8s/current/args/containerd-template.toml` and `/var/snap/microk8s/8565/args/containerd.toml` (`[plugins."io.containerd.grpc.v1.cri".containerd.runtimes.runsc]`) to `runtime_type = "io.containerd.runsc.v1"`.
  - Installed the containerd v1.6.28 bootstrap compatibility wrapper on `/usr/bin/containerd-shim-runsc-v1` (wrapping `/usr/bin/containerd-shim-runsc-v1.real` `v2.1.5`) so MicroK8s `containerd v1.6.28` extracts the plain unix socket path from the v2 JSON ttrpc bootstrap output.
- **Disposable Rust gVisor Probe (`Job: audit-verify-gvisor-rust-013` / `Pod: audit-verify-gvisor-rust-013-mjv54`) — `RUNTIME_VERIFIED`**:
  - Job UID: `931b7fd4-f100-4e50-b917-5a9e621f8e31`
  - Pod UID: `4631d53c-095d-4699-a3dc-8c06de3b2f02`
  - Container ID: `containerd://6d28b823ae7dea3163c4138fc23018cbd56352b21113801000fa25a2fa991ffd`
  - Image Digest: `docker.io/library/rust@sha256:9f841bbe9e7d8e37ceb96ed907265a3a0df7f44e3737d0b100e7907a679acb36`
  - Started / Finished: `2026-11-09T21:07:17Z` -> `2026-11-09T21:07:19Z` (`2s`, `exitCode: 0`, `phase: Succeeded`)
  - Captured stdout: `[DG-RVP-013] gVisor Rust Execution Verified OK`
- **In-Pod Negative Isolation Probe (`Job: audit-verify-gvisor-negative-013` / `Pod: audit-verify-gvisor-negative-013-4pkxz`) — `RUNTIME_VERIFIED`**:
  - Job UID: `79f78952-1bbf-4a2e-8b6c-aeef82e42cbe`
  - Pod UID: `36d4ae1e-20c3-4e11-89c4-4c0a88dd5e0f`
  - Container ID: `containerd://e6687423f379ad00474303663fb080a590c597c316e05b83064c760782dddc61`
  - Exit Code: `0` (`phase: Succeeded`)
  - Observed JSON output:
    ```json
    {
      "dmesg": "[    0.000000] Starting gVisor...\n[    3.384165] Ready!",
      "sa_token_exists": false,
      "ro_rootfs": "BLOCKED: PermissionError",
      "etc_shadow": "BLOCKED: PermissionError",
      "imds_egress": "BLOCKED: TimeoutError"
    }
    ```

---

## 4. P0 — Observed Fact-Based Runtime Attestation & End-to-End Mission (`US3`, `Phase 8` / `T015`–`T018`, `T029`)

- **Live End-to-End Autonomous Mission (`real-mission-e7-001-1794258497` / Hatchet Run `e32a4202-2a30-44a4-b1dd-ef23f620e481`) — `RUNTIME_VERIFIED`**:
  - Triggered via `job.batch/real-mission-trigger` (`pod/real-mission-trigger-54sdj`) in namespace `agents`.
  - **All 5 Hatchet Workflow Steps Succeeded**:
    1. `rustant-plan/0e528dae-bb7b-45c3-8aa5-7792ad61fdf2` (`21:09:08Z`)
    2. `zeroclaw-execute` (SAST Forensic Scan + gVisor sandbox execution)
    3. `zeroclaw-validate/98075b25-cd3d-45fd-9a8b-35ff6d642465` (`21:12:04Z`) — compiled and ran `cargo test` inside gVisor Pod `sandbox-job-a68ab421-mmzwz` (`test tests::it_works ... ok`, `exit_code: 0`, `duration_ms: 10061`)
    4. `rustant-review/08983855-f6ec-481f-b594-7b23225a7980` (`21:12:04Z`)
    5. `factory-deliver/58a28b45-e2f7-46bb-a102-d67df78246e2` (`21:12:05Z`)
  - **Observed Live `SandboxAttestation` Emitted by `factory-mcp-server`**:
    ```json
    {
      "cluster_id": "microk8s-server.internal.lgcorzo",
      "node_name": "server.internal.lgcorzo",
      "job_name": "sandbox-job-a68ab421",
      "pod_name": "sandbox-job-a68ab421-mmzwz",
      "pod_uid": "af7316c2-e003-4110-b9f3-1117992cf4e3",
      "container_id": "containerd://eeaacc796e7b041a26a4303a40eef3eb390c7381b9e6839ebde9ad445b4917bb",
      "container_sha": "docker.io/library/rust@sha256:9f841bbe9e7d8e37ceb96ed907265a3a0df7f44e3737d0b100e7907a679acb36",
      "runtime_class": "gvisor",
      "runtime_type": "io.containerd.runsc.v1",
      "seccomp_profile": "RuntimeDefault",
      "is_root": false,
      "capabilities_dropped": ["ALL"],
      "exit_code": 0,
      "command_hash": "c528d5d4e06bf7ed7bc66aea454a70a39a60fadb9592334ffd13ea5d9acb290a",
      "observed_at": "2026-11-09T21:12:04.070610577Z"
    }
    ```

---

## 5. P1 — FinOps Concurrency, Evidence Gate Hardening & Public Claims Alignment (`US4`, `US5` / `T019`–`T027`)

- **Evidence Gate Hardening (`crates/factory-cli/src/evidence.rs`)**:
  - `validate_evidence_record` strictly prevents `verification_class == "live_runtime"` records from being generated or accepted when `environment.kind` is `"local"` or `"ci"`, when `method.tool` is `"cargo-test"` or `"pytest"`, or when `artifacts[]` only contain Markdown (`.md`) or source code files rather than structured runtime telemetry (`.json`/`.log`). Verified by `test_live_runtime_rejects_unit_test_runner_and_markdown_only_artifacts` in [`crates/factory-cli/tests/evidence_contract_tests.rs`](../../crates/factory-cli/tests/evidence_contract_tests.rs).
- **Attestation Separation (`crates/factory-mcp-server/src/sandbox.rs`, `launch_sandbox_pod.rs`)**:
  - `SandboxAttestation` explicitly separates `requested_spec: RequestedSandboxSpec` (parameters requested in the Kubernetes Job/Pod spec) from `observed_runtime: ObservedRuntimeFacts` (fields genuinely observed from the Kubernetes Pod API: `node_name`, `pod_name`, `pod_uid`, `pod_phase`, `container_id`, `container_image_digest`, `exit_code`, and `host_runtime_handler_observed: None` when only the K8s Pod API is queried). Verified by `test_sandbox_attestation_distinguishes_requested_spec_from_observed_runtime` in [`crates/factory-mcp-server/tests/launch_sandbox_pod_tests.rs`](../../crates/factory-mcp-server/tests/launch_sandbox_pod_tests.rs).
- **FinOps Validation, Multi-Process Concurrency & Shared-Storage Scope**:
  - Unified `.lock` file acquisition (`lock_shared()` / `lock_exclusive()`), strict validation rejecting negative, `NaN`, or infinite `estimated_cost` / `current_spend` / `max_daily_budget`, atomic `check_and_reserve_budget`, and atomic temp-file rename in [`crates/factory-infrastructure/src/finops_stop_store.rs`](../../crates/factory-infrastructure/src/finops_stop_store.rs).
  - Verified across both threads (`test_finops_concurrent_check_and_reserve_prevents_over_admission`), 10 distinct OS processes (`test_finops_cross_process_concurrency_prevents_over_admission`), and invalid financial inputs (`test_finops_rejects_invalid_negative_nan_or_infinite_amounts`) in [`crates/factory-infrastructure/tests/finops_concurrency_test.rs`](../../crates/factory-infrastructure/tests/finops_concurrency_test.rs).
  - Documented explicitly that `FileFinOpsStopStore` only guarantees cross-process consistency on a single node or across pods mounting a shared POSIX `ReadWriteMany` volume.
- **Public Claims & Evidence Artifact Validator (`lgcorzo.github.io`)**:
  - Updated `lgcorzo.github.io` (`scripts/check_public_claims.py` and `tests/test_public_claims.py`) so that it validates not only HTML text strings and `data/canonical-claims.json`, but also the actual evidence records and SHA-256 digests (`evidence/verified_status.json`, `evidence/records/FR-008.json`, `evidence/security/live-gvisor-isolation.json`), rejecting `live_runtime` claims backed only by unit tests or Markdown files.
