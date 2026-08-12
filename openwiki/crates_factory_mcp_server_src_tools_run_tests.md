---
type: "module-documentation"
title: "run_tests.rs"
source_path: "crates/factory-mcp-server/src/tools/run_tests.rs"
description: "Detailed documentation for run_tests.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "dfd90f5"
---

# File: run_tests.rs

**Source Path:** `crates/factory-mcp-server/src/tools/run_tests.rs`

## Overview

### Purpose
Provides implementation for run_tests.rs.

### Responsibilities
* Handles logic related to run_tests.

### Main Workflow
* Initialization and execution of run_tests logic.

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
Why it exists:
Provides capabilities related to RunTestsTool.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

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

```mermaid
classDiagram
    direction BT
    class RunTestsTool {
        -call(_params: Value:Any) anyhow::Result<CallToolResult>
        -description() String
        -input_schema() Value
        -name() String
        +new(driver: Arc<dyn SandboxDriver>:Any) Self
    }
    Tool <|-- RunTestsTool : Inheritance / Specialization

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Run_testsService
    Caller->>Svc: call()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## UML

### Class Diagram
```plantuml
@startuml
class RunTestsTool {
    -call(_params: Value:Any) : anyhow::Result<CallToolResult>
    -description() : String
    -input_schema() : Value
    -name() : String
    +new(driver: Arc<dyn SandboxDriver>:Any) : Self
}
Tool <|-- RunTestsTool : Inheritance
@enduml

```

### Package Diagram
```plantuml
@startuml
package "run_tests" {
  [Module Components]
}
@enduml

```

### Sequence Diagram
```plantuml
@startuml
autonumber
participant Caller as "Client Interface"
participant Svc as "Run_testsService"
Caller -> Svc: new()
note over Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

### Component Diagram
```plantuml
@startuml
component "run_tests" as comp
component "async_trait::async_trait" as async_trait::async_trait
comp --> async_trait::async_trait
component "crate::protocol::{CallToolResult, McpContent}" as crate::protocol::{CallToolResult, McpContent}
comp --> crate::protocol::{CallToolResult, McpContent}
component "crate::sandbox::SandboxDriver" as crate::sandbox::SandboxDriver
comp --> crate::sandbox::SandboxDriver
component "crate::tools::Tool" as crate::tools::Tool
comp --> crate::tools::Tool
component "serde_json::{json, Value}" as serde_json::{json, Value}
comp --> serde_json::{json, Value}
component "std::sync::Arc" as std::sync::Arc
comp --> std::sync::Arc
@enduml

```

### Dependency Graph
```plantuml
@startuml
[run_tests]
[run_tests] --> [async_trait::async_trait]
[run_tests] --> [crate::protocol::{CallToolResult, McpContent}]
[run_tests] --> [crate::sandbox::SandboxDriver]
[run_tests] --> [crate::tools::Tool]
[run_tests] --> [serde_json::{json, Value}]
[run_tests] --> [std::sync::Arc]
@enduml

```

### Call Graph
```plantuml
@startuml
[API] --> RunTestsTool::new
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
