---
type: "module-documentation"
title: "kafka.rs"
source_path: "crates/factory-infrastructure/src/kafka.rs"
description: "Detailed documentation for kafka.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "ec7bd0f"
---

# File: kafka.rs

**Source Path:** `crates/factory-infrastructure/src/kafka.rs`

## Overview

### Purpose
Provides implementation for kafka.rs.

### Responsibilities
* Handles logic related to kafka.

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
No description provided.

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
No description provided.

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
No description provided.

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

```plantuml
@startuml
interface KafkaClient {
}
class RdKafkaClient {
    +new(brokers: &str:Any) : anyhow::Result<Self>
    -publish(topic: &str:Any, key: &str:Any, payload: &[u8]:Any) : anyhow::Result<()>
}
KafkaClient <|-- RdKafkaClient : extends/implements
class SimpleMockKafkaClient {
    +new(_brokers: &str:Any) : anyhow::Result<Self>
    -publish(topic: &str:Any, key: &str:Any, payload: &[u8]:Any) : anyhow::Result<()>
}
KafkaClient <|-- SimpleMockKafkaClient : extends/implements
@enduml

```

## Package Diagram

```plantuml
@startuml
package "crates" {
    package "factory-infrastructure" {
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
component "kafka" as Main
component "async_trait::async_trait" as async_trait__async_trait
Main --> async_trait__async_trait : uses
component "chrono::Utc" as chrono__Utc
Main --> chrono__Utc : uses
component "rdkafka::config::ClientConfig" as rdkafka__config__ClientConfig
Main --> rdkafka__config__ClientConfig : uses
component "rdkafka::producer::{FutureProducer, FutureRecord}" as rdkafka__producer___FutureProducer__FutureRecord_
Main --> rdkafka__producer___FutureProducer__FutureRecord_ : uses
component "std::time::Duration" as std__time__Duration
Main --> std__time__Duration : uses
@enduml

```

## Dependency Graph

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

## Call Graph

```plantuml
@startuml
Caller --> RdKafkaClient::new
Caller --> SimpleMockKafkaClient::new
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "KafkaService" as Svc
Caller -> Svc: execute()
note right of Svc: Processing internal logic
Svc --> Caller: result
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
