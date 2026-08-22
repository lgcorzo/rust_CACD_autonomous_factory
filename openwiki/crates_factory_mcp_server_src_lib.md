---
type: "module-documentation"
title: "lib.rs"
source_path: "crates/factory-mcp-server/src/lib.rs"
description: "Detailed documentation for lib.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "ec7bd0f"
---

# File: lib.rs

**Source Path:** `crates/factory-mcp-server/src/lib.rs`

## Overview

### Purpose
Provides implementation for lib.rs.

### Responsibilities
* Handles logic related to lib.

### Dependencies
* axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
    Json,
}, crate::protocol::CallToolResult, crate::protocol::{JsonRpcRequest, JsonRpcResponse, McpTool}, crate::tools::MockTool, crate::tools::Tool, crate::tools::{
            bridge::BridgeTool, deep_research_tool::DeepResearchTool,
            execute_code::ExecuteCodeTool, index_code::IndexCodeTool,
            launch_sandbox_pod::LaunchSandboxPodTool, plan_mission::PlanMissionTool,
            retrieve_context::RetrieveContextTool, run_tests::RunTestsTool,
            search_jira::SearchJiraTool, security_review::SecurityReviewTool,
            spec_kit_tasks_to_issues::SpecKitTasksToIssuesTool, spec_kit_tool::SpecKitTool,
            update_mission_status::UpdateMissionStatusTool,
        }, factory_infrastructure::{
            HttpGitlabClient, HttpJiraClient, HttpR2rClient, KafkaClient, SimpleMockKafkaClient,
        }, serde_json::{json, Value}, std::collections::HashMap, std::convert::Infallible, std::sync::Arc, std::time::Duration, super::*, tokio::sync::{mpsc, RwLock}, tokio_stream::wrappers::UnboundedReceiverStream, tokio_stream::{Stream, StreamExt}

### Imported modules
* None

### Exported classes
* McpServer

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### McpServer

**Overview:**
No description provided.

**Constructor:**

##### `new()`
Parameters:
Dependencies: Inherited from context
Initialization: Sets up McpServer

**Attributes:**

* `sessions` (Arc<RwLock<HashMap<String, mpsc::UnboundedSender<JsonRpcResponse>>>>): Purpose - Stores sessions data. Constraints - Valid Arc<RwLock<HashMap<String, mpsc::UnboundedSender<JsonRpcResponse>>>>.
* `tools` (Arc<RwLock<HashMap<String, Box<dyn Tool>>>>): Purpose - Stores tools data. Constraints - Valid Arc<RwLock<HashMap<String, Box<dyn Tool>>>>.

**Public Methods:**

##### `add_tool(tool: Box<dyn Tool> (Any)) -> None`

###### Description
No description provided.

###### Inputs
* `tool: Box<dyn Tool>`: type=Any, meaning=Input for tool: Box<dyn Tool>, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: None
Semantic meaning: Result of add_tool
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
let result = instance.add_tool();
```

##### `handle_request(request: JsonRpcRequest (Any)) -> JsonRpcResponse`

###### Description
No description provided.

###### Inputs
* `request: JsonRpcRequest`: type=Any, meaning=Input for request: JsonRpcRequest, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: JsonRpcResponse
Semantic meaning: Result of handle_request
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
let result = instance.handle_request();
```

##### `post_handler(State(server): State<Arc<McpServer>> (Any), Query(params): Query<HashMap<String, String>> (Any), Json(request): Json<JsonRpcRequest> (Any)) -> Json<JsonRpcResponse>`

###### Description
No description provided.

###### Inputs
* `State(server): State<Arc<McpServer>>`: type=Any, meaning=Input for State(server): State<Arc<McpServer>>, valid values=Any valid Any, optional=No, default value=None
* `Query(params): Query<HashMap<String, String>>`: type=Any, meaning=Input for Query(params): Query<HashMap<String, String>>, valid values=Any valid Any, optional=No, default value=None
* `Json(request): Json<JsonRpcRequest>`: type=Any, meaning=Input for Json(request): Json<JsonRpcRequest>, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: Json<JsonRpcResponse>
Semantic meaning: Result of post_handler
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
let result = instance.post_handler();
```

##### `register_default_tools() -> anyhow::Result<()>`

###### Description
No description provided.

###### Inputs
None.

###### Output
Return type: anyhow::Result<()>
Semantic meaning: Result of register_default_tools
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
let result = instance.register_default_tools();
```

##### `sse_handler(State(server): State<Arc<McpServer>> (Any)) -> Sse<impl Stream<Item = Result<Event, Infallible>>>`

###### Description
No description provided.

