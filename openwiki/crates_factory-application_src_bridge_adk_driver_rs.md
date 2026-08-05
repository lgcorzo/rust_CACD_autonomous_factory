---
type: "module-documentation"
title: "adk_driver.rs"
source_path: "crates/factory-application/src/bridge/adk_driver.rs"
description: "Detailed documentation for adk_driver.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-05T05:55:37Z"
---

# File: adk_driver.rs

**Source Path:** `crates/factory-application/src/bridge/adk_driver.rs`

## Overview

### Purpose
Provides implementation for adk_driver.rs.

### Responsibilities
* Handles logic related to adk_driver.

### Dependencies
* async_trait::async_trait, factory_core::executor::{CodeSurgeryExecutor, ExecutionResult, SurgicalPatch}, std::fs, factory_core::error::FactoryError

### Imported modules
*

### Exported classes
* NativeADKDriver

### Exported interfaces
*

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### NativeADKDriver

**Overview:**
Why it exists:
Provides capabilities related to NativeADKDriver.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `workspace_root` (std::path::PathBuf): Purpose - Stores workspace_root data. Constraints - Valid std::path::PathBuf.

**Public Methods:**

None.

**Private Methods:**

* `apply_patch(_mission_id: &str (Any), patch: &SurgicalPatch (Any)) -> Result<ExecutionResult, FactoryError>`: Internal helper logic.
* `verify_syntax(_file_path: &std::path::Path (Any)) -> Result<bool, FactoryError>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class NativeADKDriver {
        -apply_patch(_mission_id: &str:Any, patch: &SurgicalPatch:Any) Result<ExecutionResult, FactoryError>
        -verify_syntax(_file_path: &std::path::Path:Any) Result<bool, FactoryError>
    }
    CodeSurgeryExecutor <|-- NativeADKDriver : Inheritance / Specialization

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Adk_driverService
    Caller->>Svc: apply_patch()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Examples

```
// Example usage of adk_driver.rs components
import { ... } from 'crates/factory-application/src/bridge/adk_driver.rs';
```

## Cross References
* **Parent module:** `crates/factory-application/src/bridge`
* **Dependencies:** async_trait::async_trait, factory_core::executor::{CodeSurgeryExecutor, ExecutionResult, SurgicalPatch}, std::fs, factory_core::error::FactoryError
