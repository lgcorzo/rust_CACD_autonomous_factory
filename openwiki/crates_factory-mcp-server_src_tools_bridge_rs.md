---
type: "module-documentation"
title: "bridge.rs"
source_path: "crates/factory-mcp-server/src/tools/bridge.rs"
description: "Detailed documentation for bridge.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-06T17:55:58Z"
---

# File: bridge.rs

**Source Path:** `crates/factory-mcp-server/src/tools/bridge.rs`

## Overview

### Purpose
Provides implementation for bridge.rs.

### Responsibilities
* Handles logic related to bridge.

### Dependencies
* async_trait::async_trait, serde_json::{json, Value}, crate::tools::Tool, std::path::PathBuf, crate::protocol::CallToolResult, std::fs

### Imported modules
*

### Exported classes
* BridgeTool

### Exported interfaces
*

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### BridgeTool

**Overview:**
Why it exists:
Provides capabilities related to BridgeTool.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

None.

**Public Methods:**

##### `load_state(mission_id: &str (Any)) -> anyhow::Result<Value>`

###### Description
Executes load_state.

###### Inputs
* `mission_id: &str`: type=Any, meaning=Input for mission_id: &str, valid values=Any valid Any, optional=No, default value=None

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

##### `save_state(mission_id: &str (Any), state: Value (Any)) -> anyhow::Result<Value>`

###### Description
Executes save_state.

###### Inputs
* `mission_id: &str`: type=Any, meaning=Input for mission_id: &str, valid values=Any valid Any, optional=No, default value=None
* `state: Value`: type=Any, meaning=Input for state: Value, valid values=Any valid Any, optional=No, default value=None

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

* `get_checkpoint_path(mission_id: &str (Any)) -> PathBuf`: Internal helper logic.
* `name() -> String`: Internal helper logic.
* `description() -> String`: Internal helper logic.
* `input_schema() -> Value`: Internal helper logic.
* `call(params: Value (Any)) -> anyhow::Result<CallToolResult>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class BridgeTool {
        -get_checkpoint_path(mission_id: &str:Any) PathBuf
        +load_state(mission_id: &str:Any) anyhow::Result<Value>
        +save_state(mission_id: &str:Any, state: Value:Any) anyhow::Result<Value>
        -name() String
        -description() String
        -input_schema() Value
        -call(params: Value:Any) anyhow::Result<CallToolResult>
    }
    Tool <|-- BridgeTool : Inheritance / Specialization

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as BridgeService
    Caller->>Svc: get_checkpoint_path()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```


## Examples

```
// Example usage of bridge.rs components
import { ... } from 'crates/factory-mcp-server/src/tools/bridge.rs';
```


## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** async_trait::async_trait, serde_json::{json, Value}, crate::tools::Tool, std::path::PathBuf, crate::protocol::CallToolResult, std::fs
