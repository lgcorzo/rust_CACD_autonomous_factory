# Integrated Security, Kubernetes & GitOps Audit Report

**Audit Target**: Dark Gravity CA/CD Autonomous Software Factory & Supporting Ecosystem  
**Feature**: `011-verified-autonomous-execution`  
**Collected At (UTC)**: `2026-11-06T17:10:05Z`  
**Auditor**: Antigravity Autonomous Security & Assurance Engineer  
**Status**: `Phase 1 - Revision-Bound Baseline Established`

---

## 1. Executive Summary

This report updates the `010-integrated-k8s-gitops-audit` baseline against the exact current source and GitOps revisions inspected at `2026-11-06T17:10:05Z`. In accordance with `FR-001`, `FR-002`, `FR-015`, `FR-022`, and `SC-009`, every prior audit claim and `evidence/` artifact is explicitly classified as **Current**, **Historical**, **Contradictory**, or **Unverified** against the inspected revisions.

No capability is treated as `runtime_verified` based solely on source code, unit tests, static YAML manifests, or older commit-bound evidence.

---

## 2. Inspected Repository Revisions (`2026-11-06T17:10:05Z`)

| Repository | Branch | Full Commit SHA | Access Status | Prior Audit SHA (`2026-11-02`) |
|---|---|---|---|---|
| `rust_CACD_autonomous_factory` | `011-verified-autonomous-execution` (`main`: `a894328a1d7c4c63a159a5d3795e6fb2a54ca8e3`) | `5464b9ca025f64751ab3144cc82f2c3f75c88ca7` | `accessible` | `62aa735331a254be1b32afe8b60c9a983398e3e8` |
| `gitops_internal_lgcorzo` | `main` | `48b9c0e333ce541fc741c056bd67e2932fdc50e9` | `accessible` | `c24b52c837d9a6f256c9ecad734df870a76419e1` |
| `mlops-python-package` | `main` | `bf05a509b1e41597c823d030f2108c9e9c00ed83` | `accessible` | Not previously audited |
| `fastapi-autogen-team` | `main` | `6b9c4513eea630a04255a68412f2042cd61dbea1` | `accessible` | Not previously audited |
| `lgcorzo.github.io` | `main` | `e69db33ccceb3e7fdfaaa7bca5d593dedb296f7c` | `accessible` | Not previously audited |
| `autonomous_factory` | `main` | `4a8f2f3eb0b4484c0627919a5e3020bc1beb02f7` | `accessible` | Not previously audited |
| `lince-rs` | `N/A` | `N/A` | `not_accessible` | Not previously audited |

---

## 3. Classification of Existing `evidence/` & Prior Audit Artifacts

Every existing artifact in `evidence/` and `docs/audit/` has been compared against `rust_CACD_autonomous_factory@5464b9ca025f64751ab3144cc82f2c3f75c88ca7` and `gitops_internal_lgcorzo@48b9c0e333ce541fc741c056bd67e2932fdc50e9`:

| Artifact Path | Recorded Date / Commit | Classification | Rationale Against Current Revisions |
|---|---|---|---|
| `evidence/verified_status.json` | `2026-10-18T08:57:38Z` (`93024823eaa7d8322e32aff177daa481ea49a17a`) | **Historical** (Stale for `011`) | Bound to commit `93024823...` on branch `hardening/T5.1-jules-benchmark-protocol`. Lacks `schema_version`, `requirement_id`, `verification_class`, and independent verifier identity (`FR-019`–`FR-021`). |
| `evidence/baseline.md` | `2026-10-01` (`fbdc5848`) | **Historical & Contradictory** | Claims `zeroclaw-worker` and `sandbox-exec` run with gVisor and Cilium deny-all egress at `fbdc5848`, whereas `run_tests.rs:91` in `5464b9ca...` still spawns `tokio::process::Command::new("sh")` directly on the host and `gitops_internal_lgcorzo` uses `factory-sandboxes` namespace. |
| `evidence/security-tests.md` | `2026-10-15` | **Historical** | Documents secret redaction and 30 MiB memory benchmarks from October 2026; not bound to current commit `5464b9ca...` or current `launch_sandbox_pod.rs` Job spec. |
| `evidence/reliability-tests.md` | `2026-10-15` | **Historical & Contradictory** | Claims Hatchet step recovery and idempotent cursor management prevent duplicate side effects, whereas `crates/factory-application/src/workflows/autonomous_mission.rs:406` currently constructs `FinOpsAgent::default()` per task invocation and lacks durable idempotency/retry state across workers. |
| `evidence/security/adversarial-tests.md`, `corpus_results.json`, `heldout_results.json` | `2026-10-16` / `2026-11-02` | **Historical** | Point-in-time AST heuristic scanner benchmark results; valid only as unit/corpus history, not live sandbox isolation proof. |
| `evidence/missions/mission-001/*` | `2026-10-16` | **Historical** | Synthetic/historical mission trace artifacts; not bound to `5464b9ca...`. |
| `docs/audit/INTEGRATED_AUDIT.md` (prior `010` version) | `2026-11-02` (`62aa7353...` / `c24b52c8...`) | **Partially Current / Partially Historical** | Several findings from `010` were partially addressed in commit `a894328a` (`#591`), while critical defects remain active in `5464b9ca...` as detailed in Section 4 below. |

