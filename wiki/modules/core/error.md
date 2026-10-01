---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-core::error — Error Types"
source_path: "crates/factory-core/src/error.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-core::error — Error Types."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-core::error — Error Types

> **Source**: `crates/factory-core/src/error.rs`
> **Layer**: Domain

---

## UML Class Diagram

```mermaid
classDiagram
    class FactoryError {
        <<enumeration>>
        Config(String)
        Network(String)
        Internal(String)
        Security(String)
        Agent(String)
        Mission(String)
        Unexpected(anyhow::Error)
        Storage(String)
        IoError(String)
        RemediationError(String)
    }
    class Result~T~ {
        <<type alias>>
        std::result::Result~T, FactoryError~
    }

    FactoryError ..|> thiserror::Error : derives
    Result --> FactoryError
```

## Error Variant Usage

| Variant | Raised By | Context |
|:---|:---|:---|
| `Config` | `AgentModelConfig::load()` | Missing YAML/JSON config files |
| `Network` | Infrastructure adapters | HTTP/gRPC connection failures |
| `Internal` | Application workflows | Logic errors in DAG processing |
| `Security` | `Ed25519SecurityValidator` | NHI signature verification failures |
| `Agent` | Agent trait implementations | Agent execution failures |
| `Mission` | `autonomous_mission` workflow | Mission lifecycle errors |
| `Unexpected` | `anyhow::Error` via `#[from]` | Catch-all for unclassified errors |
| `Storage` | `CursorStore`, S3 adapters | Database/object storage errors |
| `IoError` | File system operations | Local file I/O failures |
| `RemediationError` | Pipeline remediation workflow | CI/CD fix attempt failures |

---

> *Related: [lib.rs](lib.md) · [security.rs](security.md)*