###### Inputs
* `State(server): State<Arc<McpServer>>`: type=Any, meaning=Input for State(server): State<Arc<McpServer>>, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: Sse<impl Stream<Item = Result<Event, Infallible>>>
Semantic meaning: Result of sse_handler
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
let result = instance.sse_handler();
```

**Private Methods:**

* `default() -> Self`: Internal helper logic.
* `error_response(id: Option<Value> (Any), code: i32 (Any), message: &str (Any)) -> JsonRpcResponse`: Internal helper logic.
* `handle_call_tool(request: JsonRpcRequest (Any)) -> JsonRpcResponse`: Internal helper logic.
* `handle_list_tools(id: Option<Value> (Any)) -> JsonRpcResponse`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class McpServer {
    +add_tool(tool: Box<dyn Tool>:Any) : None
    -default() : Self
    -error_response(id: Option<Value>:Any, code: i32:Any, message: &str:Any) : JsonRpcResponse
    -handle_call_tool(request: JsonRpcRequest:Any) : JsonRpcResponse
    -handle_list_tools(id: Option<Value>:Any) : JsonRpcResponse
    +handle_request(request: JsonRpcRequest:Any) : JsonRpcResponse
    +new() : Self
    +post_handler(State(server): State<Arc<McpServer>>:Any, Query(params): Query<HashMap<String, String>>:Any, Json(request): Json<JsonRpcRequest>:Any) : Json<JsonRpcResponse>
    +register_default_tools() : anyhow::Result<()>
    +sse_handler(State(server): State<Arc<McpServer>>:Any) : Sse<impl Stream<Item = Result<Event, Infallible>>>
}
Default <|-- McpServer : extends/implements
@enduml

```

## Package Diagram

```plantuml
@startuml
package "crates" {
    package "factory-mcp-server" {
        package "src" {
            class Module
        }
    }
}
@enduml

```

## Component Diagram

```plantuml
@startuml
component "lib" as Main
component "axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
    Json,
}" as axum________extract___Query__State_______response__sse___Event__Sse_______Json___
Main --> axum________extract___Query__State_______response__sse___Event__Sse_______Json___ : uses
component "crate::protocol::CallToolResult" as crate__protocol__CallToolResult
Main --> crate__protocol__CallToolResult : uses
component "crate::protocol::{JsonRpcRequest, JsonRpcResponse, McpTool}" as crate__protocol___JsonRpcRequest__JsonRpcResponse__McpTool_
Main --> crate__protocol___JsonRpcRequest__JsonRpcResponse__McpTool_ : uses
component "crate::tools::MockTool" as crate__tools__MockTool
Main --> crate__tools__MockTool : uses
component "crate::tools::Tool" as crate__tools__Tool
Main --> crate__tools__Tool : uses
component "crate::tools::{
            bridge::BridgeTool, deep_research_tool::DeepResearchTool,
            execute_code::ExecuteCodeTool, index_code::IndexCodeTool,
            launch_sandbox_pod::LaunchSandboxPodTool, plan_mission::PlanMissionTool,
            retrieve_context::RetrieveContextTool, run_tests::RunTestsTool,
            search_jira::SearchJiraTool, security_review::SecurityReviewTool,
            spec_kit_tasks_to_issues::SpecKitTasksToIssuesTool, spec_kit_tool::SpecKitTool,
            update_mission_status::UpdateMissionStatusTool,
        }" as crate__tools________________bridge__BridgeTool__deep_research_tool__DeepResearchTool______________execute_code__ExecuteCodeTool__index_code__IndexCodeTool______________launch_sandbox_pod__LaunchSandboxPodTool__plan_mission__PlanMissionTool______________retrieve_context__RetrieveContextTool__run_tests__RunTestsTool______________search_jira__SearchJiraTool__security_review__SecurityReviewTool______________spec_kit_tasks_to_issues__SpecKitTasksToIssuesTool__spec_kit_tool__SpecKitTool______________update_mission_status__UpdateMissionStatusTool___________
Main --> crate__tools________________bridge__BridgeTool__deep_research_tool__DeepResearchTool______________execute_code__ExecuteCodeTool__index_code__IndexCodeTool______________launch_sandbox_pod__LaunchSandboxPodTool__plan_mission__PlanMissionTool______________retrieve_context__RetrieveContextTool__run_tests__RunTestsTool______________search_jira__SearchJiraTool__security_review__SecurityReviewTool______________spec_kit_tasks_to_issues__SpecKitTasksToIssuesTool__spec_kit_tool__SpecKitTool______________update_mission_status__UpdateMissionStatusTool___________ : uses
component "factory_infrastructure::{
            HttpGitlabClient, HttpJiraClient, HttpR2rClient, KafkaClient, SimpleMockKafkaClient,
        }" as factory_infrastructure________________HttpGitlabClient__HttpJiraClient__HttpR2rClient__KafkaClient__SimpleMockKafkaClient___________
Main --> factory_infrastructure________________HttpGitlabClient__HttpJiraClient__HttpR2rClient__KafkaClient__SimpleMockKafkaClient___________ : uses
component "serde_json::{json, Value}" as serde_json___json__Value_
Main --> serde_json___json__Value_ : uses
component "std::collections::HashMap" as std__collections__HashMap
Main --> std__collections__HashMap : uses
component "std::convert::Infallible" as std__convert__Infallible
Main --> std__convert__Infallible : uses
component "std::sync::Arc" as std__sync__Arc
Main --> std__sync__Arc : uses
component "std::time::Duration" as std__time__Duration
Main --> std__time__Duration : uses
component "super::*" as super___
Main --> super___ : uses
component "tokio::sync::{mpsc, RwLock}" as tokio__sync___mpsc__RwLock_
Main --> tokio__sync___mpsc__RwLock_ : uses
component "tokio_stream::wrappers::UnboundedReceiverStream" as tokio_stream__wrappers__UnboundedReceiverStream
Main --> tokio_stream__wrappers__UnboundedReceiverStream : uses
component "tokio_stream::{Stream, StreamExt}" as tokio_stream___Stream__StreamExt_
Main --> tokio_stream___Stream__StreamExt_ : uses
@enduml

```

