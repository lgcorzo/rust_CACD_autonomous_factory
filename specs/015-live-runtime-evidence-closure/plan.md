# Implementation Plan — Live Runtime Evidence & Governance Closure

**Program:** DG-RVP-015  
**Baseline Commit:** `0a1918081ac6307051aee73ffadddb372e032db8`  
**Status:** In Execution  

## 1. Approach

### Phase 1: Live Runtime Corroboration (FR-008)
- Execute authorized live gVisor isolation probes on the local MicroK8s cluster. The namespace is **TO CONFIRM** before execution: `deploy/` defines no `Namespace` objects, and checked-in configuration refers to different namespace names.
- Capture:
  - Pod spec admitted by K8s API server
  - Container runtime logs from `snap.microk8s.daemon-containerd` correlating container ID with `/usr/bin/runsc`
  - Container execution output verifying gVisor sandbox boot (`Starting gVisor...`, `4.19.0-gvisor`)
  - Negative isolation checks (rejection of unhardened pods, rejection of invalid runtime classes, read-only rootfs prevention)
- Produce canonical raw observation JSON and SHA-256 bound evidence record in `evidence/records/FR-008.json`.

### Phase 2: CI Governance Gate Hardening
- Implement `scripts/verify_spec_artifacts.py` validating that all `specs/` directories contain valid `spec.md`, `plan.md`, and `tasks.md`.
- Replace the fragile branch-name condition (`if: startsWith(github.head_ref, 'specs/') || ...`) in `.github/workflows/pipeline.yml` so that `Verify Spec Artifacts` runs unconditionally on all pull requests and pushes to `main`.
- Add negative tests verifying that `verify_spec_artifacts.py` fails on missing plan/spec/tasks or broken file references.

### Phase 3: Audit Documentation & Provenance
- Create `docs/audit/REMEDIATION_REPORT_015.md`
- Create `docs/audit/FR008_LIVE_RUNTIME_EVIDENCE.md`
- Create `docs/audit/GOVERNANCE_GATES_VERIFICATION.md`
- Create `docs/audit/DEPLOYMENT_PROVENANCE.md`
