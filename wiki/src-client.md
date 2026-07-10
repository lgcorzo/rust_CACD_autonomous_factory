# src-client

## Overview

Directory-based community: factory-infrastructure/src

- **Size**: 76 nodes
- **Cohesion**: 0.0806
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| HttpAethalgardClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/aethalgard.rs | 16-23 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/aethalgard.rs | 17-22 |
| AethalgardClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/aethalgard.rs | 26-55 |
| notify_remediation | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/aethalgard.rs | 27-54 |
| GitlabIssue | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/gitlab.rs | 5-11 |
| HttpGitlabClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/gitlab.rs | 30-38 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/gitlab.rs | 31-37 |
| GitlabClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/gitlab.rs | 41-78 |
| create_issue | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/gitlab.rs | 42-77 |
| test_gitlab_create_issue_success | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/gitlab.rs | 88-124 |
| test_gitlab_create_issue_unauthorized | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/gitlab.rs | 127-142 |
| HttpJiraClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 16-25 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 17-24 |
| JiraClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 28-70 |
| search_issues | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 29-69 |
| test_jira_search_success | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 80-108 |
| test_jira_search_no_results | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 111-128 |
| test_jira_search_unauthorized | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 131-146 |
| test_jira_search_server_error | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs | 149-166 |
| publish_thought | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/kafka.rs | 10-24 |
| RdKafkaClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/kafka.rs | 31-39 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/kafka.rs | 32-38 |
| KafkaClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/kafka.rs | 65-75 |
| publish | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/kafka.rs | 66-74 |
| SimpleMockKafkaClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/kafka.rs | 57-61 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/kafka.rs | 58-60 |
| McpHttpClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 51-58 |
| McpClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 118-146 |
| call_tool_json | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 119-145 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 52-57 |
| McpSseClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 67-115 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 68-74 |
| get_session_url | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 76-114 |
| test_call_tool_http_success | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 155-173 |
| test_call_tool_sse_success | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs | 176-206 |
| HttpR2rClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 18-71 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 19-26 |
| get_token | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 28-70 |
| R2rClient | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 74-173 |
| search | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 75-133 |
| push_osr_metric | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 135-172 |
| test_r2r_search_success | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 183-223 |
| test_r2r_login_failure | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 226-242 |
| test_r2r_search_failure_after_login | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs | 245-271 |
| AwsS3Storage | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/s3.rs | 10-16 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/s3.rs | 11-15 |
| S3Storage | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/s3.rs | 19-43 |
| put_object | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/s3.rs | 20-29 |
| get_object | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/s3.rs | 31-42 |
| Ed25519Validator | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/security_validator.rs | 12-28 |

*... and 26 more members.*

## Execution Flows

- **push_osr_metric** (criticality: 0.48, depth: 1)
- **publish_thought** (criticality: 0.36, depth: 1)

## Dependencies

### Outgoing

- `json` (37 edge(s))
- `to_string` (32 edge(s))
- `Ok` (31 edge(s))
- `assert` (22 edge(s))
- `mount` (19 edge(s))
- `respond_with` (19 edge(s))
- `Mock::given` (19 edge(s))
- `method` (19 edge(s))
- `ResponseTemplate::new` (19 edge(s))
- `format` (17 edge(s))
- `assert_eq` (17 edge(s))
- `send` (15 edge(s))
- `status` (15 edge(s))
- `MockServer::start` (15 edge(s))
- `uri` (15 edge(s))

### Incoming

- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/mcp_client.rs` (8 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/jira.rs` (7 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/kafka.rs` (7 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/sentry.rs` (7 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/gitlab.rs` (6 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/r2r.rs` (6 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/ziti.rs` (5 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/security_validator.rs` (4 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/vault.rs` (4 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/aethalgard.rs` (3 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-infrastructure/src/s3.rs` (3 edge(s))
