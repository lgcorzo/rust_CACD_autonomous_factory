---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "retrieve_context.rs"
source_path: "crates/factory-mcp-server/src/tools/retrieve_context.rs"
description: "Detailed documentation for retrieve_context.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-24T05:31:17Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "d073f5f"
---

# File: retrieve_context.rs

**Source Path:** `crates/factory-mcp-server/src/tools/retrieve_context.rs`

## Overview

### Purpose
Provides implementation for retrieve_context.rs.

### Responsibilities
* Handles logic related to retrieve_context.

### Dependencies
* async_trait::async_trait, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, factory_infrastructure::R2rClient, serde_json::{json, Value}, std::sync::Arc, super::*

### Imported modules
* None

### Exported classes
* ManualMockR2rClient, RetrieveContextTool

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### ManualMockR2rClient

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

* `should_fail` (bool): Purpose - Stores should_fail data. Constraints - Valid bool.

**Public Methods:**

None.

**Private Methods:**

* `map_stacktrace_to_ast(query (&str)) -> anyhow::Result<String>`: Internal helper logic.
* `push_osr_metric(_metric (&factory_core::OsrMetric)) -> anyhow::Result<()>`: Internal helper logic.
* `search(_query (&str)) -> anyhow::Result<String>`: Internal helper logic.

#### RetrieveContextTool

**Overview:**
No description provided.

**Constructor:**

##### `new(r2r_client (Arc<dyn R2rClient>))`
Parameters: r2r_client (Arc<dyn R2rClient>)
Dependencies: Inherited from context
Initialization: Sets up RetrieveContextTool

**Attributes:**

* `r2r_client` (Arc<dyn R2rClient>): Purpose - Stores r2r_client data. Constraints - Valid Arc<dyn R2rClient>.

**Public Methods:**

None.

**Private Methods:**

* `call(params (Value)) -> anyhow::Result<CallToolResult>`: Internal helper logic.
* `description() -> String`: Internal helper logic.
* `input_schema() -> Value`: Internal helper logic.
* `name() -> String`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class ManualMockR2rClient {
    -map_stacktrace_to_ast(query: &str) anyhow::Result<String>
    -push_osr_metric(_metric: &factory_core::OsrMetric) anyhow::Result<()>
    -search(_query: &str) anyhow::Result<String>
}
R2rClient <|-- ManualMockR2rClient : extends/implements
class RetrieveContextTool {
    -call(params: Value) anyhow::Result<CallToolResult>
    -description() String
    -input_schema() Value
    -name() String
    +new(r2r_client: Arc<dyn R2rClient>) Self
}
Tool <|-- RetrieveContextTool : extends/implements
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
component "retrieve_context" as Main
component "async_trait::async_trait" as async_trait__async_trait
Main --> async_trait__async_trait : uses
component "crate::protocol::{CallToolResult, McpContent}" as crate__protocol___CallToolResult__McpContent_
Main --> crate__protocol___CallToolResult__McpContent_ : uses
component "crate::tools::Tool" as crate__tools__Tool
Main --> crate__tools__Tool : uses
component "factory_infrastructure::R2rClient" as factory_infrastructure__R2rClient
Main --> factory_infrastructure__R2rClient : uses
component "serde_json::{json, Value}" as serde_json___json__Value_
Main --> serde_json___json__Value_ : uses
component "std::sync::Arc" as std__sync__Arc
Main --> std__sync__Arc : uses
component "super::*" as super___
Main --> super___ : uses
@enduml

```

## Dependency Graph

```plantuml
@startuml
[retrieve_context]
[retrieve_context] --> [async_trait::async_trait]
[retrieve_context] --> [crate::protocol::{CallToolResult, McpContent}]
[retrieve_context] --> [crate::tools::Tool]
[retrieve_context] --> [factory_infrastructure::R2rClient]
[retrieve_context] --> [serde_json::{json, Value}]
[retrieve_context] --> [std::sync::Arc]
[retrieve_context] --> [super::*]
@enduml

```

## Call Graph

```plantuml
@startuml
Caller --> RetrieveContextTool::new
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "Retrieve_contextService" as Svc
Caller -> Svc: map_stacktrace_to_ast()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

## Examples

```
// Example usage of retrieve_context.rs components
import { ... } from 'crates/factory-mcp-server/src/tools/retrieve_context.rs';
```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** async_trait::async_trait, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, factory_infrastructure::R2rClient, serde_json::{json, Value}, std::sync::Arc, super::*
