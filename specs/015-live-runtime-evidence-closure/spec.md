# Specification — DG-RVP-015: Live Runtime Evidence & Governance Closure

**Program:** DG-RVP-015  
**Priority:** P0 / P1  
**Target Repository:** `lgcorzo/rust_CACD_autonomous_factory`  
**Related Repositories:** `lgcorzo/gitops_internal_lgcorzo`, `lgcorzo/lgcorzo.github.io`  
**Baseline Commit:** `0a1918081ac6307051aee73ffadddb372e032db8`  
**Release Tag:** `v0.3.4`  

## 1. Objectives

1. **FR-008 Live gVisor Runtime Evidence**:
   - Collect and corroborate independent live runtime observations proving that the deployed factory image (`docker.io/lgcorzo/dark-gravity-factory:0a1918081ac6307051aee73ffadddb372e032db8`) runs under the gVisor (`runsc`) sandbox in the MicroK8s Kubernetes cluster.
   - Correlate:
     - CI Docker build digest: `sha256:331cd9f24e23c8b280fd1ed643e2f6d6ff773211b6429a31baac48052bf60b3b`
     - GitOps deployment commit: `4e6c01c7ce30483a4dfc8dd534c3adcb8b5c4fd7`
     - Live Kubernetes Pod UID, container ID, and image digest
     - Node runtime configuration and containerd daemon logs proving `runsc` execution
     - Negative security isolation probes (dmesg gVisor boot log, fake kernel version `4.19.0-gvisor`, read-only rootfs enforcement).

2. **Governance Gates Verification (`Verify Spec Artifacts`)**:
   - Resolve why `Verify Spec Artifacts` was skipped on non-`specs/` branches in GitHub Actions.
   - Implement `scripts/verify_spec_artifacts.py` to validate `spec.md`, `plan.md`, `tasks.md`, and referenced evidence records across all specification packages on both push and pull request events.

3. **Deployment Provenance & Traceability**:
   - Document complete provenance: `BUILT → PUBLISHED → DEPLOYED → RUNTIME_VERIFIED`.

