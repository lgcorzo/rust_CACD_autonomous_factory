---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "update_mission_status.rs"
source_path: "crates/factory-mcp-server/src/tools/update_mission_status.rs"
description: "Detailed documentation for update_mission_status.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-25T05:53:44Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "e2707de"
---

# File: update_mission_status.rs

**Source Path:** `crates/factory-mcp-server/src/tools/update_mission_status.rs`

## Overview

### Purpose
Provides implementation for update_mission_status.rs.

### Responsibilities
* Handles logic related to update_mission_status.

### Dependencies
* async_trait::async_trait, chrono::Local, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, serde_json::{json, Value}, super::*, tokio::fs::{File, OpenOptions}, tokio::io::AsyncWriteExt

### Imported modules
* None

### Exported classes
* UpdateMissionStatusTool

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### UpdateMissionStatusTool

**Overview:**
No description provided.

**Constructor:**

##### `new(docs_path (String))`
Parameters: docs_path (String)
Dependencies: Inherited from context
Initialization: Sets up UpdateMissionStatusTool

**Attributes:**

* `docs_path` (String): Purpose - Stores docs_path data. Constraints - Valid String.

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
class UpdateMissionStatusTool {
    -call(params: Value) anyhow::Result<CallToolResult>
    -description() String
    -input_schema() Value
    -name() String
    +new(docs_path: String) Self
}
Tool <|-- UpdateMissionStatusTool : extends/implements
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
component "update_mission_status" as Main
component "async_trait::async_trait" as async_trait__async_trait
Main --> async_trait__async_trait : uses
component "chrono::Local" as chrono__Local
Main --> chrono__Local : uses
component "crate::protocol::{CallToolResult, McpContent}" as crate__protocol___CallToolResult__McpContent_
Main --> crate__protocol___CallToolResult__McpContent_ : uses
component "crate::tools::Tool" as crate__tools__Tool
Main --> crate__tools__Tool : uses
component "serde_json::{json, Value}" as serde_json___json__Value_
Main --> serde_json___json__Value_ : uses
component "super::*" as super___
Main --> super___ : uses
component "tokio::fs::{File, OpenOptions}" as tokio__fs___File__OpenOptions_
Main --> tokio__fs___File__OpenOptions_ : uses
component "tokio::io::AsyncWriteExt" as tokio__io__AsyncWriteExt
Main --> tokio__io__AsyncWriteExt : uses
@enduml

```

## Dependency Graph

```plantuml
@startuml
[update_mission_status]
[update_mission_status] --> [async_trait::async_trait]
[update_mission_status] --> [chrono::Local]
[update_mission_status] --> [crate::protocol::{CallToolResult, McpContent}]
[update_mission_status] --> [crate::tools::Tool]
[update_mission_status] --> [serde_json::{json, Value}]
[update_mission_status] --> [super::*]
[update_mission_status] --> [tokio::fs::{File, OpenOptions}]
[update_mission_status] --> [tokio::io::AsyncWriteExt]
@enduml

```

## Call Graph

```plantuml
@startuml
Caller --> UpdateMissionStatusTool::new
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "Update_mission_statusService" as Svc
Caller -> Svc: call()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

## Examples

```
// Example usage of update_mission_status.rs components
import { ... } from 'crates/factory-mcp-server/src/tools/update_mission_status.rs';
```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** async_trait::async_trait, chrono::Local, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, serde_json::{json, Value}, super::*, tokio::fs::{File, OpenOptions}, tokio::io::AsyncWriteExt
