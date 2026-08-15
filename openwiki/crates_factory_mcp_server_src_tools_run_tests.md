---
type: "module-documentation"
title: "run_tests.rs"
source_path: "crates/factory-mcp-server/src/tools/run_tests.rs"
description: "Detailed documentation for run_tests.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "9c1db1c"
---

# File: run_tests.rs

**Source Path:** `crates/factory-mcp-server/src/tools/run_tests.rs`

## Overview

### Purpose
Provides implementation for run_tests.rs.

### Responsibilities
* Handles logic related to run_tests.

### Dependencies
* async_trait::async_trait, crate::protocol::{CallToolResult, McpContent}, crate::sandbox::SandboxDriver, crate::tools::Tool, serde_json::{json, Value}, std::sync::Arc

### Imported modules
* None

### Exported classes
* RunTestsTool

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### RunTestsTool

**Overview:**
No description provided.

**Constructor:**

##### `new(driver: Arc<dyn SandboxDriver> (Any))`
Parameters: driver: Arc<dyn SandboxDriver> (Any)
Dependencies: Inherited from context
Initialization: Sets up RunTestsTool

**Attributes:**

* `driver` (Arc<dyn SandboxDriver>): Purpose - Stores driver data. Constraints - Valid Arc<dyn SandboxDriver>.

**Public Methods:**

None.

**Private Methods:**

* `call(_params: Value (Any)) -> anyhow::Result<CallToolResult>`: Internal helper logic.
* `description() -> String`: Internal helper logic.
* `input_schema() -> Value`: Internal helper logic.
* `name() -> String`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class RunTestsTool {
    -call(_params: Value:Any) : anyhow::Result<CallToolResult>
    -description() : String
    -input_schema() : Value
    -name() : String
    +new(driver: Arc<dyn SandboxDriver>:Any) : Self
}
Tool <|-- RunTestsTool : extends/implements
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "Run_testsService" as Svc
Caller -> Svc: call()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```


## Examples

```
// Example usage of run_tests.rs components
import { ... } from 'crates/factory-mcp-server/src/tools/run_tests.rs';
```


## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** async_trait::async_trait, crate::protocol::{CallToolResult, McpContent}, crate::sandbox::SandboxDriver, crate::tools::Tool, serde_json::{json, Value}, std::sync::Arc
