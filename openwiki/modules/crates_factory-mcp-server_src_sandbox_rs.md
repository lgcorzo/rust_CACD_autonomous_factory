---
type: "module-documentation"
title: "sandbox.rs"
source_path: "crates/factory-mcp-server/src/sandbox.rs"
description: "Detailed documentation for sandbox.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: sandbox.rs

**Source Path:** `crates/factory-mcp-server/src/sandbox.rs`

## Overview

### Purpose
Provides implementation for sandbox.rs.

### Responsibilities
* Handles logic related to sandbox.

### Dependencies
* serde::{Deserialize, Serialize}, serde_json::json, super::*, std::time::Duration, crate::tools::launch_sandbox_pod::LaunchSandboxPodTool, async_trait::async_trait, crate::tools::Tool, tokio::time::timeout, tokio::process::Command

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### ExecutionResult

**Overview:** Represents ExecutionResult.

**Public Methods:**

None.

#### SandboxDriver

**Overview:** Represents SandboxDriver.

**Public Methods:**

None.

#### NativeSurgerySandboxDriver

**Overview:** Represents NativeSurgerySandboxDriver.

**Public Methods:**

None.

#### SubprocessDriver

**Overview:** Represents SubprocessDriver.

**Public Methods:**

None.

#### SandboxMode

**Overview:** Represents SandboxMode.

**Public Methods:**

None.

#### GvisorK8sDriver

**Overview:** Represents GvisorK8sDriver.

**Public Methods:**

None.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class ExecutionResult {
    }
    class SandboxDriver {
        <<trait>>
    }
    class NativeSurgerySandboxDriver {
        -execute(_code: &str:Any, _language: &str:Any) anyhow::Result<ExecutionResult>
        -execute_surgery(id: &str:Any, patch: &factory_core::executor::SurgicalPatch:Any) factory_core::error::Result<factory_core::executor::ExecutionResult>
    }
    SandboxDriver <|-- NativeSurgerySandboxDriver : Inheritance / Specialization
    class SubprocessDriver {
        -execute(code: &str:Any, language: &str:Any) anyhow::Result<ExecutionResult>
    }
    SandboxDriver <|-- SubprocessDriver : Inheritance / Specialization
    class SandboxMode {
        <<enumeration>>
    }
    class GvisorK8sDriver {
        -execute(code: &str:Any, language: &str:Any) anyhow::Result<ExecutionResult>
    }
    SandboxDriver <|-- GvisorK8sDriver : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as SandboxService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src`
* **Dependencies:** serde::{Deserialize, Serialize}, serde_json::json, super::*, std::time::Duration, crate::tools::launch_sandbox_pod::LaunchSandboxPodTool, async_trait::async_trait, crate::tools::Tool, tokio::time::timeout, tokio::process::Command
