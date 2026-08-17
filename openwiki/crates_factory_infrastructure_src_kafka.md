---
type: "module-documentation"
title: "kafka.rs"
source_path: "crates/factory-infrastructure/src/kafka.rs"
description: "Detailed documentation for kafka.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "198b215"
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
