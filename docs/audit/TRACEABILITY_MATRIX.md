# Dark Gravity Architecture & Requirement Traceability Matrix

**Specification**: `011-verified-autonomous-execution`  
**Collected At (UTC)**: `2026-11-06T17:10:05Z`  
**Inspected Revisions**:
- `rust_CACD_autonomous_factory`: `5464b9ca025f64751ab3144cc82f2c3f75c88ca7` (branch `011-verified-autonomous-execution`)
- `gitops_internal_lgcorzo`: `48b9c0e333ce541fc741c056bd67e2932fdc50e9` (branch `main`)
- `mlops-python-package`: `bf05a509b1e41597c823d030f2108c9e9c00ed83` (branch `main`)
- `fastapi-autogen-team`: `6b9c4513eea630a04255a68412f2042cd61dbea1` (branch `main`)
- `lgcorzo.github.io`: `e69db33ccceb3e7fdfaaa7bca5d593dedb296f7c` (branch `main`)

---

## 1. Governance Rules (`FR-015`–`FR-021`, `SC-004`–`SC-006`)

1. **Requirement Status Lifecycle**: `OPEN → IMPLEMENTED → TESTED → VERIFIED`. Direct promotion from `IMPLEMENTED` to `VERIFIED` is forbidden (`FR-018`).
2. **Security Control Lifecycle Stages**: Each control is tracked independently across `designed`, `implemented`, `declared`, `reconciled`, `enforced`, and `runtime_verified` (`FR-015`).
3. **No Mock Promotion**: `mock` evidence never satisfies a requirement for `unit`, `integration`, `infrastructure`, or `live_runtime` verification (`FR-006`, `FR-028`).

---

## 2. Functional Requirements Traceability (`FR-001`–`FR-030`)

