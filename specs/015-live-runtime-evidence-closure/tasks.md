# Tasks: DG-RVP-015 Live Runtime Evidence & Governance Closure

## Phase 1: Live gVisor Evidence & Corroboration (P0)
- [x] T001 Inspect MicroK8s node `server.internal.lgcorzo` containerd runtime template and verify `runsc` configuration
- [x] T002 Execute live gVisor pod probe in `factory-sandboxes` with deployed factory image `0a191808`
- [x] T003 Capture independent containerd daemon logs correlating container ID with `/usr/bin/runsc`
- [x] T004 Execute negative security probes: unhardened pod rejection, invalid runtime class rejection, root filesystem write denial
- [x] T005 Correlate OCI image digest `sha256:331cd9f24e23c8b280fd1ed643e2f6d6ff773211b6429a31baac48052bf60b3b` between Docker Publish and deployed Kubernetes pod
- [ ] T006 Write canonical observation artifact and generate cryptographic evidence record for FR-008

## Phase 2: CI Governance Gates Fix (P0)
- [x] T007 Implement `scripts/verify_spec_artifacts.py` validating SDD artifact consistency
- [ ] T008 Update `.github/workflows/pipeline.yml` to execute `Verify Spec Artifacts` unconditionally on all PRs and pushes to `main`
- [ ] T009 Add negative tests for `verify_spec_artifacts.py` proving rejection of broken or missing spec artifacts

## Phase 3: Traceability & Audit Deliverables (P1)
- [ ] T010 Create `docs/audit/REMEDIATION_REPORT_015.md`
- [ ] T011 Create `docs/audit/FR008_LIVE_RUNTIME_EVIDENCE.md`
- [ ] T012 Create `docs/audit/GOVERNANCE_GATES_VERIFICATION.md`
- [ ] T013 Create `docs/audit/DEPLOYMENT_PROVENANCE.md`
- [ ] T014 Run full test suite, format check, and clippy verification
- [ ] T015 Commit, push branch, open PR, and verify all GitHub Actions checks pass green

