# src-cli

## Overview

Directory-based community: factory-cli/src

- **Size**: 3 nodes
- **Cohesion**: 0.0000
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| Cli | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-cli/src/main.rs | 5-8 |
| Commands | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-cli/src/main.rs | 11-23 |
| main | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-cli/src/main.rs | 26-61 |

## Execution Flows

No execution flows pass through this community.

## Dependencies

### Outgoing

- `hatchet_sdk::worker::worker::Register::add_task_or_workflow` (2 edge(s))
- `tracing_subscriber::fmt::init` (1 edge(s))
- `Cli::parse` (1 edge(s))
- `tracing::info` (1 edge(s))
- `hatchet_sdk::Hatchet::from_env` (1 edge(s))
- `unwrap` (1 edge(s))
- `build` (1 edge(s))
- `slots` (1 edge(s))
- `worker` (1 edge(s))
- `factory_application::workflows::create_mission_workflow` (1 edge(s))
- `clone` (1 edge(s))
- `factory_application::workflows::create_develop_task_workflow` (1 edge(s))
- `start` (1 edge(s))
- `Ok` (1 edge(s))

### Incoming

- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-cli/src/main.rs` (3 edge(s))
