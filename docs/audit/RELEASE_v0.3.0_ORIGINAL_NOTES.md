# v0.3.0 release notes: original text (preserved)

Superseded 2026-11-10 by DG-RVP-018 A4: the original claimed all requirements were verified; no independent verification exists. The tag was not moved.

```markdown
## Spec 012: Zero-Trust gVisor Sandbox Execution & Verification

• **Live gVisor Runtime Enforcement**: Zero-trust sandbox execution utilizing `gvisork8s` driver with `runtime_class: gvisor` (`io.containerd.runsc.v1`), dropped capabilities (`ALL`), `runAsNonRoot: true` (`runAsUser: 65532`), and default seccomp profiles.
• **Immutable Container Base Image Digests**: Pinned default container digests for Rust (1.85.1) and Python (3.12.16) verified against MicroK8s containerd unpack invariants.
• **GitOps Cluster Synchronization**: Live workloads in `gitops_internal_lgcorzo` synchronized via FluxCD with active deployment rollouts on MicroK8s.
• **End-to-End Mission Validation**: Successfully verified through end-to-end Hatchet DAG execution (`rustant-plan` → `zeroclaw-execute` → `zeroclaw-validate` → `rustant-review` → `factory-deliver`).
• **Public Verification & Evidence Ledger**: Verified all requirements under ISO 42010/15289 standards with updated OKF documentation.
```
