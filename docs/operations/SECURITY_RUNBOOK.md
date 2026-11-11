# Security Runbook: Deployment, Incident Response & Recovery

**Specification**: `010-integrated-k8s-gitops-audit`  
**Date**: `2026-11-02`

> **Namespace note:** `deploy/` contains no `Namespace` objects. Admission bindings select `development`; sandbox configuration refers to `factory-sandbox` and the `openziti` namespace label. The live sandbox namespace is **TO CONFIRM**. Set `SANDBOX_NAMESPACE` only after confirming the target cluster.

---

## 1. Deployment & Verification Procedures

### 1.1 Verifying gVisor on the Cluster
Before launching sandbox workloads, verify that the `gvisor` runtime class exists and points to `runsc`:
```bash
kubectl get runtimeclass gvisor -o yaml
```
Expected output:
```yaml
apiVersion: node.k8s.io/v1
kind: RuntimeClass
metadata:
  name: gvisor
handler: runsc
```

### 1.2 Verifying Sandbox Pod Security Profile
Check restricted pod security on the namespace confirmed for the target cluster:
```bash
kubectl get namespace "$SANDBOX_NAMESPACE" -o jsonpath='{.metadata.labels}'
```
Expected labels for this check (live values are **TO CONFIRM**):
- `pod-security.kubernetes.io/enforce: restricted`
- `pod-security.kubernetes.io/audit: restricted`
- `pod-security.kubernetes.io/warn: restricted`

---

## 2. Incident Response Playbooks

### Playbook 1: Unisolated Workload / Missing gVisor Runtime
- **Symptom**: Sandbox jobs fail to schedule with `RuntimeClass "gvisor" not found` or `SubprocessDriver` logs production refusal.
- **Action**:
  1. Do NOT enable host fallback.
  2. Verify that MicroK8s has the container runtime configured: `microk8s status` and check containerd config `/var/snap/microk8s/current/args/containerd-template.toml`.
  3. Ensure `runsc` binary is installed and executable in `/snap/microk8s/current/bin/runsc` or `/usr/local/bin/runsc`.

### Playbook 2: FinOps Budget Breach & Mission Lock
- **Symptom**: Missions fail admission with `FinOps budget hard-stop latch is active`.
- **Action**:
  1. Inspect LiteLLM spend: `curl -H "Authorization: Bearer $LITELLM_KEY" http://litellm:4000/spend/logs`.
  2. Identify the offending agent or mission ID.
  3. Reset the latch only after executive cost approval: restart `FinOpsAgent` or update `FINOPS_MAX_DAILY_BUDGET`.

### Playbook 3: Secret Leakage or Compromised Token
- **Symptom**: A Vault role token or git credential appears in logs.
- **Action**:
  1. Immediately revoke the token in Vault:
     ```bash
     vault token revoke <TOKEN_ACCESSOR>
     ```
  2. Review Git history with Gitleaks:
     ```bash
     gitleaks git --verbose
     ```
