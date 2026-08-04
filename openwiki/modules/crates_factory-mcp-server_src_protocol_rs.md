---
type: "module-documentation"
title: "protocol.rs"
source_path: "crates/factory-mcp-server/src/protocol.rs"
description: "Detailed documentation for protocol.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: protocol.rs

**Source Path:** `crates/factory-mcp-server/src/protocol.rs`

## Overview

### Purpose
Provides implementation for protocol.rs.

### Responsibilities
* Handles logic related to protocol.

### Dependencies
* serde::{Deserialize, Serialize}

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### JsonRpcRequest

**Overview:** Represents JsonRpcRequest.

**Public Methods:**

None.

#### JsonRpcResponse

**Overview:** Represents JsonRpcResponse.

**Public Methods:**

None.

#### JsonRpcError

**Overview:** Represents JsonRpcError.

**Public Methods:**

None.

#### McpTool

**Overview:** Represents McpTool.

**Public Methods:**

None.

#### CallToolResult

**Overview:** Represents CallToolResult.

**Public Methods:**

None.

#### McpContent

**Overview:** Represents McpContent.

**Public Methods:**

None.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class JsonRpcRequest {
    }
    class JsonRpcResponse {
    }
    class JsonRpcError {
    }
    class McpTool {
    }
    class CallToolResult {
    }
    class McpContent {
        <<enumeration>>
    }

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as ProtocolService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src`
* **Dependencies:** serde::{Deserialize, Serialize}