| Requirement ID | Owning User Story | Acceptance Condition Summary | Required Evidence Classes | Verification Source / Owning Artifact or Test | Current Status |
|---|---|---|---|---|:---:|
| `FR-001` | Setup / `US4` / `US6` | Inspect current revision of every in-scope accessible repository before making repo-specific claims | `documentation`, `infrastructure` | `docs/audit/ECOSYSTEM_INVENTORY.md`, `git rev-parse HEAD` | `IMPLEMENTED` |
| `FR-002` | Setup / `US4` / `US6` | Identify inaccessible repos/environments as `NOT_ACCESSIBLE` or `NOT_VERIFIED` without inferring contents | `documentation` | `docs/audit/ECOSYSTEM_INVENTORY.md`, `docs/audit/GITOPS_RUNTIME_VERIFICATION.md` | `IMPLEMENTED` |
| `FR-003` | `US1` (P1) | Route every untrusted test or generated-code execution through an explicitly approved isolation boundary (`SandboxDriver`) | `unit`, `integration` | `crates/factory-mcp-server/src/tools/run_tests.rs`, `tests/run_tests_security_tests.rs` | `OPEN` |
| `FR-004` | `US1` (P1) | Refuse untrusted execution when approved isolation boundary, runtime, or policy cannot be established; no silent host fallback | `unit`, `integration` | `crates/factory-mcp-server/src/sandbox.rs`, `tests/sandbox_driver_tests.rs`, `tests/gvisor_integration.rs` | `OPEN` |
| `FR-005` | `US1` (P1) | Execution reports distinguish `passed`, `failed`, `not_executed`, `infrastructure_error`, `timeout`, and `mocked` outcomes | `unit`, `integration` | `crates/factory-mcp-server/src/tools/run_tests.rs`, `tests/run_tests_security_tests.rs` | `OPEN` |
| `FR-006` | `US1` / `US5` (P1) | Mocked or simulated result never satisfies a requirement for real test, integration, or runtime execution | `unit`, `integration` | `crates/factory-mcp-server/tests/run_tests_security_tests.rs`, `crates/factory-cli/tests/evidence_contract_tests.rs` | `OPEN` |
| `FR-007` | `US1` (P1) | Supported execution languages/capabilities match available & verified runtimes; unsupported capabilities rejected | `unit`, `integration` | `crates/factory-mcp-server/src/sandbox.rs`, `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs`, `tests/sandbox_driver_tests.rs` | `OPEN` |
| `FR-008` | `US1` / `US4` (P1) | Sandbox workloads enforce approved security, resource, deadline (`activeDeadlineSeconds`), and cleanup (`ttlSecondsAfterFinished`) controls | `unit`, `infrastructure`, `live_runtime` | `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs`, `../gitops_internal_lgcorzo/infrastructure/agents/sandbox-job.yaml`, `../gitops_internal_lgcorzo/tests/test_sandbox_job_policy.sh` | `OPEN` |
| `FR-009` | `US2` (P1) | FinOps monitoring and mission admission consult shared authoritative budget state; active hard stop prevents new mission admission | `unit`, `integration` | `crates/factory-infrastructure/src/finops_stop_store.rs`, `crates/factory-application/src/agents/finops.rs`, `crates/factory-application/tests/finops_admission_tests.rs` | `OPEN` |
| `FR-010` | `US2` (P1) | Mission processing preserves correct behavior for duplicate events, retries, cancellation, worker restart, and in-progress missions under hard stop | `unit`, `integration` | `crates/factory-application/src/workflows/autonomous_mission.rs`, `crates/factory-application/tests/workflow_recovery_tests.rs`, `docs/audit/WORKFLOW_RELIABILITY.md` | `OPEN` |
| `FR-011` | `US3` (P1) | Externally reachable privileged capabilities enforce effective authentication & authorization boundary; CORS is not treated as auth | `unit`, `integration` | `crates/factory-mcp-server/src/lib.rs`, `crates/factory-mcp-server/src/main.rs`, `crates/factory-mcp-server/tests/route_boundary_tests.rs` | `OPEN` |
| `FR-012` | `US3` (P1) | Protected webhook handling rejects missing/invalid production secrets, validates payload bounds & replay, and verifies mission dispatch | `unit`, `integration` | `crates/factory-mcp-server/src/github_webhook.rs`, `crates/factory-mcp-server/tests/webhook_security_tests.rs` | `OPEN` |
| `FR-013` | `US3` (P1) | Event-stream sessions have bounded resource use, defined ownership and expiry, and cleanup on disconnect | `unit`, `integration` | `crates/factory-mcp-server/src/lib.rs`, `crates/factory-mcp-server/tests/route_boundary_tests.rs` | `OPEN` |
| `FR-014` | `US3` (P1) | Identity and credential reviews assess issuer, audience, purpose, validity, revocation, replay, rotation, secret exposure, and assurance limits | `unit`, `documentation` | `crates/factory-core/src/security/nhi.rs`, `docs/audit/NHI_SECURITY_REVIEW.md` | `OPEN` |
| `FR-015` | `US4` (P1) | Each audited control is classified separately across `designed`, `implemented`, `declared`, `reconciled`, `enforced`, and `runtime_verified` | `documentation`, `infrastructure` | `docs/audit/TRACEABILITY_MATRIX.md`, `docs/audit/GITOPS_RUNTIME_VERIFICATION.md` | `IMPLEMENTED` |
| `FR-016` | `US4` (P1) | Runtime verification identifies exact source revision, deployment revision, environment, and observed state; missing access remains an explicit gap | `infrastructure`, `live_runtime`, `documentation` | `docs/audit/GITOPS_RUNTIME_VERIFICATION.md` | `OPEN` |
| `FR-017` | `US5` (P1) | Each requirement has explicit expected behavior, applicable test/runtime evidence, and completion conditions | `documentation`, `unit` | `docs/audit/TRACEABILITY_MATRIX.md`, `crates/factory-cli/src/evidence.rs` | `IMPLEMENTED` |
| `FR-018` | `US5` (P1) | Requirement status progresses through `OPEN → IMPLEMENTED → TESTED → VERIFIED`; `IMPLEMENTED → VERIFIED` is prohibited | `unit`, `integration` | `crates/factory-cli/src/evidence.rs`, `crates/factory-cli/tests/evidence_contract_tests.rs` | `OPEN` |
| `FR-019` | `US5` (P1) | Verification is independent of implementation agent and inspects source, tests, results, configuration, and required runtime evidence | `unit`, `documentation` | `crates/factory-cli/src/evidence.rs`, `docs/audit/VERIFICATION_EVIDENCE_REPORT.md` | `OPEN` |
| `FR-020` | `US5` (P1) | Evidence records identify requirement, source revision, timestamp, environment, verification method, command/test, exit result, outcome, and artifacts | `unit`, `integration` | `specs/011-verified-autonomous-execution/contracts/evidence-record.schema.json`, `crates/factory-cli/src/evidence.rs` | `OPEN` |
| `FR-021` | `US5` (P1) | Verification rejects stale, missing, failed, unreproducible, or insufficient evidence and keeps affected requirement unverified | `unit`, `integration` | `crates/factory-cli/src/evidence.rs`, `crates/factory-cli/tests/evidence_contract_tests.rs`, `scripts/generate_evidence.sh` | `OPEN` |
| `FR-022` | All Stories | Produce ecosystem inventory, integrated findings, traceability matrix, security test plan, GitOps/runtime report, workflow reliability review, NHI review, evidence report, and remediation summary | `documentation` | `docs/audit/*.md` | `OPEN` |
| `FR-023` | `US6` (P2) | Review `rust_CACD_autonomous_factory`, `gitops_internal_lgcorzo`, `mlops-python-package`, `fastapi-autogen-team`, `lgcorzo.github.io`, plus `lince-rs` and `autonomous_factory` when accessible | `documentation`, `unit` | `docs/audit/ECOSYSTEM_INVENTORY.md`, `docs/audit/reviews/mlops-python-package.md`, `docs/audit/reviews/fastapi-autogen-team.md` | `OPEN` |
| `FR-024` | `US6` (P2) | Public technical claims distinguish `designed`, `implemented`, `tested`, `deployed`, `runtime_verified`, `experimental`, and `unverified` capabilities | `unit`, `documentation` | `../lgcorzo.github.io/data/canonical-claims.json`, `../lgcorzo.github.io/scripts/check_public_claims.py` | `OPEN` |
| `FR-025` | `US6` (P2) | Repeated architectural facts, lifecycle descriptions, verification statuses, and repository statistics have a consistent dated source with automated contradiction detection | `unit`, `integration` | `../lgcorzo.github.io/tests/test_public_claims.py`, `../lgcorzo.github.io/.github/workflows/site-consistency.yml` | `OPEN` |
| `FR-026` | `US6` (P2) | Repository renames, public interface changes, and broad cross-repo changes require documented compatibility and migration decisions | `documentation` | `docs/audit/reviews/fastapi-autogen-team.md`, `docs/audit/reviews/mlops-python-package.md` | `OPEN` |
| `FR-027` | `US1`–`US5` | Security-sensitive changes have positive and negative tests; green CI is not presented as proof of live cluster enforcement | `unit`, `integration`, `documentation` | `docs/audit/SECURITY_TEST_PLAN.md`, `docs/audit/README.md` | `OPEN` |
| `FR-028` | `US4` / `US5` | Evidence and reports preserve the distinction between `mock`, `unit`, `integration`, `infrastructure`, and `live_runtime` verification | `unit`, `documentation` | `docs/audit/README.md`, `crates/factory-cli/src/evidence.rs` | `OPEN` |
| `FR-029` | `US6` (P2) | Public-site review assesses metadata, structured info, accessibility, responsive behavior, and measured performance; unmeasured items are not marked verified | `documentation`, `accessibility`, `performance` | `docs/audit/PUBLIC_SITE_REVIEW.md` | `OPEN` |
| `FR-030` | `US6` (P2) | Supporting-repo reviews record dependency/runtime compatibility, reproducibility, security checks, and public-interface implications | `documentation`, `unit` | `docs/audit/reviews/mlops-python-package.md`, `docs/audit/reviews/fastapi-autogen-team.md` | `OPEN` |

