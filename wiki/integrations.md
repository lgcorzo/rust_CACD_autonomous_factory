# External Integrations: Jira and R2R (Graph RAG)

This document describes the integration architecture and configuration for Jira and R2R within the Dark Gravity Autonomous Agent Factory.

## Architecture

The integration follows a clean architecture pattern separating the infrastructure clients from the MCP tool definitions.

### Infrastructure Layer (`factory-infrastructure`)

- **Traits**: Defines `JiraClient` and `R2rClient` interfaces to enable dependency injection and unit testing.
- **Implementations**: `HttpJiraClient` and `HttpR2rClient` provide robust, asynchronous communication using `reqwest`.
- **Authentication**:
  - **Jira**: Basic Auth (Username + API Token).
  - **R2R**: Bearer Token (Self-refreshing login via `/v3/users/login`).

### MCP Tool Layer (`factory-mcp-server`)

- **`search_jira`**: Exposes JQL search capabilities to agents.
- **`retrieve_context`**: Provides Graph RAG context retrieval using R2R's vector and graph search.

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
