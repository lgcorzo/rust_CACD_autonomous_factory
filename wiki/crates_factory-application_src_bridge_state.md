---
type: module
title: "state.rs"
source_path: "crates/factory-application/src/bridge/state.rs"
description: "Documentation for crates/factory-application/src/bridge/state.rs"
tags: [rust, module]
last_verified_commit: "4cb40d0"
---

# state.rs

Source File: `crates/factory-application/src/bridge/state.rs`

## Component Architecture

```mermaid
classDiagram
    class StepCheckpoint
    class BridgeState
    class BridgeStatus {
        <<enum>>
    }
```

## Execution Flow

```mermaid
flowchart TD
    Start --> new
    new --> get_checkpoint_key
    get_checkpoint_key --> load_checkpoint
    load_checkpoint --> save_checkpoint
    save_checkpoint --> End
```
