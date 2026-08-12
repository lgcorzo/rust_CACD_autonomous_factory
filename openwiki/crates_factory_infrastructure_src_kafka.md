---
type: "module-documentation"
title: "kafka.rs"
source_path: "crates/factory-infrastructure/src/kafka.rs"
description: "Detailed documentation for kafka.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "dfd90f5"
---

# File: kafka.rs

**Source Path:** `crates/factory-infrastructure/src/kafka.rs`

## Overview

### Purpose
Provides implementation for kafka.rs.

### Responsibilities
* Handles logic related to kafka.

### Main Workflow
* Initialization and execution of kafka logic.

### Dependencies
* async_trait::async_trait, chrono::Utc, rdkafka::config::ClientConfig, rdkafka::producer::{FutureProducer, FutureRecord}, std::time::Duration

### Imported modules
* None

### Exported classes
* RdKafkaClient, SimpleMockKafkaClient

### Exported interfaces
* KafkaClient

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### KafkaClient

**Overview:**
Why it exists:
Provides capabilities related to KafkaClient.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

None.

**Public Methods:**

None.

**Private Methods:**

None.

#### RdKafkaClient

**Overview:**
Why it exists:
Provides capabilities related to RdKafkaClient.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new(brokers: &str (Any))`
Parameters: brokers: &str (Any)
Dependencies: Inherited from context
Initialization: Sets up RdKafkaClient

**Attributes:**

* `producer` (FutureProducer): Purpose - Stores producer data. Constraints - Valid FutureProducer.

**Public Methods:**

None.

**Private Methods:**

* `publish(topic: &str (Any), key: &str (Any), payload: &[u8] (Any)) -> anyhow::Result<()>`: Internal helper logic.

#### SimpleMockKafkaClient

**Overview:**
Why it exists:
Provides capabilities related to SimpleMockKafkaClient.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new(_brokers: &str (Any))`
Parameters: _brokers: &str (Any)
Dependencies: Inherited from context
Initialization: Sets up SimpleMockKafkaClient

**Attributes:**

None.

**Public Methods:**

None.

**Private Methods:**

* `publish(topic: &str (Any), key: &str (Any), payload: &[u8] (Any)) -> anyhow::Result<()>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class KafkaClient {
        <<trait>>
    }
    class RdKafkaClient {
        +new(brokers: &str:Any) anyhow::Result<Self>
        -publish(topic: &str:Any, key: &str:Any, payload: &[u8]:Any) anyhow::Result<()>
    }
    KafkaClient <|-- RdKafkaClient : Inheritance / Specialization
    class SimpleMockKafkaClient {
        +new(_brokers: &str:Any) anyhow::Result<Self>
        -publish(topic: &str:Any, key: &str:Any, payload: &[u8]:Any) anyhow::Result<()>
    }
    KafkaClient <|-- SimpleMockKafkaClient : Inheritance / Specialization

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as KafkaService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## UML

### Class Diagram
```plantuml
@startuml
interface KafkaClient <<trait>> {
}
class RdKafkaClient {
    +new(brokers: &str:Any) : anyhow::Result<Self>
    -publish(topic: &str:Any, key: &str:Any, payload: &[u8]:Any) : anyhow::Result<()>
}
KafkaClient <|-- RdKafkaClient : Inheritance
class SimpleMockKafkaClient {
    +new(_brokers: &str:Any) : anyhow::Result<Self>
    -publish(topic: &str:Any, key: &str:Any, payload: &[u8]:Any) : anyhow::Result<()>
}
KafkaClient <|-- SimpleMockKafkaClient : Inheritance
@enduml

```

### Package Diagram
```plantuml
@startuml
package "kafka" {
  [Module Components]
}
@enduml

```

### Sequence Diagram
```plantuml
@startuml
autonumber
participant Caller as "Client Interface"
participant Svc as "KafkaService"
Caller -> Svc: execute()
note over Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

### Component Diagram
```plantuml
@startuml
component "kafka" as comp
component "async_trait::async_trait" as async_trait::async_trait
comp --> async_trait::async_trait
component "chrono::Utc" as chrono::Utc
comp --> chrono::Utc
component "rdkafka::config::ClientConfig" as rdkafka::config::ClientConfig
comp --> rdkafka::config::ClientConfig
component "rdkafka::producer::{FutureProducer, FutureRecord}" as rdkafka::producer::{FutureProducer, FutureRecord}
comp --> rdkafka::producer::{FutureProducer, FutureRecord}
component "std::time::Duration" as std::time::Duration
comp --> std::time::Duration
@enduml

```

### Dependency Graph
```plantuml
@startuml
[kafka]
[kafka] --> [async_trait::async_trait]
[kafka] --> [chrono::Utc]
[kafka] --> [rdkafka::config::ClientConfig]
[kafka] --> [rdkafka::producer::{FutureProducer, FutureRecord}]
[kafka] --> [std::time::Duration]
@enduml

```

### Call Graph
```plantuml
@startuml
[API] --> RdKafkaClient::new
[API] --> SimpleMockKafkaClient::new
@enduml

```

## Examples

```
// Example usage of kafka.rs components
import { ... } from 'crates/factory-infrastructure/src/kafka.rs';
```

## Cross References
* **Parent module:** `crates/factory-infrastructure/src`
* **Dependencies:** async_trait::async_trait, chrono::Utc, rdkafka::config::ClientConfig, rdkafka::producer::{FutureProducer, FutureRecord}, std::time::Duration