---

## 3. Measurable Success Criteria Traceability (`SC-001`–`SC-012`)

| Criterion ID | Owning User Story | Measurable Target | Required Evidence Classes | Verification Method / Source | Current Status |
|---|---|---|---|---|:---:|
| `SC-001` | `US1` | 100% of in-scope untrusted execution entry points use approved isolation boundary; 100% of isolation-unavailable scenarios fail closed | `unit`, `integration` | `cargo test -p factory-mcp-server --test run_tests_security_tests --test sandbox_driver_tests` | `OPEN` |
| `SC-002` | `US1` | 0 mocked, simulated, missing-runner, or unsupported-runtime cases reported as real successful test executions | `unit`, `integration` | `cargo test -p factory-mcp-server --test run_tests_security_tests` | `OPEN` |
| `SC-003` | `US2` | 100% of mission admission checks use shared authoritative hard-stop state; budget-exhaustion integration scenario rejects all post-stop admissions | `unit`, `integration` | `cargo test -p factory-application --test finops_admission_tests --test workflow_recovery_tests` | `OPEN` |
| `SC-004` | `US4` | 100% of security controls in traceability matrix state lifecycle stage evidence; 0 controls marked `runtime_verified` without live evidence tied to current revision | `infrastructure`, `documentation` | `docs/audit/TRACEABILITY_MATRIX.md`, `docs/audit/GITOPS_RUNTIME_VERIFICATION.md` | `OPEN` |
| `SC-005` | `US5` | 100% of requirements marked `VERIFIED` have complete, inspectable evidence for all mandatory acceptance conditions; stale/missing evidence produces 0 `VERIFIED` status | `unit`, `integration` | `crates/factory-cli/tests/evidence_contract_tests.rs`, `docs/audit/VERIFICATION_EVIDENCE_REPORT.md` | `OPEN` |
| `SC-006` | `US5` | Independent verification flow rejects all test cases containing failed, missing, mocked-only, stale, or self-reported-only evidence | `unit`, `integration` | `cargo test -p factory-cli --test evidence_contract_tests` | `OPEN` |
| `SC-007` | Setup / `US6` | 100% of accessible in-scope repos have recorded revision & review status; inaccessible repos explicitly identified without inferred findings | `documentation` | `docs/audit/ECOSYSTEM_INVENTORY.md` | `IMPLEMENTED` |
| `SC-008` | `US6` | Automated documentation checks report 0 unresolved contradictions among canonical facts selected for publication | `unit`, `integration` | `python3 ../lgcorzo.github.io/scripts/check_public_claims.py`, `pytest ../lgcorzo.github.io/tests/test_public_claims.py` | `OPEN` |
| `SC-009` | Setup / Polish | Every deliverable distinguishes current findings from historical evidence and records unresolved limitations, access gaps, and remaining risks | `documentation` | `docs/audit/INTEGRATED_AUDIT.md`, `docs/audit/REMEDIATION_SUMMARY.md` | `OPEN` |
| `SC-010` | Polish | Maintainers can determine from final report, for every P0/P1 finding, its verified impact, evidence, remediation status, and outstanding live-runtime validation | `documentation` | `docs/audit/REMEDIATION_SUMMARY.md` | `OPEN` |
| `SC-011` | `US6` | 100% of public-site performance, accessibility, and runtime claims labeled verified are backed by measurements or runtime evidence | `documentation`, `accessibility`, `performance` | `docs/audit/PUBLIC_SITE_REVIEW.md`, `../lgcorzo.github.io/data/canonical-claims.json` | `OPEN` |
| `SC-012` | `US6` | Every reviewed supporting repository has recorded compatibility & reproducibility outcome; no public-interface change accepted without compatibility evidence | `documentation`, `unit` | `docs/audit/reviews/mlops-python-package.md`, `docs/audit/reviews/fastapi-autogen-team.md` | `OPEN` |

