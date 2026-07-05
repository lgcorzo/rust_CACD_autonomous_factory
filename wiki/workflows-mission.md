# workflows-mission

## Overview

Directory-based community: factory-application/src/workflows

- **Size**: 6 nodes
- **Cohesion**: 0.0000
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| MissionInput | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/autonomous_mission.rs | 10-14 |
| MissionOutput | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/autonomous_mission.rs | 17-22 |
| create_mission_workflow | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/autonomous_mission.rs | 24-236 |
| TaskInput | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/develop_task.rs | 9-13 |
| TaskOutput | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/develop_task.rs | 16-18 |
| create_develop_task_workflow | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/develop_task.rs | 20-48 |

## Execution Flows

No execution flows pass through this community.

## Dependencies

### Outgoing

- `clone` (33 edge(s))
- `to_string` (13 edge(s))
- `publish_thought` (10 edge(s))
- `unwrap` (8 edge(s))
- `build` (7 edge(s))
- `Ok` (7 edge(s))
- `task` (6 edge(s))
- `Box::pin` (6 edge(s))
- `unwrap_or_else` (5 edge(s))
- `Uuid::new_v4` (5 edge(s))
- `Arc::new` (4 edge(s))
- `add_parent` (4 edge(s))
- `parent_output` (3 edge(s))
- `ZeroClawAgent::new` (3 edge(s))
- `unwrap_or` (3 edge(s))

### Incoming

- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/autonomous_mission.rs` (3 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/workflows/develop_task.rs` (3 edge(s))
