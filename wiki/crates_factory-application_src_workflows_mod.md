# factory-application::workflows — Workflow Registry

> **Source**: `crates/factory-application/src/workflows/mod.rs`
> **Layer**: Application

---

## Inter-Workflow Dependency Map

```mermaid
graph TB
    AM["autonomous_mission.rs<br/>6-Phase Hatchet DAG"] --> DT["develop_task.rs<br/>TDD Execution"]
    AM --> PR["pipeline_remediation.rs<br/>CI Fix"]
    AM --> CC["comment_control.rs<br/>PR Directives"]
    AM --> CB["circuit_breaker.rs<br/>Aethelgard"]
    AM --> DR["deep_research.rs<br/>Knowledge Search"]
    DT --> CB

    style AM fill:#2196F3,stroke:#1565C0,color:#fff
```

---

> *Related: [autonomous_mission.rs](crates_factory-application_src_workflows_autonomous_mission.md)*
