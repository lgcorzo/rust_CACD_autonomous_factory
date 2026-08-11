---
type: "module-documentation"
title: "mcp_client.rs"
source_path: "crates/factory-infrastructure/src/mcp_client.rs"
description: "Detailed documentation for mcp_client.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "dfd90f5"
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
Why it exists:
Provides capabilities related to McpClient.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

None.

**Public Methods:**

None.

**Private Methods:**

None.

#### McpHttpClient

**Overview:**
Why it exists:
Provides capabilities related to McpHttpClient.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new(base_url: String (Any))`
Parameters: base_url: String (Any)
Dependencies: Inherited from context
Initialization: Sets up McpHttpClient

**Attributes:**

* `base_url` (String): Purpose - Stores base_url data. Constraints - Valid String.
* `client` (Client): Purpose - Stores client data. Constraints - Valid Client.

**Public Methods:**

None.

**Private Methods:**

* `call_tool_json(name: &str (Any), arguments: Value (Any)) -> anyhow::Result<Value>`: Internal helper logic.

#### McpSseClient

**Overview:**
/// A client that uses SSE handshake to find the session endpoint

**Constructor:**

##### `new(base_url: String (Any))`
Parameters: base_url: String (Any)
Dependencies: Inherited from context
Initialization: Sets up McpSseClient

**Attributes:**

* `base_url` (String): Purpose - Stores base_url data. Constraints - Valid String.
* `client` (Client): Purpose - Stores client data. Constraints - Valid Client.
* `session_url` (OnceCell<String>): Purpose - Stores session_url data. Constraints - Valid OnceCell<String>.

**Public Methods:**

None.

**Private Methods:**

* `call_tool_json(name: &str (Any), arguments: Value (Any)) -> anyhow::Result<Value>`: Internal helper logic.
* `get_session_url() -> anyhow::Result<String>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class McpClient {
        <<trait>>
    }
    class McpHttpClient {
        -call_tool_json(name: &str:Any, arguments: Value:Any) anyhow::Result<Value>
        +new(base_url: String:Any) Self
    }
    McpClient <|-- McpHttpClient : Inheritance / Specialization
    class McpSseClient {
        -call_tool_json(name: &str:Any, arguments: Value:Any) anyhow::Result<Value>
        -get_session_url() anyhow::Result<String>
        +new(base_url: String:Any) Self
    }
    McpClient <|-- McpSseClient : Inheritance / Specialization

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Mcp_clientService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```



## Examples

```
// Example usage of mcp_client.rs components
import { ... } from 'crates/factory-infrastructure/src/mcp_client.rs';
```



## Cross References
* **Parent module:** `crates/factory-infrastructure/src`
* **Dependencies:** anyhow::anyhow, futures_util::StreamExt, reqwest::Client, serde_json::{json, Value}, super::*, tokio::sync::OnceCell, wiremock::matchers::{method, path}, wiremock::{Mock, MockServer, ResponseTemplate}
