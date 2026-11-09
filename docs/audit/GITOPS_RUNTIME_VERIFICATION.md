# GitOps Manifest vs. Live Cluster Runtime Verification Report

**Feature**: `013-verified-autonomous-execution-closure` (updating `011` / `012`)  
**User Story**: User Story 4 — Distinguish declared controls from enforced controls (`FR-015`, `FR-016`, `SC-004`)  
**Collected At (UTC)**: `2026-11-09T19:00:00Z`  
**Environment**: `server.internal.lgcorzo` (MicroK8s `v1.31.14`, Kustomize `v5.4.2`, Ubuntu 24.04.5 LTS, kernel `6.17.0-14-generic`)

---

## 1. Exact Inspected Source & Deployment Revisions (`T026`, `T027`, `013-T003`, `013-T030`)

| Layer | Repository / Resource | Inspected Branch / Ref | Full Commit SHA / Revision | Status |
|---|---|---|---|---|
| **Application Source** | `lgcorzo/rust_CACD_autonomous_factory` | `main` (`v0.3.2`) | `0d8e4176f334751aed653e700e4be75f61b383a5` | Inspected & Verified |
| **GitOps Source (PR #200, #201)** | `lgcorzo/gitops_internal_lgcorzo` | `main` ([PR #200](https://github.com/lgcorzo/gitops_internal_lgcorzo/pull/200), [PR #201](https://github.com/lgcorzo/gitops_internal_lgcorzo/pull/201)) | `aed98f1207dc3dea2352c346a1b8db8ae6a58258` | Merged & Reconciled |
| **Flux Reconciled GitRepository** | `flux-system/gitops-internal` | `main` | `aed98f1207dc3dea2352c346a1b8db8ae6a58258` | `Ready=True` (`stored artifact for revision 'main@sha1:aed98f1207dc3dea2352c346a1b8db8ae6a58258'`) |
| **Flux Reconciled Kustomizations** | `flux-system/infrastructure`, `infrastructure-apps`, `security` | `main` | `aed98f1207dc3dea2352c346a1b8db8ae6a58258` | `Ready=True` (`Applied revision: main@sha1:aed98f1207dc3dea2352c346a1b8db8ae6a58258`) |
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
2. **Live Mission & Disposable gVisor Workload Execution (`job/sandbox-job-818ed351` & `pod/sandbox-job-818ed351-fksrx`)**:
   - **Admission & Scheduling (`ENFORCED` & `OBSERVED` in `013`)**: During live mission `real-mission-e7-001-1794250359` (Hatchet run `c08fedda-e484-4246-9903-98e87545f99e`), `factory-mcp-server` created `job.batch/sandbox-job-818ed351` in `factory-sandboxes`, which spawned `pod/sandbox-job-818ed351-fksrx` (`uid: b89380f3-532e-4db4-9c4c-f97bee0ab656`, `runtimeClassName: gvisor`, `containerID: containerd://e4c4f265420310891ae2989f02b2221f4218c778f2d3880f81fd108a1e5450bb`, image `docker.io/library/rust@sha256:9f841bbe9e7d8e37ceb96ed907265a3a0df7f44e3737d0b100e7907a679acb36`, host processes `runsc-gofer` `PID 669620` and `runsc-sandbox` `PID 669625`).
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
       - `gVisor` (`runsc`) workload completion and in-pod negative network egress checks are **Declared** and **Reconciled**, and `runsc-sandbox` process creation is **Observed**, but end-to-end gVisor Job completion is **NOT `runtime_verified`** (Classified as `FAIL / PARTIAL` due to host containerd `runtime_type` misconfiguration on `server.internal.lgcorzo`, awaiting privileged `sudo snap restart microk8s.daemon-containerd`).
       - Per `FR-004` and `SC-001`, `GvisorK8sDriver` and `LaunchSandboxPodTool` **fail closed** (`is_error: true`) when the Job times out rather than falling back to host execution or simulating success.

