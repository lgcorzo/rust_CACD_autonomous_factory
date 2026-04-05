# 🚀 Deployment Guide: Dark Gravity CA/CD

## 🏗️ Kubernetes Infrastructure Overview

The factory is deployed across several namespaces to maximize service isolation and security.

| Namespace | Components | Role |
| :--- | :--- | :--- |
| `orchestrators` | Hatchet Engine, PostgreSQL, RabbitMQ | Durable state & flow management. |
| `llm-apps` | LiteLLM, OpenWebUI, R2R | Intelligence & RAG services. |
| `agents` | OpenCode Workers, Rust MCP Server | Active agentic workload. |
| `confluent` | Kafka, Kafka-UI | High-throughput messaging. |
| `storage` | MinIO, Shared PV/PVC | Workspace & artifact store. |

---

## 🔐 Secret Management with Sealed Secrets

We use **Bitnami Sealed Secrets** to securely manage sensitive API keys (e.g., GitHub, Jira, LiteLLM) in a GitOps workflow.

### 🛠️ How to Seal a Secret

To create a new secret from a local YAML file:
```bash
# 1. Create a regular Kubernetes secret locally
kubectl create secret generic opencode-secrets \
  --from-literal=LITELLM_API_KEY="your-key" \
  --from-literal=OPENCODE_API_KEY="your-key" \
  --dry-run=client -o yaml > secret.yaml

# 2. Seal the secret (encrypt it)
kubeseal --format yaml < secret.yaml > opencode-sealed-secret.yaml

# 3. Apply the sealed secret to the cluster
kubectl apply -f opencode-sealed-secret.yaml
```
> [!IMPORTANT]
> **Only** the `SealedSecret` manifest should be committed to Git. The `secret.yaml` contains plain text and must be deleted.

---

## 🌩️ KEDA Autoscaling (Event-Driven Ops)

The **OpenCode agent** scales horizontally based on message lag in Kafka using **KEDA**.

```yaml
apiVersion: keda.sh/v1alpha1
kind: ScaledObject
metadata:
  name: opencode-agent-scaler
  namespace: agents
spec:
  scaleTargetRef:
    name: opencode-agent
  triggers:
    - type: kafka
      metadata:
        bootstrapServers: my-kafka.confluent.svc:9092
        topic: mission-input
        lagThreshold: "1"
        consumerGroup: opencode-workers
```

---

## 🛡️ Zero Trust Network Policies

By default, the `agents` namespace has restricted egress.

- **Hatchet**: `orchestrators.svc.cluster.local:7077`
- **LiteLLM**: `litellm.llm-apps.svc.cluster.local:4000`
- **R2R**: `r2r.knowledge.svc.cluster.local:7272`
- **GitHub**: Outbound via HTTPS (enabled via egress gateway).

---

## 🛠️ Operations & Troubleshooting

### 📉 Monitoring with Grafana
- **`agent.thought` Topic**: Subscribe to this Kafka topic to see the full "Chain of Thought" (CoT) for any mission.
- **Liveness Probes**: All pods have health check endpoints (e.g., `/health` for the MCP server).

### 🔄 Reconciliation
If the system becomes inconsistent:
1. Restart the **Hatchet Engine** to trigger durable step recovery.
2. Monitor **KEDA** ScaledObject logs to verify worker scaling.
3. Check **LiteLLM** logs for token budget or rate limit issues.
