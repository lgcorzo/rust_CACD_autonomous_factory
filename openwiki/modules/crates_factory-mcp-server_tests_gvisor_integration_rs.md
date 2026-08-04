---
type: "module-documentation"
title: "gvisor_integration.rs"
source_path: "crates/factory-mcp-server/tests/gvisor_integration.rs"
description: "Detailed documentation for gvisor_integration.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: gvisor_integration.rs

**Source Path:** `crates/factory-mcp-server/tests/gvisor_integration.rs`

## Overview

### Purpose
Provides implementation for gvisor_integration.rs.

### Responsibilities
* Handles logic related to gvisor_integration.

### Dependencies
* serde_json::json, kube::{
    api::{Api, PostParams},
    Client,
}, factory_mcp_server::sandbox::{GvisorK8sDriver, SandboxDriver}, k8s_openapi::api::core::v1::Namespace

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
    participant Svc as Gvisor_integrationService
    Caller->>Svc: test_gvisor_k8s_driver_live_connection()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-mcp-server/tests`
* **Dependencies:** serde_json::json, kube::{
    api::{Api, PostParams},
    Client,
}, factory_mcp_server::sandbox::{GvisorK8sDriver, SandboxDriver}, k8s_openapi::api::core::v1::Namespace
