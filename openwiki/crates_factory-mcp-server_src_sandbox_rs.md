---
type: "module-documentation"
title: "sandbox.rs"
source_path: "crates/factory-mcp-server/src/sandbox.rs"
description: "Detailed documentation for sandbox.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-07T06:39:28Z"
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
*

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

* `stdout` (String): Purpose - Stores stdout data. Constraints - Valid String.
* `stderr` (String): Purpose - Stores stderr data. Constraints - Valid String.
* `exit_code` (Option<i32>): Purpose - Stores exit_code data. Constraints - Valid Option<i32>.
* `is_success` (bool): Purpose - Stores is_success data. Constraints - Valid bool.

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

## Examples

```
// Example usage of sandbox.rs components
import { ... } from 'crates/factory-mcp-server/src/sandbox.rs';
```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src`
* **Dependencies:** async_trait::async_trait, crate::tools::Tool, crate::tools::launch_sandbox_pod::LaunchSandboxPodTool, serde::{Deserialize, Serialize}, serde_json::json, std::time::Duration, super::*, tokio::process::Command, tokio::time::timeout
