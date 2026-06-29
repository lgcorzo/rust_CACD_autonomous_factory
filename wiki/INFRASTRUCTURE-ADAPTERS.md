# INFRASTRUCTURE-ADAPTERS: Dark Gravity Connectors

This document details the **Adapters** that connect the autonomous factory to external ecosystems and internal infrastructure.

---

## Messaging & Telemetry (The Nervous System)

### Kafka Event Bus
- **Client**: `SimpleMockKafkaClient` in `factory-infrastructure/src/kafka.rs` — publishes `publish_thought` events.
- **Topics**: `mission-input`, `agent-thought`, `mission-artifact`
- **Serialization**: JSON (via `serde_json`)

---

## Authentication & Identity (Zero Trust)

### Security Validator
- **Mechanism**: `SecurityValidator` trait in `factory-core/src/security.rs` with `validate_signature` and `audit_content` methods.
- **Audit Results**: `AuditResult` struct with `is_safe` boolean and `findings` vector.

### OpenZiti Dark-Network Overlay
- **Mesh**: All inter-service communication via OpenZiti mTLS tunnels.
- **Integration**: `factory-infrastructure/src/ziti.rs` — `OpenZitiIdentity` struct with `get_token` and `service_name` methods.
- **Mocking**: `MockZitiIdentity` available for testing.

---

## Vector Store & Corporate Memory

### R2R GraphRAG
- **Client**: `HttpR2rClient` in `factory-infrastructure/src/r2r.rs`:
  - Authenticates via `get_token()` (login endpoint)
  - Searches via `search()` (retrieval endpoint)
- **Mocking**: `ManualMockR2rClient` in `factory-mcp-server/src/tools/retrieve_context.rs`

---

## MCP Tools (The Interface)

All tools are provided by the Axum-based MCP server in `factory-mcp-server`:

| Tool | Module | Transport |
| :--- | :--- | :--- |
| `plan_mission` | `tools/plan_mission.rs` | JSON-RPC over SSE |
| `execute_code` | `tools/execute_code.rs` | JSON-RPC over SSE |
| `run_tests` | `tools/run_tests.rs` | JSON-RPC over SSE |
| `retrieve_context` | `tools/retrieve_context.rs` | JSON-RPC over SSE |
| `index_code` | `tools/index_code.rs` | JSON-RPC over SSE |
| `security_review` | `tools/security_review.rs` | JSON-RPC over SSE |
| `search_jira` | `tools/search_jira.rs` | JSON-RPC over SSE |
| `update_mission_status` | `tools/update_mission_status.rs` | JSON-RPC over SSE |

### MCP Client Infrastructure

- **McpHttpClient**: Direct HTTP calls to MCP endpoint
- **McpSseClient**: SSE handshake + session-based communication
- Both implement the `McpClient` trait in `factory-infrastructure/src/mcp_client.rs`

---

## Execution Sandbox

| Driver | Isolation | Communication |
| :--- | :--- | :--- |
| `SubprocessDriver` | Local subprocess | tokio stdin/stdout |
| `FirecrackerDriver` | KVM hardware micro-VM | AF_VSOCK (planned implementation) |

Both implement the `SandboxDriver` trait in `factory-mcp-server/src/sandbox.rs`.

---

## External Service Clients

| Client | File | Key Methods |
| :--- | :--- | :--- |
| `HttpJiraClient` / `JiraClient` | `jira.rs` | `search_issues(query)` |
| `HttpR2rClient` / `R2rClient` | `r2r.rs` | `search(query, limit)` |
| `SimpleMockKafkaClient` / `KafkaClient` | `kafka.rs` | `publish(topic, payload)` |
| `McpHttpClient` / `McpSseClient` | `mcp_client.rs` | `call_tool_json(name, args)` |
| `AwsS3Storage` / `S3Storage` | `s3.rs` | `put_object(key, data)`, `get_object(key)` |
| `OpenZitiIdentity` / `ZitiIdentity` | `ziti.rs` | `get_token()`, `service_name()` |

---

## Environment Configuration

| Variable | Description | Source |
| :--- | :--- | :--- |
| `HATCHET_CLIENT_TOKEN` | Auth for Hatchet engine | Environment |
| `LITELLM_API_BASE` | Internal gateway to LLM models | Environment |
| `ZITI_IDENTITY_FILE` | OpenZiti network identity profile | Environment |
| `OPENAI_API_KEY` | API key for LLM gateway | Environment |
| `KAFKA_BOOTSTRAP_SERVERS` | Kafka broker address | Environment |

---

## CRG-Verified Dependencies

Based on `code-review-graph` analysis, the infrastructure layer has the following dependency graph:

- **Jira Client** → `wiremock` for HTTP mocking, `serde_json` for parsing
- **R2R Client** → JWT auth flow (login → token → search), `wiremock` for testing
- **Kafka Client** → Published via `publish_thought` to agent-thought topic
- **MCP Client** → SSE handshake (`get_session_url`) + HTTP calls (`call_tool_json`)
- **S3 Storage** → `put_object` / `get_object` with configurable bucket
- **Ziti Identity** → mTLS token retrieval with service name

---

*Last updated: 2026-06-23 — Verified against actual codebase via CRG analysis*