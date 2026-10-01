---
iso_doc_type: "Description"
iso_viewpoint: "ArchitectureDescription"
type: "architecture"
title: "Mission Data Model — Event-Driven Architecture"
description: "ISO 42010 ArchitectureDescription / ISO 15289 Description documentation for Mission Data Model — Event-Driven Architecture."
tags: ['iso42010', 'okf', 'architecture_description', 'c4']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# Mission Data Model — Event-Driven Architecture

> **Purpose**: Mission lifecycle data model and event-driven architecture flow.

---

## Event-Driven Architecture

```mermaid
flowchart LR
    GH["GitHub API"] --> POLLER["PollerDaemonService"]
    GL["GitLab API"] --> POLLER
    JR["Jira API"] --> POLLER

    POLLER --> KAFKA["Kafka"]
    KAFKA --> HATCHET["Hatchet DAG"]
    HATCHET --> AGENTS["Agent Pool"]
    AGENTS --> KAFKA
    KAFKA --> TELEMETRY["Telemetry Export"]
```

## Mission State Machine

```mermaid
stateDiagram-v2
    [*] --> Pending: Issue ingested
    Pending --> Running: Hatchet dispatched
    Running --> Completed: All phases pass
    Running --> Failed: Unrecoverable error
    Running --> Blocked: Circuit breaker trip
    Blocked --> Running: HITL override
    Blocked --> Failed: Mission aborted
    Completed --> [*]
    Failed --> [*]
```

---

> *Related: [Experiment Lifecycle](runtime_sequences.md) · [lib.rs](../modules/core/lib.md)*
