# PRODUCTION-OPERATIONS: Deployment & Scaling

This document details the **GitOps** and **Kubernetes** operations for the **Dark Gravity** autonomous factory.

---

## GitOps Delivery Model

The factory is deployed using a GitOps controller (FluxCD) to ensure the cluster state matches the git repository.

- **Source**: `gitops_internal_lgcorzo` repository
- **Target**: Production Cluster (namespace: `factory`)
- **Reconciliation**: FluxCD detects delta → applies manifests via Server-Side Apply (SSA) → health checks verify pods running

---

## Kubernetes Components

| Component | Resource | Replicas | Port | Namespace |
| :--- | :--- | :--- | :--- | :--- |
| **MCP Server** | `factory-mcp-server` (Deployment + Service) | 2 | 8100 | `factory` |
| **OpenCode Agent** | `opencode-agent` (Deployment + ConfigMap) | KEDA-scaled | - | `factory` |
| **Hatchet Engine** | Hatchet server | 1 | 7077 | `orchestrators` |
| **LiteLLM Gateway** | LiteLLM service | 1 | 11435 | `llm-apps` |
| **R2R GraphRAG** | R2R service | 1 | - | `llm-apps` |
| **Confluent Kafka** | Kafka cluster | 3 | 9092 | `confluent` |
| **CloudNativePG** | PostgreSQL 16 + pgvector | 3 | 5432 | `storage` |
| **OpenZiti** | Edge Router | 1 | - | `ziti` |

---

## Autoscaling (KEDA)

We use **KEDA** (Kubernetes Event-Driven Autoscaling) to scale the workforce based on mission volume.

### ScaledObject Configuration
- **Trigger**: Kafka (lag in `mission-input` topic)
- **Min Replicas**: 0 (Scale to zero when idle)
- **Max Replicas**: 10
- **Broker**: `my-kafka.confluent.svc:9092`

```yaml
apiVersion: keda.sh/v1alpha1
kind: ScaledObject
metadata:
  name: opencode-agent-scaler
  namespace: factory
spec:
  scaleTargetRef:
    name: opencode-agent
  minReplicaCount: 0
  maxReplicaCount: 10
  triggers:
    - type: kafka
      metadata:
        bootstrapServers: my-kafka.confluent.svc:9092
        topic: mission-input
        lagThreshold: "1"
```

---

## Secrets & Security

All sensitive data is encrypted using **Bitnami Sealed Secrets**.

| Secret | Description | Source |
| :--- | :--- | :--- |
| `LITELLM_API_KEY` | LiteLLM gateway auth | SealedSecret |
| `OPENCODE_API_KEY` | OpenCode agent auth | SealedSecret |
| `HATCHET_CLIENT_TOKEN` | Hatchet engine auth | SealedSecret |
| `GITHUB_APP_PRIVATE_KEY` | GitHub App RSA key | SealedSecret |
| `ZITI_IDENTITY_FILE` | OpenZiti identity profile | SealedSecret |
| `SENTRY_DSN` | Sentry error tracking DSN | SealedSecret |

---

## CI/CD Pipeline

The GitHub Actions pipeline (`.github/workflows/pipeline.yml`) runs on push/PR to `main`:

| Step | Command |
| :--- | :--- |
| Format Check | `cargo fmt --all -- --check` |
| Lint | `cargo clippy --workspace -- -D warnings` |
| Test | `cargo test --workspace -- --skip smoke` |
| Docker Build & Push | Pushes `lgcorzo/dark-gravity-factory:latest` and `:sha` on main/release |

---

## Wiki Sync

The `wiki/` folder is bidirectionally synced to the GitHub Wiki via `.github/workflows/docs-to-wiki.yml`. Wiki edits trigger a PR back to the repository.

---

## Monitoring & Observability

| Component | Focus | Tooling |
| :--- | :--- | :--- |
| **System Health** | CPU, Memory, Pod Status | Prometheus / Grafana |
| **Agent Thought** | Reasoning & Strategies | Kafka (`agent-thought` stream) |
| **Error Tracking** | Production Exception Capturing | Sentry |
| **Cost Attribution** | LLM token spend per Epic | StackSpend / Finout (Vtags) |
| **Documentation Quality** | Orphan Symbol Rate | OSR telemetry via deepwiki-rs |
