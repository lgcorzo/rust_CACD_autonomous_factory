---
type: "module-documentation"
title: "workflow_tests.rs"
source_path: "crates/factory-application/tests/workflow_tests.rs"
description: "Detailed documentation for workflow_tests.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: workflow_tests.rs

**Source Path:** `crates/factory-application/tests/workflow_tests.rs`

## Overview

### Purpose
Provides implementation for workflow_tests.rs.

### Responsibilities
* Handles logic related to workflow_tests.

### Dependencies
* factory_infrastructure::{MockMcpClient, MockR2rClient}, std::sync::Arc, factory_application::agents::RustantAgent, serde_json::json

## Public API & Architecture

### Exported Classes / Structs / Interfaces

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class EmptyModule {
    }

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Workflow_testsService
    Caller->>Svc: test_rustant_agent_with_mock_mcp()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-application/tests`
* **Dependencies:** factory_infrastructure::{MockMcpClient, MockR2rClient}, std::sync::Arc, factory_application::agents::RustantAgent, serde_json::json
