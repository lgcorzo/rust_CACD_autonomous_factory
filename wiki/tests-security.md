# tests-security

## Overview

Directory-based community: factory-mcp-server/tests

- **Size**: 5 nodes
- **Cohesion**: 0.0000
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| test_gvisor_k8s_driver_live_connection | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/tests/gvisor_integration.rs | 10-53 |
| test_security_review_sql_injection | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/tests/security_tests.rs | 8-25 |
| test_security_review_command_injection | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/tests/security_tests.rs | 29-46 |
| test_security_review_hardcoded_secret | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/tests/security_tests.rs | 50-66 |
| test_security_review_safe_code | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/tests/security_tests.rs | 70-83 |

## Execution Flows

No execution flows pass through this community.

## Dependencies

### Outgoing

- `unwrap` (9 edge(s))
- `json` (5 edge(s))
- `assert` (4 edge(s))
- `SecurityReviewTool::new` (4 edge(s))
- `call` (4 edge(s))
- `panic` (4 edge(s))
- `serde_json::from_str` (4 edge(s))
- `assert_eq` (4 edge(s))
- `println` (3 edge(s))
- `install_default` (1 edge(s))
- `rustls::crypto::ring::default_provider` (1 edge(s))
- `Client::try_default` (1 edge(s))
- `Api::all` (1 edge(s))
- `clone` (1 edge(s))
- `serde_json::from_value` (1 edge(s))

### Incoming

- `unwrap` (9 edge(s))
- `json` (5 edge(s))
- `assert` (4 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/tests/security_tests.rs` (4 edge(s))
- `SecurityReviewTool::new` (4 edge(s))
- `call` (4 edge(s))
- `panic` (4 edge(s))
- `serde_json::from_str` (4 edge(s))
- `assert_eq` (4 edge(s))
- `println` (3 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/tests/gvisor_integration.rs` (1 edge(s))
- `install_default` (1 edge(s))
- `rustls::crypto::ring::default_provider` (1 edge(s))
- `Client::try_default` (1 edge(s))
- `Api::all` (1 edge(s))
