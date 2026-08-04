---
type: "module-documentation"
title: "auditor.rs"
source_path: "crates/factory-application/src/agents/auditor.rs"
description: "Detailed documentation for auditor.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: auditor.rs

**Source Path:** `crates/factory-application/src/agents/auditor.rs`

## Overview

### Purpose
Provides implementation for auditor.rs.

### Responsibilities
* Handles logic related to auditor.

### Dependencies
* crate::Agent, async_trait::async_trait, super::*, serde_json::{Value, json}

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### AuditorAgent

**Overview:** Represents AuditorAgent.

**Public Methods:**

##### `new() -> Self`
Executes new.

##### `analyze_dag_logs(mission_id: &str (Any)) -> anyhow::Result<Vec<Value>>`
/// Queries Hatchet API for recent failed mission DAGs.

##### `audit_mission(mission_id: &str (Any), failures: &[Value] (Any)) -> anyhow::Result<Value>`
/// Uses LiteLLM to process failures and output recommendations.

##### `evaluate_prompts(mission_id: &str (Any), targets: &[factory_core::Targets] (Any), recommendations: &[Value] (Any)) -> anyhow::Result<String>`
/// Self-Improving Prompt Engineering evaluation loop.

/// Analyzes Hatchet failure recommendations against Target ground truths to propose a new system prompt.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class AuditorAgent {
        -default() Self
        +new() Self
        +analyze_dag_logs(mission_id: &str:Any) anyhow::Result<Vec<Value>>
        +audit_mission(mission_id: &str:Any, failures: &[Value]:Any) anyhow::Result<Value>
        +evaluate_prompts(mission_id: &str:Any, targets: &[factory_core::Targets]:Any, recommendations: &[Value]:Any) anyhow::Result<String>
        -name() String
        -execute(task_description: &str:Any) anyhow::Result<Value>
    }
    Default <|-- AuditorAgent : Inheritance / Specialization
    Agent <|-- AuditorAgent : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as AuditorService
    Caller->>Svc: default()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-application/src/agents`
* **Dependencies:** crate::Agent, async_trait::async_trait, super::*, serde_json::{Value, json}
