# security-agent

## Overview

Directory-based community: factory-core/src/security

- **Size**: 5 nodes
- **Cohesion**: 0.0909
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| AgentSubject | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/security/nhi.rs | 5-9 |
| CryptographicProof | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/security/nhi.rs | 12-18 |
| VerifiableCredential | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/security/nhi.rs | 33-94 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/security/nhi.rs | 34-47 |
| sign | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/security/nhi.rs | 50-93 |

## Execution Flows

No execution flows pass through this community.

## Dependencies

### Outgoing

- `format` (4 edge(s))
- `to_string` (3 edge(s))
- `vec` (2 edge(s))
- `Utc::now` (2 edge(s))
- `map_err` (2 edge(s))
- `serde_json::to_string` (2 edge(s))
- `crate::error::FactoryError::Security` (2 edge(s))
- `encode` (2 edge(s))
- `as_bytes` (2 edge(s))
- `clone` (1 edge(s))
- `serde_json::json` (1 edge(s))
- `to_bytes` (1 edge(s))
- `Some` (1 edge(s))
- `Ok` (1 edge(s))

### Incoming

- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/security/nhi.rs` (4 edge(s))
