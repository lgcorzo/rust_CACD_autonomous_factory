# Security Architecture — Dark Gravity CA/CD Autonomous Factory

> **Purpose**: Comprehensive documentation of the Zero Trust security model, Ed25519 Non-Human Identity credentials, sandbox containment, SAST gates, and threat model mapping.

---

## 1. Zero Trust Security Model Overview

The Dark Gravity Factory operates under a **Zero Trust** security posture: no implicit trust is granted to any agent, service, or network path. Every action requires explicit cryptographic verification.

```mermaid
graph TB
    subgraph "Trust Boundary: External"
        GH["GitHub API"]
        GL["GitLab API"]
        JR["Jira API"]
    end
    
    subgraph "Trust Boundary: Network Layer"
        ZITI["OpenZiti Overlay"]
    end
    
    subgraph "Trust Boundary: Identity Layer"
        NHI["Ed25519 NHI Issuer"]
        VAULT["Vault Secret Store"]
        JIT["JitToken Manager"]
    end
    
    subgraph "Trust Boundary: Execution Layer"
        SAST["SAST Gate: score >= 8.0"]
        SANDBOX["gVisor Sandbox Pod"]
        CB["Circuit Breaker: max 3 retries"]
    end
    
    subgraph "Trust Boundary: Governance Layer"
        HITL["HITL 4-Vertex Mesh"]
    end
    
    GH --> ZITI
    GL --> ZITI
    ZITI --> NHI
    NHI --> SAST
    SAST --> SANDBOX
    SANDBOX --> CB
    CB --> HITL

    style ZITI fill:#2196F3,stroke:#1565C0,color:#fff
    style NHI fill:#4CAF50,stroke:#2E7D32,color:#fff
    style SANDBOX fill:#FF9800,stroke:#E65100,color:#fff
    style HITL fill:#9C27B0,stroke:#6A1B9A,color:#fff
```

---

## 2. Ed25519 Non-Human Identity (NHI) Lifecycle

Every autonomous mission requires a cryptographic identity. The NHI lifecycle ensures traceability from issue ingestion through code delivery.

```mermaid
sequenceDiagram
    participant Poller as PollerDaemonService
    participant NHI as NHI Issuer
    participant Vault as HashiCorp Vault
    participant Agent as ZeroClawAgent
    participant Validator as SecurityValidator

    Poller->>NHI: Request credential for mission
    NHI->>Vault: Retrieve Ed25519 signing key
    Vault-->>NHI: SigningKey (zeroized after use)
    NHI->>NHI: Sign mission payload with Ed25519
    NHI-->>Poller: VerifiableCredential (JSON-LD)
    
    Poller->>Agent: Dispatch task with NHI credential
    Agent->>Validator: validate_signature(data, sig)
    Validator->>Validator: Base64 decode + Ed25519 verify
    Validator-->>Agent: Verification result (bool)
    
    alt Signature Valid
        Agent->>Agent: Execute task in sandbox
    else Signature Invalid
        Agent->>Agent: REJECT - FactoryError::Security
    end
```

### Key Security Types

| Type | Module | Description |
|:---|:---|:---|
| `Ed25519SecurityValidator` | `factory-core::security` | Implements `SecurityValidator` trait with `ed25519-dalek` |
| `JitToken` | `factory-core::security` | Ephemeral token with `zeroize::ZeroizeOnDrop` — wiped from memory on drop |
| `SecurityBounds` | `factory-core::security` | Trait for token validation and JIT issuance |
| `AuditResult` | `factory-core::security` | Content audit output: `is_safe`, `findings` |

---

## 3. Sandbox Containment Architecture

Agent code execution is isolated in gVisor-sandboxed Kubernetes pods with strict resource constraints.

```mermaid
graph TB
    subgraph "K8s Pod: Sandbox"
        subgraph "gVisor Runtime"
            APP["Application Container"]
            SIDE["Sidecar Container"]
        end
        NET["Network Policy: No Egress"]
        RES["Resource Quota"]
    end
    
    MCP["factory-mcp-server"] --> |"launch_sandbox_pod"| APP
    APP --> |"max 30 MiB RAM, 0.25 CPU"| RES
    SIDE --> |"max 20 MiB RAM, 0.10 CPU"| RES
    NET --> |"network_egress_allowed: false"| APP

    style APP fill:#FF9800,stroke:#E65100,color:#fff
    style NET fill:#f44336,stroke:#c62828,color:#fff
```

