---
type: "module-documentation"
title: "list_minio_objects.rs"
source_path: "crates/factory-mcp-server/src/tools/list_minio_objects.rs"
description: "Detailed documentation for list_minio_objects.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "7982a81"
---

# File: list_minio_objects.rs

**Source Path:** `crates/factory-mcp-server/src/tools/list_minio_objects.rs`

## Overview

### Purpose
Provides implementation for list_minio_objects.rs.

### Responsibilities
* Handles logic related to list_minio_objects.

### Dependencies
* async_trait::async_trait, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, serde_json::{json, Value}, std::env, super::*

### Imported modules
* None

### Exported classes
* ListMinioObjectsTool

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### ListMinioObjectsTool

**Overview:**
Why it exists:
Provides capabilities related to ListMinioObjectsTool.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new()`
Parameters:
Dependencies: Inherited from context
Initialization: Sets up ListMinioObjectsTool

**Attributes:**

None.

**Public Methods:**

None.

**Private Methods:**

* `call(params: Value (Any)) -> anyhow::Result<CallToolResult>`: Internal helper logic.
* `default() -> Self`: Internal helper logic.
* `description() -> String`: Internal helper logic.
* `input_schema() -> Value`: Internal helper logic.
* `name() -> String`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class ListMinioObjectsTool {
    -call(params: Value:Any) : anyhow::Result<CallToolResult>
    -default() : Self
    -description() : String
    -input_schema() : Value
    -name() : String
    +new() : Self
}
Default <|-- ListMinioObjectsTool : Inheritance / Specialization
Tool <|-- ListMinioObjectsTool : Inheritance / Specialization
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant Caller as "Client Interface"
participant Svc as "List_minio_objectsService"
Caller -> Svc : call()
note over Svc : Processing internal logic
Svc --> Caller : result
@enduml

```

## Examples

```
// Example usage of list_minio_objects.rs components
import { ... } from 'crates/factory-mcp-server/src/tools/list_minio_objects.rs';
```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** async_trait::async_trait, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, serde_json::{json, Value}, std::env, super::*
