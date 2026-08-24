---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "sandbox.rs"
source_path: "crates/factory-mcp-server/src/sandbox.rs"
description: "Detailed documentation for sandbox.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-24T05:31:17Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "d073f5f"
---

# File: sandbox.rs

**Source Path:** `crates/factory-mcp-server/src/sandbox.rs`

## Overview

### Purpose
Provides implementation for sandbox.rs.

### Responsibilities
* Handles logic related to sandbox.

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
No description provided.

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
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

None.

**Public Methods:**

None.

**Private Methods:**

* `execute(code (&str), language (&str)) -> anyhow::Result<ExecutionResult>`: Internal helper logic.

#### NativeSurgerySandboxDriver

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

* `execution_engine` (std::sync::Arc<dyn factory_core::executor::CodeSurgeryExecutor>): Purpose - Stores execution_engine data. Constraints - Valid std::sync::Arc<dyn factory_core::executor::CodeSurgeryExecutor>.

**Public Methods:**

None.

**Private Methods:**

* `execute(_code (&str), _language (&str)) -> anyhow::Result<ExecutionResult>`: Internal helper logic.
* `execute_surgery(id (&str), patch (&factory_core::executor::SurgicalPatch)) -> factory_core::error::Result<factory_core::executor::ExecutionResult>`: Internal helper logic.

#### SandboxDriver

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

#### SandboxMode

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

#### SubprocessDriver

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

None.

**Public Methods:**

None.

**Private Methods:**

* `execute(code (&str), language (&str)) -> anyhow::Result<ExecutionResult>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class ExecutionResult {
}
class GvisorK8sDriver {
    -execute(code: &str, language: &str) anyhow::Result<ExecutionResult>
}
SandboxDriver <|-- GvisorK8sDriver : extends/implements
class NativeSurgerySandboxDriver {
    -execute(_code: &str, _language: &str) anyhow::Result<ExecutionResult>
    -execute_surgery(id: &str, patch: &factory_core::executor::SurgicalPatch) factory_core::error::Result<factory_core::executor::ExecutionResult>
}
SandboxDriver <|-- NativeSurgerySandboxDriver : extends/implements
interface SandboxDriver {
}
enum SandboxMode {
}
class SubprocessDriver {
    -execute(code: &str, language: &str) anyhow::Result<ExecutionResult>
}
SandboxDriver <|-- SubprocessDriver : extends/implements
@enduml

```

## Package Diagram

```plantuml
@startuml
package "crates" {
    package "factory-mcp-server" {
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
component "sandbox" as Main
component "async_trait::async_trait" as async_trait__async_trait
Main --> async_trait__async_trait : uses
component "crate::tools::Tool" as crate__tools__Tool
Main --> crate__tools__Tool : uses
component "crate::tools::launch_sandbox_pod::LaunchSandboxPodTool" as crate__tools__launch_sandbox_pod__LaunchSandboxPodTool
Main --> crate__tools__launch_sandbox_pod__LaunchSandboxPodTool : uses
component "serde::{Deserialize, Serialize}" as serde___Deserialize__Serialize_
Main --> serde___Deserialize__Serialize_ : uses
component "serde_json::json" as serde_json__json
Main --> serde_json__json : uses
component "std::time::Duration" as std__time__Duration
Main --> std__time__Duration : uses
component "super::*" as super___
Main --> super___ : uses
component "tokio::process::Command" as tokio__process__Command
Main --> tokio__process__Command : uses
component "tokio::time::timeout" as tokio__time__timeout
Main --> tokio__time__timeout : uses
@enduml

```

## Dependency Graph

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
participant "SandboxService" as Svc
Caller -> Svc: execute()
note right of Svc: Processing internal logic
Svc --> Caller: result
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
