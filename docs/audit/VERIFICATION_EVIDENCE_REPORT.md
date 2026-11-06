# Independent Verification Evidence Report (`FR-017`–`FR-021`, `FR-028`, `SC-005`–`SC-006`)

**Specification**: `011-verified-autonomous-execution` (Phase 7 — User Story 5)  
**Evaluated At (UTC)**: `2026-11-06T17:58:00Z`  
**Evaluator / Verifier Identity**: `factory-cli-independent-verifier` (`role: independent_verifier`)  
**Implementation Agent Identity**: `antigravity-implementer` (`role: implementer`)  

---

## 1. Contract & Policy Validation Architecture (`T030`–`T033`)

The independent verification engine in [`crates/factory-cli/src/evidence.rs`](../../crates/factory-cli/src/evidence.rs) and CLI command `factory-cli validate-evidence` ([`crates/factory-cli/src/main.rs`](../../crates/factory-cli/src/main.rs)) enforce two JSON contracts (`specs/011-verified-autonomous-execution/contracts/evidence-record.schema.json` and `verification-state.schema.json`) plus strict business-policy invariants:

| Policy Invariant | Requirement | Enforcement Mechanism in `evidence.rs` | Regression Test in `evidence_contract_tests.rs` | Status |
|---|---|---|---|:---:|
| **Exact 40–64 hex commit SHA** | `FR-001`, `FR-020` | `validate_revision_ref` validates `^[a-fA-F0-9]{40,64}$` on `source_revision` and `deployment_revision` | `test_rejects_short_or_non_hex_commit_sha_and_invalid_requirement_id` | `PASS` |
| **Live Runtime Requires Deployment Revision** | `FR-016`, `FR-020` | `validate_evidence_record` rejects `verification_class == "live_runtime"` when `deployment_revision` is `null` | `test_live_runtime_requires_deployment_revision` | `PASS` |
| **Artifact SHA-256 Content Integrity** | `FR-020`, `FR-021` | `validate_evidence_record` computes `Sha256::digest` on local `file://` artifacts and fails on missing files or digest mismatch | `test_valid_evidence_record_and_verified_state_pass` + `validate-evidence` CLI | `PASS` |
| **No `IMPLEMENTED → VERIFIED` Jump** | `FR-018`, `SC-005` | `is_allowed_transition` only permits `None → OPEN → IMPLEMENTED → TESTED → VERIFIED` and `VERIFIED → OPEN` | `test_rejects_forbidden_implemented_to_verified_transition_jump` | `PASS` |
| **Independent Verifier Identity** | `FR-019`, `SC-006` | Rejects `VERIFIED` state if `verified_by.id == implementation_agent_id` or if `verified_by.id` authored the `IMPLEMENTED` transition | `test_rejects_self_reported_verifier_matching_implementer` | `PASS` |
| **No Mock, Failed, or Stale Promotion** | `FR-006`, `FR-021`, `FR-028`, `SC-005`, `SC-006` | Rejects `VERIFIED` state if any referenced evidence has `verification_class == "mock"`, `outcome != "passed"`, or `source_revision.commit != expected_source_commit` | `test_rejects_mocked_failed_or_stale_evidence_for_verified_requirement` | `PASS` |

---

## 2. Independent Inspection of Generated Evidence (`T034`)

