# tests-security

## Overview

Directory-based community: factory-mcp-server/tests

- **Size**: 4 nodes
- **Cohesion**: 0.0000
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| test_security_review_sql_injection | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/tests/security_tests.rs | 7-24 |
| test_security_review_command_injection | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/tests/security_tests.rs | 27-44 |
| test_security_review_hardcoded_secret | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/tests/security_tests.rs | 47-63 |
| test_security_review_safe_code | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/tests/security_tests.rs | 66-79 |

## Execution Flows

No execution flows pass through this community.

## Dependencies

### Outgoing

- `unwrap` (8 edge(s))
- `json` (4 edge(s))
- `call` (4 edge(s))
- `panic` (4 edge(s))
- `serde_json::from_str` (4 edge(s))
- `assert_eq` (4 edge(s))
- `assert` (3 edge(s))

### Incoming

- `unwrap` (8 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/tests/security_tests.rs` (4 edge(s))
- `json` (4 edge(s))
- `call` (4 edge(s))
- `panic` (4 edge(s))
- `serde_json::from_str` (4 edge(s))
- `assert_eq` (4 edge(s))
- `assert` (3 edge(s))
