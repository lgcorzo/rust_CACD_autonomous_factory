---
type: "module-documentation"
title: "auditor.rs"
source_path: "crates/factory-application/src/agents/auditor.rs"
description: "Detailed documentation for auditor.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "ec7bd0f"
---

# File: auditor.rs

**Source Path:** `crates/factory-application/src/agents/auditor.rs`

## Overview

### Purpose
Provides implementation for auditor.rs.

### Responsibilities
* Handles logic related to auditor.

### Dependencies
* async_trait::async_trait, crate::Agent, serde_json::{Value, json}, super::*

### Imported modules
* None

### Exported classes
* AuditorAgent

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### AuditorAgent

**Overview:**
No description provided.

**Constructor:**

##### `new()`
Parameters:
Dependencies: Inherited from context
Initialization: Sets up AuditorAgent

**Attributes:**

None.

**Public Methods:**

##### `analyze_dag_logs(mission_id: &str (Any)) -> anyhow::Result<Vec<Value>>`

###### Description
/// Queries Hatchet API for recent failed mission DAGs.

###### Inputs
* `mission_id: &str`: type=Any, meaning=Input for mission_id: &str, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: anyhow::Result<Vec<Value>>
Semantic meaning: Result of analyze_dag_logs
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
let result = instance.analyze_dag_logs();
```

##### `audit_mission(mission_id: &str (Any), failures: &[Value] (Any)) -> anyhow::Result<Value>`

###### Description
/// Uses LiteLLM to process failures and output recommendations.

###### Inputs
* `mission_id: &str`: type=Any, meaning=Input for mission_id: &str, valid values=Any valid Any, optional=No, default value=None
* `failures: &[Value]`: type=Any, meaning=Input for failures: &[Value], valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: anyhow::Result<Value>
Semantic meaning: Result of audit_mission
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
let result = instance.audit_mission();
```

##### `evaluate_prompts(mission_id: &str (Any), targets: &[factory_core::Targets] (Any), recommendations: &[Value] (Any)) -> anyhow::Result<String>`

###### Description
/// Self-Improving Prompt Engineering evaluation loop.

/// Analyzes Hatchet failure recommendations against Target ground truths to propose a new system prompt.

###### Inputs
* `mission_id: &str`: type=Any, meaning=Input for mission_id: &str, valid values=Any valid Any, optional=No, default value=None
* `targets: &[factory_core::Targets]`: type=Any, meaning=Input for targets: &[factory_core::Targets], valid values=Any valid Any, optional=No, default value=None
* `recommendations: &[Value]`: type=Any, meaning=Input for recommendations: &[Value], valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: anyhow::Result<String>
Semantic meaning: Result of evaluate_prompts
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
let result = instance.evaluate_prompts();
```

**Private Methods:**

* `default() -> Self`: Internal helper logic.
* `execute(task_description: &str (Any)) -> anyhow::Result<Value>`: Internal helper logic.
* `name() -> String`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class AuditorAgent {
    +analyze_dag_logs(mission_id: &str:Any) : anyhow::Result<Vec<Value>>
    +audit_mission(mission_id: &str:Any, failures: &[Value]:Any) : anyhow::Result<Value>
    -default() : Self
    +evaluate_prompts(mission_id: &str:Any, targets: &[factory_core::Targets]:Any, recommendations: &[Value]:Any) : anyhow::Result<String>
    -execute(task_description: &str:Any) : anyhow::Result<Value>
    -name() : String
    +new() : Self
}
Agent <|-- AuditorAgent : extends/implements
Default <|-- AuditorAgent : extends/implements
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
component "auditor" as Main
component "async_trait::async_trait" as async_trait__async_trait
Main --> async_trait__async_trait : uses
component "crate::Agent" as crate__Agent
Main --> crate__Agent : uses
component "serde_json::{Value, json}" as serde_json___Value__json_
Main --> serde_json___Value__json_ : uses
component "super::*" as super___
Main --> super___ : uses
@enduml

```

## Dependency Graph

```plantuml
@startuml
[auditor]
[auditor] --> [async_trait::async_trait]
[auditor] --> [crate::Agent]
[auditor] --> [serde_json::{Value, json}]
[auditor] --> [super::*]
@enduml

```

## Call Graph

```plantuml
@startuml
Caller --> AuditorAgent::analyze_dag_logs
Caller --> AuditorAgent::audit_mission
Caller --> AuditorAgent::evaluate_prompts
Caller --> AuditorAgent::new
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "AuditorService" as Svc
Caller -> Svc: analyze_dag_logs()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

## Examples

```
// Example usage of auditor.rs components
import { ... } from 'crates/factory-application/src/agents/auditor.rs';
```

## Cross References
* **Parent module:** `crates/factory-application/src/agents`
* **Dependencies:** async_trait::async_trait, crate::Agent, serde_json::{Value, json}, super::*
