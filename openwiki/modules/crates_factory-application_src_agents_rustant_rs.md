---
type: "module-documentation"
title: "rustant.rs"
source_path: "crates/factory-application/src/agents/rustant.rs"
description: "Detailed documentation for rustant.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: rustant.rs

**Source Path:** `crates/factory-application/src/agents/rustant.rs`

## Overview

### Purpose
Provides implementation for rustant.rs.

### Responsibilities
* Handles logic related to rustant.

### Dependencies
* crate::Agent, async_trait::async_trait, std::sync::Arc, serde_json::{Value, json}, factory_infrastructure::{McpClient, R2rClient}

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### RustantAgent

**Overview:** Represents RustantAgent.

**Public Methods:**

##### `new(mcp_client: Arc<dyn McpClient> (Any), r2r_client: Arc<dyn R2rClient> (Any)) -> Self`
Executes new.

##### `plan_mission(mission_id: &str (Any), goal: &str (Any)) -> anyhow::Result<Value>`
Executes plan_mission.

##### `review_mission(mission_id: &str (Any), mission_results: &str (Any)) -> anyhow::Result<Value>`
Executes review_mission.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class RustantAgent {
        +new(mcp_client: Arc<dyn McpClient>:Any, r2r_client: Arc<dyn R2rClient>:Any) Self
        +plan_mission(mission_id: &str:Any, goal: &str:Any) anyhow::Result<Value>
        +review_mission(mission_id: &str:Any, mission_results: &str:Any) anyhow::Result<Value>
        -name() String
        -execute(task_description: &str:Any) anyhow::Result<Value>
    }
    Agent <|-- RustantAgent : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as RustantService
    Caller->>Svc: new()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-application/src/agents`
* **Dependencies:** crate::Agent, async_trait::async_trait, std::sync::Arc, serde_json::{Value, json}, factory_infrastructure::{McpClient, R2rClient}
