# Quickstart & Verification Guide: Secure gVisor Execution & Evidence Governance

**Feature**: `012-secure-gvisor-execution`
**Date**: 2026-11-08

## 1. Prerequisites
- Running MicroK8s cluster with containerd `runsc` runtime class enabled.
- Docker daemon or image registry credentials for `docker.io/lgcorzo/dark-gravity-factory`.
- Active git branch `012-secure-gvisor-execution`.

## 2. Validation Scenarios

### Scenario A: Verify gVisor Runtime Class on Cluster
Confirm that the Kubernetes cluster recognizes the `gvisor` RuntimeClass:
```bash
kubectl get runtimeclass
# Expected: gvisor   io.containerd.runsc.v1
```

### Scenario B: Verify SubprocessDriver Rejection of Untrusted Code
Run unit/integration tests to ensure Subprocess execution fails closed for untrusted payloads:
```bash
cargo test -p factory-mcp-server --test sandbox_driver_tests test_subprocess_driver_rejects_untrusted_in_all_envs
```

### Scenario C: Verify Real Cargo Compilation in gVisor Sandbox Pod
Execute a live verification pod in Kubernetes under gVisor to compile and run Rust tests:
```bash
kubectl run gvisor-rust-verify --rm -i --restart=Never \
  --image=docker.io/lgcorzo/dark-gravity-factory:cc902edaa0e3561f1b789a29878927c2558c03ec \
  --overrides='{
    "spec": {
      "runtimeClassName": "gvisor",
      "containers": [{
        "name": "verify",
        "image": "docker.io/lgcorzo/dark-gravity-factory:cc902edaa0e3561f1b789a29878927c2558c03ec",
        "command": ["cargo", "test"],
        "resources": {
          "limits": { "memory": "512Mi", "cpu": "1000m" },
          "requests": { "memory": "64Mi", "cpu": "100m" }
        },
        "securityContext": {
          "runAsUser": 65534,
          "runAsGroup": 65534,
          "runAsNonRoot": true,
          "allowPrivilegeEscalation": false,
          "readOnlyRootFilesystem": true,
          "capabilities": { "drop": ["ALL"] }
        }
      }]
    }
  }'
```

### Scenario D: Multi-Process FinOps Atomic Locking
Verify atomic concurrency behavior across multiple threads/processes:
```bash
cargo test -p factory-application --test finops_admission_tests test_concurrent_multiprocess_admissions
```

### Scenario E: Evidence Gate Freshness Validation
Generate fresh evidence against HEAD and execute validator:
```bash
factory-cli validate-evidence --evidence-file docs/audit/VERIFICATION_EVIDENCE.json --target-commit $(git rev-parse HEAD) --strict-attestation
```
