---
type: "module-documentation"
title: "rustant.rs"
source_path: "crates/factory-application/src/agents/rustant.rs"
description: "Detailed documentation for rustant.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "dfd90f5"
---

# File: rustant.rs

**Source Path:** `crates/factory-application/src/agents/rustant.rs`

## Overview

### Purpose
Provides implementation for rustant.rs.

### Responsibilities
* Handles logic related to rustant.

### Dependencies
* async_trait::async_trait, crate::Agent, factory_infrastructure::{McpClient, R2rClient}, serde_json::{Value, json}, std::sync::Arc

### Imported modules
* None

### Exported classes
* RustantAgent

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### RustantAgent

**Overview:**
Why it exists:
Provides capabilities related to RustantAgent.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new(mcp_client: Arc<dyn McpClient> (Any), r2r_client: Arc<dyn R2rClient> (Any))`
Parameters: mcp_client: Arc<dyn McpClient> (Any), r2r_client: Arc<dyn R2rClient> (Any)
Dependencies: Inherited from context
Initialization: Sets up RustantAgent

**Attributes:**

* `mcp_client` (Arc<dyn McpClient>): Purpose - Stores mcp_client data. Constraints - Valid Arc<dyn McpClient>.
* `r2r_client` (Arc<dyn R2rClient>): Purpose - Stores r2r_client data. Constraints - Valid Arc<dyn R2rClient>.

**Public Methods:**

##### `plan_mission(mission_id: &str (Any), goal: &str (Any)) -> anyhow::Result<Value>`

###### Description
Executes plan_mission.

###### Inputs
* `mission_id: &str`: type=Any, meaning=Input for mission_id: &str, valid values=Any valid Any, optional=No, default value=None
* `goal: &str`: type=Any, meaning=Input for goal: &str, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: anyhow::Result<Value>
Semantic meaning: Result of plan_mission
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
let result = instance.plan_mission();
```

##### `review_mission(mission_id: &str (Any), mission_results: &str (Any)) -> anyhow::Result<Value>`

###### Description
Executes review_mission.

###### Inputs
* `mission_id: &str`: type=Any, meaning=Input for mission_id: &str, valid values=Any valid Any, optional=No, default value=None
* `mission_results: &str`: type=Any, meaning=Input for mission_results: &str, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: anyhow::Result<Value>
Semantic meaning: Result of review_mission
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
let result = instance.review_mission();
```

**Private Methods:**

* `execute(task_description: &str (Any)) -> anyhow::Result<Value>`: Internal helper logic.
* `name() -> String`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class RustantAgent {
        -execute(task_description: &str:Any) anyhow::Result<Value>
        -name() String
        +new(mcp_client: Arc<dyn McpClient>:Any, r2r_client: Arc<dyn R2rClient>:Any) Self
        +plan_mission(mission_id: &str:Any, goal: &str:Any) anyhow::Result<Value>
        +review_mission(mission_id: &str:Any, mission_results: &str:Any) anyhow::Result<Value>
    }
    Agent <|-- RustantAgent : Inheritance / Specialization

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as RustantService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```



## Examples

```
// Example usage of rustant.rs components
import { ... } from 'crates/factory-application/src/agents/rustant.rs';
```



## Cross References
* **Parent module:** `crates/factory-application/src/agents`
* **Dependencies:** async_trait::async_trait, crate::Agent, factory_infrastructure::{McpClient, R2rClient}, serde_json::{Value, json}, std::sync::Arc
