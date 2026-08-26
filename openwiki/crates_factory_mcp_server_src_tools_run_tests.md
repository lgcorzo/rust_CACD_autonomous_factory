---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "run_tests.rs"
source_path: "crates/factory-mcp-server/src/tools/run_tests.rs"
description: "Detailed documentation for run_tests.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-26T06:00:08Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "e2707de"
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

##### `new(driver (Arc<dyn SandboxDriver>))`
Parameters: driver (Arc<dyn SandboxDriver>)
Dependencies: Inherited from context
Initialization: Sets up RunTestsTool

**Attributes:**

* `driver` (Arc<dyn SandboxDriver>): Purpose - Stores driver data. Constraints - Valid Arc<dyn SandboxDriver>.

**Public Methods:**

None.

**Private Methods:**

* `call(_params (Value)) -> anyhow::Result<CallToolResult>`: Internal helper logic.
* `description() -> String`: Internal helper logic.
* `input_schema() -> Value`: Internal helper logic.
* `name() -> String`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class RunTestsTool {
    -call(_params: Value) anyhow::Result<CallToolResult>
    -description() String
    -input_schema() Value
    -name() String
    +new(driver: Arc<dyn SandboxDriver>) Self
}
Tool <|-- RunTestsTool : extends/implements
@enduml

```

## Package Diagram

```plantuml
@startuml
package "crates" {
    package "factory-mcp-server" {
        package "src" {
            package "tools" {
                class Module
            }
        }
    }
}
@enduml

```

## Component Diagram

```plantuml
@startuml
component "run_tests" as Main
component "async_trait::async_trait" as async_trait__async_trait
Main --> async_trait__async_trait : uses
component "crate::protocol::{CallToolResult, McpContent}" as crate__protocol___CallToolResult__McpContent_
Main --> crate__protocol___CallToolResult__McpContent_ : uses
component "crate::sandbox::SandboxDriver" as crate__sandbox__SandboxDriver
Main --> crate__sandbox__SandboxDriver : uses
component "crate::tools::Tool" as crate__tools__Tool
Main --> crate__tools__Tool : uses
component "serde_json::{json, Value}" as serde_json___json__Value_
Main --> serde_json___json__Value_ : uses
component "std::sync::Arc" as std__sync__Arc
Main --> std__sync__Arc : uses
@enduml

```

## Dependency Graph

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

## Call Graph

```plantuml
@startuml
Caller --> RunTestsTool::new
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
