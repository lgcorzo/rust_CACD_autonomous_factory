---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "mcp_client.rs"
source_path: "crates/factory-infrastructure/src/mcp_client.rs"
description: "Detailed documentation for mcp_client.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-31T05:39:54Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "7bf01b8"
---

# File: mcp_client.rs

**Source Path:** `crates/factory-infrastructure/src/mcp_client.rs`

## Overview

### Purpose
Provides implementation for mcp_client.rs.

### Responsibilities
* Handles logic related to mcp_client.

### Dependencies
* anyhow::anyhow, futures_util::StreamExt, reqwest::Client, serde_json::{json, Value}, super::*, tokio::sync::OnceCell, wiremock::matchers::{method, path}, wiremock::{Mock, MockServer, ResponseTemplate}

### Imported modules
* None

### Exported classes
* McpHttpClient, McpSseClient

### Exported interfaces
* McpClient

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### McpClient

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

None.

**Public Methods:**

##### `call_tool_json(self (Self), name (&str), arguments (Value)) -> anyhow::Result<Value>`

###### Description
No description provided.

###### Inputs
* `self`: type=Self, meaning=Input for self, valid values=Any valid Self, optional=No, default value=None
* `name`: type=&str, meaning=Input for name, valid values=Any valid &str, optional=No, default value=None
* `arguments`: type=Value, meaning=Input for arguments, valid values=Any valid Value, optional=No, default value=None

###### Output
Return type: anyhow::Result<Value>
Semantic meaning: Result of call_tool_json
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
let result = instance.call_tool_json();
```

**Private Methods:**

None.

#### McpHttpClient

**Overview:**
No description provided.

**Constructor:**

##### `new(base_url (String))`
Parameters: base_url (String)
Dependencies: Inherited from context
Initialization: Sets up McpHttpClient

**Attributes:**

* `base_url` (String): Purpose - Stores base_url data. Constraints - Valid String.
* `client` (Client): Purpose - Stores client data. Constraints - Valid Client.

**Public Methods:**

None.

**Private Methods:**

* `call_tool_json(self (Self), name (&str), arguments (Value)) -> anyhow::Result<Value>`: Internal helper logic.

#### McpSseClient

**Overview:**
/// A client that uses SSE handshake to find the session endpoint

**Constructor:**

##### `new(base_url (String))`
Parameters: base_url (String)
Dependencies: Inherited from context
Initialization: Sets up McpSseClient

**Attributes:**

* `base_url` (String): Purpose - Stores base_url data. Constraints - Valid String.
* `client` (Client): Purpose - Stores client data. Constraints - Valid Client.
* `session_url` (OnceCell<String>): Purpose - Stores session_url data. Constraints - Valid OnceCell<String>.

**Public Methods:**

None.

**Private Methods:**

* `call_tool_json(self (Self), name (&str), arguments (Value)) -> anyhow::Result<Value>`: Internal helper logic.
* `get_session_url(self (Self)) -> anyhow::Result<String>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
interface McpClient {
    +call_tool_json(self: Self, name: &str, arguments: Value) anyhow::Result<Value>
}
class McpHttpClient {
    -call_tool_json(self: Self, name: &str, arguments: Value) anyhow::Result<Value>
    +new(base_url: String) Self
}
McpClient <|-- McpHttpClient : extends/implements
class McpSseClient {
    -call_tool_json(self: Self, name: &str, arguments: Value) anyhow::Result<Value>
    -get_session_url(self: Self) anyhow::Result<String>
    +new(base_url: String) Self
}
McpClient <|-- McpSseClient : extends/implements
@enduml

```

## Package Diagram

```plantuml
@startuml
package "crates" {
    package "factory-infrastructure" {
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
component "mcp_client" as Main
component "anyhow::anyhow" as anyhow__anyhow
Main --> anyhow__anyhow : uses
component "futures_util::StreamExt" as futures_util__StreamExt
Main --> futures_util__StreamExt : uses
component "reqwest::Client" as reqwest__Client
Main --> reqwest__Client : uses
component "serde_json::{json, Value}" as serde_json___json__Value_
Main --> serde_json___json__Value_ : uses
component "super::*" as super___
Main --> super___ : uses
component "tokio::sync::OnceCell" as tokio__sync__OnceCell
Main --> tokio__sync__OnceCell : uses
component "wiremock::matchers::{method, path}" as wiremock__matchers___method__path_
Main --> wiremock__matchers___method__path_ : uses
component "wiremock::{Mock, MockServer, ResponseTemplate}" as wiremock___Mock__MockServer__ResponseTemplate_
Main --> wiremock___Mock__MockServer__ResponseTemplate_ : uses
@enduml

```

## Dependency Graph

```plantuml
@startuml
[mcp_client]
[mcp_client] --> [anyhow::anyhow]
[mcp_client] --> [futures_util::StreamExt]
[mcp_client] --> [reqwest::Client]
[mcp_client] --> [serde_json::{json, Value}]
[mcp_client] --> [super::*]
[mcp_client] --> [tokio::sync::OnceCell]
[mcp_client] --> [wiremock::matchers::{method, path}]
[mcp_client] --> [wiremock::{Mock, MockServer, ResponseTemplate}]
@enduml

```

## Call Graph

```plantuml
@startuml
Caller --> McpClient::call_tool_json
Caller --> McpHttpClient::new
Caller --> McpSseClient::new
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "Mcp_clientService" as Svc
Caller -> Svc: call_tool_json()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

## Examples

```
// Example usage of mcp_client.rs components
import { ... } from 'crates/factory-infrastructure/src/mcp_client.rs';
```

## Cross References
* **Parent module:** `crates/factory-infrastructure/src`
* **Dependencies:** anyhow::anyhow, futures_util::StreamExt, reqwest::Client, serde_json::{json, Value}, super::*, tokio::sync::OnceCell, wiremock::matchers::{method, path}, wiremock::{Mock, MockServer, ResponseTemplate}
