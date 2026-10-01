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

> *Related: [develop_task.rs](crates_factory-application_src_workflows_develop_task.md) · [HITL Governance](HITL-GOVERNANCE.md)*
