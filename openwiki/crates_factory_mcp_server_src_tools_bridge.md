---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "bridge.rs"
source_path: "crates/factory-mcp-server/src/tools/bridge.rs"
description: "Detailed documentation for bridge.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-31T05:39:54Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "7bf01b8"
---

# File: bridge.rs

**Source Path:** `crates/factory-mcp-server/src/tools/bridge.rs`

## Overview

### Purpose
Provides implementation for bridge.rs.

### Responsibilities
* Handles logic related to bridge.

### Dependencies
* async_trait::async_trait, crate::protocol::CallToolResult, crate::tools::Tool, serde_json::{json, Value}, std::fs, std::path::PathBuf

### Imported modules
* None

### Exported classes
* BridgeTool

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### BridgeTool

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

None.

**Public Methods:**

##### `load_state(mission_id (&str)) -> anyhow::Result<Value>`

###### Description
No description provided.

###### Inputs
* `mission_id`: type=&str, meaning=Input for mission_id, valid values=Any valid &str, optional=No, default value=None

###### Output
Return type: anyhow::Result<Value>
Semantic meaning: Result of load_state
Possible null values: Conditional
Exceptions: None handled explicitly

###### Side Effects
Database updates: None
File operations: None
Network calls: None
Cache: None
State changes: Updates internal variables

###### Complexity
Time Complexity: O(1) mostly
Space Complexity: O(1) mostly

###### Example
```
let result = instance.load_state();
```

##### `save_state(mission_id (&str), state (Value)) -> anyhow::Result<Value>`

###### Description
No description provided.

###### Inputs
* `mission_id`: type=&str, meaning=Input for mission_id, valid values=Any valid &str, optional=No, default value=None
* `state`: type=Value, meaning=Input for state, valid values=Any valid Value, optional=No, default value=None

###### Output
Return type: anyhow::Result<Value>
Semantic meaning: Result of save_state
Possible null values: Conditional
Exceptions: None handled explicitly

###### Side Effects
Database updates: None
File operations: None
Network calls: None
Cache: None
State changes: Updates internal variables

###### Complexity
Time Complexity: O(1) mostly
Space Complexity: O(1) mostly

###### Example
```
let result = instance.save_state();
```

**Private Methods:**

* `call(self (Self), params (Value)) -> anyhow::Result<CallToolResult>`: Internal helper logic.
* `description(self (Self)) -> String`: Internal helper logic.
* `get_checkpoint_path(mission_id (&str)) -> PathBuf`: Internal helper logic.
* `input_schema(self (Self)) -> Value`: Internal helper logic.
* `name(self (Self)) -> String`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class BridgeTool {
    -call(self: Self, params: Value) anyhow::Result<CallToolResult>
    -description(self: Self) String
    -get_checkpoint_path(mission_id: &str) PathBuf
    -input_schema(self: Self) Value
    +load_state(mission_id: &str) anyhow::Result<Value>
    -name(self: Self) String
    +save_state(mission_id: &str, state: Value) anyhow::Result<Value>
}
Tool <|-- BridgeTool : extends/implements
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
component "bridge" as Main
component "async_trait::async_trait" as async_trait__async_trait
Main --> async_trait__async_trait : uses
component "crate::protocol::CallToolResult" as crate__protocol__CallToolResult
Main --> crate__protocol__CallToolResult : uses
component "crate::tools::Tool" as crate__tools__Tool
Main --> crate__tools__Tool : uses
component "serde_json::{json, Value}" as serde_json___json__Value_
Main --> serde_json___json__Value_ : uses
component "std::fs" as std__fs
Main --> std__fs : uses
component "std::path::PathBuf" as std__path__PathBuf
Main --> std__path__PathBuf : uses
@enduml

```

## Dependency Graph

```plantuml
@startuml
[bridge]
[bridge] --> [async_trait::async_trait]
[bridge] --> [crate::protocol::CallToolResult]
[bridge] --> [crate::tools::Tool]
[bridge] --> [serde_json::{json, Value}]
[bridge] --> [std::fs]
[bridge] --> [std::path::PathBuf]
@enduml

```

## Call Graph

```plantuml
@startuml
Caller --> BridgeTool::load_state
Caller --> BridgeTool::save_state
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "BridgeService" as Svc
Caller -> Svc: call()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

## Examples

```
// Example usage of bridge.rs components
import { ... } from 'crates/factory-mcp-server/src/tools/bridge.rs';
```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** async_trait::async_trait, crate::protocol::CallToolResult, crate::tools::Tool, serde_json::{json, Value}, std::fs, std::path::PathBuf
