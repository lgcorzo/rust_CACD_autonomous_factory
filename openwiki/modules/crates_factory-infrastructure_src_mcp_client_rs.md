---
type: "module-documentation"
title: "mcp_client.rs"
source_path: "crates/factory-infrastructure/src/mcp_client.rs"
description: "Detailed documentation for mcp_client.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: mcp_client.rs

**Source Path:** `crates/factory-infrastructure/src/mcp_client.rs`

## Overview

### Purpose
Provides implementation for mcp_client.rs.

### Responsibilities
* Handles logic related to mcp_client.

### Dependencies
* tokio::sync::OnceCell, anyhow::anyhow, reqwest::Client, wiremock::matchers::{method, path}, serde_json::{json, Value}, futures_util::StreamExt, wiremock::{Mock, MockServer, ResponseTemplate}, super::*

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### McpClient

**Overview:** Represents McpClient.

**Public Methods:**

None.

#### McpHttpClient

**Overview:** Represents McpHttpClient.

**Public Methods:**

##### `new(base_url: String (Any)) -> Self`
Executes new.

#### McpSseClient

**Overview:** /// A client that uses SSE handshake to find the session endpoint

**Public Methods:**

##### `new(base_url: String (Any)) -> Self`
Executes new.

### Exported Functions

None.

## Internal Architecture & Execution Flow

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
        +new(base_url: String:Any) Self
        -get_session_url() anyhow::Result<String>
        -call_tool_json(name: &str:Any, arguments: Value:Any) anyhow::Result<Value>
    }
    McpClient <|-- McpSseClient : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Mcp_clientService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-infrastructure/src`
* **Dependencies:** tokio::sync::OnceCell, anyhow::anyhow, reqwest::Client, wiremock::matchers::{method, path}, serde_json::{json, Value}, futures_util::StreamExt, wiremock::{Mock, MockServer, ResponseTemplate}, super::*
