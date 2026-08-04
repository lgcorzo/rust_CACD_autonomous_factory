---
type: "module-documentation"
title: "index_code.rs"
source_path: "crates/factory-mcp-server/src/tools/index_code.rs"
description: "Detailed documentation for index_code.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: index_code.rs

**Source Path:** `crates/factory-mcp-server/src/tools/index_code.rs`

## Overview

### Purpose
Provides implementation for index_code.rs.

### Responsibilities
* Handles logic related to index_code.

### Dependencies
* super::*, serde_json::{json, Value}, crate::tools::Tool, crate::protocol::{CallToolResult, McpContent}, async_trait::async_trait

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### IndexCodeTool

**Overview:** Represents IndexCodeTool.

**Public Methods:**

##### `new(r2r_base_url: String (Any)) -> Self`
Executes new.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class IndexCodeTool {
        +new(r2r_base_url: String:Any) Self
        -name() String
        -description() String
        -input_schema() Value
        -call(params: Value:Any) anyhow::Result<CallToolResult>
    }
    Tool <|-- IndexCodeTool : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Index_codeService
    Caller->>Svc: new()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** super::*, serde_json::{json, Value}, crate::tools::Tool, crate::protocol::{CallToolResult, McpContent}, async_trait::async_trait
