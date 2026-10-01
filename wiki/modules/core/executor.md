---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-core::executor — Code Surgery Execution"
source_path: "crates/factory-core/src/executor.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-core::executor — Code Surgery Execution."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-core::executor — Code Surgery Execution

> **Source**: `crates/factory-core/src/executor.rs`
> **Layer**: Domain

---

## UML Class Diagram

```mermaid
classDiagram
    class SurgicalPatch {
        +PathBuf file_path
        +String search_block
        +String replace_block
    }
    class ExecutionResult {
        +bool success
        +Option~String~ commit_sha
        +usize lines_modified
    }
    class CodeSurgeryExecutor {
        <<trait>>
        +apply_patch(mission_id, patch) Result~ExecutionResult~
        +verify_syntax(file_path) Result~bool~
    }

    CodeSurgeryExecutor ..> SurgicalPatch : accepts
    CodeSurgeryExecutor ..> ExecutionResult : returns
```

## Sequence: Task Dispatch Flow

```mermaid
sequenceDiagram
    participant ZC as ZeroClawAgent
    participant Exec as CodeSurgeryExecutor
    participant FS as File System
    participant Verify as Syntax Verifier

    ZC->>Exec: apply_patch(mission_id, SurgicalPatch)
    Exec->>FS: Read file at file_path
    Exec->>FS: Find search_block in content
    Exec->>FS: Replace with replace_block
    Exec->>FS: Write modified content
    Exec->>Verify: verify_syntax(file_path)
    
    alt Syntax Valid
        Verify-->>Exec: true
        Exec-->>ZC: ExecutionResult(success=true)
    else Syntax Invalid
        Verify-->>Exec: false
        Exec->>FS: Revert to original content
        Exec-->>ZC: ExecutionResult(success=false)
    end
```

---

> *Related: [lib.rs](lib.md) · [ZeroClawAgent](../application/agents/zeroclaw.md)*