---

## 4. Current Findings Ranked by Severity (`5464b9ca` / `48b9c0e3`)

### 4.1 P0 Findings (Critical — Execution Isolation, Budget Enforcement & External Boundaries)

#### Finding AUD-001 (P0): `run_tests` Executes Directly on Host Shell (`sh -c`) and Reports Mocked Runs as `Passed`
- **Status**: **Current (Active Defect in `5464b9ca025f64751ab3144cc82f2c3f75c88ca7`)**
- **Location**: `crates/factory-mcp-server/src/tools/run_tests.rs:31-34, 70-94`
- **Observed State**:
  1. `RunTestsTool` holds `#[allow(dead_code)] driver: Arc<dyn SandboxDriver>` and never invokes `self.driver`.
  2. Lines `91-92` execute `tokio::process::Command::new("sh").arg("-c").arg(test_cmd)` directly on the factory host regardless of `SandboxDriver` or environment mode.
  3. Lines `70-87` intercept any command starting with `"mock"` and return `status: "success"`, `outcome: TestExecutionOutcome::Passed`, `exit_code: Some(0)`, conflating mocked runs with real passing executions (`FR-005`, `FR-006`).
- **Required Remediation**: `T006`–`T009` (Route through `SandboxDriver`, validate executable/argument boundaries, add `TestExecutionOutcome::Mocked`, fail closed when sandbox/runner is unavailable).

#### Finding AUD-002 (P0): `SubprocessDriver` Stubs Out Rust/Go Code Execution and Lacks Runtime Rejection Boundaries
- **Status**: **Current (Active Defect in `5464b9ca025f64751ab3144cc82f2c3f75c88ca7`)**
- **Location**: `crates/factory-mcp-server/src/sandbox.rs:105-127`
- **Observed State**:
  1. While `SubprocessDriver::execute` checks `self.is_production()`, in non-production mode its `"rust"` branch ignores the caller's `code` parameter and runs `rustc -e 'println!("{}", "Hello from Rust!")'`, and its `"go"` branch runs `go run -` with a comment stating `"We'll mock it like rustc for now"`.
  2. `launch_sandbox_pod.rs:60, 201` uses image `python:3.11-slim` while attempting `rustc -e` for `"rust"` workloads, and omits `readOnlyRootFilesystem: true`, `activeDeadlineSeconds`, and `ephemeral-storage` bounds that are declared in `gitops_internal_lgcorzo/infrastructure/agents/sandbox-job.yaml`.
- **Required Remediation**: `T007`, `T010`, `T011`.

#### Finding AUD-003 (P0): FinOps Hard-Stop Latch Is Process-Local and Re-Instantiated Per Mission Admission
- **Status**: **Current (Active Defect in `5464b9ca025f64751ab3144cc82f2c3f75c88ca7`)**
- **Location**:
  - `crates/factory-application/src/agents/finops.rs:54, 100`
  - `crates/factory-application/src/workflows/autonomous_mission.rs:303, 406-407`
- **Observed State**:
  - In `autonomous_mission.rs:303`, the background budget monitor spawns `let finops_agent = FinOpsAgent::default();` with its own fresh `Arc<AtomicBool>` (`halted = false`).
  - In `autonomous_mission.rs:406`, the `rustant-plan` admission check constructs a brand-new `let finops_checker = FinOpsAgent::default();` whose `halted` latch is always `false`.
  - Consequently, when the budget monitor trips a hard stop, new missions submitted to the workflow (or across multiple workers) are **never blocked** (`FR-009`, `SC-003`).
- **Required Remediation**: `T012`–`T015` (Shared, durable `FinOpsStopStore` with versioned stop state and fail-closed admission).

#### Finding AUD-004 (P0): Mission Workflow Lacks Durable Idempotency, Retry State, and Worker-Restart Recovery Guarantees
- **Status**: **Current (Active Defect in `5464b9ca025f64751ab3144cc82f2c3f75c88ca7`)**
- **Location**: `crates/factory-application/src/workflows/autonomous_mission.rs:392-620`
- **Observed State**:
  - Duplicate webhook/event deliveries generate new mission runs or repeat task execution without consulting an idempotency registry or persisted step state (`FR-010`).
  - Cancellation and worker-restart compensation semantics are not persisted in a deterministic mission state store.
- **Required Remediation**: `T013`, `T016`, `T017`.

