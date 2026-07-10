# bridge-checkpoint

## Overview

Directory-based community: factory-application/src/bridge

- **Size**: 11 nodes
- **Cohesion**: 0.1091
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| NativeADKDriver | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/bridge/adk_driver.rs | 6-8 |
| CodeSurgeryExecutor | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/bridge/adk_driver.rs | 11-45 |
| apply_patch | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/bridge/adk_driver.rs | 12-39 |
| verify_syntax | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/bridge/adk_driver.rs | 41-44 |
| BridgeStatus | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/bridge/state.rs | 6-12 |
| StepCheckpoint | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/bridge/state.rs | 15-20 |
| BridgeState | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/bridge/state.rs | 32-85 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/bridge/state.rs | 33-45 |
| get_checkpoint_key | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/bridge/state.rs | 47-49 |
| load_checkpoint | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/bridge/state.rs | 51-68 |
| save_checkpoint | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/bridge/state.rs | 70-84 |

## Execution Flows

No execution flows pass through this community.

## Dependencies

### Outgoing

- `Ok` (5 edge(s))
- `to_string` (4 edge(s))
- `map_err` (2 edge(s))
- `FactoryError::IoError` (2 edge(s))
- `format` (2 edge(s))
- `Self::get_checkpoint_key` (2 edge(s))
- `as_secs` (2 edge(s))
- `unwrap` (2 edge(s))
- `duration_since` (2 edge(s))
- `std::time::SystemTime::now` (2 edge(s))
- `join` (1 edge(s))
- `fs::read_to_string` (1 edge(s))
- `contains` (1 edge(s))
- `Err` (1 edge(s))
- `FactoryError::RemediationError` (1 edge(s))

### Incoming

- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/bridge/state.rs` (4 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/bridge/adk_driver.rs` (2 edge(s))
