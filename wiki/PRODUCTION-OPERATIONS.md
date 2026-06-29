# PRODUCTION-OPERATIONS: Deployment & Scaling

This document details the **GitOps** and **Kubernetes** operations for the **Dark Gravity** autonomous factory.

---

## GitOps Delivery Model

The factory is deployed using a GitOps controller (FluxCD **planned**) to ensure the cluster state matches the git repository.

---

## Kubernetes Components

| Component | Resource | Port | Namespace |
| :--- | :--- | :--- | :--- |
| **MCP Server** | `factory-mcp-server` (Deployment + Service) | 8100 | `factory` |
| **Hatchet Engine** | Hatchet server | 7077 | `orchestrators` |
| **LiteLLM Gateway** | LiteLLM service | — | `llm-apps` |
| **R2R GraphRAG** | R2R service | — | `llm-apps` |
| **Confluent Kafka** | Kafka cluster | 9092 | `confluent` |
| **CloudNativePG** | PostgreSQL 16 | 5432 | `storage` |
| **OpenZiti** | Edge Router | — | `ziti` |

---

## Autoscaling (KEDA)

We use **KEDA** (Kubernetes Event-Driven Autoscaling) to scale the workforce based on mission volume.

### ScaledObject Configuration
- **Trigger**: Kafka (lag in `mission-input` topic)
- **Min Replicas**: 0 (Scale to zero when idle)
- **Max Replicas**: 10

---

## CI/CD Pipeline

The GitHub Actions pipeline (`.github/workflows/pipeline.yml`) runs on push/PR to `main`:

| Step | Command |
| :--- | :--- |
| Format Check | `cargo fmt --all -- --check` |
| Lint | `cargo clippy --workspace -- -D warnings` |
| Test | `cargo test --workspace -- --skip smoke` |
| Docker Build & Push | Pushes `lgcorzo/dark-gravity-factory` on main/release |

---

## Wiki Sync

The `wiki/` folder is synced to the GitHub Wiki via `.github/workflows/docs-to-wiki.yml`. Documentation is maintained using CRG (code-review-graph) and Graphify for accuracy verification.

---

## Monitoring & Observability

| Component | Focus | Tooling |
| :--- | :--- | :--- |
| **System Health** | CPU, Memory, Pod Status | Prometheus / Grafana |
| **Agent Thought** | Reasoning & Strategies | Kafka (`agent-thought` stream) |
| **Error Tracking** | Production Exception Capturing | Sentry **(planned)** |
| **Documentation Quality** | Code Structure & Accuracy | CRG + Graphify |

---

## CRG + Graphify Integration

The project uses two complementary tools for code intelligence and documentation:

### code-review-graph (CRG)
- **Purpose**: Semantic code search, dependency analysis, community detection
- **Output**: `.code-review-graph/` directory with graph DB, wiki pages, and visualizations
- **Status**: Nodes: 254, Edges: 1,522, Embeddings: 195 (using `lite_embedding`)

### Graphify
- **Purpose**: Code structure extraction, community detection, wiki report generation
- **Output**: `graphify-out/` directory with `GRAPH_REPORT.md`, `graph.json`, `graph.html`
- **Status**: Nodes: 611, Edges: 965, Communities: 53

---

*Last updated: 2026-06-23 — Verified against actual codebase via CRG analysis*