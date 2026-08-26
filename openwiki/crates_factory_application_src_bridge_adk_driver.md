---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "adk_driver.rs"
source_path: "crates/factory-application/src/bridge/adk_driver.rs"
description: "Detailed documentation for adk_driver.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-26T06:00:08Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "e2707de"
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

* `apply_patch(_mission_id (&str), patch (&SurgicalPatch)) -> Result<ExecutionResult, FactoryError>`: Internal helper logic.
* `verify_syntax(_file_path (&std::path::Path)) -> Result<bool, FactoryError>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class NativeADKDriver {
    -apply_patch(_mission_id: &str, patch: &SurgicalPatch) Result<ExecutionResult, FactoryError>
    -verify_syntax(_file_path: &std::path::Path) Result<bool, FactoryError>
}
CodeSurgeryExecutor <|-- NativeADKDriver : extends/implements
@enduml

```

## Package Diagram

```plantuml
@startuml
package "crates" {
    package "factory-application" {
        package "src" {
            package "bridge" {
                class Module
            }
        }
    }
}
@enduml

```

## Component Diagram

```plantuml
@startuml
component "adk_driver" as Main
component "async_trait::async_trait" as async_trait__async_trait
Main --> async_trait__async_trait : uses
component "factory_core::error::FactoryError" as factory_core__error__FactoryError
Main --> factory_core__error__FactoryError : uses
component "factory_core::executor::{CodeSurgeryExecutor, ExecutionResult, SurgicalPatch}" as factory_core__executor___CodeSurgeryExecutor__ExecutionResult__SurgicalPatch_
Main --> factory_core__executor___CodeSurgeryExecutor__ExecutionResult__SurgicalPatch_ : uses
component "std::fs" as std__fs
Main --> std__fs : uses
@enduml

```

## Dependency Graph

```plantuml
@startuml
[adk_driver]
[adk_driver] --> [async_trait::async_trait]
[adk_driver] --> [factory_core::error::FactoryError]
[adk_driver] --> [factory_core::executor::{CodeSurgeryExecutor, ExecutionResult, SurgicalPatch}]
[adk_driver] --> [std::fs]
@enduml

```

## Call Graph

```plantuml
@startuml
Caller --> Module : no public API
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
