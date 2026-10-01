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

> *Related: [lib.rs](crates_factory-core_src_lib.md) · [security.rs](crates_factory-core_src_security.md)*