---

## 4. Security Controls Lifecycle Matrix (`FR-015`, `SC-004`)

| Control ID | Security Control | `designed` | `implemented` (App Code) | `declared` (GitOps Manifest) | `reconciled` (Flux State) | `enforced` (K8s / App Boundary) | `runtime_verified` (Live Workload) |
|---|---|---|---|---|---|---|---|
| `CTRL-01` | **Sandbox Execution Driver Boundary** (`SandboxDriver`) | `PASS` (`spec.md` US1) | `PARTIAL` (`sandbox.rs` has `SubprocessDriver` production check, but `run_tests.rs` bypasses driver via `sh -c`) | `N/A` (App-level injection) | `N/A` | `FAIL` (`run_tests.rs` runs on host shell) | `NOT_VERIFIED` |
| `CTRL-02` | **gVisor `RuntimeClass` Isolation** (`runsc`) | `PASS` (`spec.md` US1/US4) | `PARTIAL` (`launch_sandbox_pod.rs` enforces `require_gvisor` only when `FACTORY_ENV=production` or `USE_GVISOR=true`) | `PASS` (`gitops_internal_lgcorzo/infrastructure/agents/sandbox-job.yaml` declares `runtimeClassName: gvisor`) | `NOT_VERIFIED` | `NOT_VERIFIED` | `NOT_VERIFIED` |
| `CTRL-03` | **Test Outcome Integrity & Non-Simulation** | `PASS` (`spec.md` FR-005/FR-006) | `FAIL` (`run_tests.rs:70` returns `Passed` for `"mock"` commands) | `N/A` | `N/A` | `FAIL` | `NOT_VERIFIED` |
| `CTRL-04` | **Pod & Container Security Context Hardening** (non-root `65534`, `readOnlyRootFilesystem`, drop `ALL` caps, seccomp) | `PASS` (`spec.md` FR-008) | `PARTIAL` (`launch_sandbox_pod.rs` omits `readOnlyRootFilesystem: true` and `/tmp` emptyDir) | `PASS` (`sandbox-job.yaml` declares non-root, `readOnlyRootFilesystem: true`, drop `ALL`, seccomp `RuntimeDefault`) | `NOT_VERIFIED` | `NOT_VERIFIED` | `NOT_VERIFIED` |
| `CTRL-05` | **Dedicated Sandbox Namespace & PodSecurity Standard (`restricted`)** | `PASS` (`spec.md` US4) | `PASS` (`launch_sandbox_pod.rs` defaults to `factory-sandboxes`) | `PASS` (`sandboxes-namespace.yaml` declares `pod-security.kubernetes.io/enforce: restricted`) | `NOT_VERIFIED` | `NOT_VERIFIED` | `NOT_VERIFIED` |
| `CTRL-06` | **Sandbox Network Egress & Ingress Default-Deny** | `PASS` (`spec.md` US4) | `N/A` (CNI policy) | `PASS` (`sandboxes-networkpolicy.yaml` declares default-deny Ingress/Egress in `factory-sandboxes`) | `NOT_VERIFIED` | `NOT_VERIFIED` | `NOT_VERIFIED` |
| `CTRL-07` | **Namespace-Scoped Sandbox Launcher RBAC** | `PASS` (`spec.md` US4) | `N/A` (K8s RBAC) | `PASS` (`infrastructure/agents/rbac.yaml` scopes `Role`/`RoleBinding` to `factory-sandboxes`) | `NOT_VERIFIED` | `NOT_VERIFIED` | `NOT_VERIFIED` |
| `CTRL-08` | **Sandbox Resource Limits, Deadline & TTL Cleanup** | `PASS` (`spec.md` FR-008) | `PARTIAL` (`launch_sandbox_pod.rs` sets `ttlSecondsAfterFinished: 300`, CPU/mem limits, but lacks `activeDeadlineSeconds` & `ephemeral-storage`) | `PARTIAL` (`sandbox-job.yaml` sets CPU/mem/ephemeral-storage & TTL, but lacks `activeDeadlineSeconds` and has unrendered `{{JOB_NAME}}` placeholders) | `NOT_VERIFIED` | `NOT_VERIFIED` | `NOT_VERIFIED` |
| `CTRL-09` | **Authoritative Cross-Worker FinOps Hard-Stop Admission Gate** | `PASS` (`spec.md` US2) | `FAIL` (`autonomous_mission.rs` creates independent `FinOpsAgent::default()` latches per call) | `N/A` | `N/A` | `FAIL` | `NOT_VERIFIED` |
| `CTRL-10` | **Idempotent Mission Event Handling & Recovery State** | `PASS` (`spec.md` US2) | `FAIL` (No durable idempotency/step recovery store in `autonomous_mission.rs`) | `N/A` | `N/A` | `FAIL` | `NOT_VERIFIED` |
| `CTRL-11` | **Webhook Authentication, Payload Bounds, Replay Protection & Verifiable Dispatch** | `PASS` (`spec.md` US3) | `FAIL` (`github_webhook.rs` skips HMAC if secret env is empty, lacks replay/size checks, and does not dispatch mission) | `N/A` | `N/A` | `FAIL` | `NOT_VERIFIED` |
| `CTRL-12` | **Privileged Route Authentication & Bounded Event-Stream Sessions** | `PASS` (`spec.md` US3) | `FAIL` (`lib.rs` / `main.rs` lack route auth middleware and bounded SSE session lifecycle) | `N/A` | `N/A` | `FAIL` | `NOT_VERIFIED` |
| `CTRL-13` | **NHI Ed25519 Verifiable Credentials & Vault Repo-Scoped ACLs** | `PASS` (`spec.md` US3) | `PASS` (`factory-core/src/security/nhi.rs`, `factory-infrastructure/src/vault.rs`) | `PASS` (`infrastructure/security/`) | `NOT_VERIFIED` | `PASS` (Unit/integration verified in `mission_input_security_tests.rs` & `vault_isolation_tests.rs`) | `NOT_VERIFIED` |
| `CTRL-14` | **Independent Evidence Contract & Transition Policy Gate** | `PASS` (`spec.md` US5) | `FAIL` (`factory-cli` and `generate_evidence.sh` do not yet enforce `contracts/*.schema.json` or independent verifier rules) | `N/A` | `N/A` | `FAIL` | `N/A` |
