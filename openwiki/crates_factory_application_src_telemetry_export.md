---
type: "module-documentation"
title: "telemetry_export.rs"
source_path: "crates/factory-application/src/telemetry_export.rs"
description: "Detailed documentation for telemetry_export.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "dfd90f5"
---

# File: telemetry_export.rs

**Source Path:** `crates/factory-application/src/telemetry_export.rs`

## Overview

### Purpose
Provides implementation for telemetry_export.rs.

### Responsibilities
* Handles logic related to telemetry_export.

### Main Workflow
* Initialization and execution of telemetry_export logic.

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
Why it exists:
Provides capabilities related to TelemetryExporter.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new(kafka_brokers: String (Any), openwebui_db_url: String (Any))`
Parameters: kafka_brokers: String (Any), openwebui_db_url: String (Any)
Dependencies: Inherited from context
Initialization: Sets up TelemetryExporter

**Attributes:**

* `http_client` (Client): Purpose - Stores http_client data. Constraints - Valid Client.
* `kafka_brokers` (String): Purpose - Stores kafka_brokers data. Constraints - Valid String.
* `openwebui_db_url` (String): Purpose - Stores openwebui_db_url data. Constraints - Valid String.

**Public Methods:**

##### `start_export_loop(self: Arc<Self> (Any)) -> anyhow::Result<()>`

###### Description
/// Starts a background task consuming `agent-thought` from Kafka and exporting to OpenWebUI.

###### Inputs
* `self: Arc<Self>`: type=Any, meaning=Input for self: Arc<Self>, valid values=Any valid Any, optional=No, default value=None

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

* `push_to_openwebui(thought: &Value (Any)) -> anyhow::Result<()>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class TelemetryExporter {
        +new(kafka_brokers: String:Any, openwebui_db_url: String:Any) Self
        -push_to_openwebui(thought: &Value:Any) anyhow::Result<()>
        +start_export_loop(self: Arc<Self>:Any) anyhow::Result<()>
    }

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Telemetry_exportService
    Caller->>Svc: new()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## UML

### Class Diagram
```plantuml
@startuml
class TelemetryExporter {
    +new(kafka_brokers: String:Any, openwebui_db_url: String:Any) : Self
    -push_to_openwebui(thought: &Value:Any) : anyhow::Result<()>
    +start_export_loop(self: Arc<Self>:Any) : anyhow::Result<()>
}
@enduml

```

### Package Diagram
```plantuml
@startuml
package "telemetry_export" {
  [Module Components]
}
@enduml

```

### Sequence Diagram
```plantuml
@startuml
autonumber
participant Caller as "Client Interface"
participant Svc as "Telemetry_exportService"
Caller -> Svc: new()
note over Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

### Component Diagram
```plantuml
@startuml
component "telemetry_export" as comp
component "rdkafka::Message" as rdkafka::Message
comp --> rdkafka::Message
component "rdkafka::consumer::{Consumer, StreamConsumer}" as rdkafka::consumer::{Consumer, StreamConsumer}
comp --> rdkafka::consumer::{Consumer, StreamConsumer}
component "reqwest::Client" as reqwest::Client
comp --> reqwest::Client
component "serde_json::Value" as serde_json::Value
comp --> serde_json::Value
component "std::sync::Arc" as std::sync::Arc
comp --> std::sync::Arc
@enduml

```

### Dependency Graph
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

### Call Graph
```plantuml
@startuml
[API] --> TelemetryExporter::new
[API] --> TelemetryExporter::start_export_loop
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
