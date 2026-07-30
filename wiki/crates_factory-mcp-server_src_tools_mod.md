---
type: module
title: "mod.rs"
source_path: "crates/factory-mcp-server/src/tools/mod.rs"
description: "Documentation for crates/factory-mcp-server/src/tools/mod.rs"
tags: [rust, module]
last_verified_commit: "beeed91"
---

# mod.rs

Source File: `crates/factory-mcp-server/src/tools/mod.rs`

## Component Architecture

```mermaid
classDiagram
    class Tool {
        <<trait>>
    }
```

## Execution Flow

```mermaid
flowchart TD
    Start --> name
    name --> description
    description --> input_schema
    input_schema --> call
    call --> End
```