#### Finding AUD-005 (P0): GitHub Webhook Endpoint Accepts Unsigned Requests When `GITHUB_WEBHOOK_SECRET` Is Unset and Simulates Mission Dispatch
- **Status**: **Current (Active Defect in `5464b9ca025f64751ab3144cc82f2c3f75c88ca7`)**
- **Location**: `crates/factory-mcp-server/src/github_webhook.rs:59-147`
- **Observed State**:
  1. Line `67` checks `if !webhook_secret.is_empty()` — if `GITHUB_WEBHOOK_SECRET` is unset in a protected environment, signature verification is silently skipped (`FR-012`).
  2. No payload size limit or delivery replay (`X-GitHub-Delivery`) tracking is enforced.
  3. Lines `136-145` return `HTTP 200 {"status": "mission_triggered"}` without actually submitting the mission to any dispatcher or queue (`_server` is unused).
- **Required Remediation**: `T018`, `T020`.

#### Finding AUD-006 (P0): Privileged MCP HTTP Routes Lack Effective Caller Authentication and Unbounded SSE Sessions
- **Status**: **Current (Active Defect in `5464b9ca025f64751ab3144cc82f2c3f75c88ca7`)**
- **Location**: `crates/factory-mcp-server/src/lib.rs`, `crates/factory-mcp-server/src/main.rs`
- **Observed State**:
  - Privileged tool invocation endpoints (`/mcp`, `/sse`, `/message`) rely on network/CORS assumptions rather than enforcing caller authentication/authorization in protected configurations (`FR-011`).
  - SSE / event-stream sessions lack explicit per-session ownership, maximum active session bounds, expiry, and disconnect cleanup (`FR-013`).
- **Required Remediation**: `T019`, `T021`, `T022`.

---

### 4.2 P1 Findings (High — Declared vs. Enforced Controls, Evidence Governance & Public Claims)

#### Finding AUD-007 (P1): Drift Between Dynamically Generated Sandbox Job Spec and GitOps Sandbox Job Template
- **Status**: **Current (`5464b9ca` vs `gitops_internal_lgcorzo@48b9c0e3`)**
- **Location**:
  - `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs:29-83`
  - `gitops_internal_lgcorzo/infrastructure/agents/sandbox-job.yaml:1-44`
- **Observed State**:
  - `gitops_internal_lgcorzo` (`48b9c0e3`) now declares `factory-sandboxes` namespace (`sandboxes-namespace.yaml`), default-deny NetworkPolicy (`sandboxes-networkpolicy.yaml`), scoped RBAC (`rbac.yaml`), and `readOnlyRootFilesystem: true` in `sandbox-job.yaml`, but `sandbox-job.yaml` contains literal `{{JOB_NAME}}` template placeholders and lacks `activeDeadlineSeconds`.
  - Meanwhile, `launch_sandbox_pod.rs` omits `readOnlyRootFilesystem: true`, `activeDeadlineSeconds`, `ephemeral-storage` limits, and writable `/tmp` `emptyDir` volume mounts needed under `readOnlyRootFilesystem`.
  - Neither layer has current `runtime_verified` evidence tied to `5464b9ca` / `48b9c0e3` (`FR-015`, `FR-016`).
- **Required Remediation**: `T010`, `T024`–`T029`.

#### Finding AUD-008 (P1): Evidence Generation (`scripts/generate_evidence.sh`) Does Not Enforce Versioned Contracts or Independent Verifier Transitions
- **Status**: **Current (`5464b9ca025f64751ab3144cc82f2c3f75c88ca7`)**
- **Location**: `scripts/generate_evidence.sh`, `crates/factory-cli/src/main.rs`, `evidence/verified_status.json`
- **Observed State**:
  - `verified_status.json` is a flat summary object without requirement-level `EvidenceRecord` or `VerificationState` validation against `contracts/evidence-record.schema.json` and `contracts/verification-state.schema.json` (`FR-018`–`FR-021`).
- **Required Remediation**: `T030`–`T034`.

#### Finding AUD-009 (P1): Public Website (`lgcorzo.github.io`) & Supporting Repo Claims Lack Automated Canonical Claim Validation
- **Status**: **Current (`lgcorzo.github.io@e69db33ccceb3e7fdfaaa7bca5d593dedb296f7c`)**
- **Location**: `../lgcorzo.github.io/*.html`
- **Observed State**:
  - Public pages present architectural, lifecycle, and security claims without a machine-checked `data/canonical-claims.json` registry or CI contradiction check (`FR-024`, `FR-025`, `FR-029`).
- **Required Remediation**: `T035`–`T043`.

---

## 5. Summary of Unverified / Unavailable Layers at Baseline

- **Live Kubernetes Cluster (`server.internal.lgcorzo`)**: Marked `NOT_VERIFIED` against `5464b9ca` / `48b9c0e3` until `T011`, `T027`, and `T028` are executed against the cluster or recorded as `unavailable` if cluster access is not reachable from the current execution environment.
- **Repository `lince-rs`**: Marked `NOT_ACCESSIBLE`; zero findings inferred (`FR-002`).
