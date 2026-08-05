---
type: "module-documentation"
title: "zeroclaw_sast_integration.rs"
source_path: "crates/factory-application/tests/zeroclaw_sast_integration.rs"
description: "Detailed documentation for zeroclaw_sast_integration.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-05T05:55:37Z"
---

# File: zeroclaw_sast_integration.rs

**Source Path:** `crates/factory-application/tests/zeroclaw_sast_integration.rs`

## Overview

### Purpose
Provides implementation for zeroclaw_sast_integration.rs.

### Responsibilities
* Handles logic related to zeroclaw_sast_integration.

### Dependencies
* std::sync::Arc, factory_infrastructure::MockMcpClient, serde_json::json, factory_application::agents::ZeroClawAgent

### Imported modules
*

### Exported classes
* None

### Exported interfaces
* None

### Exported functions
*

## Public API

### Exported Classes / Structs / Interfaces

### Exported Functions

None.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class EmptyModule {
    }

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Zeroclaw_sast_integrationService
    Caller->>Svc: test_zeroclaw_blocks_execution_on_sast_failure()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Examples

```
// Example usage of zeroclaw_sast_integration.rs components
import { ... } from 'crates/factory-application/tests/zeroclaw_sast_integration.rs';
```

## Cross References
* **Parent module:** `crates/factory-application/tests`
* **Dependencies:** std::sync::Arc, factory_infrastructure::MockMcpClient, serde_json::json, factory_application::agents::ZeroClawAgent
