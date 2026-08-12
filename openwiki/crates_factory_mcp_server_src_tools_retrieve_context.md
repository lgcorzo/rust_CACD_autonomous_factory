---
type: "module-documentation"
title: "retrieve_context.rs"
source_path: "crates/factory-mcp-server/src/tools/retrieve_context.rs"
description: "Detailed documentation for retrieve_context.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "dfd90f5"
---

# File: retrieve_context.rs

**Source Path:** `crates/factory-mcp-server/src/tools/retrieve_context.rs`

## Overview

### Purpose
Provides implementation for retrieve_context.rs.

### Responsibilities
* Handles logic related to retrieve_context.

### Main Workflow
* Initialization and execution of retrieve_context logic.

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

* `push_osr_metric(_metric: &factory_core::OsrMetric (Any)) -> anyhow::Result<()>`: Internal helper logic.
* `search(_query: &str (Any)) -> anyhow::Result<String>`: Internal helper logic.

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

* `call(params: Value (Any)) -> anyhow::Result<CallToolResult>`: Internal helper logic.
* `description() -> String`: Internal helper logic.
* `input_schema() -> Value`: Internal helper logic.
* `name() -> String`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class ManualMockR2rClient {
        -push_osr_metric(_metric: &factory_core::OsrMetric:Any) anyhow::Result<()>
        -search(_query: &str:Any) anyhow::Result<String>
    }
    R2rClient <|-- ManualMockR2rClient : Inheritance / Specialization
    class RetrieveContextTool {
        -call(params: Value:Any) anyhow::Result<CallToolResult>
        -description() String
        -input_schema() Value
        -name() String
        +new(r2r_client: Arc<dyn R2rClient>:Any) Self
    }
    Tool <|-- RetrieveContextTool : Inheritance / Specialization

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Retrieve_contextService
    Caller->>Svc: push_osr_metric()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## UML

### Class Diagram
```plantuml
@startuml
class ManualMockR2rClient {
    -push_osr_metric(_metric: &factory_core::OsrMetric:Any) : anyhow::Result<()>
    -search(_query: &str:Any) : anyhow::Result<String>
}
R2rClient <|-- ManualMockR2rClient : Inheritance
class RetrieveContextTool {
    -call(params: Value:Any) : anyhow::Result<CallToolResult>
    -description() : String
    -input_schema() : Value
    -name() : String
    +new(r2r_client: Arc<dyn R2rClient>:Any) : Self
}
Tool <|-- RetrieveContextTool : Inheritance
@enduml

```

### Package Diagram
```plantuml
@startuml
package "retrieve_context" {
  [Module Components]
}
@enduml

```

### Sequence Diagram
```plantuml
@startuml
autonumber
participant Caller as "Client Interface"
participant Svc as "Retrieve_contextService"
Caller -> Svc: execute()
note over Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

### Component Diagram
```plantuml
@startuml
component "retrieve_context" as comp
component "async_trait::async_trait" as async_trait::async_trait
comp --> async_trait::async_trait
component "crate::protocol::{CallToolResult, McpContent}" as crate::protocol::{CallToolResult, McpContent}
comp --> crate::protocol::{CallToolResult, McpContent}
component "crate::tools::Tool" as crate::tools::Tool
comp --> crate::tools::Tool
component "factory_infrastructure::R2rClient" as factory_infrastructure::R2rClient
comp --> factory_infrastructure::R2rClient
component "serde_json::{json, Value}" as serde_json::{json, Value}
comp --> serde_json::{json, Value}
component "std::sync::Arc" as std::sync::Arc
comp --> std::sync::Arc
component "super::*" as super::*
comp --> super::*
@enduml

```

### Dependency Graph
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

### Call Graph
```plantuml
@startuml
[API] --> RetrieveContextTool::new
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
