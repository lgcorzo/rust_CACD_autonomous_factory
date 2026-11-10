# FR-008 Live Runtime Evidence & Container Isolation Report

**Programme:** DG-RVP-015  
**Requirement ID:** FR-008  
**Verification Class:** `live_runtime`  
**Target Node:** `server.internal.lgcorzo` (MicroK8s v1.31.14)  
**Container Runtime:** `containerd://1.6.28`  
**Runtime Handler:** `/usr/bin/runsc` (`io.containerd.runsc.v1`)  
**Timestamp:** `2026-11-10T07:15:30Z`  
**Verifier Identity:** `dg-rvp-015-live-verifier` (`independent_verifier`)

---

## 1. Executive Summary

This report documents the independent verification of capability **FR-008** (Sandboxed Execution under gVisor). Unlike unit test runners or simulated sandboxes, this verification was conducted directly against a live running Kubernetes cluster (`server.internal.lgcorzo`) executing the official released container image for Dark Gravity Factory (`v0.3.4`).

The verification strictly differentiates between **requested** configuration and **independently observed** node telemetry.

---

## 2. Cryptographic Image & Provenance Traceability

| Artifact | Identifier / Value | Verification Source | Match Status |
| :--- | :--- | :--- | :--- |
| **Source Commit** | `0a1918081ac6307051aee73ffadddb372e032db8` | Git `rust_CACD_autonomous_factory` | Baseline Validated |
| **Release Tag** | `v0.3.4` | GitHub Releases | Published |
| **Published OCI Digest** | `sha256:331cd9f24e23c8b280fd1ed643e2f6d6ff773211b6429a31baac48052bf60b3b` | Docker Hub / GitHub CI #42770919650 | Cryptographically Verified |
| **Deployed GitOps Commit** | `4e6c01c7ce30483a4dfc8dd534c3adcb8b5c4fd7` | Flux Kustomization (`gitops_internal_lgcorzo`) | Reconciled |
| **Node Container Image ID** | `docker.io/lgcorzo/dark-gravity-factory@sha256:331cd9f24e23c8b280fd1ed643e2f6d6ff773211b6429a31baac48052bf60b3b` | Kubernetes Pod Status (`containerStatuses[0].imageID`) | **EXACT MATCH** |

---

## 3. Separation of Requested vs Observed Attributes

### 3.1 Requested Specification
- `runtimeClassName`: `gvisor`
- `image`: `docker.io/lgcorzo/dark-gravity-factory:0a1918081ac6307051aee73ffadddb372e032db8`
- `securityContext.readOnlyRootFilesystem`: `true`
- `securityContext.allowPrivilegeEscalation`: `false`
- `securityContext.runAsNonRoot`: `true`
- `securityContext.capabilities.drop`: `["ALL"]`

### 3.2 Independently Observed Facts
- **Pod UID**: `81d2f4fc-c048-41fc-abde-eca6f93fa4da` (`test-factory-image-probe`)
- **Container ID**: `containerd://d2fbc0ea5b3ecbb56cff3d96fe0b866e848654786742d57a44d8277a76a8b8d9`
- **Host Node Kernel**: `Linux server.internal.lgcorzo 6.17.0-14-generic`
- **Container Observed Kernel (`uname -a`)**: `Linux test-factory-image-probe 4.19.0-gvisor #1 SMP Sun Jan 10 15:06:54 PST 2016 x86_64 Linux`
- **Runtime Handler**: Containerd log stream records `/usr/bin/runsc` process lifecycle for the container sandbox.
- **Root Filesystem Enforcement**: Probed via write attempt (`touch /test_file`) returning exit code 1 (`touch: /test_file: Permission denied`).

---

## 4. Negative Probes & Control Verification

1. **Unrestricted Pod Admission**: An unhardened pod specification submitted without `allowPrivilegeEscalation: false` and `runAsNonRoot: true` was immediately blocked by Kubernetes Admission with `violates PodSecurity "restricted:latest"`.
2. **Missing RuntimeClass**: A pod requesting an unconfigured runtime class was rejected with `RuntimeClass "non-existent-runtime" not found`.
3. **Non-gVisor Pod Execution**: A control pod running without `runtimeClassName: gvisor` returned the host kernel `6.17.0-14-generic`, confirming that the `4.19.0-gvisor` kernel is exclusively provided when gVisor sandboxing is enabled.

---

## 5. Evidence Artifacts

- **Run Log**: `evidence/runs/FR-008.log` (SHA-256: `fa9ab2df1c28e18100cab37a5a19de1b60357515214173017039d53a918593d2`)
- **Telemetry JSON**: `evidence/security/live-gvisor-isolation-015.json` (SHA-256: `a5a2a7fb5d4a28a7533fc8a44629eac470de6d80a31d6f89fff41aacf7cdfb6d`)
- **Record**: `evidence/records/FR-008.json`
- **Verification State**: `evidence/states/FR-008.json` (`VERIFIED`)
