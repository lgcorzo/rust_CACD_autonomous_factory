---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "doc_agent.rs"
source_path: "crates/factory-application/src/agents/doc_agent.rs"
description: "Detailed documentation for doc_agent.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-25T05:53:44Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "e2707de"
---

# File: doc_agent.rs

**Source Path:** `crates/factory-application/src/agents/doc_agent.rs`

## Overview

### Purpose
Provides implementation for doc_agent.rs.

### Responsibilities
* Handles logic related to doc_agent.

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
No description provided.

**Constructor:**

##### `new(mcp_client (Arc<dyn McpClient>), r2r_client (Arc<dyn R2rClient>), superpowers_skills_root (std::path::PathBuf))`
Parameters: mcp_client (Arc<dyn McpClient>), r2r_client (Arc<dyn R2rClient>), superpowers_skills_root (std::path::PathBuf)
Dependencies: Inherited from context
Initialization: Sets up DocumentationAgent

**Attributes:**

* `mcp_client` (Arc<dyn McpClient>): Purpose - Stores mcp_client data. Constraints - Valid Arc<dyn McpClient>.
* `r2r_client` (Arc<dyn R2rClient>): Purpose - Stores r2r_client data. Constraints - Valid Arc<dyn R2rClient>.
* `superpowers_skills_root` (std::path::PathBuf): Purpose - Stores superpowers_skills_root data. Constraints - Valid std::path::PathBuf.

**Public Methods:**

##### `extract_code_deltas(commit_sha (&str)) -> anyhow::Result<String>`

###### Description
No description provided.

###### Inputs
* `commit_sha`: type=&str, meaning=Input for commit_sha, valid values=Any valid &str, optional=No, default value=None

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

##### `generate_hazitek_report(mission_id (&str)) -> anyhow::Result<factory_core::ComplianceReport>`

###### Description
No description provided.

###### Inputs
* `mission_id`: type=&str, meaning=Input for mission_id, valid values=Any valid &str, optional=No, default value=None

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

##### `run_post_merge_pipeline(mission_id (&str)) -> anyhow::Result<Value>`

###### Description
No description provided.

###### Inputs
* `mission_id`: type=&str, meaning=Input for mission_id, valid values=Any valid &str, optional=No, default value=None

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

* `execute(task_description (&str)) -> anyhow::Result<Value>`: Internal helper logic.
* `name() -> String`: Internal helper logic.
* `verify_osr() -> anyhow::Result<f32>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class DocumentationAgent {
    -execute(task_description: &str) anyhow::Result<Value>
    +extract_code_deltas(commit_sha: &str) anyhow::Result<String>
    +generate_hazitek_report(mission_id: &str) anyhow::Result<factory_core::ComplianceReport>
    -name() String
    +new(mcp_client: Arc<dyn McpClient>, r2r_client: Arc<dyn R2rClient>, superpowers_skills_root: std::path::PathBuf) Self
    +run_post_merge_pipeline(mission_id: &str) anyhow::Result<Value>
    -verify_osr() anyhow::Result<f32>
}
Agent <|-- DocumentationAgent : extends/implements
@enduml

```

## Package Diagram

```plantuml
@startuml
package "crates" {
    package "factory-application" {
        package "src" {
            package "agents" {
                class Module
            }
        }
    }
}
@enduml

```

## Component Diagram

```plantuml
@startuml
component "doc_agent" as Main
component "async_trait::async_trait" as async_trait__async_trait
Main --> async_trait__async_trait : uses
component "crate::Agent" as crate__Agent
Main --> crate__Agent : uses
component "factory_infrastructure::{McpClient, R2rClient}" as factory_infrastructure___McpClient__R2rClient_
Main --> factory_infrastructure___McpClient__R2rClient_ : uses
component "factory_infrastructure::{MockMcpClient, MockR2rClient}" as factory_infrastructure___MockMcpClient__MockR2rClient_
Main --> factory_infrastructure___MockMcpClient__MockR2rClient_ : uses
component "serde_json::{Value, json}" as serde_json___Value__json_
Main --> serde_json___Value__json_ : uses
component "std::sync::Arc" as std__sync__Arc
Main --> std__sync__Arc : uses
component "std::time::Duration" as std__time__Duration
Main --> std__time__Duration : uses
component "super::*" as super___
Main --> super___ : uses
@enduml

```

## Dependency Graph

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

## Call Graph

```plantuml
@startuml
Caller --> DocumentationAgent::extract_code_deltas
Caller --> DocumentationAgent::generate_hazitek_report
Caller --> DocumentationAgent::new
Caller --> DocumentationAgent::run_post_merge_pipeline
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "Doc_agentService" as Svc
Caller -> Svc: execute()
note right of Svc: Processing internal logic
Svc --> Caller: result
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
