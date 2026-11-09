# GitOps Manifest vs. Live Cluster Runtime Verification Report

**Feature**: `013-verified-autonomous-execution-closure` (updating `011` / `012`)  
**User Story**: User Story 4 — Distinguish declared controls from enforced controls (`FR-015`, `FR-016`, `SC-004`)  
**Collected At (UTC)**: `2026-11-09T21:12:30Z`  
**Environment**: `server.internal.lgcorzo` (MicroK8s `v1.31.14`, Kustomize `v5.4.2`, Ubuntu 24.04.5 LTS, kernel `6.17.0-14-generic`)

---

## 1. Exact Inspected Source & Deployment Revisions (`T026`, `T027`, `013-T003`, `013-T030`)

| Layer | Repository / Resource | Inspected Branch / Ref | Full Commit SHA / Revision | Status |
|---|---|---|---|---|
| **Application Source (Deployed Image)** | `lgcorzo/rust_CACD_autonomous_factory` | `main` (`v0.3.3`) | `7ab32fa66b770761d559a55eb4ed01b3752eac16` (audit docs & evidence finalized on `main` via [PR #755](https://github.com/lgcorzo/rust_CACD_autonomous_factory/pull/755)–[PR #757](https://github.com/lgcorzo/rust_CACD_autonomous_factory/pulls)) | Inspected, Released & Verified |
| **GitOps Source (PR #202)** | `lgcorzo/gitops_internal_lgcorzo` | `main` ([PR #202](https://github.com/lgcorzo/gitops_internal_lgcorzo/pull/202)) | `2716e548fa5bd7563ebfce232ee68a6a24bb11b2` | Merged & Reconciled |
| **Flux Reconciled GitRepository** | `flux-system/gitops-internal` | `main` | `2716e548fa5bd7563ebfce232ee68a6a24bb11b2` | `Ready=True` (`stored artifact for revision 'main@sha1:2716e548fa5bd7563ebfce232ee68a6a24bb11b2'`) |
| **Flux Reconciled Kustomizations** | `flux-system/infrastructure`, `infrastructure-apps`, `security` | `main` | `2716e548fa5bd7563ebfce232ee68a6a24bb11b2` | `Ready=True` (`Applied revision: main@sha1:2716e548fa5bd7563ebfce232ee68a6a24bb11b2`) |
| **Supporting Flux Kustomizations** | `flux-system/rust-agent-team` (`fastapi-autogen-team`) | `main` | `07a14f64e58b8498a38d0fa9866468eb5a24a459` | `Ready=True` |

---

## 2. Comparison of Dynamically Generated Job (`launch_sandbox_pod.rs`) vs. GitOps Manifest (`sandbox-job.yaml`) (`T026`)

Validated via `bash ../gitops_internal_lgcorzo/tests/test_sandbox_job_policy.sh` and `cargo test -p factory-mcp-server --test launch_sandbox_pod_tests`:

| Control Field | `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs` | `gitops_internal_lgcorzo/infrastructure/agents/sandbox-job.yaml` | Alignment Status |
|---|---|---|---|
| `metadata.namespace` | `factory-sandboxes` | `factory-sandboxes` | **Aligned** |
| `spec.ttlSecondsAfterFinished` | `300` | `300` | **Aligned** |
| `spec.activeDeadlineSeconds` | `60` | `60` | **Aligned** |
| `spec.backoffLimit` | `0` | `0` | **Aligned** |
| `template.spec.runtimeClassName` | `gvisor` (mandatory via `ExecutionSecurityPolicy`; fails closed if false or missing on cluster) | `gvisor` | **Aligned** |
| `template.spec.automountServiceAccountToken` | `false` | `false` | **Aligned** |
| `template.spec.securityContext` | `runAsNonRoot: true`, `runAsUser: 65534`, `runAsGroup: 65534`, `seccompProfile: RuntimeDefault` | `runAsNonRoot: true`, `runAsUser: 65534`, `runAsGroup: 65534`, `seccompProfile: RuntimeDefault` | **Aligned** |
| `containers[0].image` | Immutable `@sha256` digest (`rust@sha256:9f841bbe...` / `python@sha256:5e2bdb1b...`) | Immutable `@sha256` digest | **Aligned** |
| `containers[0].securityContext` | `allowPrivilegeEscalation: false`, `readOnlyRootFilesystem: true`, `capabilities.drop: ["ALL"]` | `allowPrivilegeEscalation: false`, `readOnlyRootFilesystem: true`, `capabilities.drop: ["ALL"]` | **Aligned** |
| `volumes` / `volumeMounts` | `sandbox-tmp` (`emptyDir.sizeLimit: 128Mi`) mounted at `/tmp` | `sandbox-tmp` (`emptyDir.sizeLimit: 128Mi`) mounted at `/tmp` | **Aligned** |
| `resources.limits` | `memory: 512Mi`, `cpu: 500m`, `ephemeral-storage: 200Mi` | `memory: 512Mi`, `cpu: 500m`, `ephemeral-storage: 200Mi` | **Aligned** |
| `resources.requests` | `memory: 64Mi`, `cpu: 50m`, `ephemeral-storage: 50Mi` | `memory: 64Mi`, `cpu: 50m`, `ephemeral-storage: 50Mi` | **Aligned** |

---

## 3. Observed Live Cluster State & Runtime Verification (`T027`, `T028`, `013-T013`, `013-T014`, `013-T029`)

### 3.1 Observed Cluster Resources (`2716e548fa5bd7563ebfce232ee68a6a24bb11b2`)
1. **`RuntimeClass` (`kubectl get runtimeclass`)**:
   - `gvisor` -> `HANDLER: runsc` (`/usr/bin/runsc` version `release-20261019.0`, OCI spec `1.2.1`, `/usr/bin/containerd-shim-runsc-v1` configured with `runtime_type = "io.containerd.runsc.v1"`).
2. **Namespace `factory-sandboxes` (`kubectl get ns factory-sandboxes -o yaml`)**:
   - Active (`uid: 36b0dc43-9ed2-4bb1-9591-14e9a540c6f2`), reconciled by `flux-system/infrastructure`.
   - Labels enforced:
     - `pod-security.kubernetes.io/enforce: restricted` (`enforce-version: latest`)
     - `pod-security.kubernetes.io/audit: restricted`
     - `pod-security.kubernetes.io/warn: restricted`
3. **NetworkPolicy (`kubectl get networkpolicy -n factory-sandboxes`)**:
   - `sandbox-default-deny-all` active on `podSelector: app.kubernetes.io/component=sandbox-workload` (`Ingress` empty deny-all; `Egress` restricts RFC1918 `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, and link-local metadata `169.254.169.254/32`).
4. **Namespace-Scoped RBAC (`kubectl get role,rolebinding -n factory-sandboxes`)**:
   - `role.rbac.authorization.k8s.io/factory-sandbox-job-launcher` and `rolebinding.rbac.authorization.k8s.io/factory-worker-sandbox-job-binding` active in `factory-sandboxes`.

### 3.2 Live Admission, Negative Isolation & End-to-End Mission Checks (`RUNTIME_VERIFIED`)

1. **Negative Check — PodSecurity Standard `restricted` Enforcement (`RUNTIME_VERIFIED`)**:
   - Submitted pod `audit-negative-psa-check` with `runAsNonRoot: false`, `runAsUser: 0` in `factory-sandboxes`.
   - **Observed Result**: Immediately rejected by the Kubernetes API server admission controller:
     ```text
     Error from server (Forbidden): error when creating "STDIN": pods "audit-negative-psa-check" is forbidden: violates PodSecurity "restricted:latest": allowPrivilegeEscalation != false (container "root-container" must set securityContext.allowPrivilegeEscalation=false), unrestricted capabilities (container "root-container" must set securityContext.capabilities.drop=["ALL"]), runAsNonRoot != true (pod must not set securityContext.runAsNonRoot=false), runAsUser=0 (pod must not set runAsUser=0), seccompProfile (pod or container "root-container" must set securityContext.seccompProfile.type to "RuntimeDefault" or "Localhost")
     ```
2. **Disposable gVisor Rust Probe (`job/audit-verify-gvisor-rust-013` / `pod/audit-verify-gvisor-rust-013-mjv54` — `RUNTIME_VERIFIED`)**:
   - Job UID: `931b7fd4-f100-4e50-b917-5a9e621f8e31`, Pod UID: `4631d53c-095d-4699-a3dc-8c06de3b2f02`
   - Container ID: `containerd://6d28b823ae7dea3163c4138fc23018cbd56352b21113801000fa25a2fa991ffd`
   - Completed in `2s` (`21:07:17Z` -> `21:07:19Z`, `exitCode: 0`, `phase: Succeeded`), printing `[DG-RVP-013] gVisor Rust Execution Verified OK`.
3. **In-Pod Negative Isolation Probe (`job/audit-verify-gvisor-negative-013` / `pod/audit-verify-gvisor-negative-013-4pkxz` — `RUNTIME_VERIFIED`)**:
   - Job UID: `79f78952-1bbf-4a2e-8b6c-aeef82e42cbe`, Pod UID: `36d4ae1e-20c3-4e11-89c4-4c0a88dd5e0f`
   - Container ID: `containerd://e6687423f379ad00474303663fb080a590c597c316e05b83064c760782dddc61`
   - Completed in `3s` (`exitCode: 0`, `phase: Succeeded`), confirming gVisor kernel `dmesg` (`Starting gVisor...`), `sa_token_exists: false`, `ro_rootfs: BLOCKED: PermissionError`, `etc_shadow: BLOCKED: PermissionError`, and `imds_egress: BLOCKED: TimeoutError`.
4. **Live End-to-End Autonomous Mission (`real-mission-e7-001-1794258497` / Hatchet Run `e32a4202-2a30-44a4-b1dd-ef23f620e481` — `RUNTIME_VERIFIED`)**:
   - All 5 Hatchet workflow steps (`rustant-plan`, `zeroclaw-execute`, `zeroclaw-validate`, `rustant-review`, `factory-deliver`) completed with `status: success`.
   - `zeroclaw-validate` ran `cargo test` inside gVisor Pod `sandbox-job-a68ab421-mmzwz` (`uid: af7316c2-e003-4110-b9f3-1117992cf4e3`, `container_id: containerd://eeaacc796e7b041a26a4303a40eef3eb390c7381b9e6839ebde9ad445b4917bb`, `exit_code: 0`, `command_hash: c528d5d4e06bf7ed7bc66aea454a70a39a60fadb9592334ffd13ea5d9acb290a`).
