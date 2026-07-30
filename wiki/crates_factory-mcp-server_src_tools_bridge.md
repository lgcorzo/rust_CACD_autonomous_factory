---
type: module
title: "bridge.rs"
source_path: "crates/factory-mcp-server/src/tools/bridge.rs"
description: "Documentation for crates/factory-mcp-server/src/tools/bridge.rs"
tags: [rust, module]
last_verified_commit: "beeed91"
---

# bridge.rs

Source File: `crates/factory-mcp-server/src/tools/bridge.rs`

## Component Architecture

```mermaid
classDiagram
    class BridgeTool
```

## Execution Flow

```mermaid
flowchart TD
    Start --> get_checkpoint_path
    get_checkpoint_path --> load_state
    load_state --> save_state
    save_state --> name
    name --> description
    description --> input_schema
    input_schema --> call
    call --> End
```
