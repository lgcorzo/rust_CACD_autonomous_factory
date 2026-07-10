# src-telemetry

## Overview

Directory-based community: factory-application/src

- **Size**: 4 nodes
- **Cohesion**: 0.1212
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| TelemetryExporter | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/telemetry_export.rs | 13-86 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/telemetry_export.rs | 14-20 |
| start_export_loop | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/telemetry_export.rs | 23-68 |
| push_to_openwebui | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/telemetry_export.rs | 70-85 |

## Execution Flows

- **start_export_loop** (criticality: 0.36, depth: 1)

## Dependencies

### Outgoing

- `anyhow::anyhow` (4 edge(s))
- `Ok` (2 edge(s))
- `Err` (2 edge(s))
- `map_err` (2 edge(s))
- `tracing::warn` (2 edge(s))
- `Client::new` (1 edge(s))
- `format` (1 edge(s))
- `send` (1 edge(s))
- `json` (1 edge(s))
- `post` (1 edge(s))
- `is_success` (1 edge(s))
- `status` (1 edge(s))
- `create` (1 edge(s))
- `set` (1 edge(s))
- `rdkafka::ClientConfig::new` (1 edge(s))

### Incoming

- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-application/src/telemetry_export.rs` (2 edge(s))
