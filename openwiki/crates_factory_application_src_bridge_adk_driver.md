---
type: "module-documentation"
title: "adk_driver.rs"
source_path: "crates/factory-application/src/bridge/adk_driver.rs"
description: "Detailed documentation for adk_driver.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "e48839f"
---

# File: adk_driver.rs

**Source Path:** `crates/factory-application/src/bridge/adk_driver.rs`

## Overview

### Purpose
Provides implementation for adk_driver.rs.

### Responsibilities
* Handles logic related to adk_driver.

### Dependencies
* async_trait::async_trait, factory_core::error::FactoryError, factory_core::executor::{CodeSurgeryExecutor, ExecutionResult, SurgicalPatch}, std::fs

### Imported modules
* None

### Exported classes
* NativeADKDriver

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### NativeADKDriver

**Overview:**
No description provided.

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

```plantuml
@startuml
class NativeADKDriver {
    -apply_patch(_mission_id: &str:Any, patch: &SurgicalPatch:Any) : Result<ExecutionResult, FactoryError>
    -verify_syntax(_file_path: &std::path::Path:Any) : Result<bool, FactoryError>
}
CodeSurgeryExecutor <|-- NativeADKDriver : extends/implements
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "Adk_driverService" as Svc
Caller -> Svc: apply_patch()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

## Examples

```
// Example usage of adk_driver.rs components
import { ... } from 'crates/factory-application/src/bridge/adk_driver.rs';
```

## Cross References
* **Parent module:** `crates/factory-application/src/bridge`
* **Dependencies:** async_trait::async_trait, factory_core::error::FactoryError, factory_core::executor::{CodeSurgeryExecutor, ExecutionResult, SurgicalPatch}, std::fs
