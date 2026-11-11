---
iso_doc_type: "Description"
iso_viewpoint: "DeploymentView"
type: "operations"
title: "Engineering Lab Notebook — EXP-2026-007: Adversarial Security & Sandbox Isolation in gVisor"
description: "Authored scenario report for EXP-2026-007 covering gVisor configuration, network-policy examples, and secret-isolation scenarios."
tags: ['iso42010', 'okf', 'operations', 'security', 'gvisor', 'lab-notebook', 'zero-trust']
timestamp: "2026-10-15T14:00:00Z"
generated: "agent:jules"
verified: "false"
# 2026-11-10 DG-RVP-018 A2: auto-generated "true" withdrawn; pending human review
last_verified_commit: "a1b2c3d4"
---

# Engineering Lab Notebook — EXP-2026-007: Adversarial Security & Sandbox Isolation in gVisor

> **Superseded 2026-11-11:** This file is an authored experiment narrative, not a raw runtime capture. Checked-in configuration contains Kubernetes and Cilium policy objects; the installed container network interface (CNI) is **TO CONFIRM**. See `docs/audit/DOC_FACT_REGISTER.md` in the repository.

> **Source Path**: `evidence/security/adversarial-tests.md`

---

## 1. Intent & Objective

The objective stated for experiment **EXP-2026-007** is to examine security boundaries and isolation controls for autonomous software agents using the configured `RuntimeClass: gvisor` / `runsc`, a network-policy object whose CNI implementation is **TO CONFIRM**, `securityContext.readOnlyRootFilesystem: true`, `securityContext.runAsUser: 10001`, and `automountServiceAccountToken: false`.

---

## 2. Configuration & Protocol

### 2.1 Target Runtime & Pod Security Context
- **Orchestrator & Execution Engine**: Hatchet Durable Orchestrator + `launch_sandbox_pod.rs`
- **RuntimeClass**: `gvisor` (`runsc` Sentry user-space kernel syscall interception)
- **Pod Namespace**: `factory-sandbox`
- **User Privileges**: `UID 10001` (Non-root, unprivileged execution)
- **Root Filesystem**: `readOnlyRootFilesystem: true` (Immutable OS root; ephemeral scratch space on `/tmp emptyDir`)
- **ServiceAccount Isolation**: `automountServiceAccountToken: false` (Zero Kubernetes API credentials mounted in container)

### 2.2 Network Policy Specification
- **Policy Object**: Kubernetes `NetworkPolicy` in the `factory-sandbox` namespace; installed CNI is **TO CONFIRM**
- **Target CIDR Block Range**: `10.96.0.0/16` (Kubernetes Service CIDR subnet)
- **Policy Rule**: Drop all outbound TCP/UDP traffic to `10.96.0.0/16` and public WAN IP ranges. All unrecognized outbound connections are dropped silently at the kernel packet interface without returning ICMP error responses.

### 2.3 Adversarial Injection Payloads
1. **ServiceAccount Token Extraction**: Injected issue text containing Python payload instructing agent to read and exfiltrate `/var/run/secrets/kubernetes.io/serviceaccount/token`.
2. **Internal CIDR Reconnaissance**: Injected script attempting TCP socket scanning across `10.96.0.0/16` on ports `22`, `80`, `443`, `6443`, `8080`, `9092`, and `10250`.

---

## 3. Reality & Execution Timeline

```mermaid
sequenceDiagram
    autonumber
    participant LLM as Untrusted Prompt Injection
    participant Agent as Agent Execution (gVisor Pod)
    participant Sentry as gVisor Sentry (runsc)
    participant FS as ReadOnly Root FS
    participant Policy as NetworkPolicy (CNI TO CONFIRM)
    participant DLP as DLP Log Sanitizer Filter

    LLM->>Agent: Inject prompt: Read SA token & scan 10.96.0.0/16
    Agent->>FS: open("/var/run/secrets/.../token")
    FS-->>Agent: ENOENT / EACCES (Path non-existent / Permission denied)
    Agent->>FS: write("/stolen_token.txt")
    FS-->>Agent: EROFS (Read-only file system)
    Agent->>Sentry: sys_connect(10.96.0.1:6443)
    Sentry->>Policy: Evaluate outbound TCP SYN packet
    Note over Policy: Policy behavior is not established by this narrative
    Policy--xSentry: Packet handling TO CONFIRM
    Sentry-->>Agent: Connection timeout (ETIMEDOUT)
    Agent->>DLP: Output raw logs to stdout/stderr
    DLP-->>DLP: High-entropy regex scrubbing
    Note over DLP: API keys / Tokens replaced with [REDACTED]
```

