---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "kafka.rs"
source_path: "crates/factory-infrastructure/src/kafka.rs"
description: "Detailed documentation for kafka.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-31T05:39:54Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "7bf01b8"
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

##### `publish(self (Self), topic (&str), key (&str), payload (&[u8])) -> anyhow::Result<()>`

###### Description
No description provided.

###### Inputs
* `self`: type=Self, meaning=Input for self, valid values=Any valid Self, optional=No, default value=None
* `topic`: type=&str, meaning=Input for topic, valid values=Any valid &str, optional=No, default value=None
* `key`: type=&str, meaning=Input for key, valid values=Any valid &str, optional=No, default value=None
* `payload`: type=&[u8], meaning=Input for payload, valid values=Any valid &[u8], optional=No, default value=None

###### Output
Return type: anyhow::Result<()>
Semantic meaning: Result of publish
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
let result = instance.publish();
```

##### `publish_thought(self (Self), mission_id (&str), thought (&str), agent (&str)) -> anyhow::Result<()>`

###### Description
No description provided.

###### Inputs
* `self`: type=Self, meaning=Input for self, valid values=Any valid Self, optional=No, default value=None
* `mission_id`: type=&str, meaning=Input for mission_id, valid values=Any valid &str, optional=No, default value=None
* `thought`: type=&str, meaning=Input for thought, valid values=Any valid &str, optional=No, default value=None
* `agent`: type=&str, meaning=Input for agent, valid values=Any valid &str, optional=No, default value=None

###### Output
Return type: anyhow::Result<()>
Semantic meaning: Result of publish_thought
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
let result = instance.publish_thought();
```

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

* `publish(self (Self), topic (&str), key (&str), payload (&[u8])) -> anyhow::Result<()>`: Internal helper logic.

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

* `publish(self (Self), topic (&str), key (&str), payload (&[u8])) -> anyhow::Result<()>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
interface KafkaClient {
    +publish(self: Self, topic: &str, key: &str, payload: &[u8]) anyhow::Result<()>
    +publish_thought(self: Self, mission_id: &str, thought: &str, agent: &str) anyhow::Result<()>
}
class RdKafkaClient {
    +new(brokers: &str) anyhow::Result<Self>
    -publish(self: Self, topic: &str, key: &str, payload: &[u8]) anyhow::Result<()>
}
KafkaClient <|-- RdKafkaClient : extends/implements
class SimpleMockKafkaClient {
    +new(_brokers: &str) anyhow::Result<Self>
    -publish(self: Self, topic: &str, key: &str, payload: &[u8]) anyhow::Result<()>
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
Caller --> KafkaClient::publish
Caller --> KafkaClient::publish_thought
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
Caller -> Svc: publish()
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
