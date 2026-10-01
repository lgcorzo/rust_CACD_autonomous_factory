---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-core::security::nhi — Non-Human Identity Credentials"
source_path: "crates/factory-core/src/security/nhi.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-core::security::nhi — Non-Human Identity Credentials."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-core::security::nhi — Non-Human Identity Credentials

> **Source**: `crates/factory-core/src/security/nhi.rs`
> **Layer**: Domain

---

## UML Class Diagram

```mermaid
classDiagram
    class VerifiableCredential {
        +Vec~String~ context
        +String id
        +Vec~String~ credential_type
        +String issuer
        +DateTime issuance_date
        +AgentSubject credential_subject
        +Option~CryptographicProof~ proof
        +new(id, issuer, subject) VerifiableCredential
        +sign(signing_key, key_id) Result
        +sign_async(signing_key, key_id) Result
        +sign_batch_async(credentials, key, key_id)$ Result
        +verify(verifying_key) Result~bool~
        +verify_batch_async(credentials, key)$ Result~bool~
    }
    class AgentSubject {
        +String id
        +Vec~String~ roles
        +Vec~String~ allowed_namespaces
    }
    class CryptographicProof {
        +String proof_type
        +DateTime created
        +String verification_method
        +String proof_purpose
        +String jws
    }

    VerifiableCredential "1" *-- "1" AgentSubject
    VerifiableCredential "1" *-- "0..1" CryptographicProof

    note for VerifiableCredential "W3C VC Data Model\nEd25519Signature2020\nJWS: Header..Signature"
```

## NHI Credential Issuance and Verification Sequence

```mermaid
sequenceDiagram
    participant Factory as Factory Orchestrator
    participant NHI as NHI Module
    participant Vault as Vault
    participant Agent as Agent Pod

    Factory->>NHI: Create VerifiableCredential(id, issuer, AgentSubject)
    NHI->>Vault: Retrieve Ed25519 SigningKey
    Vault-->>NHI: SigningKey
    NHI->>NHI: sign(signing_key, key_id)
    Note right of NHI: 1. Serialize VC without proof<br/>2. Create JWS header (EdDSA)<br/>3. Sign Header.Payload<br/>4. Attach CryptographicProof
    NHI-->>Factory: Signed VerifiableCredential
    
    Factory->>Agent: Dispatch task + VC
    Agent->>Agent: verify(verifying_key)
    Note right of Agent: 1. Extract JWS parts (Header..Sig)<br/>2. Decode Base64 signature<br/>3. Reconstruct signing input<br/>4. Ed25519 verify

    alt Valid
        Agent-->>Factory: Task executed
    else Invalid
        Agent-->>Factory: FactoryError::Security
    end
```

## Batch Operations

| Method | Description | Concurrency |
|:---|:---|:---|
| `sign_batch_async` | Signs multiple VCs concurrently | `tokio::task::spawn_blocking` per VC |
| `verify_batch_async` | Verifies multiple VCs concurrently | Returns `false` on first failure |

---

> *Related: [security.rs](security.md) · [Security Architecture](../../security/security_architecture.md)*
