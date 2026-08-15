---
type: "module-documentation"
title: "autonomous_mission.rs"
source_path: "crates/factory-application/src/workflows/autonomous_mission.rs"
description: "Detailed documentation for autonomous_mission.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "9c1db1c"
---

# File: autonomous_mission.rs

**Source Path:** `crates/factory-application/src/workflows/autonomous_mission.rs`

## Overview

### Purpose
Provides implementation for autonomous_mission.rs.

### Responsibilities
* Handles logic related to autonomous_mission.

### Dependencies
* crate::agents::{AuditorAgent, FinOpsAgent, RustantAgent, ZeroClawAgent}, factory_core::proto::v1::MissionInput as ProtoInput, factory_infrastructure::{
    HttpR2rClient, KafkaClient, McpClient, McpHttpClient, R2rClient,
    aethalgard::{AethalgardClient, HttpAethalgardClient},
}, hatchet_sdk::Hatchet, hatchet_sdk::runnables::Workflow, prost::Message, serde::{Deserialize, Serialize}, std::hash::{Hash, Hasher}, std::sync::Arc, super::*, uuid::Uuid

### Imported modules
* None

### Exported classes
* MissionInput, MissionOutput

### Exported interfaces
* None

### Exported functions
* create_mission_workflow

## Public API

### Exported Classes / Structs / Interfaces

#### MissionInput

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

* `goal` (String): Purpose - Stores goal data. Constraints - Valid String.
* `mission_id` (Option<String>): Purpose - Stores mission_id data. Constraints - Valid Option<String>.
* `repository_path` (String): Purpose - Stores repository_path data. Constraints - Valid String.

**Public Methods:**

##### `from_protobuf(bytes: &[u8] (Any)) -> Result<Self, prost::DecodeError>`

###### Description
No description provided.

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
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

* `mission_id` (String): Purpose - Stores mission_id data. Constraints - Valid String.
* `pr_url` (Option<String>): Purpose - Stores pr_url data. Constraints - Valid Option<String>.
* `status` (String): Purpose - Stores status data. Constraints - Valid String.
* `summary` (String): Purpose - Stores summary data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

None.

### Exported Functions

#### `create_mission_workflow(hatchet: &Hatchet (Any), mcp_url: String (Any), r2r_url: String (Any), kafka_brokers: String (Any), aethalgard_webhook_url: String (Any)) -> Workflow<MissionInput, MissionOutput>`
No description provided.

## Internal architecture

```plantuml
@startuml
class MissionInput {
    +from_protobuf(bytes: &[u8]:Any) : Result<Self, prost::DecodeError>
}
class MissionOutput {
}
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "Autonomous_missionService" as Svc
Caller -> Svc: from_protobuf()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```


## Examples

```
// Example usage of autonomous_mission.rs components
import { ... } from 'crates/factory-application/src/workflows/autonomous_mission.rs';
```


## Cross References
* **Parent module:** `crates/factory-application/src/workflows`
* **Dependencies:** crate::agents::{AuditorAgent, FinOpsAgent, RustantAgent, ZeroClawAgent}, factory_core::proto::v1::MissionInput as ProtoInput, factory_infrastructure::{
    HttpR2rClient, KafkaClient, McpClient, McpHttpClient, R2rClient,
    aethalgard::{AethalgardClient, HttpAethalgardClient},
}, hatchet_sdk::Hatchet, hatchet_sdk::runnables::Workflow, prost::Message, serde::{Deserialize, Serialize}, std::hash::{Hash, Hasher}, std::sync::Arc, super::*, uuid::Uuid
