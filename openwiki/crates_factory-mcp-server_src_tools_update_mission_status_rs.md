---
type: "module-documentation"
title: "update_mission_status.rs"
source_path: "crates/factory-mcp-server/src/tools/update_mission_status.rs"
description: "Detailed documentation for update_mission_status.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-09T06:11:32Z"
---

# File: update_mission_status.rs

**Source Path:** `crates/factory-mcp-server/src/tools/update_mission_status.rs`

## Overview

### Purpose
Provides implementation for update_mission_status.rs.

### Responsibilities
* Handles logic related to update_mission_status.

### Dependencies
* async_trait::async_trait, chrono::Local, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, serde_json::{json, Value}, super::*, tokio::fs::{File, OpenOptions}, tokio::io::AsyncWriteExt

### Imported modules
* None

### Exported classes
* UpdateMissionStatusTool

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### UpdateMissionStatusTool

**Overview:**
Why it exists:
Provides capabilities related to UpdateMissionStatusTool.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new(docs_path: String (Any))`
Parameters: docs_path: String (Any)
Dependencies: Inherited from context
Initialization: Sets up UpdateMissionStatusTool

**Attributes:**

* `docs_path` (String): Purpose - Stores docs_path data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

* `call(params: Value (Any)) -> anyhow::Result<CallToolResult>`: Internal helper logic.
* `description() -> String`: Internal helper logic.
* `input_schema() -> Value`: Internal helper logic.
* `name() -> String`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class UpdateMissionStatusTool {
        -call(params: Value:Any) anyhow::Result<CallToolResult>
        -description() String
        -input_schema() Value
        -name() String
        +new(docs_path: String:Any) Self
    }
    Tool <|-- UpdateMissionStatusTool : Inheritance / Specialization

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Update_mission_statusService
    Caller->>Svc: call()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```


## Examples

```
// Example usage of update_mission_status.rs components
import { ... } from 'crates/factory-mcp-server/src/tools/update_mission_status.rs';
```


## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** async_trait::async_trait, chrono::Local, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, serde_json::{json, Value}, super::*, tokio::fs::{File, OpenOptions}, tokio::io::AsyncWriteExt
