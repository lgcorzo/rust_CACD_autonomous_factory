---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-application::agents — Agent Registry"
source_path: "crates/factory-application/src/agents/mod.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-application::agents — Agent Registry."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-application::agents — Agent Registry

> **Source**: `crates/factory-application/src/agents/mod.rs`
> **Layer**: Application

---

## UML: Agent Implementations

```mermaid
classDiagram
    class Agent {
        <<trait>>
        +name() String
        +execute(task_description) Result~Value~
    }
    Agent <|.. RustantAgent : Planner
    Agent <|.. ZeroClawAgent : Developer
    Agent <|.. AuditorAgent : Security
    Agent <|.. FinOpsAgent : Cost Control
    Agent <|.. QAObserverAgent : Quality
    Agent <|.. DocumentationAgent : Wiki
```

## Module Re-exports

| Agent | Public API |
|:---|:---|
| `RustantAgent` | `pub use rustant::RustantAgent` |
| `ZeroClawAgent` | `pub use zeroclaw::ZeroClawAgent` |
| `AuditorAgent` | `pub use auditor::AuditorAgent` |
| `FinOpsAgent` | `pub use finops::FinOpsAgent` |
| `QAObserverAgent` | `pub use qa_observer::QAObserverAgent` |
| `DocumentationAgent` | `pub use doc_agent::DocumentationAgent` |

---

> *Related: [Agent Specifications](../../../architecture/agent_specifications.md) · [lib.rs](../lib.md)*
