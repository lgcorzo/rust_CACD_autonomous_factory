---
type: module
title: "mcp_client.rs"
source_path: "crates/factory-infrastructure/src/mcp_client.rs"
description: "Documentation for crates/factory-infrastructure/src/mcp_client.rs"
tags: [rust, module]
last_verified_commit: "17a28f4"
---

# mcp_client.rs

Source File: `crates/factory-infrastructure/src/mcp_client.rs`

## Component Architecture

```mermaid
classDiagram
    class McpClient {
        <<trait>>
    }
    class McpHttpClient
    class McpSseClient
```

## Execution Flow

```mermaid
flowchart TD
    Start --> call_tool_json
    call_tool_json --> call_tool_json
    call_tool_json --> new
    new --> new
    new --> get_session_url
    get_session_url --> call_tool_json
    call_tool_json --> test_call_tool_http_success
    test_call_tool_http_success --> test_call_tool_sse_success
    test_call_tool_sse_success --> End
```
