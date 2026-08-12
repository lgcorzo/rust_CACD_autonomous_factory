---
type: "module-documentation"
title: "executor.rs"
source_path: "crates/factory-core/src/executor.rs"
description: "Detailed documentation for executor.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "dfd90f5"
---

# File: executor.rs

**Source Path:** `crates/factory-core/src/executor.rs`

## Overview

### Purpose
Provides implementation for executor.rs.

### Responsibilities
* Handles logic related to executor.

### Main Workflow
* Initialization and execution of executor logic.

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
Why it exists:
Provides capabilities related to CodeSurgeryExecutor.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

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
Why it exists:
Provides capabilities related to ExecutionResult.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

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
Why it exists:
Provides capabilities related to SurgicalPatch.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

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

```mermaid
classDiagram
    direction BT
    class CodeSurgeryExecutor {
        <<trait>>
    }
    class ExecutionResult {
    }
    class SurgicalPatch {
    }

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as ExecutorService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## UML

### Class Diagram
```plantuml
@startuml
interface CodeSurgeryExecutor <<trait>> {
}
class ExecutionResult {
}
class SurgicalPatch {
}
@enduml

```

### Package Diagram
```plantuml
@startuml
package "executor" {
  [Module Components]
}
@enduml

```

### Sequence Diagram
```plantuml
@startuml
autonumber
participant Caller as "Client Interface"
participant Svc as "ExecutorService"
Caller -> Svc: execute()
note over Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

### Component Diagram
```plantuml
@startuml
component "executor" as comp
component "async_trait::async_trait" as async_trait::async_trait
comp --> async_trait::async_trait
component "crate::error::FactoryError" as crate::error::FactoryError
comp --> crate::error::FactoryError
component "std::path::PathBuf" as std::path::PathBuf
comp --> std::path::PathBuf
@enduml

```

### Dependency Graph
```plantuml
@startuml
[executor]
[executor] --> [async_trait::async_trait]
[executor] --> [crate::error::FactoryError]
[executor] --> [std::path::PathBuf]
@enduml

```

### Call Graph
```plantuml
@startuml
[API] --> [No Public API]
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
