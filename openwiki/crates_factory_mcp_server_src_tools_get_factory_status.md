---
type: "module-documentation"
title: "get_factory_status.rs"
source_path: "crates/factory-mcp-server/src/tools/get_factory_status.rs"
description: "Detailed documentation for get_factory_status.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "9c1db1c"
---

# File: get_factory_status.rs

**Source Path:** `crates/factory-mcp-server/src/tools/get_factory_status.rs`

## Overview

### Purpose
Provides implementation for get_factory_status.rs.

### Responsibilities
* Handles logic related to get_factory_status.

### Dependencies
* async_trait::async_trait, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, serde_json::{json, Value}, std::env, super::*

### Imported modules
* None

### Exported classes
* GetFactoryStatusTool

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### GetFactoryStatusTool

**Overview:**
No description provided.

**Constructor:**

##### `new()`
Parameters:
Dependencies: Inherited from context
Initialization: Sets up GetFactoryStatusTool

**Attributes:**

None.

**Public Methods:**

None.

**Private Methods:**

* `call(_params: Value (Any)) -> anyhow::Result<CallToolResult>`: Internal helper logic.
* `default() -> Self`: Internal helper logic.
* `description() -> String`: Internal helper logic.
* `input_schema() -> Value`: Internal helper logic.
* `name() -> String`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class GetFactoryStatusTool {
    -call(_params: Value:Any) : anyhow::Result<CallToolResult>
    -default() : Self
    -description() : String
    -input_schema() : Value
    -name() : String
    +new() : Self
}
Default <|-- GetFactoryStatusTool : extends/implements
Tool <|-- GetFactoryStatusTool : extends/implements
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "Get_factory_statusService" as Svc
Caller -> Svc: call()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```


## Examples

```
// Example usage of get_factory_status.rs components
import { ... } from 'crates/factory-mcp-server/src/tools/get_factory_status.rs';
```


## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** async_trait::async_trait, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, serde_json::{json, Value}, std::env, super::*
