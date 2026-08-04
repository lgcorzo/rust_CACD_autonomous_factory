---
type: "module-documentation"
title: "run_tests.rs"
source_path: "crates/factory-mcp-server/src/tools/run_tests.rs"
description: "Detailed documentation for run_tests.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: run_tests.rs

**Source Path:** `crates/factory-mcp-server/src/tools/run_tests.rs`

## Overview

### Purpose
Provides implementation for run_tests.rs.

### Responsibilities
* Handles logic related to run_tests.

### Dependencies
* crate::protocol::{CallToolResult, McpContent}, crate::sandbox::SandboxDriver, async_trait::async_trait, serde_json::{json, Value}, std::sync::Arc, crate::tools::Tool

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### RunTestsTool

**Overview:** Represents RunTestsTool.

**Public Methods:**

##### `new(driver: Arc<dyn SandboxDriver> (Any)) -> Self`
Executes new.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class RunTestsTool {
        +new(driver: Arc<dyn SandboxDriver>:Any) Self
        -name() String
        -description() String
        -input_schema() Value
        -call(_params: Value:Any) anyhow::Result<CallToolResult>
    }
    Tool <|-- RunTestsTool : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Run_testsService
    Caller->>Svc: new()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** crate::protocol::{CallToolResult, McpContent}, crate::sandbox::SandboxDriver, async_trait::async_trait, serde_json::{json, Value}, std::sync::Arc, crate::tools::Tool
