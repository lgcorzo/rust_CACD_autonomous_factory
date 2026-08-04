---
type: "module-documentation"
title: "spec_kit_tasks_to_issues.rs"
source_path: "crates/factory-mcp-server/src/tools/spec_kit_tasks_to_issues.rs"
description: "Detailed documentation for spec_kit_tasks_to_issues.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: spec_kit_tasks_to_issues.rs

**Source Path:** `crates/factory-mcp-server/src/tools/spec_kit_tasks_to_issues.rs`

## Overview

### Purpose
Provides implementation for spec_kit_tasks_to_issues.rs.

### Responsibilities
* Handles logic related to spec_kit_tasks_to_issues.

### Dependencies
* crate::protocol::CallToolResult, async_trait::async_trait, factory_infrastructure::GitlabClient, serde_json::{json, Value}, std::sync::Arc, crate::tools::Tool

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### SpecKitTasksToIssuesTool

**Overview:** Represents SpecKitTasksToIssuesTool.

**Public Methods:**

##### `new(gitlab_client: Arc<dyn GitlabClient> (Any)) -> Self`
Executes new.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class SpecKitTasksToIssuesTool {
        +new(gitlab_client: Arc<dyn GitlabClient>:Any) Self
        -name() String
        -description() String
        -input_schema() Value
        -call(params: Value:Any) anyhow::Result<CallToolResult>
    }
    Tool <|-- SpecKitTasksToIssuesTool : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Spec_kit_tasks_to_issuesService
    Caller->>Svc: new()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** crate::protocol::CallToolResult, async_trait::async_trait, factory_infrastructure::GitlabClient, serde_json::{json, Value}, std::sync::Arc, crate::tools::Tool
