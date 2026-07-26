---
type: module
title: "kafka.rs"
source_path: "crates/factory-infrastructure/src/kafka.rs"
description: "Documentation for crates/factory-infrastructure/src/kafka.rs"
tags: [rust, module]
last_verified_commit: "4cb40d0"
---

# kafka.rs

Source File: `crates/factory-infrastructure/src/kafka.rs`

## Component Architecture

```mermaid
classDiagram
    class RdKafkaClient
    class SimpleMockKafkaClient
    class KafkaClient {
        <<trait>>
    }
```

## Execution Flow

```mermaid
flowchart TD
    Start --> publish
    publish --> publish_thought
    publish_thought --> new
    new --> publish
    publish --> new
    new --> publish
    publish --> End
```
