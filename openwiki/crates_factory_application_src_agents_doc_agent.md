---
type: "module-documentation"
title: "doc_agent.rs"
source_path: "crates/factory-application/src/agents/doc_agent.rs"
description: "Detailed documentation for doc_agent.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "dfd90f5"
---

# File: doc_agent.rs

**Source Path:** `crates/factory-application/src/agents/doc_agent.rs`

## Overview

### Purpose
Provides implementation for doc_agent.rs.

### Responsibilities
* Handles logic related to doc_agent.

### Main Workflow
* Initialization and execution of doc_agent logic.

### Dependencies
* async_trait::async_trait, crate::Agent, factory_infrastructure::{McpClient, R2rClient}, factory_infrastructure::{MockMcpClient, MockR2rClient}, serde_json::{Value, json}, std::sync::Arc, std::time::Duration, super::*

### Imported modules
* None

### Exported classes
* DocumentationAgent

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### DocumentationAgent

**Overview:**
Why it exists:
Provides capabilities related to DocumentationAgent.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new(mcp_client: Arc<dyn McpClient> (Any), r2r_client: Arc<dyn R2rClient> (Any), superpowers_skills_root: std::path::PathBuf (Any))`
Parameters: mcp_client: Arc<dyn McpClient> (Any), r2r_client: Arc<dyn R2rClient> (Any), superpowers_skills_root: std::path::PathBuf (Any)
Dependencies: Inherited from context
Initialization: Sets up DocumentationAgent

**Attributes:**

* `mcp_client` (Arc<dyn McpClient>): Purpose - Stores mcp_client data. Constraints - Valid Arc<dyn McpClient>.
* `r2r_client` (Arc<dyn R2rClient>): Purpose - Stores r2r_client data. Constraints - Valid Arc<dyn R2rClient>.
* `superpowers_skills_root` (std::path::PathBuf): Purpose - Stores superpowers_skills_root data. Constraints - Valid std::path::PathBuf.

**Public Methods:**

##### `extract_code_deltas(commit_sha: &str (Any)) -> anyhow::Result<String>`

###### Description
Executes extract_code_deltas.

###### Inputs
* `commit_sha: &str`: type=Any, meaning=Input for commit_sha: &str, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: anyhow::Result<String>
Semantic meaning: Result of extract_code_deltas
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
let result = instance.extract_code_deltas();
```

##### `generate_hazitek_report(mission_id: &str (Any)) -> anyhow::Result<factory_core::ComplianceReport>`

###### Description
Executes generate_hazitek_report.

###### Inputs
* `mission_id: &str`: type=Any, meaning=Input for mission_id: &str, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: anyhow::Result<factory_core::ComplianceReport>
Semantic meaning: Result of generate_hazitek_report
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
let result = instance.generate_hazitek_report();
```

##### `run_post_merge_pipeline(mission_id: &str (Any)) -> anyhow::Result<Value>`

###### Description
Executes run_post_merge_pipeline.

###### Inputs
* `mission_id: &str`: type=Any, meaning=Input for mission_id: &str, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: anyhow::Result<Value>
Semantic meaning: Result of run_post_merge_pipeline
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
let result = instance.run_post_merge_pipeline();
```

**Private Methods:**

* `execute(task_description: &str (Any)) -> anyhow::Result<Value>`: Internal helper logic.
* `name() -> String`: Internal helper logic.
* `verify_osr() -> anyhow::Result<f32>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class DocumentationAgent {
        -execute(task_description: &str:Any) anyhow::Result<Value>
        +extract_code_deltas(commit_sha: &str:Any) anyhow::Result<String>
        +generate_hazitek_report(mission_id: &str:Any) anyhow::Result<factory_core::ComplianceReport>
        -name() String
        +new(mcp_client: Arc<dyn McpClient>:Any, r2r_client: Arc<dyn R2rClient>:Any, superpowers_skills_root: std::path::PathBuf:Any) Self
        +run_post_merge_pipeline(mission_id: &str:Any) anyhow::Result<Value>
        -verify_osr() anyhow::Result<f32>
    }
    Agent <|-- DocumentationAgent : Inheritance / Specialization

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Doc_agentService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## UML

### Class Diagram
```plantuml
@startuml
class DocumentationAgent {
    -execute(task_description: &str:Any) : anyhow::Result<Value>
    +extract_code_deltas(commit_sha: &str:Any) : anyhow::Result<String>
    +generate_hazitek_report(mission_id: &str:Any) : anyhow::Result<factory_core::ComplianceReport>
    -name() : String
    +new(mcp_client: Arc<dyn McpClient>:Any, r2r_client: Arc<dyn R2rClient>:Any, superpowers_skills_root: std::path::PathBuf:Any) : Self
    +run_post_merge_pipeline(mission_id: &str:Any) : anyhow::Result<Value>
    -verify_osr() : anyhow::Result<f32>
}
Agent <|-- DocumentationAgent : Inheritance
@enduml

```

### Package Diagram
```plantuml
@startuml
package "doc_agent" {
  [Module Components]
}
@enduml

```

### Sequence Diagram
```plantuml
@startuml
autonumber
participant Caller as "Client Interface"
participant Svc as "Doc_agentService"
Caller -> Svc: extract_code_deltas()
note over Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

### Component Diagram
```plantuml
@startuml
component "doc_agent" as comp
component "async_trait::async_trait" as async_trait::async_trait
comp --> async_trait::async_trait
component "crate::Agent" as crate::Agent
comp --> crate::Agent
component "factory_infrastructure::{McpClient, R2rClient}" as factory_infrastructure::{McpClient, R2rClient}
comp --> factory_infrastructure::{McpClient, R2rClient}
component "factory_infrastructure::{MockMcpClient, MockR2rClient}" as factory_infrastructure::{MockMcpClient, MockR2rClient}
comp --> factory_infrastructure::{MockMcpClient, MockR2rClient}
component "serde_json::{Value, json}" as serde_json::{Value, json}
comp --> serde_json::{Value, json}
component "std::sync::Arc" as std::sync::Arc
comp --> std::sync::Arc
component "std::time::Duration" as std::time::Duration
comp --> std::time::Duration
component "super::*" as super::*
comp --> super::*
@enduml

```

### Dependency Graph
```plantuml
@startuml
[doc_agent]
[doc_agent] --> [async_trait::async_trait]
[doc_agent] --> [crate::Agent]
[doc_agent] --> [factory_infrastructure::{McpClient, R2rClient}]
[doc_agent] --> [factory_infrastructure::{MockMcpClient, MockR2rClient}]
[doc_agent] --> [serde_json::{Value, json}]
[doc_agent] --> [std::sync::Arc]
[doc_agent] --> [std::time::Duration]
[doc_agent] --> [super::*]
@enduml

```

### Call Graph
```plantuml
@startuml
[API] --> DocumentationAgent::extract_code_deltas
[API] --> DocumentationAgent::generate_hazitek_report
[API] --> DocumentationAgent::new
[API] --> DocumentationAgent::run_post_merge_pipeline
@enduml

```

## Examples

```
// Example usage of doc_agent.rs components
import { ... } from 'crates/factory-application/src/agents/doc_agent.rs';
```

## Cross References
* **Parent module:** `crates/factory-application/src/agents`
* **Dependencies:** async_trait::async_trait, crate::Agent, factory_infrastructure::{McpClient, R2rClient}, factory_infrastructure::{MockMcpClient, MockR2rClient}, serde_json::{Value, json}, std::sync::Arc, std::time::Duration, super::*
