---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-application::workflows::deep_research — Deep Research Orchestration"
source_path: "crates/factory-application/src/workflows/deep_research.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-application::workflows::deep_research — Deep Research Orchestration."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-application::workflows::deep_research — Deep Research Orchestration

> **Source**: `crates/factory-application/src/workflows/deep_research.rs`  
> **Layer**: Application  
> **Role**: Hatchet-orchestrated multi-step deep knowledge research combining LiteLLM query decomposition and R2R vector retrieval.

---

## Deep Research Hatchet Workflow

```mermaid
graph TB
    START["Input: DeepSearchInput (query, job_id)"] --> PLAN["Plan Step: Decompose Query into Sub-queries"]
    PLAN --> EXEC["Execute Step: R2R Vector Retrieval + LLM Synthesis"]
    EXEC --> OKF["Format Step: Compile Output into OKF Document"]
    OKF --> OUT["Output: DeepSearchOutput"]

    style START fill:#9C27B0,stroke:#6A1B9A,color:#fff
    style OKF fill:#4CAF50,stroke:#2E7D32,color:#fff
```

## Security & FinOps Controls

- **FinOps Tags**: Inserts `litellm-tags` headers (`{"epic": "DeepSearch", "task": "ResearchDAG"}`) on all requests to LiteLLM for exact per-query cost attribution.
- **Memory Zeroization**: Implements `Zeroize` on temporary buffers holding sensitive query embeddings or retrieved contexts.

---

> *Related: [r2r.rs](../../infrastructure/r2r.md) · [trigger_deep_search.rs](../../cli/trigger_deep_search.md)*
