# 🔌 factory-mcp-server

The **Presentation/Interface Layer** of the Dark Gravity Autonomous Agent Factory. This crate implements the **Model Context Protocol (MCP)** server, providing standardized tools to agents over SSE/HTTP.

## 🏗️ DDD Role: Interface Layer

Following **Domain-Driven Design (DDD)**, `factory-mcp-server` translates technical requests from external clients (agents) into infrastructure and application commands.

### Key Responsibilities

- **MCP Protocol Implementation**: Full support for tool discovery, execution, and resources.
- **SSE Transport**: Persistent Server-Sent Events (SSE) for long-running tool execution (e.g., code generation, unit tests).
- **Tool Registry**: Central point for registering and invoking `search_jira`, `retrieve_context`, `execute_code`, etc.

## 🛠️ Key Components

- **`tools/`**: Concrete implementations of `McpTool` with structural documentation.
- **`router/`**: Axum routes for SSE handlers and POST tool invocations.
- **`server/`**: Core MCP server lifecycle management.

## 🧪 Testing

- **Tool Unit Tests**: Exhaustive testing of tool logic using manual and trait-based mocks.
- **Protocol Conformance**: (Planned) MCP SDK verification suite.
