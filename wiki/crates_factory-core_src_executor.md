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

> *Related: [lib.rs](crates_factory-core_src_lib.md) · [ZeroClawAgent](crates_factory-application_src_agents_zeroclaw.md)*
