---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-application::workflows::circuit_breaker — Aethelgard"
source_path: "crates/factory-application/src/workflows/circuit_breaker.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-application::workflows::circuit_breaker — Aethelgard."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-application::workflows::circuit_breaker — Aethelgard

> **Source**: `crates/factory-application/src/workflows/circuit_breaker.rs`
> **Layer**: Application

---

## Aethelgard State Machine

```mermaid
stateDiagram-v2
    [*] --> Active
    Active --> Retry: Task failure
    Retry --> Active: Attempt < 3
    Retry --> Deadlock: Attempt == 3 AND same diff hash
    Deadlock --> Stuck: Circuit breaker trips
    Stuck --> HumanOverride: HITL Vertex 3 alert
    HumanOverride --> Active: Architect provides override
    HumanOverride --> Aborted: Mission rejected
    Aborted --> [*]
```

## Deadlock Detection

| Condition | Threshold | Action |
|:---|:---|:---|
| Consecutive failures | 3 attempts | Check diff hash |
| Diff hash unchanged | Same hash across 3 attempts | Trip circuit breaker |
| Diff hash changed | Different hash | Reset counter, allow retry |

---

> *Related: [develop_task.rs](develop_task.md) · [HITL Governance](../../../security/hitl_governance.md)*
