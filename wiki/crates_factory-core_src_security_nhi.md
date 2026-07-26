---
type: module
title: "nhi.rs"
source_path: "crates/factory-core/src/security/nhi.rs"
description: "Documentation for crates/factory-core/src/security/nhi.rs"
tags: [rust, module]
last_verified_commit: "4cb40d0"
---

# nhi.rs

Source File: `crates/factory-core/src/security/nhi.rs`

## Component Architecture

```mermaid
classDiagram
    class AgentSubject
    class CryptographicProof
    class VerifiableCredential
```

## Execution Flow

```mermaid
flowchart TD
    Start --> new
    new --> sign
    sign --> End
```
