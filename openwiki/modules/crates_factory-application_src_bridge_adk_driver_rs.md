---
type: "module-documentation"
title: "adk_driver.rs"
source_path: "crates/factory-application/src/bridge/adk_driver.rs"
description: "Detailed documentation for adk_driver.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: adk_driver.rs

**Source Path:** `crates/factory-application/src/bridge/adk_driver.rs`

## Overview

### Purpose
Provides implementation for adk_driver.rs.

### Responsibilities
* Handles logic related to adk_driver.

### Dependencies
* async_trait::async_trait, std::fs, factory_core::error::FactoryError, factory_core::executor::{CodeSurgeryExecutor, ExecutionResult, SurgicalPatch}

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### NativeADKDriver

**Overview:** Represents NativeADKDriver.

**Public Methods:**

None.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class NativeADKDriver {
        -apply_patch(_mission_id: &str:Any, patch: &SurgicalPatch:Any) Result<ExecutionResult, FactoryError>
        -verify_syntax(_file_path: &std::path::Path:Any) Result<bool, FactoryError>
    }
    CodeSurgeryExecutor <|-- NativeADKDriver : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Adk_driverService
    Caller->>Svc: apply_patch()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-application/src/bridge`
* **Dependencies:** async_trait::async_trait, std::fs, factory_core::error::FactoryError, factory_core::executor::{CodeSurgeryExecutor, ExecutionResult, SurgicalPatch}
