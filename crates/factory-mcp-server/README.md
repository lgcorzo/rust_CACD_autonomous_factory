# 🔌 factory-mcp-server

The **Presentation/Interface Layer** of the Dark Gravity Autonomous Agent Factory. This crate implements the **Model Context Protocol (MCP)** server, providing standardized tools to agents over SSE/HTTP.

## 🏗️ DDD Role: Interface Layer

Following **Domain-Driven Design (DDD)**, `factory-mcp-server` translates technical requests from external clients (agents) into infrastructure and application commands.

### Key Responsibilities

- **MCP Protocol Implementation**: Supporting the Model Context Protocol (MCP) spec, enabling seamless integration with LLM client runtimes.
- **SSE Transport**: Providing a high-concurrency, asynchronous transport using Server-Sent Events (SSE) via `Axum` to stream live execution status and handle long-running operations.
- **OpenAI-compatible Tool Integration**: Using `async_openai` to power LLM-driven tools like `plan_mission` with flexible model routing.
- **Unified Tool Registry**: Routing tool execution requests for codebase indexing, Graph RAG queries, code execution in Firecracker, and Jira updates.

## 🛠️ Key Components

- **`src/tools/`**: MCP-compliant tool handlers:
  - **`plan_mission.rs`**: Uses `async_openai` LLM client to generate mission plans from high-level goals.
  - **`execute_code.rs`**: Directs code compilation and running in Firecracker sandboxes.
  - **`retrieve_context.rs`**: Queries R2R Graph RAG for semantic codebase context.
  - **`search_jira.rs`** & **`update_mission_status.rs`**: Syncs tasks and epics.
- **`src/protocol.rs`**: Defines JSON-RPC request/response payloads over SSE.
- **`src/sandbox.rs`**: Integrates Firecracker isolation via direct `AF_VSOCK` communication.

## 🧪 Testing & Verification

- **Tool Execution Testing**: Mocking inner infrastructure dependencies to assert correct JSON-RPC output formatting.
- **SSE Connection Resilience**: Validating SSE heartbeat, chunked response streaming, and auto-reconnection flows.
- **LLM Integration Check**: Verifying that `plan_mission` correctly invokes the async OpenAI client and handles response/error cases.
