---
type: "module-documentation"
title: "zeroclaw_sast_integration.rs"
source_path: "crates/factory-application/tests/zeroclaw_sast_integration.rs"
description: "Detailed documentation for zeroclaw_sast_integration.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: zeroclaw_sast_integration.rs

**Source Path:** `crates/factory-application/tests/zeroclaw_sast_integration.rs`

## Overview

### Purpose
Provides implementation for zeroclaw_sast_integration.rs.

### Responsibilities
* Handles logic related to zeroclaw_sast_integration.

### Dependencies
* factory_application::agents::ZeroClawAgent, serde_json::json, std::sync::Arc, factory_infrastructure::MockMcpClient

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
    participant Svc as Zeroclaw_sast_integrationService
    Caller->>Svc: test_zeroclaw_blocks_execution_on_sast_failure()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-application/tests`
* **Dependencies:** factory_application::agents::ZeroClawAgent, serde_json::json, std::sync::Arc, factory_infrastructure::MockMcpClient
