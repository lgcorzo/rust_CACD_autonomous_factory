# 🔌 INFRASTRUCTURE-ADAPTERS: [PROJECT_NAME] Connectors

This document details the **Adapters** that connect the autonomous factory to external ecosystems and internal infrastructure.

---

## 🔐 Authentication & Identity (Project Sovereignty)

The factory is a **Zero Trust** environment. No external secret (PAT, API Key) should be stored in plain text.

### GitHub / GitLab Integration
- **Mechanism**: GitHub Apps (RSA Private Keys).
- **Storage**: **Sealed Secrets** decrypted via Bitnami/Sealed Secrets Controller inside the cluster.
- **Protocol**: JWT (JSON Web Tokens) used to request ephemeral installation tokens.

---

## 🛠️ MCP Tools (The Interface)

Agents interact with the world via the **Model Context Protocol**. Every tool is an infrastructure-layer implementation of a domain requirement.

| Tool | Focus | Crate / Provider |
| :--- | :--- | :--- |
| `github_client` | Commits, PRs, Comments | `factory-infrastructure` |
| `r2r_client` | Retrieval & Knowledge Graph | `factory-infrastructure` |
| `kafka_producer` | Telemetry & Events | `factory-infrastructure` |
| `sandbox_client` | Micro-VM orchestrator | `factory-mcp-server` |

---

## 📡 Messaging & Telemetry (The Nervous System)

**Kafka** is the durable message bus for all agentic internal communication.

- **Topic: mission-input**: High-priority ingestion point for external triggers.
- **Topic: agent-thoughts**: Real-time stream of what agents are thinking (consumed by metrics agents).
- **Topic: mission-logs**: Audit trail for all artifact generation.

---

## 🏗️ Environment Configuration

| Variable | Description | Source |
| :--- | :--- | :--- |
| `KAFKA_BOOTSTRAP_SERVERS` | Internal Kafka broker address | ConfigMap |
| `HATCHET_CLIENT_TOKEN` | Auth for the backbone engine | SealedSecret |
| `GITHUB_APP_PRIVATE_KEY` | Private key for App Auth | SealedSecret |
| `LITELLM_API_BASE` | Internal gateway to LLM models | ConfigMap |
