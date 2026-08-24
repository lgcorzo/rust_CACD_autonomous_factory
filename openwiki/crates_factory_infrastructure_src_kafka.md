---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "kafka.rs"
source_path: "crates/factory-infrastructure/src/kafka.rs"
description: "Detailed documentation for kafka.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-24T05:31:17Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "d073f5f"
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

##### `new(brokers (&str))`
Parameters: brokers (&str)
Dependencies: Inherited from context
Initialization: Sets up RdKafkaClient

**Attributes:**

* `producer` (FutureProducer): Purpose - Stores producer data. Constraints - Valid FutureProducer.

**Public Methods:**

None.

**Private Methods:**

* `publish(topic (&str), key (&str), payload (&[u8])) -> anyhow::Result<()>`: Internal helper logic.

#### SimpleMockKafkaClient

**Overview:**
No description provided.

**Constructor:**

##### `new(_brokers (&str))`
Parameters: _brokers (&str)
Dependencies: Inherited from context
Initialization: Sets up SimpleMockKafkaClient

**Attributes:**

None.

**Public Methods:**

None.

**Private Methods:**

* `publish(topic (&str), key (&str), payload (&[u8])) -> anyhow::Result<()>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
interface KafkaClient {
}
class RdKafkaClient {
    +new(brokers: &str) anyhow::Result<Self>
    -publish(topic: &str, key: &str, payload: &[u8]) anyhow::Result<()>
}
KafkaClient <|-- RdKafkaClient : extends/implements
class SimpleMockKafkaClient {
    +new(_brokers: &str) anyhow::Result<Self>
    -publish(topic: &str, key: &str, payload: &[u8]) anyhow::Result<()>
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
