---
type: "module-documentation"
title: "update_mission_status.rs"
source_path: "crates/factory-mcp-server/src/tools/update_mission_status.rs"
description: "Detailed documentation for update_mission_status.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: update_mission_status.rs

**Source Path:** `crates/factory-mcp-server/src/tools/update_mission_status.rs`

## Overview

### Purpose
Provides implementation for update_mission_status.rs.

### Responsibilities
* Handles logic related to update_mission_status.

### Dependencies
* serde_json::{json, Value}, async_trait::async_trait, crate::tools::Tool, crate::protocol::{CallToolResult, McpContent}, std::fs::{File, OpenOptions}, std::io::Write, chrono::Local

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### UpdateMissionStatusTool

**Overview:** Represents UpdateMissionStatusTool.

**Public Methods:**

##### `new(docs_path: String (Any)) -> Self`
Executes new.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class UpdateMissionStatusTool {
        +new(docs_path: String:Any) Self
        -name() String
        -description() String
        -input_schema() Value
        -call(params: Value:Any) anyhow::Result<CallToolResult>
    }
    Tool <|-- UpdateMissionStatusTool : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Update_mission_statusService
    Caller->>Svc: new()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** serde_json::{json, Value}, async_trait::async_trait, crate::tools::Tool, crate::protocol::{CallToolResult, McpContent}, std::fs::{File, OpenOptions}, std::io::Write, chrono::Local
