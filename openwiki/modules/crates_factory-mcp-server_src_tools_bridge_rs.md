---
type: "module-documentation"
title: "bridge.rs"
source_path: "crates/factory-mcp-server/src/tools/bridge.rs"
description: "Detailed documentation for bridge.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: bridge.rs

**Source Path:** `crates/factory-mcp-server/src/tools/bridge.rs`

## Overview

### Purpose
Provides implementation for bridge.rs.

### Responsibilities
* Handles logic related to bridge.

### Dependencies
* std::fs, crate::protocol::CallToolResult, serde_json::{json, Value}, std::path::PathBuf, crate::tools::Tool, async_trait::async_trait

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### BridgeTool

**Overview:** Represents BridgeTool.

**Public Methods:**

##### `load_state(mission_id: &str (Any)) -> anyhow::Result<Value>`
Executes load_state.

##### `save_state(mission_id: &str (Any), state: Value (Any)) -> anyhow::Result<Value>`
Executes save_state.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class BridgeTool {
        -get_checkpoint_path(mission_id: &str:Any) PathBuf
        +load_state(mission_id: &str:Any) anyhow::Result<Value>
        +save_state(mission_id: &str:Any, state: Value:Any) anyhow::Result<Value>
        -name() String
        -description() String
        -input_schema() Value
        -call(params: Value:Any) anyhow::Result<CallToolResult>
    }
    Tool <|-- BridgeTool : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as BridgeService
    Caller->>Svc: get_checkpoint_path()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** std::fs, crate::protocol::CallToolResult, serde_json::{json, Value}, std::path::PathBuf, crate::tools::Tool, async_trait::async_trait
