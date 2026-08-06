---
type: "module-documentation"
title: "spec_kit_tasks_to_issues.rs"
source_path: "crates/factory-mcp-server/src/tools/spec_kit_tasks_to_issues.rs"
description: "Detailed documentation for spec_kit_tasks_to_issues.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-06T17:55:58Z"
---

# File: spec_kit_tasks_to_issues.rs

**Source Path:** `crates/factory-mcp-server/src/tools/spec_kit_tasks_to_issues.rs`

## Overview

### Purpose
Provides implementation for spec_kit_tasks_to_issues.rs.

### Responsibilities
* Handles logic related to spec_kit_tasks_to_issues.

### Dependencies
* serde_json::{json, Value}, crate::tools::Tool, async_trait::async_trait, std::sync::Arc, factory_infrastructure::GitlabClient, crate::protocol::CallToolResult

### Imported modules
*

### Exported classes
* SpecKitTasksToIssuesTool

### Exported interfaces
*

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### SpecKitTasksToIssuesTool

**Overview:**
Why it exists:
Provides capabilities related to SpecKitTasksToIssuesTool.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new(gitlab_client: Arc<dyn GitlabClient> (Any))`
Parameters: gitlab_client: Arc<dyn GitlabClient> (Any)
Dependencies: Inherited from context
Initialization: Sets up SpecKitTasksToIssuesTool

**Attributes:**

* `gitlab_client` (Arc<dyn GitlabClient>): Purpose - Stores gitlab_client data. Constraints - Valid Arc<dyn GitlabClient>.

**Public Methods:**

None.

**Private Methods:**

* `name() -> String`: Internal helper logic.
* `description() -> String`: Internal helper logic.
* `input_schema() -> Value`: Internal helper logic.
* `call(params: Value (Any)) -> anyhow::Result<CallToolResult>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

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

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Spec_kit_tasks_to_issuesService
    Caller->>Svc: new()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```


## Examples

```
// Example usage of spec_kit_tasks_to_issues.rs components
import { ... } from 'crates/factory-mcp-server/src/tools/spec_kit_tasks_to_issues.rs';
```


## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** serde_json::{json, Value}, crate::tools::Tool, async_trait::async_trait, std::sync::Arc, factory_infrastructure::GitlabClient, crate::protocol::CallToolResult
