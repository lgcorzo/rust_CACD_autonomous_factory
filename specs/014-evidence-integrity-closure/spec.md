# Specification: DG-RVP-014: Evidence Integrity & Independent Verification Closure

**Feature Name**: `014-evidence-integrity-closure`  
**Program**: DG-RVP-014  
**Target Outcome**: Evidence integrity controls implemented; independent assessment is not claimed.
**Baseline Commit**: `ec07add0b8ff778ccde04b2bd4459b7a8cc0a3d1`  
**Target Repository**: `lgcorzo/rust_CACD_autonomous_factory`  

---

## 1. Problem Statement
The Dark Gravity audit identified remaining architectural defects in evidence qualification, runtime attestation, FinOps budget validation, and automated CI verification:
1. An evidence record for `FR-008` was categorized as `live_runtime` despite originating from `cargo test` in a `local` workstation and pointing to a Markdown file.
2. Runtime attestations previously echoed requested Job specifications into observed runtime fields without independently corroborating node CRI facts.
3. FinOps budget stores did not validate input values (`NaN`, negative, infinity) and lacked multi-process concurrency testing.
4. CI checked file existence rather than enforcing deep evidence integrity (digests, classifications, fresh commits).
5. Public website claims were validated by static link checks rather than deep artifact and record validation.

---

## 2. Mandatory Remediation Requirements

### 2.1 Evidence Gate Integrity (P0)
- **EVID-001**: Implement canonical evidence classification hierarchy (`UNIT`, `COMPONENT`, `INTEGRATION`, `KUBERNETES_ADMISSION`, `LIVE_RUNTIME`, `SECURITY_NEGATIVE`, `CHAOS`, `MANUAL_REVIEW`). Lower-level classes can never satisfy higher-level requirements.
- **EVID-002**: Reject `live_runtime` records from unit test runners (`cargo-test`, `pytest`) or local/CI environments.
- **EVID-003**: Require structured runtime telemetry (`.json` or `.log`) for `live_runtime`; reject Markdown-only artifacts.
- **EVID-004**: Enforce strict exit code and outcome coherence (`outcome: passed` requires `exit_code: 0`).
- **EVID-005**: Ten mandatory negative tests in `crates/factory-cli/tests/evidence_contract_tests.rs`.

### 2.2 Runtime Attestation Separation (P0)
- **ATST-001**: Structurally partition `SandboxAttestation` into:
  - `requested_spec`: Requested parameters from Job spec.
  - `observed_runtime`: Truthful facts queried from K8s Pod API and host CRI.
  - `verified_checks`: Corroboration results.
- **ATST-002**: Never infer `io.containerd.runsc.v1` merely from `require_gvisor: true`. Record `UNVERIFIED` if host runtime handler is unobserved.
- **ATST-003**: Implement comprehensive negative attestation tests in `crates/factory-mcp-server/tests/launch_sandbox_pod_tests.rs`.

### 2.3 FinOps Validation & Concurrency (P1)
- **FIN-001**: Validate all financial inputs (`estimated_cost`, `current_spend`, `max_daily_budget`): reject negative, `NaN`, infinite, or non-positive values fail-closed.
- **FIN-002**: Multi-process concurrency tests spawning 10 independent OS child processes to prove advisory file locking works across distinct processes.
- **FIN-003**: Fault injection tests (corrupt state, truncated state, leftover tmp files, storage unavailability).
- **FIN-004**: Explicitly document single-node / shared POSIX (`ReadWriteMany`) storage boundary.

### 2.4 Traceability & CI (P1)
- **TRAC-001**: Align `specs/013-.../tasks.md` and `verification-matrix.md` with exact implementation commits and test names.
- **CI-001**: Add Evidence Integrity CI stage in `.github/workflows/pipeline.yml` running `factory-cli validate-evidence`.
- **SITE-001**: Update `lgcorzo.github.io/scripts/check_public_claims.py` to validate evidence files, SHA-256 digests, and non-mock statuses.
