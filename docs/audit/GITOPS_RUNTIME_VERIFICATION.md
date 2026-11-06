# GitOps Manifest vs. Live Cluster Runtime Verification Report

**Feature**: `011-verified-autonomous-execution`  
**User Story**: User Story 4 — Distinguish declared controls from enforced controls (`FR-015`, `FR-016`, `SC-004`)  
**Collected At (UTC)**: `2026-11-06T17:45:00Z`  
**Environment**: `server.internal.lgcorzo` (MicroK8s `v1.31.14`, Kustomize `v5.4.2`, Ubuntu 24.04.5 LTS, kernel `6.17.0-14-generic`)

---

## 1. Exact Inspected Source & Deployment Revisions (`T026`, `T027`)

| Layer | Repository / Resource | Inspected Branch / Ref | Full Commit SHA / Revision | Status |
|---|---|---|---|---|
| **Application Source** | `lgcorzo/rust_CACD_autonomous_factory` | `011-verified-autonomous-execution` | `0d96c8144d46535ce3f9405931dc29e1a03dcc83` | Inspected |
| **GitOps Source (PR #175)** | `lgcorzo/gitops_internal_lgcorzo` | `011-sandbox-job-policy-alignment` ([PR #175](https://github.com/lgcorzo/gitops_internal_lgcorzo/pull/175)) | `8f655e5eca9f0b22c6f2c8b7f3d259e0d888eb1b` | Declared & Tested (`test_sandbox_job_policy.sh`) |
| **Flux Reconciled GitRepository** | `flux-system/gitops-internal` | `main` | `48b9c0e333ce541fc741c056bd67e2932fdc50e9` | `Ready=True` (`stored artifact for revision 'main@sha1:48b9c0e333ce541fc741c056bd67e2932fdc50e9'`) |
| **Flux Reconciled Kustomizations** | `flux-system/infrastructure`, `infrastructure-apps`, `security` | `main` | `48b9c0e333ce541fc741c056bd67e2932fdc50e9` | `Ready=True` (`Applied revision: main@sha1:48b9c0e333ce541fc741c056bd67e2932fdc50e9`) |
| **Supporting Flux Kustomizations** | `flux-system/rust-agent-team` (`fastapi-autogen-team`) | `main` | `6b9c4513eea630a04255a68412f2042cd61dbea1` | `Ready=True` |

> **Revision Delta Note (`FR-016`)**: The live cluster is currently reconciled to `gitops_internal_lgcorzo@48b9c0e333ce541fc741c056bd67e2932fdc50e9` (`main`). The sandbox-job template alignment in `8f655e5eca9f0b22c6f2c8b7f3d259e0d888eb1b` is open in [PR #175](https://github.com/lgcorzo/gitops_internal_lgcorzo/pull/175) awaiting human review and merge per the HITL policy.

---

## 2. Comparison of Dynamically Generated Job (`launch_sandbox_pod.rs`) vs. GitOps Manifest (`sandbox-job.yaml`) (`T026`)

Validated via `bash ../gitops_internal_lgcorzo/tests/test_sandbox_job_policy.sh` and `cargo test -p factory-mcp-server --test launch_sandbox_pod_tests`:

| Control Field | `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs` (`0d96c814`) | `gitops_internal_lgcorzo/infrastructure/agents/sandbox-job.yaml` (`8f655e5e`) | Alignment Status |
|---|---|---|---|
| `metadata.namespace` | `factory-sandboxes` | `factory-sandboxes` | **Aligned** |
| `spec.ttlSecondsAfterFinished` | `300` | `300` | **Aligned** |
| `spec.activeDeadlineSeconds` | `60` | `60` | **Aligned** (Added in `T010` / `T025`) |
| `spec.backoffLimit` | `0` | `0` | **Aligned** |
| `template.spec.runtimeClassName` | `gvisor` (mandatory; fails closed if false or missing on cluster) | `gvisor` | **Aligned** |
| `template.spec.automountServiceAccountToken` | `false` | `false` | **Aligned** |
| `template.spec.securityContext` | `runAsNonRoot: true`, `runAsUser: 65534`, `runAsGroup: 65534`, `seccompProfile: RuntimeDefault` | `runAsNonRoot: true`, `runAsUser: 65534`, `runAsGroup: 65534`, `seccompProfile: RuntimeDefault` | **Aligned** |
| `containers[0].image` | `python:3.11.11-slim` | `python:3.11.11-slim` | **Aligned** |
| `containers[0].securityContext` | `allowPrivilegeEscalation: false`, `readOnlyRootFilesystem: true`, `capabilities.drop: ["ALL"]` | `allowPrivilegeEscalation: false`, `readOnlyRootFilesystem: true`, `capabilities.drop: ["ALL"]` | **Aligned** |
| `volumes` / `volumeMounts` | `sandbox-tmp` (`emptyDir.sizeLimit: 32Mi`) mounted at `/tmp` | `sandbox-tmp` (`emptyDir.sizeLimit: 32Mi`) mounted at `/tmp` | **Aligned** |
| `resources.limits` | `memory: 30Mi`, `cpu: 250m`, `ephemeral-storage: 100Mi` | `memory: 30Mi`, `cpu: 250m`, `ephemeral-storage: 100Mi` | **Aligned** |
| `resources.requests` | `memory: 16Mi`, `cpu: 50m`, `ephemeral-storage: 50Mi` | `memory: 16Mi`, `cpu: 50m`, `ephemeral-storage: 50Mi` | **Aligned** |

---

## 3. Observed Live Cluster State & Runtime Verification (`T027`, `T028`)

### 3.1 Observed Cluster Resources (`48b9c0e333ce541fc741c056bd67e2932fdc50e9`)
1. **`RuntimeClass` (`kubectl get runtimeclass`)**:
   - `gvisor` -> `HANDLER: runsc` (Age: `6d5h`, `/usr/bin/runsc` version `release-20261019.0`, OCI spec `1.2.1`).
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

### 3.2 Live Admission & Runtime Execution Checks (`T028`)

1. **Negative Check — PodSecurity Standard `restricted` Enforcement (`RUNTIME_VERIFIED`)**:
   - Submitted pod `audit-negative-psa-check` with `runAsNonRoot: false`, `runAsUser: 0` in `factory-sandboxes`.
   - **Observed Result**: Immediately rejected by the Kubernetes API server admission controller:
     ```text
     Error from server (Forbidden): error when creating "STDIN": pods "audit-negative-psa-check" is forbidden: violates PodSecurity "restricted:latest": allowPrivilegeEscalation != false (container "root-container" must set securityContext.allowPrivilegeEscalation=false), unrestricted capabilities (container "root-container" must set securityContext.capabilities.drop=["ALL"]), runAsNonRoot != true (pod must not set securityContext.runAsNonRoot=false), runAsUser=0 (pod must not set runAsUser=0), seccompProfile (pod or container "root-container" must set securityContext.seccompProfile.type to "RuntimeDefault" or "Localhost")
     ```
2. **Disposable gVisor Workload Execution (`job/audit-verify-gvisor-controls` & `job/audit-verify-gvisor-fast`)**:
   - **Admission & Scheduling (`ENFORCED`)**: Admitted into `factory-sandboxes`, scheduled onto `server.internal.lgcorzo`, and launched via `runsc-sandbox` (`PID 430990` / `434139`) and `runsc-gofer` (`PID 430984` / `434133`).
   - **CRITICAL LIVE RUNTIME FINDING (`AUD-010` — Host Containerd CRI Configuration Defect)**:
     - Inspecting `/var/snap/microk8s/current/args/containerd-template.toml` revealed:
       ```toml
       [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.runsc]
         runtime_type = "io.containerd.runc.v2"
         [plugins."io.containerd.grpc.v1.cri".containerd.runtimes.runsc.options]
           BinaryName = "/usr/bin/runsc"
       ```
     - Because MicroK8s containerd was configured with `runtime_type = "io.containerd.runc.v2"` + `BinaryName = "/usr/bin/runsc"` instead of the dedicated gVisor shim `runtime_type = "io.containerd.runsc.v1"` (`/usr/bin/containerd-shim-runsc-v1`), `runsc-sandbox` boots the container (`Status: Running`) but does not signal container exit back to `containerd-shim-runc-v2`, causing every `runtimeClassName: gvisor` Job to hang in `Running` until `activeDeadlineSeconds` (`60s`) expires and failing to flush stdout logs to Kubelet.
     - **Impact on Lifecycle Classification (`FR-015`, `SC-004`)**:
       - `PodSecurity: restricted` admission is **Reconciled**, **Enforced**, and **Runtime-Verified**.
       - `gVisor` (`runsc`) workload completion and in-pod negative network egress checks are **Declared** and **Reconciled**, and `runsc-sandbox` process creation is **Observed**, but end-to-end gVisor Job completion is **NOT `runtime_verified`** (Classified as `FAIL / PARTIAL` due to host containerd `runtime_type` misconfiguration on `server.internal.lgcorzo`).
       - Per `FR-004` and `SC-001`, `GvisorK8sDriver` and `LaunchSandboxPodTool` **fail closed** (`is_error: true`) when the Job times out rather than falling back to host execution or simulating success.
