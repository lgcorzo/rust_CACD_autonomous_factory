# Infrastructure Adapters — Dark Gravity Factory

> **Purpose**: Comprehensive documentation of all infrastructure adapters with UML diagrams.

---

## Adapter Overview

```mermaid
graph TB
    subgraph "Event Bus"
        KAFKA["Kafka KRaft<br/>Event streaming"]
    end
    subgraph "Knowledge"
        R2R["R2R GraphRAG<br/>Code retrieval"]
    end
    subgraph "Networking"
        ZITI["OpenZiti<br/>Zero Trust overlay"]
    end
    subgraph "Secrets"
        VAULT["HashiCorp Vault<br/>Key management"]
    end
    subgraph "Observability"
        SENTRY["Sentry<br/>Error tracking"]
        S3["MinIO/S3<br/>Artifact storage"]
    end

    style KAFKA fill:#2196F3,stroke:#1565C0,color:#fff
    style ZITI fill:#4CAF50,stroke:#2E7D32,color:#fff
    style VAULT fill:#FF9800,stroke:#E65100,color:#fff
```

## Kafka KRaft Architecture

| Topic | Producer | Consumer | Purpose |
|:---|:---|:---|:---|
| `mission-events` | PollerDaemonService | Hatchet | Mission ingestion |
| `budget-exceeded` | FinOpsAgent | Ops alerts | Budget hardstop |
| `compliance-audit` | Telemetry export | S3 archiver | Audit trail |

## R2R GraphRAG Knowledge Retrieval

```mermaid
sequenceDiagram
    participant Agent as RustantAgent
    participant R2R as R2R GraphRAG
    participant VDB as Vector Database

    Agent->>R2R: retrieve_context(query)
    R2R->>VDB: Semantic search (768-dim embeddings)
    VDB-->>R2R: Top-k code chunks
    R2R->>R2R: Graph-augmented reasoning
    R2R-->>Agent: Contextual code + docs
```

---

> *Related: [Tactical Design](TACTICAL-DESIGN.md) · [Production Operations](PRODUCTION-OPERATIONS.md)*
