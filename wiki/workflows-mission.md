# workflows-mission

## Overview

Directory-based community: factory-application/src/workflows

- **Size**: 8 nodes
- **Cohesion**: 0.0039
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| MissionInput | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/autonomous_mission.rs | 19-38 |
| from_protobuf | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/autonomous_mission.rs | 20-37 |
| MissionOutput | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/autonomous_mission.rs | 41-46 |
| create_mission_workflow | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/autonomous_mission.rs | 48-393 |
| test_mission_input_from_protobuf | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/autonomous_mission.rs | 402-418 |
| TaskInput | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/develop_task.rs | 9-13 |
| TaskOutput | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/develop_task.rs | 16-18 |
| create_develop_task_workflow | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/develop_task.rs | 20-90 |

## Execution Flows

No execution flows pass through this community.

## Dependencies

### Outgoing

- `clone` (43 edge(s))
- `to_string` (25 edge(s))
- `unwrap_or_else` (12 edge(s))
- `publish_thought` (12 edge(s))
- `unwrap` (11 edge(s))
- `std::env::var` (10 edge(s))
- `Ok` (9 edge(s))
- `build` (7 edge(s))
- `unwrap_or` (6 edge(s))
- `Arc::new` (6 edge(s))
- `task` (6 edge(s))
- `Box::pin` (6 edge(s))
- `Uuid::new_v4` (5 edge(s))
- `is_empty` (4 edge(s))
- `Some` (4 edge(s))

### Incoming

- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/autonomous_mission.rs` (5 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/develop_task.rs` (3 edge(s))
