# src-cli

## Overview

Directory-based community: factory-cli/src

- **Size**: 3 nodes
- **Cohesion**: 0.0000
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| Cli | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-cli/src/main.rs | 7-10 |
| Commands | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-cli/src/main.rs | 13-43 |
| main | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-cli/src/main.rs | 46-141 |

## Execution Flows

No execution flows pass through this community.

## Dependencies

### Outgoing

- `tracing::info` (3 edge(s))
- `std::process::exit` (3 edge(s))
- `hatchet_sdk::worker::worker::Register::add_task_or_workflow` (2 edge(s))
- `path` (2 edge(s))
- `sentry::init` (1 edge(s))
- `sentry::release_name` (1 edge(s))
- `Default::default` (1 edge(s))
- `tracing_subscriber::fmt::init` (1 edge(s))
- `Cli::parse` (1 edge(s))
- `is_empty` (1 edge(s))
- `trim` (1 edge(s))
- `anyhow::bail` (1 edge(s))
- `hatchet_sdk::Hatchet::from_env` (1 edge(s))
- `unwrap` (1 edge(s))
- `build` (1 edge(s))

### Incoming

- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-cli/src/main.rs` (3 edge(s))
