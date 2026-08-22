---
type: "module-documentation"
title: "executor.rs"
source_path: "crates/factory-core/src/executor.rs"
description: "Detailed documentation for executor.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "ec7bd0f"
---

# File: executor.rs

**Source Path:** `crates/factory-core/src/executor.rs`

## Overview

### Purpose
Provides implementation for executor.rs.

### Responsibilities
* Handles logic related to executor.

### Dependencies
* async_trait::async_trait, crate::error::FactoryError, std::path::PathBuf

### Imported modules
* None

### Exported classes
* ExecutionResult, SurgicalPatch

### Exported interfaces
* CodeSurgeryExecutor

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### CodeSurgeryExecutor

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

None.

**Public Methods:**

None.

**Private Methods:**

None.

#### ExecutionResult

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

* `commit_sha` (Option<String>): Purpose - Stores commit_sha data. Constraints - Valid Option<String>.
* `lines_modified` (usize): Purpose - Stores lines_modified data. Constraints - Valid usize.
* `success` (bool): Purpose - Stores success data. Constraints - Valid bool.

**Public Methods:**

None.

**Private Methods:**

None.

#### SurgicalPatch

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

* `file_path` (PathBuf): Purpose - Stores file_path data. Constraints - Valid PathBuf.
* `replace_block` (String): Purpose - Stores replace_block data. Constraints - Valid String.
* `search_block` (String): Purpose - Stores search_block data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

None.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
interface CodeSurgeryExecutor {
}
class ExecutionResult {
}
class SurgicalPatch {
}
@enduml

```

## Package Diagram

```plantuml
@startuml
package "crates" {
    package "factory-core" {
        package "src" {
            class Module
        }
    }
}
@enduml

```

## Component Diagram

```plantuml
@startuml
component "executor" as Main
component "async_trait::async_trait" as async_trait__async_trait
Main --> async_trait__async_trait : uses
component "crate::error::FactoryError" as crate__error__FactoryError
Main --> crate__error__FactoryError : uses
component "std::path::PathBuf" as std__path__PathBuf
Main --> std__path__PathBuf : uses
@enduml

```

## Dependency Graph

```plantuml
@startuml
[executor]
[executor] --> [async_trait::async_trait]
[executor] --> [crate::error::FactoryError]
[executor] --> [std::path::PathBuf]
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
participant "ExecutorService" as Svc
Caller -> Svc: execute()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

## Examples

```
// Example usage of executor.rs components
import { ... } from 'crates/factory-core/src/executor.rs';
```

## Cross References
* **Parent module:** `crates/factory-core/src`
* **Dependencies:** async_trait::async_trait, crate::error::FactoryError, std::path::PathBuf
