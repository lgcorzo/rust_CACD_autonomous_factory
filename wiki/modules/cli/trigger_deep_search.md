---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-cli::bin::trigger_deep_search — Deep Search CLI Trigger"
source_path: "crates/factory-cli/src/bin/trigger_deep_search.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-cli::bin::trigger_deep_search — Deep Search CLI Trigger."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-cli::bin::trigger_deep_search — Deep Search CLI Trigger

> **Source**: `crates/factory-cli/src/bin/trigger_deep_search.rs`  
> **Layer**: Interface  
> **Role**: Standalone CLI binary to trigger asynchronous deep research workflows via Hatchet SDK.

---

## Execution Flow

```mermaid
sequenceDiagram
    autonumber
    participant CLI as trigger_deep_search
    participant H as Hatchet SDK
    participant DAG as deep-search-workflow

    CLI->>H: Hatchet::from_env()
    CLI->>H: workflow("deep-search-workflow").build()
    CLI->>DAG: run_no_wait(&input)
    DAG-->>CLI: WorkflowRun response
```

## CLI Usage

```bash
cargo run --bin trigger_deep_search -- --query "Research OpenZiti zero trust architectures in Rust applications"
```

---

> *Related: [deep_research.rs](../application/workflows/deep_research.md) · [Tactical Design](../../architecture/tactical_design.md)*
