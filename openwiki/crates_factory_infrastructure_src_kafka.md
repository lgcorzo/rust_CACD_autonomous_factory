---
type: "module-documentation"
title: "kafka.rs"
source_path: "crates/factory-infrastructure/src/kafka.rs"
description: "Detailed documentation for kafka.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "7982a81"
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

```plantuml
@startuml
interface KafkaClient {
}
class RdKafkaClient {
    +new(brokers: &str:Any) : anyhow::Result<Self>
    -publish(topic: &str:Any, key: &str:Any, payload: &[u8]:Any) : anyhow::Result<()>
}
KafkaClient <|-- RdKafkaClient : Inheritance / Specialization
class SimpleMockKafkaClient {
    +new(_brokers: &str:Any) : anyhow::Result<Self>
    -publish(topic: &str:Any, key: &str:Any, payload: &[u8]:Any) : anyhow::Result<()>
}
KafkaClient <|-- SimpleMockKafkaClient : Inheritance / Specialization
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant Caller as "Client Interface"
participant Svc as "KafkaService"
Caller -> Svc : execute()
note over Svc : Processing internal logic
Svc --> Caller : result
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
