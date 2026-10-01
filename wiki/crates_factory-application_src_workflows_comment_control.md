# factory-application::workflows::comment_control — PR/MR Interactive Comment Control

> **Source**: `crates/factory-application/src/workflows/comment_control.rs`  
> **Layer**: Application  
> **Role**: Parses `@darkgravity` and `@dark-gravity` interactive directives from GitHub PR comments and GitLab MR notes to drive live autonomous interaction.

---

## Interactive Directives Dispatch Flow

```mermaid
graph TB
    PR["PR / MR Comment Event"] --> CC["CommentControlService"]
    CC --> PARSE{"Directive Type"}
    PARSE -->|"/spec"| RUSTANT["RustantAgent: Re-evaluate Spec"]
    PARSE -->|"/refine"| ZEROCLAW["ZeroClawAgent: Code Refinement"]
    PARSE -->|"/status"| STATUS["Generate Real-time Factory Status"]
    PARSE -->|"/validate"| VAL["Trigger SAST & Full Test Suite"]

    RUSTANT --> POST["Post Feedback Comment to PR/MR"]
    ZEROCLAW --> POST
    STATUS --> POST
    VAL --> POST

    style CC fill:#2196F3,stroke:#1565C0,color:#fff
    style POST fill:#4CAF50,stroke:#2E7D32,color:#fff
```

## Supported Directives

| Directive | Handler Agent | Action |
|:---|:---|:---|
| `@darkgravity /spec <feedback>` | `RustantAgent` | Re-evaluates specification requirements and updates `spec.md`. |
| `@darkgravity /refine <instruction>` | `ZeroClawAgent` | Executes code surgery in isolated sandbox according to feedback. |
| `@darkgravity /status` | Service | Replies with real-time DAG state, execution metrics, and sandbox logs. |
| `@darkgravity /validate` | Service | Launches SAST security audit and automated verification tests. |

---

> *Related: [HITL Governance](HITL-GOVERNANCE.md) · [User Manual](USER-MANUAL.md)*
