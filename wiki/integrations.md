# External Integrations: Jira and R2R (Graph RAG)

This document describes the integration architecture and configuration for Jira and R2R within the Dark Gravity Autonomous Agent Factory.

## Architecture

The integration follows a clean architecture pattern separating the infrastructure clients from the MCP tool definitions.

### Infrastructure Layer (`factory-infrastructure`)

- **Traits**: Defines `JiraClient`, `R2rClient`, `KafkaClient`, and `McpClient` interfaces.
- **Implementations**:
  - `HttpJiraClient` / `HttpR2rClient`: Asynchronous REST communication via `reqwest`.
  - `SimpleMockKafkaClient`: Publishing to the **agent-thought** topic for real-time reasoning telemetry.
  - `McpHttpClient`: High-performance **SSE (Server-Sent Events)** transport for durable tool execution.
- **Authentication**:
  - **Jira**: Basic Auth (Username + API Token).
  - **R2R**: Bearer Token (Self-refreshing login via `/v3/users/login`).
  - **Kafka**: SASL/SCRAM via `rdkafka` (infrastructure specific).

### MCP Tool Layer (`factory-mcp-server`)

- **`search_jira`**: Exposes JQL search capabilities to agents.
- **`context_pruning`**: New skill for Rustant to refine R2R context results.
- **`execute_code`**: Integrated with Firecracker Micro-VMs for isolated execution.

## Configuration

The following environment variables are required for the integrations to function:

### Jira Configuration

- `JIRA_URL`: The base URL of your Jira instance (e.g., `https://your-site.atlassian.net`).
- `JIRA_USERNAME`: The email address associated with your Jira API token.
- `JIRA_API_TOKEN`: Your Atlassian API token.

### R2R Configuration

- `R2R_BASE_URL`: The base URL of the R2R service.
- `R2R_USER`: R2R username (usually an email).
- `R2R_PWD`: R2R password.

## Testing & Reliability

- **Unit Tests**: Both clients approach 95% code coverage through exhaustive mocking of API responses (Success, 401, 404, 500) using `wiremock`.
- **Dependency Injection**: MCP tools use the client traits, allowing them to be unit tested in isolation using `mockall`.

## Usage Example

Agents can use the `retrieve_context` tool to find relevant documentation:

```javascript
// Example tool call payload
{
  "query": "How do I deploy the security-audit-agent?"
}
```

And `search_jira` to track project tasks:

```javascript
// Example tool call payload
{
  "query": "project = DG AND status = 'In Progress' ORDER BY updated DESC"
}
```
