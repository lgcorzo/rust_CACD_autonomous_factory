# Task: Implement Automated Testing (Unit, Integration, Security)

- [x] Workspace Setup
  - [x] Add `mockall` to workspace dependencies
  - [x] Add `wiremock` to workspace dependencies
- [x] `factory-infrastructure` Testing
  - [x] Add `automock` to `KafkaClient` and `S3Storage`
  - [x] Unit tests for `McpHttpClient` with `wiremock`
- [x] `factory-mcp-server` Testing
  - [x] Add `automock` to `Tool` trait
  - [x] Unit tests for `McpServer` routing
  - [x] Security tests for `security_review` tool
- [x] `factory-application` Testing
  - [x] Add `automock` to `Agent` trait
  - [x] Unit tests for agents
  - [x] Integration tests for workflows
- [/] Final Verification
  - [/] `cargo test --workspace`
