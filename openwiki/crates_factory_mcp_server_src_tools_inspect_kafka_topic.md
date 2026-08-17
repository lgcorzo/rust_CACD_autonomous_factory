---
type: "module-documentation"
title: "inspect_kafka_topic.rs"
source_path: "crates/factory-mcp-server/src/tools/inspect_kafka_topic.rs"
description: "Detailed documentation for inspect_kafka_topic.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "198b215"
---

# File: inspect_kafka_topic.rs

**Source Path:** `crates/factory-mcp-server/src/tools/inspect_kafka_topic.rs`

## Overview

### Purpose
Provides implementation for inspect_kafka_topic.rs.

### Responsibilities
* Handles logic related to inspect_kafka_topic.

### Dependencies
* async_trait::async_trait, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, serde_json::{json, Value}, std::env, super::*

### Imported modules
* None

### Exported classes
* InspectKafkaTopicTool

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### InspectKafkaTopicTool

**Overview:**
No description provided.

**Constructor:**

##### `new()`
Parameters:
Dependencies: Inherited from context
Initialization: Sets up InspectKafkaTopicTool

**Attributes:**

None.

**Public Methods:**

None.

**Private Methods:**

* `call(params: Value (Any)) -> anyhow::Result<CallToolResult>`: Internal helper logic.
* `default() -> Self`: Internal helper logic.
* `description() -> String`: Internal helper logic.
* `input_schema() -> Value`: Internal helper logic.
* `name() -> String`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class InspectKafkaTopicTool {
    -call(params: Value:Any) : anyhow::Result<CallToolResult>
    -default() : Self
    -description() : String
    -input_schema() : Value
    -name() : String
    +new() : Self
}
Default <|-- InspectKafkaTopicTool : extends/implements
Tool <|-- InspectKafkaTopicTool : extends/implements
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "Inspect_kafka_topicService" as Svc
Caller -> Svc: call()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```



## Examples

```
// Example usage of inspect_kafka_topic.rs components
import { ... } from 'crates/factory-mcp-server/src/tools/inspect_kafka_topic.rs';
```



## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** async_trait::async_trait, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, serde_json::{json, Value}, std::env, super::*
