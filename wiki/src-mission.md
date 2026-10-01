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

> *Related: [Experiment Lifecycle](EXPERIMENT-LIFECYCLE.md) · [lib.rs](crates_factory-core_src_lib.md)*
