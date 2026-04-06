# External Integrations: GitHub/GitLab and R2R (Graph RAG)

This document describes the integration architecture and configuration for GitHub, GitLab, and R2R within the Dark Gravity Autonomous Agent Factory.

## Architecture

The integration follows a clean architecture pattern separating the infrastructure clients from the MCP tool definitions.

### Infrastructure Layer (`factory-infrastructure`)

- **Traits**: Defines `GitHubClient`, `GitLabClient`, `R2rClient`, `KafkaClient`, and `McpClient` interfaces.
- **Implementations**:
  - `HttpGitHubClient` / `HttpGitLabClient` / `HttpR2rClient`: Asynchronous REST communication via `reqwest`.
  - `SimpleMockKafkaClient`: Publishing to the **agent-thought** topic for real-time reasoning telemetry.
  - `McpHttpClient`: High-performance **SSE (Server-Sent Events)** transport for durable tool execution.
- **Authentication**:
  - **GitHub**: Personal Access Token (PAT) via `Authorization: Bearer`.
  - **GitLab**: Project Access Token via `PRIVATE-TOKEN`.
  - **R2R**: Bearer Token (Self-refreshing login via `/v3/users/login`).
  - **Kafka**: SASL/SCRAM via `rdkafka` (infrastructure specific).

### MCP Tool Layer (`factory-mcp-server`)

- **`search_github_issues`**: Exposes cross-repository issue searches to agents.
- **`context_pruning`**: New skill for Rustant to refine R2R context results.
- **`execute_code`**: Integrated with Firecracker Micro-VMs for isolated execution.

## Configuration

The following environment variables are required for the integrations to function:

### GitHub / GitLab Configuration

- `GITHUB_REPO`: The target repository (e.g., `owner/repo`).
- `GITHUB_TOKEN`: Your Personal Access Token.
- `GITLAB_URL`: (Optional) Custom GitLab instance URL.
- `GITLAB_TOKEN`: (Optional) GitLab Access Token.

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

And `search_github_issues` to track project tasks:

```javascript
// Example tool call payload
{
  "query": "is:open label:mission author:lgcorzo"
}
```