### SandboxConstraint Enforcement

```rust
// From factory-core::security
pub struct SandboxConstraint {
    pub max_memory_mb: u32,      // Default: 30 MiB
    pub max_cpu_cores: f32,       // Default: 0.25 cores
    pub network_egress_allowed: bool, // Default: false
}

// ZeroClawAgent validates before every execution:
// - App container: max_memory_mb <= 30, max_cpu_cores <= 0.25
// - Sidecar container: max_memory_mb <= 20, max_cpu_cores <= 0.10
// - network_egress_allowed must be false for both
```

---

## 4. SAST Gate Integration

Every code change passes through a Static Application Security Testing gate before execution. The `SastScanResult` enforces a minimum score of **8.0/10.0**.

```mermaid
flowchart TD
    DIFF["Code Diff"] --> SCAN["SastScanResult::inspect_diff"]
    SCAN --> CHECK1{"Hardcoded Secrets?"}
    CHECK1 -->|Yes| CRIT1["CRITICAL: -4.0 score"]
    CHECK1 -->|No| CHECK2{"Command Injection?"}
    CHECK2 -->|Yes| CRIT2["CRITICAL: -5.0 score"]
    CHECK2 -->|No| CHECK3{"SQL Injection?"}
    CHECK3 -->|Yes| CRIT3["CRITICAL: -4.0 score"]
    CHECK3 -->|No| CHECK4{"Unbounded Loop?"}
    CHECK4 -->|Yes| WARN1["WARNING: -2.5 score"]
    CHECK4 -->|No| PASS["Score: 10.0"]
    
    CRIT1 --> GATE
    CRIT2 --> GATE
    CRIT3 --> GATE
    WARN1 --> GATE
    PASS --> GATE
    
    GATE{"score >= 8.0 AND<br/>no critical vulns?"}
    GATE -->|Yes| ALLOW["✅ PASS: Execute in Sandbox"]
    GATE -->|No| BLOCK["❌ BLOCK: Execution Rejected"]

    style BLOCK fill:#f44336,stroke:#c62828,color:#fff
    style ALLOW fill:#4CAF50,stroke:#2E7D32,color:#fff
```

---

## 5. Circuit Breaker Anti-Deadlock (Aethelgard)

The Aethelgard circuit breaker prevents infinite agent retry loops:

```mermaid
stateDiagram-v2
    [*] --> ActiveExecution
    ActiveExecution --> TaskRetry: Execution/Test Error
    TaskRetry --> ActiveExecution: Attempt < 3
    TaskRetry --> DeadlockTripped: Attempt == 3 AND Diff Hash Stagnant
    DeadlockTripped --> AgentStuck: Trip Circuit Breaker
    AgentStuck --> HumanIntervention: Alert via HITL Vertex 3
    HumanIntervention --> ActiveExecution: Architect Override + Parameter Tune
    HumanIntervention --> MissionAborted: Mission Rejection
    MissionAborted --> [*]
```

---

## 6. STRIDE Threat Model Mapping

| Threat | Category | Control | Implementation |
|:---|:---|:---|:---|
| Agent impersonation | Spoofing | Ed25519 NHI credentials | `SecurityValidator::validate_signature()` |
| Unauthorized code execution | Tampering | SAST gate + sandbox isolation | `SastScanResult::passes_gate()` + gVisor |
| Mission data exposure | Information Disclosure | OpenZiti encrypted tunnels | `ZitiClient` Zero Trust overlay |
| Denial of service via token burn | Denial of Service | FinOps budget hardstops | `DailyBudgetConfig::hardstop_threshold_ratio` |
| Privilege escalation | Elevation of Privilege | Minimal sandbox constraints | `SandboxConstraint::validate_bounds()` |
| Audit trail tampering | Repudiation | Cryptographic NHI claims | Ed25519 signed verifiable credentials |

---

> *Related: [HITL Governance](HITL-GOVERNANCE.md) · [Verification Triad](VERIFICATION-TRIAD.md) · [Compliance & Audit](COMPLIANCE-AUDIT.md)*
