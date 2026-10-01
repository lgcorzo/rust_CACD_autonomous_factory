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

> *Related: [deep_research.rs](crates_factory-application_src_workflows_deep_research.md) · [Tactical Design](TACTICAL-DESIGN.md)*
