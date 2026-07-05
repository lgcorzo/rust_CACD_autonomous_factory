# agents-agent

## Overview

Directory-based community: factory-application/src/agents

- **Size**: 15 nodes
- **Cohesion**: 0.3095
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| RustantAgent | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/rustant.rs | 12-66 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/rustant.rs | 13-18 |
| plan_mission | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/rustant.rs | 20-44 |
| review_mission | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/rustant.rs | 46-65 |
| Agent | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/rustant.rs | 69-78 |
| name | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/rustant.rs | 70-72 |
| execute | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/rustant.rs | 74-77 |
| ZeroClawAgent | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/zeroclaw.rs | 11-86 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/zeroclaw.rs | 12-14 |
| execute_task | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/zeroclaw.rs | 16-46 |
| validate_mission | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/zeroclaw.rs | 48-71 |
| introspect_k8s | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/zeroclaw.rs | 73-85 |
| Agent | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/zeroclaw.rs | 89-98 |
| name | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/zeroclaw.rs | 90-92 |
| execute | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/zeroclaw.rs | 94-97 |

## Execution Flows

- **execute** (criticality: 0.57, depth: 1)
- **execute** (criticality: 0.48, depth: 1)

## Dependencies

### Outgoing

- `tracing::info` (5 edge(s))
- `call_tool_json` (5 edge(s))
- `json` (5 edge(s))
- `Ok` (5 edge(s))
- `to_string` (2 edge(s))
- `search` (1 edge(s))

### Incoming

- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/rustant.rs` (3 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/agents/zeroclaw.rs` (3 edge(s))
