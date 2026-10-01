# factory-core — Crate Overview

> **Source**: `crates/factory-core/src/lib.rs`
> **Layer**: Domain (Onion Architecture — innermost layer)
> **Role**: Contains all domain entities, value objects, error types, security primitives, and protobuf definitions. Has zero dependencies on other factory crates.

---

## Module Tree

```mermaid
graph TB
    LIB["lib.rs"] --> CONFIG["config.rs<br/>AgentModelConfig"]
    LIB --> ERR["error.rs<br/>FactoryError, Result"]
    LIB --> EXEC["executor.rs<br/>CodeSurgeryExecutor trait"]
    LIB --> SEC["security.rs<br/>SecurityValidator, SandboxConstraint"]
    SEC --> NHI["security/nhi.rs<br/>VerifiableCredential, NHI"]
    LIB --> PROTO["proto/v1<br/>gRPC Protobuf"]

    style LIB fill:#4CAF50,stroke:#2E7D32,color:#fff
```

## Re-export Map

| Symbol | Source Module | Re-exported As |
|:---|:---|:---|
| `AgentModelConfig` | `config.rs` | `pub use config::AgentModelConfig` |

## Domain Entities (lib.rs)

```mermaid
classDiagram
    class Mission {
        +Uuid id
        +String name
        +String description
        +DateTime created_at
        +Vec~Task~ tasks
        +MissionStatus status
    }
    class Task {
        +Uuid id
        +Uuid mission_id
        +String description
        +Option~String~ assigned_agent
        +Vec~Uuid~ dependencies
        +TaskStatus status
    }
    class MissionStatus {
        <<enumeration>>
        Pending
        Running
        Completed
        Failed
    }
    class TaskStatus {
        <<enumeration>>
        Queued
        Active
        Finished
        Blocked
    }
    class PRDirective {
        <<enumeration>>
        Spec
        Refine
        Retry
        Status
        Validate
        Interact
        +parse(text) Option~PRDirective~
    }
    class PolledIssueEvent {
        +String source_platform
        +String repository
        +u64 issue_number
        +String title
        +Vec~String~ labels
        +extract_resource_limits(body)$ Option~String~
    }
    class PRCommentEvent {
        +String source_platform
        +u64 pr_number
        +String author
        +PRDirective directive
        +Vec~String~ thread_context
    }
    class PipelineFailureEvent {
        +String source_platform
        +u64 run_id
        +String workflow_name
        +String failing_job
        +String error_log
        +Option~u64~ pr_number
    }
    class ErrorCategory {
        <<enumeration>>
        CodeCompilation
        LintViolation
        TestFailure
        SecurityAudit
        InfrastructureBuild
        InfrastructureTransient
        Unknown
        +is_remediable() bool
    }
    class FinOpsTag {
        +String team
        +String epic
        +String microservice
        +String environment
        +String cost_center
        +to_headers() Vec~Tuple~
    }
    class SddMissionPlan {
        +String mission_id
        +String spec_version
        +Vec~SddTaskItem~ tasks
        +usize total_tasks
    }
    class SddTaskItem {
        +String id
        +String description
        +bool is_parallel
        +Vec~String~ dependencies
        +Vec~String~ target_files
    }

    Mission "1" *-- "*" Task
    Mission --> MissionStatus
    Task --> TaskStatus
    PRCommentEvent --> PRDirective
    PipelineFailureEvent --> ErrorCategory
    SddMissionPlan "1" *-- "*" SddTaskItem
```

## Dependencies

| Crate | Purpose |
|:---|:---|
| `serde` / `serde_json` | Serialization |
| `chrono` | Timestamps |
| `uuid` | Entity identifiers |
| `thiserror` | Error derivation |
| `ed25519-dalek` | Cryptographic signatures |
| `zeroize` | Memory wiping for secrets |
| `async-trait` | Async trait bounds |
| `prost` | Protobuf codegen |

---

> *Related: [error.rs](crates_factory-core_src_error.md) · [executor.rs](crates_factory-core_src_executor.md) · [security.rs](crates_factory-core_src_security.md)*
