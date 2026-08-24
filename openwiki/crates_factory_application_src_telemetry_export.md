---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "telemetry_export.rs"
source_path: "crates/factory-application/src/telemetry_export.rs"
description: "Detailed documentation for telemetry_export.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-24T05:31:17Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "d073f5f"
---

# File: telemetry_export.rs

**Source Path:** `crates/factory-application/src/telemetry_export.rs`

## Overview

### Purpose
Provides implementation for telemetry_export.rs.

### Responsibilities
* Handles logic related to telemetry_export.

### Dependencies
* rdkafka::Message, rdkafka::consumer::{Consumer, StreamConsumer}, reqwest::Client, serde_json::Value, std::sync::Arc

### Imported modules
* None

### Exported classes
* TelemetryExporter

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### TelemetryExporter

**Overview:**
No description provided.

**Constructor:**

##### `new(kafka_brokers (String), openwebui_db_url (String))`
Parameters: kafka_brokers (String), openwebui_db_url (String)
Dependencies: Inherited from context
Initialization: Sets up TelemetryExporter

**Attributes:**

* `http_client` (Client): Purpose - Stores http_client data. Constraints - Valid Client.
* `kafka_brokers` (String): Purpose - Stores kafka_brokers data. Constraints - Valid String.
* `openwebui_db_url` (String): Purpose - Stores openwebui_db_url data. Constraints - Valid String.

**Public Methods:**

##### `start_export_loop(self (Arc<Self>)) -> anyhow::Result<()>`

###### Description
/// Starts a background task consuming `agent-thought` from Kafka and exporting to OpenWebUI.

###### Inputs
* `self`: type=Arc<Self>, meaning=Input for self, valid values=Any valid Arc<Self>, optional=No, default value=None

###### Output
Return type: anyhow::Result<()>
Semantic meaning: Result of start_export_loop
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
let result = instance.start_export_loop();
```

**Private Methods:**

* `push_to_openwebui(thought (&Value)) -> anyhow::Result<()>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class TelemetryExporter {
    +new(kafka_brokers: String, openwebui_db_url: String) Self
    -push_to_openwebui(thought: &Value) anyhow::Result<()>
    +start_export_loop(self: Arc<Self>) anyhow::Result<()>
}
@enduml

```

## Package Diagram

```plantuml
@startuml
package "crates" {
    package "factory-application" {
        package "src" {
            class Module
        }
    }
}
@enduml

```

## Component Diagram

```plantuml
@startuml
component "telemetry_export" as Main
component "rdkafka::Message" as rdkafka__Message
Main --> rdkafka__Message : uses
component "rdkafka::consumer::{Consumer, StreamConsumer}" as rdkafka__consumer___Consumer__StreamConsumer_
Main --> rdkafka__consumer___Consumer__StreamConsumer_ : uses
component "reqwest::Client" as reqwest__Client
Main --> reqwest__Client : uses
component "serde_json::Value" as serde_json__Value
Main --> serde_json__Value : uses
component "std::sync::Arc" as std__sync__Arc
Main --> std__sync__Arc : uses
@enduml

```

## Dependency Graph

```plantuml
@startuml
[telemetry_export]
[telemetry_export] --> [rdkafka::Message]
[telemetry_export] --> [rdkafka::consumer::{Consumer, StreamConsumer}]
[telemetry_export] --> [reqwest::Client]
[telemetry_export] --> [serde_json::Value]
[telemetry_export] --> [std::sync::Arc]
@enduml

```

## Call Graph

```plantuml
@startuml
Caller --> TelemetryExporter::new
Caller --> TelemetryExporter::start_export_loop
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "Telemetry_exportService" as Svc
Caller -> Svc: new()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

## Examples

```
// Example usage of telemetry_export.rs components
import { ... } from 'crates/factory-application/src/telemetry_export.rs';
```

## Cross References
* **Parent module:** `crates/factory-application/src`
* **Dependencies:** rdkafka::Message, rdkafka::consumer::{Consumer, StreamConsumer}, reqwest::Client, serde_json::Value, std::sync::Arc
