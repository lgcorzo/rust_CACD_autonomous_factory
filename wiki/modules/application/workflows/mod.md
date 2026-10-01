---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-application::workflows — Workflow Registry"
source_path: "crates/factory-application/src/workflows/mod.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-application::workflows — Workflow Registry."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

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

> *Related: [autonomous_mission.rs](autonomous_mission.md)*
