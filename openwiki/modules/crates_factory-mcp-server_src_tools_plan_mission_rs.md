---
type: "module-documentation"
title: "plan_mission.rs"
source_path: "crates/factory-mcp-server/src/tools/plan_mission.rs"
description: "Detailed documentation for plan_mission.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: plan_mission.rs

**Source Path:** `crates/factory-mcp-server/src/tools/plan_mission.rs`

## Overview

### Purpose
Provides implementation for plan_mission.rs.

### Responsibilities
* Handles logic related to plan_mission.

### Dependencies
* factory_core::FinOpsTag, async_trait::async_trait, crate::tools::Tool, serde_json::{json, Value}, async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        ChatCompletionResponseFormat, ChatCompletionResponseFormatType,
        CreateChatCompletionRequestArgs,
    },
    Client,
}, crate::protocol::{CallToolResult, McpContent}, reqwest::header::{HeaderMap, HeaderValue}

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### PlanMissionTool

**Overview:** Represents PlanMissionTool.

**Public Methods:**

##### `new(api_key: String (Any), base_url: String (Any), model: String (Any), finops_tag: FinOpsTag (Any)) -> Self`
Executes new.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class PlanMissionTool {
        +new(api_key: String:Any, base_url: String:Any, model: String:Any, finops_tag: FinOpsTag:Any) Self
        -name() String
        -description() String
        -input_schema() Value
        -call(params: Value:Any) anyhow::Result<CallToolResult>
    }
    Tool <|-- PlanMissionTool : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Plan_missionService
    Caller->>Svc: new()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** factory_core::FinOpsTag, async_trait::async_trait, crate::tools::Tool, serde_json::{json, Value}, async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        ChatCompletionResponseFormat, ChatCompletionResponseFormatType,
        CreateChatCompletionRequestArgs,
    },
    Client,
}, crate::protocol::{CallToolResult, McpContent}, reqwest::header::{HeaderMap, HeaderValue}
