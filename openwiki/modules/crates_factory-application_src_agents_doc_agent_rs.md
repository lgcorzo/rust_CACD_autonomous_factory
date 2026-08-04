---
type: "module-documentation"
title: "doc_agent.rs"
source_path: "crates/factory-application/src/agents/doc_agent.rs"
description: "Detailed documentation for doc_agent.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: doc_agent.rs

**Source Path:** `crates/factory-application/src/agents/doc_agent.rs`

## Overview

### Purpose
Provides implementation for doc_agent.rs.

### Responsibilities
* Handles logic related to doc_agent.

### Dependencies
* serde_json::{Value, json}, std::sync::Arc, super::*, factory_infrastructure::{MockMcpClient, MockR2rClient}, std::time::Duration, crate::Agent, async_trait::async_trait, factory_infrastructure::{McpClient, R2rClient}

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### DocumentationAgent

**Overview:** Represents DocumentationAgent.

**Public Methods:**

##### `new(mcp_client: Arc<dyn McpClient> (Any), r2r_client: Arc<dyn R2rClient> (Any), superpowers_skills_root: std::path::PathBuf (Any)) -> Self`
Executes new.

##### `run_post_merge_pipeline(mission_id: &str (Any)) -> anyhow::Result<Value>`
Executes run_post_merge_pipeline.

##### `extract_code_deltas(commit_sha: &str (Any)) -> anyhow::Result<String>`
Executes extract_code_deltas.

##### `generate_hazitek_report(mission_id: &str (Any)) -> anyhow::Result<factory_core::ComplianceReport>`
Executes generate_hazitek_report.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class DocumentationAgent {
        +new(mcp_client: Arc<dyn McpClient>:Any, r2r_client: Arc<dyn R2rClient>:Any, superpowers_skills_root: std::path::PathBuf:Any) Self
        +run_post_merge_pipeline(mission_id: &str:Any) anyhow::Result<Value>
        -verify_osr() anyhow::Result<f32>
        +extract_code_deltas(commit_sha: &str:Any) anyhow::Result<String>
        +generate_hazitek_report(mission_id: &str:Any) anyhow::Result<factory_core::ComplianceReport>
        -name() String
        -execute(task_description: &str:Any) anyhow::Result<Value>
    }
    Agent <|-- DocumentationAgent : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Doc_agentService
    Caller->>Svc: new()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-application/src/agents`
* **Dependencies:** serde_json::{Value, json}, std::sync::Arc, super::*, factory_infrastructure::{MockMcpClient, MockR2rClient}, std::time::Duration, crate::Agent, async_trait::async_trait, factory_infrastructure::{McpClient, R2rClient}
