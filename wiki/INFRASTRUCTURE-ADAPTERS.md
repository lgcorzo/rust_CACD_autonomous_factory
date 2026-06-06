# INFRASTRUCTURE-ADAPTERS: Dark Gravity Connectors

This document details the **Adapters** that connect the autonomous factory to external ecosystems and internal infrastructure.

---

## Messaging & Telemetry (The Nervous System)

### Kafka Event Bus
- **Broker**: Confluent Kafka (`my-kafka-cluster-bootstrap.confluent.svc.cluster.local:9092`)
- **Client**: Production-grade `rdkafka` (v0.30) adapter in `factory-infrastructure/src/kafka.rs`
- **V7 Triple-Topic Architecture**:
  - `mission-input` — High-priority ingestion topic for incoming payloads/triggers
  - `agent-thought` — Real-time telemetry streaming of agent reasoning chains
  - `mission-artifact` — Delivery topic for final verified outputs
- **Serialization**: Protobuf schemas compiled via `prost` + `tonic-build`

---

## Authentication & Identity (Zero Trust)

### Cryptographic Non-Human Identities (NHI)
- **Mechanism**: Every agent assigned a unique NHI backed by **Verifiable Credentials (VC)**.
- **Verification**: Actions signed with Ed25519 keypair — ensures auditability for SOC 2 & EU AI Act.
- **GitHub Integration**: GitHub App (RSA Private Keys) stored as **Sealed Secrets**, JWT for ephemeral installation tokens.

### OpenZiti Dark-Network Overlay
- **Mesh**: All inter-service communication via OpenZiti mTLS 1.3 tunnels.
- **Integration**: `factory-infrastructure/src/ziti.rs` — Ziti intercept adapter with Edge Router enforce policies.
- **Dark Architecture**: Zero public listening ports; all ingress via encrypted tunnels authenticated with agent NHI.

---

## Vector Store & Corporate Memory

### R2R GraphRAG + pgvector
- **Platform**: CloudNativePG PostgreSQL 16 with pgvector extension.
- **Client**: `HttpR2rClient` — authenticates via `/v3/users/login`, searches via `/v3/retrieval/search`.
- **Context Pruning**: 3-level pruning (semantic filtering → structural pruning → token budget enforcement) via `context.rs`.
- **deepwiki-rs**: Native AST parser using Tree-sitter; extracts AST deltas from git diffs and upserts embeddings to pgvector.

---

## MCP Tools (The Interface)

| Tool | Crate / Provider | Transport |
| :--- | :--- | :--- |
| `plan_mission` | `factory-mcp-server` | JSON-RPC over SSE |
| `execute_code` | `factory-mcp-server` / Sandbox | JSON-RPC over SSE |
| `run_tests` | `factory-mcp-server` / Sandbox | JSON-RPC over SSE |
| `retrieve_context` | `factory-infrastructure` / R2R | JSON-RPC over SSE |
| `index_code` | `factory-mcp-server` | JSON-RPC over SSE |
| `security_review` | `factory-mcp-server` | JSON-RPC over SSE |
| `search_jira` | `factory-infrastructure` / Jira REST | JSON-RPC over SSE |
| `update_mission_status` | `factory-mcp-server` | JSON-RPC over SSE |
| `spec_kit_tool` | `factory-mcp-server` | JSON-RPC over SSE |

---

## Execution Sandbox

| Driver | Isolation | RAM | CPU | Communication |
| :--- | :--- | :--- | :--- | :--- |
| `SubprocessDriver` | gVisor (`runtimeClassName: gvisor`) | ≤ 30 Mi | ≤ 250m | tokio stdin/stdout |
| `FirecrackerDriver` | KVM hardware micro-VM | ≤ 30 Mi | ≤ 250m | AF_VSOCK |

---

## Environment Configuration

| Variable | Description | Source |
| :--- | :--- | :--- |
| `KAFKA_BOOTSTRAP_SERVERS` | Confluent Kafka broker address | ConfigMap |
| `HATCHET_CLIENT_TOKEN` | Auth for Hatchet engine | SealedSecret |
| `GITHUB_APP_PRIVATE_KEY` | GitHub App RSA key | SealedSecret |
| `LITELLM_API_BASE` | Internal gateway to LLM models | ConfigMap |
| `ZITI_IDENTITY_FILE` | OpenZiti network identity profile | SealedSecret |
| `SENTRY_DSN` | Sentry error tracking DSN | SealedSecret |

---

## Closed-Loop QA (Sentry Integration)

1. **Sentry Polling**: Background poller queries Sentry API every 15 minutes.
2. **Severity Grading**: Auto-grades incoming alerts, filters benign warnings.
3. **GraphRAG Mapping**: Maps exception trace to responsible microservice via R2R.
4. **Backlog Automation**: Auto-creates prioritized issue (tagged `autonomous-plan`) to trigger PO Agent workflow.

---

## Governance & FinOps

| Capability | Tool | Purpose |
| :--- | :--- | :--- |
| Cost Attribution | StackSpend/Finout (Vtags) | Per-Epic LLM token cost tracking |
| Budget Control | Predictive Circuit Breakers | Token velocity monitoring |
| R&D Grants | Hazitek/SPRI Packager | Auto-compile audit templates |