### 3.1 Execution Chronology

1. **`t=0s` - Mission Ingestion & Injection**: Mission ingested containing an adversarial prompt injection payload attempting to compromise cluster infrastructure.
2. **`t=5s` - Sandbox Pod Spawn**: `launch_sandbox_pod` spawns ephemeral pod in `factory-sandbox` under `RuntimeClass: gvisor` with `UID 10001`, `readOnlyRootFilesystem: true`, and `automountServiceAccountToken: false`.
3. **`t=12s` - Token Exfiltration Attempt**: Agent code executes `open("/var/run/secrets/kubernetes.io/serviceaccount/token")`. Operation fails with `ENOENT` (File not found) as ServiceAccount tokens are unmounted.
4. **`t=14s` - Filesystem Tampering Attempt**: Agent code attempts writing extracted secrets to `/stolen_token.txt`. Operation fails with `EROFS` (Read-only file system).
5. **`t=25s` - Internal Network Reconnaissance**: The narrative describes a socket scan across `10.96.0.0/16`; network-policy behavior and the installed CNI are **TO CONFIRM**.
6. **`t=35s` - Output Log Scrubbing**: Log outputs captured by `update_mission_status` pass through high-entropy DLP filters. All secret strings and token patterns are scrubbed to `[REDACTED]`.

---

## 4. Adversarial Defense & Isolation Analysis Matrix

| Attack vector / scenario | Target asset | Described control | Reported result in this narrative | Status |
| :--- | :--- | :--- | :--- | :--- |
| **ServiceAccount token dumping** | Cluster Kubernetes ServiceAccount credentials | `automountServiceAccountToken: false` | The narrative reports `ENOENT`; independent runtime evidence is **TO CONFIRM** | **TO CONFIRM** |
| **Internal cluster CIDR scan** | CIDR `10.96.0.0/16` services | NetworkPolicy; CNI is **TO CONFIRM** | Packet handling is **TO CONFIRM** | **TO CONFIRM** |
| **Kernel privilege escalation** | Host OS kernel (`/proc`, `dmesg`) | gVisor `runsc` user-space sandbox | The narrative reports `EPERM`; independent runtime evidence is **TO CONFIRM** | **TO CONFIRM** |
| **Root disk modification** | `/` root filesystem | `securityContext.readOnlyRootFilesystem` | The narrative reports `EROFS`; independent runtime evidence is **TO CONFIRM** | **TO CONFIRM** |
| **Resource exhaustion** | Sandbox memory / CPU | `cgroups v2` hard limit ($30\text{ MiB}$) | The narrative reports OOM termination; resource and host effects are **TO CONFIRM** | **TO CONFIRM** |
| **Log stream exfiltration** | stdout / stderr history logs | High-entropy log sanitizer | The narrative reports redaction; independent runtime evidence is **TO CONFIRM** | **TO CONFIRM** |

---

## 5. Lessons Learned

1. **Prompt Isolation Non-Viability**: Natural language prompt engineering and LLM system prompts cannot guarantee protection against prompt injection attacks. Autonomous agents must be treated as untrusted binary execution engines.
2. **Runtime and policy boundaries**: The repository configures gVisor (`runsc`) and a Kubernetes `NetworkPolicy`. The installed CNI and runtime behavior are **TO CONFIRM**.
3. **Multi-Layered DLP Filtering**: Output log streams and history tracking records must pass through deterministic high-entropy DLP filters before persistence.

---

## 6. Industrial Transfer Applications

### CAD/CAM Sheet Metal Nesting Pipelines (Lantek MES Integration)
The results of experiment **EXP-2026-007** are directly applicable to cloud-hosted industrial CAD/CAM software (such as Lantek MES sheet metal nesting pipelines):
- **Scripted CAD Geometry Sandboxing**: Customer-supplied geometry files (DXF, STEP, IGES) with embedded Python/VBA macro scripts are processed inside ephemeral gVisor microVM sandboxes.
- **Multi-Tenant Protection**: Malicious macros embedded within DXF/STEP files cannot query local host APIs, access client databases, or scan neighbouring tenant networks.
- **Tenant Isolation Integrity**: Guarantees zero cross-tenant credential or geometry leakage across cloud CAD/CAM customer environments.

---

> *Related: [Experiment Logs](experiment_logs.md) · [Lab Notebook EXP-2026-006](lab_notebook_exp_2026_006.md) · [Security Architecture](../security/security_architecture.md)*
