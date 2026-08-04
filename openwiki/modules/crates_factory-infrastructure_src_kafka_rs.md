---
type: "module-documentation"
title: "kafka.rs"
source_path: "crates/factory-infrastructure/src/kafka.rs"
description: "Detailed documentation for kafka.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: kafka.rs

**Source Path:** `crates/factory-infrastructure/src/kafka.rs`

## Overview

### Purpose
Provides implementation for kafka.rs.

### Responsibilities
* Handles logic related to kafka.

### Dependencies
* rdkafka::producer::{FutureProducer, FutureRecord}, rdkafka::config::ClientConfig, async_trait::async_trait, chrono::Utc, std::time::Duration

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### KafkaClient

**Overview:** Represents KafkaClient.

**Public Methods:**

None.

#### RdKafkaClient

**Overview:** Represents RdKafkaClient.

**Public Methods:**

##### `new(brokers: &str (Any)) -> anyhow::Result<Self>`
Executes new.

#### SimpleMockKafkaClient

**Overview:** Represents SimpleMockKafkaClient.

**Public Methods:**

##### `new(_brokers: &str (Any)) -> anyhow::Result<Self>`
Executes new.

### Exported Functions

None.

## Internal Architecture & Execution Flow

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

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as KafkaService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-infrastructure/src`
* **Dependencies:** rdkafka::producer::{FutureProducer, FutureRecord}, rdkafka::config::ClientConfig, async_trait::async_trait, chrono::Utc, std::time::Duration