## Dependency Graph

```plantuml
@startuml
[lib]
[lib] --> [axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
    Json,
}]
[lib] --> [crate::protocol::CallToolResult]
[lib] --> [crate::protocol::{JsonRpcRequest, JsonRpcResponse, McpTool}]
[lib] --> [crate::tools::MockTool]
[lib] --> [crate::tools::Tool]
[lib] --> [crate::tools::{
            bridge::BridgeTool, deep_research_tool::DeepResearchTool,
            execute_code::ExecuteCodeTool, index_code::IndexCodeTool,
            launch_sandbox_pod::LaunchSandboxPodTool, plan_mission::PlanMissionTool,
            retrieve_context::RetrieveContextTool, run_tests::RunTestsTool,
            search_jira::SearchJiraTool, security_review::SecurityReviewTool,
            spec_kit_tasks_to_issues::SpecKitTasksToIssuesTool, spec_kit_tool::SpecKitTool,
            update_mission_status::UpdateMissionStatusTool,
        }]
[lib] --> [factory_infrastructure::{
            HttpGitlabClient, HttpJiraClient, HttpR2rClient, KafkaClient, SimpleMockKafkaClient,
        }]
[lib] --> [serde_json::{json, Value}]
[lib] --> [std::collections::HashMap]
[lib] --> [std::convert::Infallible]
[lib] --> [std::sync::Arc]
[lib] --> [std::time::Duration]
[lib] --> [super::*]
[lib] --> [tokio::sync::{mpsc, RwLock}]
[lib] --> [tokio_stream::wrappers::UnboundedReceiverStream]
[lib] --> [tokio_stream::{Stream, StreamExt}]
@enduml

```

## Call Graph

```plantuml
@startuml
Caller --> McpServer::add_tool
Caller --> McpServer::handle_request
Caller --> McpServer::new
Caller --> McpServer::post_handler
Caller --> McpServer::register_default_tools
Caller --> McpServer::sse_handler
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "LibService" as Svc
Caller -> Svc: add_tool()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

## Examples

```
// Example usage of lib.rs components
import { ... } from 'crates/factory-mcp-server/src/lib.rs';
```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src`
* **Dependencies:** axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
    Json,
}, crate::protocol::CallToolResult, crate::protocol::{JsonRpcRequest, JsonRpcResponse, McpTool}, crate::tools::MockTool, crate::tools::Tool, crate::tools::{
            bridge::BridgeTool, deep_research_tool::DeepResearchTool,
            execute_code::ExecuteCodeTool, index_code::IndexCodeTool,
            launch_sandbox_pod::LaunchSandboxPodTool, plan_mission::PlanMissionTool,
            retrieve_context::RetrieveContextTool, run_tests::RunTestsTool,
            search_jira::SearchJiraTool, security_review::SecurityReviewTool,
            spec_kit_tasks_to_issues::SpecKitTasksToIssuesTool, spec_kit_tool::SpecKitTool,
            update_mission_status::UpdateMissionStatusTool,
        }, factory_infrastructure::{
            HttpGitlabClient, HttpJiraClient, HttpR2rClient, KafkaClient, SimpleMockKafkaClient,
        }, serde_json::{json, Value}, std::collections::HashMap, std::convert::Infallible, std::sync::Arc, std::time::Duration, super::*, tokio::sync::{mpsc, RwLock}, tokio_stream::wrappers::UnboundedReceiverStream, tokio_stream::{Stream, StreamExt}
