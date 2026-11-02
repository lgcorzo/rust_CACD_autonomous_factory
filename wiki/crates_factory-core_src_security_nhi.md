---
type: module
title: "nhi.rs"
source_path: "crates/factory-core/src/security/nhi.rs"
description: "Documentation for crates/factory-core/src/security/nhi.rs"
tags: [rust, module]
last_verified_commit: "a9ce0d2d"
---

# nhi.rs

Source File: `crates/factory-core/src/security/nhi.rs`

## Component Architecture

```mermaid
classDiagram
    class VerificationOutcome {
        <<enumeration>>
    }
    class IssuerTrustStore {
        <<trait>>
    }
    class InMemoryTrustStore
    class NonceReplayCache
    class AgentSubject
    class CryptographicProof
    class ProofOptions
    class VerifiableCredential
```

## Execution Flow

```mermaid
flowchart TD
    Start --> End
```
