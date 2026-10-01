---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-application::bridge::semantica_bridge — Semantica Decision Bridge"
source_path: "crates/factory-application/src/bridge/semantica_bridge.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-application::bridge::semantica_bridge — Semantica Decision Bridge."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-application::bridge::semantica_bridge — Semantica Decision Bridge

> **Source**: `crates/factory-application/src/bridge/semantica_bridge.rs`  
> **Layer**: Application  
> **Role**: Ingests agent cognitive reasoning events and commits immutable decision records into the Semantica provenance graph.

---

## Decision Recording Sequence

```mermaid
sequenceDiagram
    autonumber
    participant Event as Kafka / Agent Event
    participant SB as SemanticaBridge
    participant SC as SemanticaClient
    participant Graph as Semantica Service

    Event->>SB: process_agent_thought_event(event_payload)
    Note over SB: Parse DecisionRecord JSON
    SB->>SC: record_decision(&record)
    SC->>Graph: POST /v1/decisions (with X-NHI-Signature)
    Graph-->>SC: 200 OK
    SC-->>SB: Ok(())
```

## Payload Data Contract

```rust
pub struct DecisionRecord {
    pub decision_id: String,
    pub agent_id: String,
    pub mission_id: String,
    pub reasoning: String,
    pub ast_node_ids: Vec<String>,
    pub timestamp: String,
}
```

---

> *Related: [semantica.rs](../../infrastructure/semantica.md) · [Tactical Design](../../../architecture/tactical_design.md)*
