# Rust Migration Plan: CACD Autonomous Factory

This document outlines the technical strategy for migrating the intelligence core of the **Dark Gravity CACD Autonomous Agent Factory** from Python to Rust.

## 🎯 Objectives
- **Performance**: Reduce cold-start times for agent workers and tool execution.
- **Reliability**: Leverage Rust's type system and memory safety for long-running autonomous missions.
- **Native Integration**: Achieve better integration with Firecracker MicroVMs and zero-trust networking (OpenZiti).
- **Concurrency**: Utilize Tokio's asynchronous runtime for massive parallel fan-out of agent tasks.

---

## 🏗️ Target Architecture (Rust DDD)

The project will be organized as a **Rust Workspace** to maintain strict separation of concerns and allow independent scaling of components.

### Workspace Structure
```text
rust-cacd-factory/
├── Cargo.toml                # Workspace configuration
├── crates/
│   ├── factory-core/         # Shared kernel: Common schemas, error types, security
│   ├── factory-application/  # Use cases: Agent logic, Hatchet workflows
│   ├── factory-mcp-server/   # MCP Server implementation
│   ├── factory-infrastructure/# Adapters: Kafka, Firecracker, R2R, S3
│   └── factory-cli/          # Command-line interface for local orchestration
├── confs/                    # Ported YAML configurations and prompts
└── .artifacts/               # Design docs and migration tracking
```

---

## 🛠️ Technology Stack

| Component          | Python Implementation | Rust Recommendation |
|-------------------|----------------------|---------------------|
| **Async Runtime** | asyncio              | **Tokio**           |
| **API / SSE**     | FastAPI / Starlette  | **Axum**            |
| **Serialization** | Pydantic / JSON      | **Serde / Serde-JSON**|
| **Workflow**      | Hatchet Python SDK   | **hatchet-sdk (Community) / Tonic (gRPC)** |
| **MCP**           | mcp-python-sdk       | **mcp-sdk-rs**      |
| **Messaging**     | kafka-python         | **rdkafka**         |
| **Sandbox**       | E2B / boto3          | **Firecracker SDK / aws-sdk-s3** |
| **Networking**    | OpenZiti Python      | **openziti-sdk**    |
| **Observability** | loguru               | **Tracing / Tracing-subscriber** |

---

## 🔄 Component Mapping

### 1. Core Kernel (`factory-core`)
- Port `src/autogen_team/core/` to a library crate.
- Define `Mission`, `Task`, and `AgentResponse` structs with `serde` derive.
- Implement shared security traits for signature validation.

### 2. MCP Server (`factory-mcp-server`)
- Re-implement `mcp_server.py` using `Axum` and `mcp-sdk-rs`.
- Define tool handlers in Rust.
- **Tools to port**:
    - `plan_mission`: LLM call with JSON schema validation.
    - `execute_code`: Firecracker sandbox orchestration.
    - `run_tests`: Isolated cargo/pytest execution.
    - `security_review`: Static analysis + LLM-based audit.
    - `retrieve_context`: R2R API wrapper.

### 3. Agent System (`factory-application`)
- Port `PlannerAgent`, `CoderAgent`, etc., as typed structs.
- Implement a `ToolCaller` trait to abstract MCP or direct tool calls.
- Use `Handlebars` or `Tera` for managing system prompts (ported from `mcp_prompts.yaml`).

### 4. Durable Workflows (Hatchet)
- **Challenge**: Porting `AutonomousMissionWorkflow` with `aio_run_many`.
- **Strategy**: Use the Rust Hatchet SDK (or raw gRPC via `tonic`) to implement the `Workflow` and `Action` handlers.
- Leverage Rust's `JoinSet` or `futures::future::join_all` for parallel task triggering within Hatchet actions.

### 5. Infrastructure Adapters (`factory-infrastructure`)
- **Kafka**: Implement `KafkaPublisher` and `KafkaSubscriber` using `rdkafka`.
- **Firecracker**: Use the native Firecracker Rust crates for direct MicroVM management, bypassing heavy Python wrappers.
- **Storage**: Use `aws-sdk-s3` for MinIO integration.

---

## 🚀 Phased Migration Strategy

### Phase 1: Foundation & Core (Weeks 1-2)
- Setup Rust workspace.
- Implement `factory-core` with all shared schemas.
- Setup `factory-infrastructure` with basic Kafka and S3 clients.

### Phase 2: MCP Server & Tools (Weeks 3-4)
- Build the `factory-mcp-server`.
- Port tools one by one:
    - Start with `retrieve_context` (easiest, just HTTP).
    - Port `plan_mission` with OpenAI/LiteLLM Rust clients.
    - Implement `execute_code` with native Firecracker support.

### Phase 3: Autonomous Agents (Weeks 5-6)
- Port the Agent business logic.
- Integrate agents with the new Rust MCP server.
- Validate tool calling behavior and JSON output consistency.

### Phase 4: Hatchet & Factory Orchestration (Weeks 7-8)
- Implement `AutonomousMissionWorkflow` in Rust.
- Setup KEDA scaling for the new Rust workers.
- Run end-to-end "Mission" tests (Plan -> Code -> Test -> Review).

---

## ⚠️ Potential Risks & Mitigations

- **Hatchet SDK Maturity**: If the Rust SDK lacks `aio_run_many` equivalent, we will implement custom fan-out logic using gRPC stream triggers.
- **LLM Tokenization**: Ensure Rust-based token estimation matches Python (use `tiktoken-rs`).
- **Prompt Consistency**: Use unit tests to verify that Rust-formatted prompts produce identical LLM behavior to the Python equivalents.

---

## 📂 Artifacts to Maintain
- `confs/rust_prompts.yaml`: Ported from `mcp_prompts.yaml`.
- `Cargo.toml`: Central dependency management.
- `docs/rust_architecture.md`: Visual representation of the new system flow.
