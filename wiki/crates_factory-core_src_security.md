---
type: module
title: "security.rs"
source_path: "crates/factory-core/src/security.rs"
description: "Documentation for crates/factory-core/src/security.rs"
tags: [rust, module]
last_verified_commit: "00ae8eaf"
---

# security.rs

Source File: `crates/factory-core/src/security.rs`

## Component Architecture

```mermaid
classDiagram
    class SandboxConstraint
    class SecurityValidator {
        <<trait>>
    }
    class AuditResult
    class SastScanResult
    class ScanPolicy
    class Ed25519SecurityValidator
    class JitToken
    class CredentialSessionRef
    class TaskExecutionContext
    class SecurityBounds {
        <<trait>>
    }
```

## Execution Flow

```mermaid
flowchart TD
    Start --> End
```
