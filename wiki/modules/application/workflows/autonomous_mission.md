---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-application::workflows::autonomous_mission — 6-Phase Hatchet DAG"
source_path: "crates/factory-application/src/workflows/autonomous_mission.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-application::workflows::autonomous_mission — 6-Phase Hatchet DAG."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-application::workflows::autonomous_mission — 6-Phase Hatchet DAG

> **Source**: `crates/factory-application/src/workflows/autonomous_mission.rs`
> **Layer**: Application

---

## 6-Phase DAG Sequence Diagram

```mermaid
sequenceDiagram
    participant Poller as PollerDaemonService
    participant Hatchet as Hatchet Orchestrator
    participant Rustant as RustantAgent
    participant ZeroClaw as ZeroClawAgent
    participant Auditor as AuditorAgent

    Note over Poller,Auditor: Phase 1: Ingestion
    Poller->>Hatchet: PolledIssueEvent / PRCommentEvent

    Note over Poller,Auditor: Phase 2: Plan
    Hatchet->>Rustant: plan_mission(goal, context)
    Rustant-->>Hatchet: SddMissionPlan

    Note over Poller,Auditor: Phase 3: Code
    Hatchet->>ZeroClaw: execute_tdd_task(task_items)
    ZeroClaw-->>Hatchet: Code patches

    Note over Poller,Auditor: Phase 4: Validation
    Hatchet->>ZeroClaw: validate_mission(test_command)
    Hatchet->>Auditor: security_review(diff)
    ZeroClaw-->>Hatchet: Test results
    Auditor-->>Hatchet: SAST results

    Note over Poller,Auditor: Phase 5: Review
    Hatchet->>Rustant: review_mission(results)
    Rustant-->>Hatchet: Review decision

    Note over Poller,Auditor: Phase 6: Delivery
    Hatchet->>Poller: Create PR/MR (awaits HITL V4)
```

---

> *Related: [circuit_breaker.rs](circuit_breaker.md) · [Experiment Lifecycle](../../../architecture/runtime_sequences.md)*
