# tests-token

## Overview

Directory-based community: factory-core/tests

- **Size**: 5 nodes
- **Cohesion**: 0.1000
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| test_manual_wipe_token | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/tests/security_tests.rs | 4-34 |
| DummyBounds | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/tests/security_tests.rs | 8-8 |
| SecurityBounds | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/tests/security_tests.rs | 10-19 |
| validate_token | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/tests/security_tests.rs | 11-13 |
| issue_jit_token | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/tests/security_tests.rs | 14-18 |

## Execution Flows

No execution flows pass through this community.

## Dependencies

### Outgoing

- `Ok` (2 edge(s))
- `String::new` (1 edge(s))
- `String::from` (1 edge(s))
- `as_ptr` (1 edge(s))
- `len` (1 edge(s))
- `wipe_token_from_memory` (1 edge(s))
- `std::slice::from_raw_parts` (1 edge(s))
- `assert` (1 edge(s))

### Incoming

- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/tests/security_tests.rs` (3 edge(s))
- `String::from` (1 edge(s))
- `as_ptr` (1 edge(s))
- `len` (1 edge(s))
- `wipe_token_from_memory` (1 edge(s))
- `std::slice::from_raw_parts` (1 edge(s))
- `assert` (1 edge(s))
