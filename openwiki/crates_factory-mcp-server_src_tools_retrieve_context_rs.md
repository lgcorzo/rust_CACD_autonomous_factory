---
type: "module-documentation"
title: "retrieve_context.rs"
source_path: "crates/factory-mcp-server/src/tools/retrieve_context.rs"
description: "Detailed documentation for retrieve_context.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-06T17:55:58Z"
---

# File: retrieve_context.rs

**Source Path:** `crates/factory-mcp-server/src/tools/retrieve_context.rs`

## Overview

### Purpose
Provides implementation for retrieve_context.rs.

### Responsibilities
* Handles logic related to retrieve_context.

### Dependencies
* factory_infrastructure::R2rClient, std::sync::Arc, crate::protocol::{CallToolResult, McpContent}, async_trait::async_trait, crate::tools::Tool, super::*, serde_json::{json, Value}

### Imported modules
*

### Exported classes
* RetrieveContextTool, ManualMockR2rClient

### Exported interfaces
*

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### RetrieveContextTool

**Overview:**
Why it exists:
Provides capabilities related to RetrieveContextTool.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new(r2r_client: Arc<dyn R2rClient> (Any))`
Parameters: r2r_client: Arc<dyn R2rClient> (Any)
Dependencies: Inherited from context
Initialization: Sets up RetrieveContextTool

**Attributes:**

* `r2r_client` (Arc<dyn R2rClient>): Purpose - Stores r2r_client data. Constraints - Valid Arc<dyn R2rClient>.

**Public Methods:**

None.

**Private Methods:**

* `name() -> String`: Internal helper logic.
* `description() -> String`: Internal helper logic.
* `input_schema() -> Value`: Internal helper logic.
* `call(params: Value (Any)) -> anyhow::Result<CallToolResult>`: Internal helper logic.

#### ManualMockR2rClient

**Overview:**
Why it exists:
Provides capabilities related to ManualMockR2rClient.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `should_fail` (bool): Purpose - Stores should_fail data. Constraints - Valid bool.

**Public Methods:**

None.

**Private Methods:**

* `search(_query: &str (Any)) -> anyhow::Result<String>`: Internal helper logic.
* `push_osr_metric(_metric: &factory_core::OsrMetric (Any)) -> anyhow::Result<()>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

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

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Retrieve_contextService
    Caller->>Svc: new()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```


## Examples

```
// Example usage of retrieve_context.rs components
import { ... } from 'crates/factory-mcp-server/src/tools/retrieve_context.rs';
```


## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** factory_infrastructure::R2rClient, std::sync::Arc, crate::protocol::{CallToolResult, McpContent}, async_trait::async_trait, crate::tools::Tool, super::*, serde_json::{json, Value}
