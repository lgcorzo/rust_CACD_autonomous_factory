# Remediation Report: DG-RVP-015
## Live Runtime Evidence & Governance Closure

**Programme ID:** DG-RVP-015  
**Baseline Commit:** `0a1918081ac6307051aee73ffadddb372e032db8` (Release v0.3.4)  
**Target Repository:** `lgcorzo/rust_CACD_autonomous_factory`  
**Related Repositories:** `lgcorzo/gitops_internal_lgcorzo`, `lgcorzo/lgcorzo.github.io`  
**Execution Date:** 2026-11-10  
**Overall Outcome:** **VERIFIED & CLOSED**

---

## 1. Executive Summary

Mission **DG-RVP-015** remediated and closed the two remaining governance and evidence items from prior audits:
1. **FR-008 Live Runtime Isolation Under gVisor**: Conducted independent live execution and node telemetry collection on MicroK8s (`server.internal.lgcorzo`), proving strict container containment under `/usr/bin/runsc` (`4.19.0-gvisor`) and exact cryptographic match with published OCI image digest `sha256:331cd9f24e23c8b280fd1ed643e2f6d6ff773211b6429a31baac48052bf60b3b`. Registered `evidence/records/FR-008.json` and transitioned `evidence/states/FR-008.json` to `VERIFIED`.
2. **CI Governance Gate (`Verify Spec Artifacts`)**: Diagnosed the root cause of `SKIPPED` execution in CI. Removed conditional branch guards in `.github/workflows/pipeline.yml`, implemented `scripts/verify_spec_artifacts.py`, backfilled all required spec templates for feature closures, and added automated negative regression tests (`tests/test_verify_spec_artifacts.py`).

---

## 2. Detailed Remediation Actions

### 2.1 FR-008 Live Runtime Evidence Closure
- Created probe workload manifesting the official published Docker image (`docker.io/lgcorzo/dark-gravity-factory:0a1918081ac6307051aee73ffadddb372e032db8`).
- Confirmed node runtime handler: MicroK8s containerd configuration defines `runtime_type = "io.containerd.runsc.v1"` and binary `/usr/bin/runsc`.
- Confirmed pod kernel reporting: `Linux test-factory-image-probe 4.19.0-gvisor #1 SMP Sun Jan 10 15:06:54 PST 2016 x86_64 Linux`.
- Confirmed host node divergence: Host OS runs Linux kernel `6.17.0-14-generic`.
- Executed negative security admission probes verifying that pods lacking non-root or privilege escalation restrictions are rejected by Kubernetes admission under `restricted:latest`.
- Collected full raw JSON telemetry into `evidence/security/live-gvisor-isolation-015.json` and run log into `evidence/runs/FR-008.log`.
- Validated records using `factory-cli validate-evidence`, confirming full policy compliance across all 11 records and 11 requirement states.

### 2.2 Governance Gates Unconditional Enforcement
- Removed restrictive `if: startsWith(...)` branch filter on `Verify Spec Artifacts` in `.github/workflows/pipeline.yml`.
- Created Python validator `scripts/verify_spec_artifacts.py` checking the presence of `spec.md`, `plan.md`, and `tasks.md`, and confirming that completed tasks reference real workspace files.
- Added comprehensive unit tests in `tests/test_verify_spec_artifacts.py` validating both positive passes and negative failure modes.
- Generated `plan.md` for `014` and complete specification artifacts (`spec.md`, `plan.md`, `tasks.md`) for `015`.

---

## 3. Deliverables Summary

- `evidence/records/FR-008.json`
- `evidence/states/FR-008.json`
- `evidence/runs/FR-008.log`
- `evidence/security/live-gvisor-isolation-015.json`
- `docs/audit/FR008_LIVE_RUNTIME_EVIDENCE.md`
- `docs/audit/GOVERNANCE_GATES_VERIFICATION.md`
- `docs/audit/DEPLOYMENT_PROVENANCE.md`
- `docs/audit/REMEDIATION_REPORT_015.md`
- `scripts/verify_spec_artifacts.py`
- `tests/test_verify_spec_artifacts.py`
- `.github/workflows/pipeline.yml`
