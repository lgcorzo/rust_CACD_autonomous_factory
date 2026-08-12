---
type: "module-documentation"
title: "search_jira.rs"
source_path: "crates/factory-mcp-server/src/tools/search_jira.rs"
description: "Detailed documentation for search_jira.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "dfd90f5"
---

# File: search_jira.rs

**Source Path:** `crates/factory-mcp-server/src/tools/search_jira.rs`

## Overview

### Purpose
Provides implementation for search_jira.rs.

### Responsibilities
* Handles logic related to search_jira.

### Main Workflow
* Initialization and execution of search_jira logic.

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
Why it exists:
Provides capabilities related to ManualMockJiraClient.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `should_fail` (bool): Purpose - Stores should_fail data. Constraints - Valid bool.

**Public Methods:**

None.

**Private Methods:**

* `search_issues(_query: &str (Any)) -> anyhow::Result<String>`: Internal helper logic.

#### SearchJiraTool

**Overview:**
Why it exists:
Provides capabilities related to SearchJiraTool.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new(jira_client: Arc<dyn JiraClient> (Any))`
Parameters: jira_client: Arc<dyn JiraClient> (Any)
Dependencies: Inherited from context
Initialization: Sets up SearchJiraTool

**Attributes:**

* `jira_client` (Arc<dyn JiraClient>): Purpose - Stores jira_client data. Constraints - Valid Arc<dyn JiraClient>.

**Public Methods:**

None.

**Private Methods:**

* `call(params: Value (Any)) -> anyhow::Result<CallToolResult>`: Internal helper logic.
* `description() -> String`: Internal helper logic.
* `input_schema() -> Value`: Internal helper logic.
* `name() -> String`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class ManualMockJiraClient {
        -search_issues(_query: &str:Any) anyhow::Result<String>
    }
    JiraClient <|-- ManualMockJiraClient : Inheritance / Specialization
    class SearchJiraTool {
        -call(params: Value:Any) anyhow::Result<CallToolResult>
        -description() String
        -input_schema() Value
        -name() String
        +new(jira_client: Arc<dyn JiraClient>:Any) Self
    }
    Tool <|-- SearchJiraTool : Inheritance / Specialization

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Search_jiraService
    Caller->>Svc: search_issues()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## UML

### Class Diagram
```plantuml
@startuml
class ManualMockJiraClient {
    -search_issues(_query: &str:Any) : anyhow::Result<String>
}
JiraClient <|-- ManualMockJiraClient : Inheritance
class SearchJiraTool {
    -call(params: Value:Any) : anyhow::Result<CallToolResult>
    -description() : String
    -input_schema() : Value
    -name() : String
    +new(jira_client: Arc<dyn JiraClient>:Any) : Self
}
Tool <|-- SearchJiraTool : Inheritance
@enduml

```

### Package Diagram
```plantuml
@startuml
package "search_jira" {
  [Module Components]
}
@enduml

```

### Sequence Diagram
```plantuml
@startuml
autonumber
participant Caller as "Client Interface"
participant Svc as "Search_jiraService"
Caller -> Svc: execute()
note over Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

### Component Diagram
```plantuml
@startuml
component "search_jira" as comp
component "async_trait::async_trait" as async_trait::async_trait
comp --> async_trait::async_trait
component "crate::protocol::{CallToolResult, McpContent}" as crate::protocol::{CallToolResult, McpContent}
comp --> crate::protocol::{CallToolResult, McpContent}
component "crate::tools::Tool" as crate::tools::Tool
comp --> crate::tools::Tool
component "factory_infrastructure::JiraClient" as factory_infrastructure::JiraClient
comp --> factory_infrastructure::JiraClient
component "serde_json::{json, Value}" as serde_json::{json, Value}
comp --> serde_json::{json, Value}
component "std::sync::Arc" as std::sync::Arc
comp --> std::sync::Arc
component "super::*" as super::*
comp --> super::*
@enduml

```

### Dependency Graph
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

### Call Graph
```plantuml
@startuml
[API] --> SearchJiraTool::new
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
