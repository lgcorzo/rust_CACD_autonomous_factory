# Verification Matrix: DG-RVP-013: Verified Autonomous Execution Closure

| Req ID | Control / Requirement | Classification | Target Component / Repo | Verification Command | Expected Outcome |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **VR-001** | Unified Fail-Closed Security Policy | `UNIT` / `COMPONENT` | `crates/factory-core`, `crates/factory-mcp-server` | `cargo test -p factory-core --lib security_policy -j 1` | Rejection of host execution, `USE_GVISOR=false` blocked for untrusted code. |
| **VR-002** | MicroK8s containerd gVisor Shim | `LIVE_RUNTIME` | Host `/var/snap/microk8s/current/args/` | Inspect containerd config, run disposable pod with `runtimeClassName: gvisor` | Pod runs under `io.containerd.runsc.v1` and outputs compiler version. |
| **VR-003** | Observed Runtime Attestation | `INTEGRATION` | `crates/factory-mcp-server` | `cargo test -p factory-mcp-server --test launch_sandbox_pod_tests -j 1` | Attestation contains real Pod UID, container ID, image digest, non-null observed fields. |
| **VR-004** | FinOps Atomic Concurrency | `COMPONENT` | `crates/factory-infrastructure` | `cargo test -p factory-infrastructure --lib finops_stop_store -j 1` | Shared lockfile, zero over-admission under multi-thread contention. |
| **VR-005** | GitOps Drift & Policy Alignment | `KUBERNETES_ADMISSION` | `gitops_internal_lgcorzo` | `flux reconcile kustomization infrastructure` | Declarative manifests mirror runtime sandbox spec. |
| **VR-006** | Truthful Public Claims | `INTEGRATION` | `lgcorzo.github.io` | `pytest tests/test_public_claims.py` | 100% agreement between canonical claims and verified evidence. |
| **VR-007** | End-to-End Autonomous Mission | `LIVE_RUNTIME` | Cluster `agents` namespace | Trigger real mission via `trigger_mission` | All 5 stages (`rustant-plan` -> `zeroclaw-execute` -> `zeroclaw-validate` -> `rustant-review` -> `factory-deliver`) pass. |

