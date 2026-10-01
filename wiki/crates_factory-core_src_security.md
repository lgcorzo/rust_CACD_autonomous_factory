# factory-core::security — Security Primitives

> **Source**: `crates/factory-core/src/security.rs`
> **Layer**: Domain

---

## UML Class Diagram

```mermaid
classDiagram
    class SandboxConstraint {
        +u32 max_memory_mb
        +f32 max_cpu_cores
        +bool network_egress_allowed
        +gvisor_default()$ SandboxConstraint
        +sidecar_default()$ SandboxConstraint
        +validate_bounds() bool
    }
    class SecurityValidator {
        <<trait>>
        +validate_signature(data, signature) Result~bool~
        +audit_content(content) Result~AuditResult~
    }
    class Ed25519SecurityValidator {
        +VerifyingKey public_key
    }
    class AuditResult {
        +bool is_safe
        +Vec~String~ findings
    }
    class SastScanResult {
        +bool is_safe
        +f32 score
        +Vec~String~ findings
        +bool critical_vulnerabilities_detected
        +passes_gate() bool
        +inspect_diff(diff)$ SastScanResult
    }
    class JitToken {
        +String token
    }
    class SecurityBounds {
        <<trait>>
        +validate_token(token) Result~bool~
        +issue_jit_token(audience) Result~JitToken~
        +wipe_token_from_memory(token)
    }

    Ed25519SecurityValidator ..|> SecurityValidator
    Ed25519SecurityValidator --> AuditResult
    SecurityBounds --> JitToken
    JitToken ..|> zeroize::ZeroizeOnDrop : derives

    note for SandboxConstraint "gVisor: 30 MiB / 0.25 CPU\nSidecar: 20 MiB / 0.10 CPU\nNo network egress"
    note for SastScanResult "Gate: score >= 8.0\nNo critical vulns"
```

## SAST Inspection Rules

| Check | Pattern | Score Impact | Critical? |
|:---|:---|:---:|:---:|
| Hardcoded Secrets | `password = "`, `api_key = "`, `secret = "`, `private_key = "` | -4.0 | ✅ |
| Command Injection | `system(`, `exec(`, `eval(` | -5.0 | ✅ |
| SQL Injection | `format!("select`, `format!("delete` | -4.0 | ✅ |
| Unbounded Loop | `loop {` without `break`/`return` | -2.5 | ❌ |

## Trust Boundary Model

```mermaid
graph LR
    OUTSIDE["Untrusted: External APIs"]
    ZITI["OpenZiti Tunnel"]
    NHI["NHI Credential Check"]
    SAST["SAST Gate: >= 8.0"]
    SANDBOX["gVisor Sandbox"]
    
    OUTSIDE -->|"Encrypted"| ZITI
    ZITI -->|"Ed25519 verify"| NHI
    NHI -->|"inspect_diff()"| SAST
    SAST -->|"Constrained execution"| SANDBOX

    style OUTSIDE fill:#f44336,stroke:#c62828,color:#fff
    style SANDBOX fill:#4CAF50,stroke:#2E7D32,color:#fff
```

---

> *Related: [nhi.rs](crates_factory-core_src_security_nhi.md) · [Security Architecture](SECURITY-ARCHITECTURE.md)*