| Requirement / Criterion | Evidence Record Path | Verification Class | Command / Test Executed | Exit Code & Outcome | Status in `evidence/states/` | Limitations / Notes |
|---|---|---|---|:---:|:---:|---|
| `FR-003` | `evidence/records/FR-003.json` | `integration` | `cargo test -p factory-mcp-server --test run_tests_security_tests` | `0` (`passed`) | `VERIFIED` | All untrusted test execution routes through `SandboxDriver`; `sh -c` host bypass removed. |
| `FR-004` | `evidence/records/FR-004.json` | `integration` | `cargo test -p factory-mcp-server --test sandbox_driver_tests` | `0` (`passed`) | `VERIFIED` | Fails closed on unavailable runtime (`gvisor`), rejected runtime, or timeout; zero host fallback. |
| `FR-005` | `evidence/records/FR-005.json` | `unit` | `cargo test -p factory-mcp-server --test run_tests_security_tests` | `0` (`passed`) | `VERIFIED` | Distinguishes `passed`, `failed`, `not_executed`, `infrastructure_error`, `timeout`, and `mocked`. |
| `FR-006` | `evidence/records/FR-006.json` | `unit` | `cargo test -p factory-cli --test evidence_contract_tests` | `0` (`passed`) | `VERIFIED` | `mock` class or `mocked` outcome is rejected by `validate_requirement_state`. |
| `FR-007` | `evidence/records/FR-007.json` | `unit` | `cargo test -p factory-mcp-server --test launch_sandbox_pod_tests` | `0` (`passed`) | `VERIFIED` | Unsupported runtimes (`cobol`, `ruby`, etc.) rejected before job creation. |
| `FR-008` | `evidence/records/FR-008.json` | `infrastructure` | `cargo test -p factory-mcp-server --test launch_sandbox_pod_tests` + `test_sandbox_job_policy.sh` | `0` (`passed`) | `TESTED` | **Kept at `TESTED` (NOT `VERIFIED`)**: While unit, manifest, and live K8s `PodSecurity: restricted` admission checks pass, `AUD-010` (host containerd `runsc` shim config on `server.internal.lgcorzo`) prevents live gVisor container completion (`live_runtime` class). |
| `FR-009` | `evidence/records/FR-009.json` | `integration` | `cargo test -p factory-application --test finops_admission_tests` | `0` (`passed`) | `VERIFIED` | Shared `FileFinOpsStopStore` and `FailingFinOpsStopStore` enforce fail-closed hard-stop across workers. |
| `FR-010` | `evidence/records/FR-010.json` | `integration` | `cargo test -p factory-application --test workflow_recovery_tests` | `0` (`passed`) | `VERIFIED` | `MissionExecutionCoordinator` deduplicates events, checkpoints steps across worker restarts, and compensates on cancellation. |
| `FR-011` | `evidence/records/FR-011.json` | `integration` | `cargo test -p factory-mcp-server --test route_boundary_tests` | `0` (`passed`) | `VERIFIED` | Bearer auth enforced on `/mcp`, `/sse`, `/feedback` even when permissive CORS headers are present. |
| `FR-012` | `evidence/records/FR-012.json` | `integration` | `cargo test -p factory-mcp-server --test webhook_security_tests` | `0` (`passed`) | `VERIFIED` | Protected mode rejects missing secret (`503`), oversized payload (`413`), replay (`409`), and dispatch failure (`502`). |
| `FR-013` | `evidence/records/FR-013.json` | `integration` | `cargo test -p factory-mcp-server --test route_boundary_tests` | `0` (`passed`) | `VERIFIED` | `SseSessionRegistry` enforces max capacity (`429`), owner binding (`403`), TTL expiry, and disconnect cleanup. |
| `FR-014` | `evidence/records/FR-014.json` | `documentation` | `cargo test -p factory-core` | `0` (`passed`) | `VERIFIED` | Documented in `docs/audit/NHI_SECURITY_REVIEW.md` with Ed25519 VC and Vault ACL test references. |
| `FR-015` | `evidence/records/FR-015.json` | `documentation` | `cargo test -p factory-cli --test evidence_contract_tests` | `0` (`passed`) | `VERIFIED` | All 14 controls in `docs/audit/TRACEABILITY_MATRIX.md` classified across all 6 lifecycle stages. |
| `FR-016` | `evidence/records/FR-016.json` | `infrastructure` | `kubectl get kustomizations -A` | `0` (`passed`) | `VERIFIED` | Exact source and Flux revisions (`main@48b9c0e333ce541fc741c056bd67e2932fdc50e9`) recorded in `docs/audit/GITOPS_RUNTIME_VERIFICATION.md`. |
| `FR-018`–`FR-021` | `evidence/records/FR-018.json`..`FR-021.json` | `unit` | `cargo test -p factory-cli --test evidence_contract_tests` | `0` (`passed`) | `VERIFIED` | Contract & policy validator wired into `scripts/generate_evidence.sh`, `Makefile`, and `.github/workflows/pipeline.yml`. |
| `SC-001`–`SC-006` | `evidence/records/SC-001.json`..`SC-006.json` | `unit` / `integration` | `cargo test` suites | `0` (`passed`) | `VERIFIED` | All 6 P1 measurable success criteria validated by automated test suites. |
