# 🔌 INFRASTRUCTURE-ADAPTERS: Dark Gravity Connectors

This document details the **Adapters** that connect the autonomous factory to external ecosystems and internal infrastructure.

---

## 🔐 Authentication & Identity (Project Sovereignty)

The factory is a **Zero Trust** environment. Access is granted through cryptographic identities, and no external secret is stored in plain text.

### Cryptographic Non-Human Identities (NHI)
- **Mechanism**: Every agent is assigned a unique Non-Human Identity (NHI) backed by **Verifiable Credentials (VC)**.
- **Verification**: Actions performed against pgvector, Kafka, and Kubernetes must be signed with the agent's unique **Ed25519** keypair, ensuring auditability and compliance (SOC 2, EU AI Act).
- **GitHub / GitLab Integration**: GitHub Apps (RSA Private Keys) stored as **Sealed Secrets** decrypted via the Bitnami controller. JWT (JSON Web Tokens) are used to request ephemeral installation tokens.

### OpenZiti Dark-Network Overlay
- **Mesh Integration**: Inter-service communication is routed via the **OpenZiti** overlay network mesh (mTLS 1.3).
- **Implementation**: Handled by the Ziti intercept adapter at `factory-infrastructure/src/ziti.rs` using intercept policies v1.
- **Dark Architecture**: Services do not expose any public listening ports; communication is only accessible via encrypted tunnels authenticated with the agent's NHI.

---

## 🛠️ MCP Tools (The Interface)

Agents interact with the world via the **Model Context Protocol**. Every tool is an infrastructure-layer implementation of a domain requirement.

| Tool | Focus | Crate / Provider |
| :--- | :--- | :--- |
| `github_client` | Commits, PRs, Comments | `factory-infrastructure` |
| `r2r_client` | Retrieval & Knowledge Graph | `factory-infrastructure` |
| `kafka_producer` | Telemetry & Events | `factory-infrastructure` |
| `sandbox_client` | Firecracker Micro-VM sandbox | `factory-mcp-server` |

> [!NOTE]
> The `sandbox_client` executes untrusted code in single-mission **Firecracker Micro-VMs** (KVM hardware virtualization). Host-guest communication is handled via `AF_VSOCK` to bypass TCP bridges, with RAM footprints clamped strictly to **15–30 Mi** for security isolation.

---

## 📡 Messaging & Telemetry (The Nervous System)

The simulated `SimpleMockKafkaClient` is deprecated in favor of a production-grade adapter using the **`rdkafka` (v0.30)** crate.

- **Broker Address**: `my-kafka-cluster-bootstrap.confluent.svc.cluster.local:9092` (located in the `confluent` namespace).
- **V7 Triple-Topic Architecture**:
  - **`mission-input`**: High-priority ingestion topic for incoming payloads/triggers.
  - **`agent-thought`**: Real-time telemetry streaming of agent thoughts and reasoning chains.
  - **`mission-artifact`**: Delivery topic for final verified outputs and deliverables.
- **Serialization**: Schema integrity on the bus is enforced using Protobuf definitions compiled via `prost` and `tonic-build` (generating Rust types like `MissionInput`).

---

## 🏗️ Environment Configuration

| Variable | Description | Source |
| :--- | :--- | :--- |
| `KAFKA_BOOTSTRAP_SERVERS` | Confluent Kafka broker address (`my-kafka-cluster-...`) | ConfigMap |
| `HATCHET_CLIENT_TOKEN` | Auth token for the Hatchet orchestration engine | SealedSecret |
| `GITHUB_APP_PRIVATE_KEY` | Private key for GitHub App authentication | SealedSecret |
| `LITELLM_API_BASE` | Internal gateway to LLM models | ConfigMap |
| `ZITI_IDENTITY_FILE` | Path to the OpenZiti network identity profile | SealedSecret |
