---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "Module: factory-mcp-server::sandbox"
source_path: "crates/factory-mcp-server/src/sandbox.rs"
description: "Multi-backend execution engine supporting Subprocess, GvisorK8s, and NativeSurgery sandbox drivers."
tags: ["factory_mcp_server", "sandbox", "gvisor", "subprocess"]
last_verified_commit: "main"
timestamp: "2026-07-31T16:35:00Z"
---

# Module Specification: `factory-mcp-server::sandbox`

* **Source File Reference:** `crates/factory-mcp-server/src/sandbox.rs` (Lines: L1-L190)
* **Upstream Dependencies:** [[Modules/FactoryCore/Executor|factory-core::executor]], [[Modules/FactoryCore/Error|factory-core::error]]
* **Downstream Consumers:** MCP Tool Handlers (`crates/factory-mcp-server/src/tools/`)

---

## 1. Architectural Role & Responsibilities

Provides the execution core for code blocks and surgical patches across three drivers:
1. `NativeSurgerySandboxDriver`: Executes AST surgical patches using `CodeSurgeryExecutor`.
2. `SubprocessDriver`: Local CLI process execution with a strict 30-second timeout.
3. `GvisorK8sDriver`: Containerized Kubernetes pod execution isolated via gVisor.

---

## 2. UML 2.0 Class Diagram

```mermaid
classDiagram
    direction BT
    class SandboxDriver {
        <<interface>>
        +execute(code: &str, language: &str)* Result~ExecutionResult~
        +execute_surgery(id: &str, patch: &SurgicalPatch) Result~ExecutionResult~
    }
    class NativeSurgerySandboxDriver {
        +execution_engine: Arc~CodeSurgeryExecutor~
        +execute_surgery(id: &str, patch: &SurgicalPatch) Result~ExecutionResult~
    }
    class SubprocessDriver {
        +execute(code: &str, language: &str) Result~ExecutionResult~
    }
    class GvisorK8sDriver {
        +execute(code: &str, language: &str) Result~ExecutionResult~
    }
    SandboxDriver <|.. NativeSurgerySandboxDriver : Realization
    SandboxDriver <|.. SubprocessDriver : Realization
    SandboxDriver <|.. GvisorK8sDriver : Realization
```

---

## 3. Driver Method Contracts

### `SubprocessDriver::execute(code: &str, language: &str)`
- **Source Line Citation:** `crates/factory-mcp-server/src/sandbox.rs:L52-L111`
- **Supported Languages**: Python (`python3 -c`), Rust (`rustc`), Go (`go run`), TypeScript (`ts-node -e`).
- **Timeout**: Enforces 30-second execution bound via `tokio::time::timeout`.

### `GvisorK8sDriver::execute(code: &str, language: &str)`
- **Source Line Citation:** `crates/factory-mcp-server/src/sandbox.rs:L121-L175`
- **Isolation**: Invokes `LaunchSandboxPodTool` to run code inside a gVisor sandboxed Kubernetes pod.
