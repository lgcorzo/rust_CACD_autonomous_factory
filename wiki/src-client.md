# src-client

## Overview

Directory-based community: factory-infrastructure/src

- **Size**: 42 nodes
- **Cohesion**: 0.0835
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| HttpJiraClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 16-25 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 17-24 |
| JiraClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 28-70 |
| search_issues | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 29-69 |
| test_jira_search_success | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 80-108 |
| test_jira_search_no_results | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 111-128 |
| test_jira_search_unauthorized | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 131-146 |
| test_jira_search_server_error | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 149-166 |
| publish_thought | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/kafka.rs | 7-21 |
| SimpleMockKafkaClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/kafka.rs | 26-30 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/kafka.rs | 27-29 |
| KafkaClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/kafka.rs | 33-43 |
| publish | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/kafka.rs | 34-42 |
| McpHttpClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 51-58 |
| McpClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 118-146 |
| call_tool_json | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 119-145 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 52-57 |
| McpSseClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 67-115 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 68-74 |
| get_session_url | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 76-114 |
| test_call_tool_http_success | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 155-173 |
| test_call_tool_sse_success | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 176-206 |
| HttpR2rClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 17-70 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 18-25 |
| get_token | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 27-69 |
| R2rClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 73-133 |
| search | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 74-132 |
| test_r2r_search_success | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 143-183 |
| test_r2r_login_failure | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 186-202 |
| test_r2r_search_failure_after_login | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 205-231 |
| AwsS3Storage | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/s3.rs | 10-16 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/s3.rs | 11-15 |
| S3Storage | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/s3.rs | 19-43 |
| put_object | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/s3.rs | 20-29 |
| get_object | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/s3.rs | 31-42 |
| OpenZitiIdentity | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/ziti.rs | 15-22 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/ziti.rs | 16-21 |
| ZitiIdentity | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/ziti.rs | 25-39 |
| get_token | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/ziti.rs | 26-34 |
| service_name | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/ziti.rs | 36-38 |
| test_open_ziti_identity_new | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/ziti.rs | 46-53 |
| test_open_ziti_identity_trait_methods | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/ziti.rs | 56-65 |

## Execution Flows

- **publish_thought** (criticality: 0.32, depth: 1)

## Dependencies

### Outgoing

- `json` (18 edge(s))
- `to_string` (18 edge(s))
- `assert` (15 edge(s))
- `Ok` (13 edge(s))
- `mount` (12 edge(s))
- `respond_with` (12 edge(s))
- `Mock::given` (12 edge(s))
- `method` (12 edge(s))
- `ResponseTemplate::new` (12 edge(s))
- `and` (10 edge(s))
- `path` (10 edge(s))
- `format` (9 edge(s))
- `MockServer::start` (9 edge(s))
- `uri` (9 edge(s))
- `send` (8 edge(s))

### Incoming

- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs` (8 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs` (7 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs` (6 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/ziti.rs` (5 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/kafka.rs` (4 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/s3.rs` (3 edge(s))
