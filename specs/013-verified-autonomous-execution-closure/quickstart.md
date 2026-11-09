# Quickstart & Verification Guide: DG-RVP-013

## 1. Prerequisites
- Access to local MicroK8s cluster on `server.internal.lgcorzo`.
- Clean workspace on branch `013-verified-autonomous-execution-closure`.
- Tools: `cargo`, `kubectl`, `flux`, `gh`, `pytest`, `python3`.

## 2. Policy Enforcement Verification (Unit & Component)
```bash
cargo test -p factory-core --lib security_policy -- --nocapture -j 1
cargo test -p factory-mcp-server --test launch_sandbox_pod_tests -- --nocapture -j 1
```
Expected: Fail-closed on missing configs, refusal of host fallback for untrusted workloads, gVisor required across all non-trusted tests.

## 3. MicroK8s gVisor Runtime Repair & Live Test
1. Verify containerd configuration:
   ```bash
   grep "runtime_type" /var/snap/microk8s/current/args/containerd.toml
   ```
   Must display: `runtime_type = "io.containerd.runsc.v1"` under `runtimes.runsc`.

2. Execute disposable gVisor Pod:
   ```bash
   kubectl run test-gvisor-probe --image=docker.io/library/rust@sha256:9f841bbe9e7d8e37ceb96ed907265a3a0df7f44e3737d0b100e7907a679acb36 \
     --restart=Never --overrides='{"spec":{"runtimeClassName":"gvisor"}}' -n factory-sandboxes -- rustc --version
   kubectl wait --for=condition=ready pod/test-gvisor-probe -n factory-sandboxes --timeout=60s
   kubectl logs test-gvisor-probe -n factory-sandboxes
   kubectl delete pod test-gvisor-probe -n factory-sandboxes
   ```

## 4. FinOps Mutual Exclusion Concurrency Test
```bash
cargo test -p factory-infrastructure --test finops_concurrency_test -- --nocapture -j 1
```
Expected: 10 concurrent threads attempt to reserve budget past the limit; all requests after limit depletion are rejected without race conditions.

## 5. Evidence & Claims Consistency
```bash
python3 scripts/validate_evidence.py
python3 /mnt/F024B17C24B145FE/Repos/lgcorzo.github.io/scripts/check_public_claims.py
```
Expected: 100% resolution, all claims match evidence records.

