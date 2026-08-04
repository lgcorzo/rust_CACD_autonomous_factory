---
type: "module-documentation"
title: "retrieve_context.rs"
source_path: "crates/factory-mcp-server/src/tools/retrieve_context.rs"
description: "Detailed documentation for retrieve_context.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: retrieve_context.rs

**Source Path:** `crates/factory-mcp-server/src/tools/retrieve_context.rs`

## Overview

### Purpose
Provides implementation for retrieve_context.rs.

### Responsibilities
* Handles logic related to retrieve_context.

### Dependencies
* serde_json::{json, Value}, crate::protocol::{CallToolResult, McpContent}, super::*, std::sync::Arc, factory_infrastructure::R2rClient, async_trait::async_trait, crate::tools::Tool

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### RetrieveContextTool

**Overview:** Represents RetrieveContextTool.

**Public Methods:**

##### `new(r2r_client: Arc<dyn R2rClient> (Any)) -> Self`
Executes new.

#### ManualMockR2rClient

**Overview:** Represents ManualMockR2rClient.

**Public Methods:**

None.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class RetrieveContextTool {
        +new(r2r_client: Arc<dyn R2rClient>:Any) Self
        -name() String
        -description() String
        -input_schema() Value
        -call(params: Value:Any) anyhow::Result<CallToolResult>
    }
    Tool <|-- RetrieveContextTool : Inheritance / Specialization
    class ManualMockR2rClient {
        -search(_query: &str:Any) anyhow::Result<String>
        -push_osr_metric(_metric: &factory_core::OsrMetric:Any) anyhow::Result<()>
    }
    R2rClient <|-- ManualMockR2rClient : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Retrieve_contextService
    Caller->>Svc: new()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** serde_json::{json, Value}, crate::protocol::{CallToolResult, McpContent}, super::*, std::sync::Arc, factory_infrastructure::R2rClient, async_trait::async_trait, crate::tools::Tool
