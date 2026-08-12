---
type: "module-documentation"
title: "sandbox.rs"
source_path: "crates/factory-mcp-server/src/sandbox.rs"
description: "Detailed documentation for sandbox.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "dfd90f5"
---

# File: sandbox.rs

**Source Path:** `crates/factory-mcp-server/src/sandbox.rs`

## Overview

### Purpose
Provides implementation for sandbox.rs.

### Responsibilities
* Handles logic related to sandbox.

### Main Workflow
* Initialization and execution of sandbox logic.

### Dependencies
* async_trait::async_trait, crate::tools::Tool, crate::tools::launch_sandbox_pod::LaunchSandboxPodTool, serde::{Deserialize, Serialize}, serde_json::json, std::time::Duration, super::*, tokio::process::Command, tokio::time::timeout

### Imported modules
* None

### Exported classes
* ExecutionResult, GvisorK8sDriver, NativeSurgerySandboxDriver, SubprocessDriver

### Exported interfaces
* SandboxDriver

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

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

* `exit_code` (Option<i32>): Purpose - Stores exit_code data. Constraints - Valid Option<i32>.
* `is_success` (bool): Purpose - Stores is_success data. Constraints - Valid bool.
* `stderr` (String): Purpose - Stores stderr data. Constraints - Valid String.
* `stdout` (String): Purpose - Stores stdout data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

None.

#### GvisorK8sDriver

**Overview:**
Why it exists:
Provides capabilities related to GvisorK8sDriver.

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

* `execute(code: &str (Any), language: &str (Any)) -> anyhow::Result<ExecutionResult>`: Internal helper logic.

#### NativeSurgerySandboxDriver

**Overview:**
Why it exists:
Provides capabilities related to NativeSurgerySandboxDriver.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `execution_engine` (std::sync::Arc<dyn factory_core::executor::CodeSurgeryExecutor>): Purpose - Stores execution_engine data. Constraints - Valid std::sync::Arc<dyn factory_core::executor::CodeSurgeryExecutor>.

**Public Methods:**

None.

**Private Methods:**

* `execute(_code: &str (Any), _language: &str (Any)) -> anyhow::Result<ExecutionResult>`: Internal helper logic.
* `execute_surgery(id: &str (Any), patch: &factory_core::executor::SurgicalPatch (Any)) -> factory_core::error::Result<factory_core::executor::ExecutionResult>`: Internal helper logic.

#### SandboxDriver

**Overview:**
Why it exists:
Provides capabilities related to SandboxDriver.

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

#### SandboxMode

**Overview:**
Why it exists:
Provides capabilities related to SandboxMode.

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

#### SubprocessDriver

**Overview:**
Why it exists:
Provides capabilities related to SubprocessDriver.

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

* `execute(code: &str (Any), language: &str (Any)) -> anyhow::Result<ExecutionResult>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class ExecutionResult {
    }
    class GvisorK8sDriver {
        -execute(code: &str:Any, language: &str:Any) anyhow::Result<ExecutionResult>
    }
    SandboxDriver <|-- GvisorK8sDriver : Inheritance / Specialization
    class NativeSurgerySandboxDriver {
        -execute(_code: &str:Any, _language: &str:Any) anyhow::Result<ExecutionResult>
        -execute_surgery(id: &str:Any, patch: &factory_core::executor::SurgicalPatch:Any) factory_core::error::Result<factory_core::executor::ExecutionResult>
    }
    SandboxDriver <|-- NativeSurgerySandboxDriver : Inheritance / Specialization
    class SandboxDriver {
        <<trait>>
    }
    class SandboxMode {
        <<enumeration>>
    }
    class SubprocessDriver {
        -execute(code: &str:Any, language: &str:Any) anyhow::Result<ExecutionResult>
    }
    SandboxDriver <|-- SubprocessDriver : Inheritance / Specialization

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as SandboxService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## UML

### Class Diagram
```plantuml
@startuml
class ExecutionResult {
}
class GvisorK8sDriver {
    -execute(code: &str:Any, language: &str:Any) : anyhow::Result<ExecutionResult>
}
SandboxDriver <|-- GvisorK8sDriver : Inheritance
class NativeSurgerySandboxDriver {
    -execute(_code: &str:Any, _language: &str:Any) : anyhow::Result<ExecutionResult>
    -execute_surgery(id: &str:Any, patch: &factory_core::executor::SurgicalPatch:Any) : factory_core::error::Result<factory_core::executor::ExecutionResult>
}
SandboxDriver <|-- NativeSurgerySandboxDriver : Inheritance
interface SandboxDriver <<trait>> {
}
enum SandboxMode {
}
class SubprocessDriver {
    -execute(code: &str:Any, language: &str:Any) : anyhow::Result<ExecutionResult>
}
SandboxDriver <|-- SubprocessDriver : Inheritance
@enduml

```

### Package Diagram
```plantuml
@startuml
package "sandbox" {
  [Module Components]
}
@enduml

```

### Sequence Diagram
```plantuml
@startuml
autonumber
participant Caller as "Client Interface"
participant Svc as "SandboxService"
Caller -> Svc: execute()
note over Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

### Component Diagram
```plantuml
@startuml
component "sandbox" as comp
component "async_trait::async_trait" as async_trait::async_trait
comp --> async_trait::async_trait
component "crate::tools::Tool" as crate::tools::Tool
comp --> crate::tools::Tool
component "crate::tools::launch_sandbox_pod::LaunchSandboxPodTool" as crate::tools::launch_sandbox_pod::LaunchSandboxPodTool
comp --> crate::tools::launch_sandbox_pod::LaunchSandboxPodTool
component "serde::{Deserialize, Serialize}" as serde::{Deserialize, Serialize}
comp --> serde::{Deserialize, Serialize}
component "serde_json::json" as serde_json::json
comp --> serde_json::json
component "std::time::Duration" as std::time::Duration
comp --> std::time::Duration
component "super::*" as super::*
comp --> super::*
component "tokio::process::Command" as tokio::process::Command
comp --> tokio::process::Command
component "tokio::time::timeout" as tokio::time::timeout
comp --> tokio::time::timeout
@enduml

```

### Dependency Graph
```plantuml
@startuml
[sandbox]
[sandbox] --> [async_trait::async_trait]
[sandbox] --> [crate::tools::Tool]
[sandbox] --> [crate::tools::launch_sandbox_pod::LaunchSandboxPodTool]
[sandbox] --> [serde::{Deserialize, Serialize}]
[sandbox] --> [serde_json::json]
[sandbox] --> [std::time::Duration]
[sandbox] --> [super::*]
[sandbox] --> [tokio::process::Command]
[sandbox] --> [tokio::time::timeout]
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
// Example usage of sandbox.rs components
import { ... } from 'crates/factory-mcp-server/src/sandbox.rs';
```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src`
* **Dependencies:** async_trait::async_trait, crate::tools::Tool, crate::tools::launch_sandbox_pod::LaunchSandboxPodTool, serde::{Deserialize, Serialize}, serde_json::json, std::time::Duration, super::*, tokio::process::Command, tokio::time::timeout
