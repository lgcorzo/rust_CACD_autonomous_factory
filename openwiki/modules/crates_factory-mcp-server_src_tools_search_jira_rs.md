---
type: "module-documentation"
title: "search_jira.rs"
source_path: "crates/factory-mcp-server/src/tools/search_jira.rs"
description: "Detailed documentation for search_jira.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: search_jira.rs

**Source Path:** `crates/factory-mcp-server/src/tools/search_jira.rs`

## Overview

### Purpose
Provides implementation for search_jira.rs.

### Responsibilities
* Handles logic related to search_jira.

### Dependencies
* std::sync::Arc, async_trait::async_trait, super::*, crate::tools::Tool, factory_infrastructure::JiraClient, serde_json::{json, Value}, crate::protocol::{CallToolResult, McpContent}

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### SearchJiraTool

**Overview:** Represents SearchJiraTool.

**Public Methods:**

##### `new(jira_client: Arc<dyn JiraClient> (Any)) -> Self`
Executes new.

#### ManualMockJiraClient

**Overview:** Represents ManualMockJiraClient.

**Public Methods:**

None.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class SearchJiraTool {
        +new(jira_client: Arc<dyn JiraClient>:Any) Self
        -name() String
        -description() String
        -input_schema() Value
        -call(params: Value:Any) anyhow::Result<CallToolResult>
    }
    Tool <|-- SearchJiraTool : Inheritance / Specialization
    class ManualMockJiraClient {
        -search_issues(_query: &str:Any) anyhow::Result<String>
    }
    JiraClient <|-- ManualMockJiraClient : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Search_jiraService
    Caller->>Svc: new()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** std::sync::Arc, async_trait::async_trait, super::*, crate::tools::Tool, factory_infrastructure::JiraClient, serde_json::{json, Value}, crate::protocol::{CallToolResult, McpContent}
