---
type: module
title: "aethalgard.rs"
source_path: "crates/factory-infrastructure/src/aethalgard.rs"
description: "Documentation for crates/factory-infrastructure/src/aethalgard.rs"
tags: [rust, module]
last_verified_commit: "17a28f4"
---

# aethalgard.rs

Source File: `crates/factory-infrastructure/src/aethalgard.rs`

## Component Architecture

```mermaid
classDiagram
    class AethalgardClient {
        <<trait>>
    }
    class HttpAethalgardClient
```

## Execution Flow

```mermaid
flowchart TD
    Start --> notify_remediation
    notify_remediation --> new
    new --> notify_remediation
    notify_remediation --> End
```
