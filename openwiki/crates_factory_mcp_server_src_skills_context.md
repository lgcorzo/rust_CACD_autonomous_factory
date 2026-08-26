---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "context.rs"
source_path: "crates/factory-mcp-server/src/skills/context.rs"
description: "Detailed documentation for context.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-26T06:00:08Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "e2707de"
---

# File: context.rs

**Source Path:** `crates/factory-mcp-server/src/skills/context.rs`

## Overview

### Purpose
Provides implementation for context.rs.

### Responsibilities
* Handles logic related to context.

### Dependencies
* serde_json::{json, Value}, super::*

### Imported modules
* None

### Exported classes
* ContextSkill

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### ContextSkill

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

None.

**Public Methods:**

##### `format_for_llm(pruned_context (&str)) -> Value`

###### Description
No description provided.

###### Inputs
* `pruned_context`: type=&str, meaning=Input for pruned_context, valid values=Any valid &str, optional=No, default value=None

###### Output
Return type: Value
Semantic meaning: Result of format_for_llm
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
let result = instance.format_for_llm();
```

##### `prune_context(raw_context (&str), max_chars (usize)) -> String`

###### Description
No description provided.

###### Inputs
* `raw_context`: type=&str, meaning=Input for raw_context, valid values=Any valid &str, optional=No, default value=None
* `max_chars`: type=usize, meaning=Input for max_chars, valid values=Any valid usize, optional=No, default value=None

###### Output
Return type: String
Semantic meaning: Result of prune_context
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
let result = instance.prune_context();
```

**Private Methods:**

None.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class ContextSkill {
    +format_for_llm(pruned_context: &str) Value
    +prune_context(raw_context: &str, max_chars: usize) String
}
@enduml

```

## Package Diagram

```plantuml
@startuml
package "crates" {
    package "factory-mcp-server" {
        package "src" {
            package "skills" {
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
component "context" as Main
component "serde_json::{json, Value}" as serde_json___json__Value_
Main --> serde_json___json__Value_ : uses
component "super::*" as super___
Main --> super___ : uses
@enduml

```

## Dependency Graph

```plantuml
@startuml
[context]
[context] --> [serde_json::{json, Value}]
[context] --> [super::*]
@enduml

```

## Call Graph

```plantuml
@startuml
Caller --> ContextSkill::format_for_llm
Caller --> ContextSkill::prune_context
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "ContextService" as Svc
Caller -> Svc: format_for_llm()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

## Examples

```
// Example usage of context.rs components
import { ... } from 'crates/factory-mcp-server/src/skills/context.rs';
```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/skills`
* **Dependencies:** serde_json::{json, Value}, super::*
