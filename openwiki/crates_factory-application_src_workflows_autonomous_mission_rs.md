---
type: "module-documentation"
title: "autonomous_mission.rs"
source_path: "crates/factory-application/src/workflows/autonomous_mission.rs"
description: "Detailed documentation for autonomous_mission.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-06T17:55:58Z"
---

# File: autonomous_mission.rs

**Source Path:** `crates/factory-application/src/workflows/autonomous_mission.rs`

## Overview

### Purpose
Provides implementation for autonomous_mission.rs.

### Responsibilities
* Handles logic related to autonomous_mission.

### Dependencies
* hatchet_sdk::Hatchet, serde::{Deserialize, Serialize}, factory_infrastructure::{
    HttpR2rClient, KafkaClient, McpClient, McpHttpClient, R2rClient,
    aethalgard::{AethalgardClient, HttpAethalgardClient},
}, std::sync::Arc, crate::agents::{AuditorAgent, FinOpsAgent, RustantAgent, ZeroClawAgent}, uuid::Uuid, hatchet_sdk::runnables::Workflow, super::*, factory_core::proto::v1::MissionInput as ProtoInput, prost::Message

### Imported modules
*

### Exported classes
* MissionInput, MissionOutput

### Exported interfaces
*

### Exported functions
* create_mission_workflow

## Public API

### Exported Classes / Structs / Interfaces

#### MissionInput

**Overview:**
Why it exists:
Provides capabilities related to MissionInput.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `mission_id` (Option<String>): Purpose - Stores mission_id data. Constraints - Valid Option<String>.
* `goal` (String): Purpose - Stores goal data. Constraints - Valid String.
* `repository_path` (String): Purpose - Stores repository_path data. Constraints - Valid String.

**Public Methods:**

##### `from_protobuf(bytes: &[u8] (Any)) -> Result<Self, prost::DecodeError>`

###### Description
Executes from_protobuf.

###### Inputs
* `bytes: &[u8]`: type=Any, meaning=Input for bytes: &[u8], valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: Result<Self, prost::DecodeError>
Semantic meaning: Result of from_protobuf
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
let result = instance.from_protobuf();
```

**Private Methods:**

None.

#### MissionOutput

**Overview:**
Why it exists:
Provides capabilities related to MissionOutput.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `mission_id` (String): Purpose - Stores mission_id data. Constraints - Valid String.
* `status` (String): Purpose - Stores status data. Constraints - Valid String.
* `summary` (String): Purpose - Stores summary data. Constraints - Valid String.
* `pr_url` (Option<String>): Purpose - Stores pr_url data. Constraints - Valid Option<String>.

**Public Methods:**

None.

**Private Methods:**

None.

### Exported Functions

#### `create_mission_workflow(hatchet: &Hatchet (Any), mcp_url: String (Any), r2r_url: String (Any), kafka_brokers: String (Any), aethalgard_webhook_url: String (Any)) -> Workflow<MissionInput, MissionOutput>`
Executes create_mission_workflow.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class MissionInput {
        +from_protobuf(bytes: &[u8]:Any) Result<Self, prost::DecodeError>
    }
    class MissionOutput {
    }

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Autonomous_missionService
    Caller->>Svc: from_protobuf()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```


## Examples

```
// Example usage of autonomous_mission.rs components
import { ... } from 'crates/factory-application/src/workflows/autonomous_mission.rs';
```


## Cross References
* **Parent module:** `crates/factory-application/src/workflows`
* **Dependencies:** hatchet_sdk::Hatchet, serde::{Deserialize, Serialize}, factory_infrastructure::{
    HttpR2rClient, KafkaClient, McpClient, McpHttpClient, R2rClient,
    aethalgard::{AethalgardClient, HttpAethalgardClient},
}, std::sync::Arc, crate::agents::{AuditorAgent, FinOpsAgent, RustantAgent, ZeroClawAgent}, uuid::Uuid, hatchet_sdk::runnables::Workflow, super::*, factory_core::proto::v1::MissionInput as ProtoInput, prost::Message
