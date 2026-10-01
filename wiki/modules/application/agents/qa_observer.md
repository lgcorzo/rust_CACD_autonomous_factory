---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-application::agents::qa_observer — QAObserverAgent"
source_path: "crates/factory-application/src/agents/qa_observer.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-application::agents::qa_observer — QAObserverAgent."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-application::agents::qa_observer — QAObserverAgent

> **Source**: `crates/factory-application/src/agents/qa_observer.rs`
> **Layer**: Application

---

## Quality Gate Enforcement

```mermaid
flowchart TD
    INPUT["Code Changes"] --> UNIT["Unit Tests"]
    INPUT --> LINT["Clippy Lints"]
    INPUT --> SAST["SAST Scan"]

    UNIT --> GATE{"All Pass?"}
    LINT --> GATE
    SAST --> GATE

    GATE -->|Yes| PASS["✅ Quality Gate Passed"]
    GATE -->|No| FAIL["❌ Quality Gate Failed"]

    style PASS fill:#4CAF50,stroke:#2E7D32,color:#fff
    style FAIL fill:#f44336,stroke:#c62828,color:#fff
```

---

> *Related: [AuditorAgent](auditor.md) · [Verification Triad](../../../security/verification_triad.md)*
