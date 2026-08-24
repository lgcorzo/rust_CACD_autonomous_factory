---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "search_jira.rs"
source_path: "crates/factory-mcp-server/src/tools/search_jira.rs"
description: "Detailed documentation for search_jira.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-24T05:31:17Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "d073f5f"
---

# File: search_jira.rs

**Source Path:** `crates/factory-mcp-server/src/tools/search_jira.rs`

## Overview

### Purpose
Provides implementation for search_jira.rs.

### Responsibilities
* Handles logic related to search_jira.

### Dependencies
* async_trait::async_trait, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, factory_infrastructure::JiraClient, serde_json::{json, Value}, std::sync::Arc, super::*

### Imported modules
* None

### Exported classes
* ManualMockJiraClient, SearchJiraTool

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### ManualMockJiraClient

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

* `should_fail` (bool): Purpose - Stores should_fail data. Constraints - Valid bool.

**Public Methods:**

None.

**Private Methods:**

* `search_issues(_query (&str)) -> anyhow::Result<String>`: Internal helper logic.

#### SearchJiraTool

**Overview:**
No description provided.

**Constructor:**

##### `new(jira_client (Arc<dyn JiraClient>))`
Parameters: jira_client (Arc<dyn JiraClient>)
Dependencies: Inherited from context
Initialization: Sets up SearchJiraTool

**Attributes:**

* `jira_client` (Arc<dyn JiraClient>): Purpose - Stores jira_client data. Constraints - Valid Arc<dyn JiraClient>.

**Public Methods:**

None.

**Private Methods:**

* `call(params (Value)) -> anyhow::Result<CallToolResult>`: Internal helper logic.
* `description() -> String`: Internal helper logic.
* `input_schema() -> Value`: Internal helper logic.
* `name() -> String`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class ManualMockJiraClient {
    -search_issues(_query: &str) anyhow::Result<String>
}
JiraClient <|-- ManualMockJiraClient : extends/implements
class SearchJiraTool {
    -call(params: Value) anyhow::Result<CallToolResult>
    -description() String
    -input_schema() Value
    -name() String
    +new(jira_client: Arc<dyn JiraClient>) Self
}
Tool <|-- SearchJiraTool : extends/implements
@enduml

```

## Package Diagram

```plantuml
@startuml
package "crates" {
    package "factory-mcp-server" {
        package "src" {
            package "tools" {
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
component "search_jira" as Main
component "async_trait::async_trait" as async_trait__async_trait
Main --> async_trait__async_trait : uses
component "crate::protocol::{CallToolResult, McpContent}" as crate__protocol___CallToolResult__McpContent_
Main --> crate__protocol___CallToolResult__McpContent_ : uses
component "crate::tools::Tool" as crate__tools__Tool
Main --> crate__tools__Tool : uses
component "factory_infrastructure::JiraClient" as factory_infrastructure__JiraClient
Main --> factory_infrastructure__JiraClient : uses
component "serde_json::{json, Value}" as serde_json___json__Value_
Main --> serde_json___json__Value_ : uses
component "std::sync::Arc" as std__sync__Arc
Main --> std__sync__Arc : uses
component "super::*" as super___
Main --> super___ : uses
@enduml

```

## Dependency Graph

```plantuml
@startuml
[search_jira]
[search_jira] --> [async_trait::async_trait]
[search_jira] --> [crate::protocol::{CallToolResult, McpContent}]
[search_jira] --> [crate::tools::Tool]
[search_jira] --> [factory_infrastructure::JiraClient]
[search_jira] --> [serde_json::{json, Value}]
[search_jira] --> [std::sync::Arc]
[search_jira] --> [super::*]
@enduml

```

## Call Graph

```plantuml
@startuml
Caller --> SearchJiraTool::new
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "Search_jiraService" as Svc
Caller -> Svc: search_issues()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

## Examples

```
// Example usage of search_jira.rs components
import { ... } from 'crates/factory-mcp-server/src/tools/search_jira.rs';
```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** async_trait::async_trait, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, factory_infrastructure::JiraClient, serde_json::{json, Value}, std::sync::Arc, super::*
