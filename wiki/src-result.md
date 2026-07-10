# src-result

## Overview

Directory-based community: factory-core/src

- **Size**: 26 nodes
- **Cohesion**: 0.0500
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| FactoryError | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/error.rs | 4-34 |
| SurgicalPatch | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/executor.rs | 6-10 |
| ExecutionResult | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/executor.rs | 13-17 |
| Metadata | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs | 12-17 |
| Inputs | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs | 21-23 |
| Outputs | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs | 27-30 |
| Targets | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs | 34-37 |
| Mission | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs | 41-48 |
| MissionStatus | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs | 52-57 |
| Task | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs | 61-68 |
| TaskStatus | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs | 72-77 |
| SHAPValues | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs | 81-85 |
| FeatureImportances | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs | 89-92 |
| SpecArtifact | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs | 95-99 |
| OsrMetric | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs | 102-107 |
| FinOpsTag | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs | 110-116 |
| ComplianceReport | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs | 119-123 |
| UserFeedbackPayload | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs | 126-132 |
| SandboxConstraint | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/security.rs | 7-11 |
| AuditResult | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/security.rs | 21-24 |
| Ed25519SecurityValidator | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/security.rs | 26-28 |
| SecurityValidator | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/security.rs | 31-54 |
| validate_signature | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/security.rs | 32-45 |
| audit_content | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/security.rs | 47-53 |
| JitToken | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/security.rs | 59-61 |
| wipe_token_from_memory | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/security.rs | 67-70 |

## Execution Flows

No execution flows pass through this community.

## Dependencies

### Outgoing

- `Ok` (2 edge(s))
- `map_err` (2 edge(s))
- `crate::error::FactoryError::Security` (2 edge(s))
- `format` (2 edge(s))
- `vec` (1 edge(s))
- `decode` (1 edge(s))
- `Signature::from_slice` (1 edge(s))
- `is_ok` (1 edge(s))
- `verify` (1 edge(s))
- `zeroize` (1 edge(s))

### Incoming

- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/lib.rs` (15 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/security.rs` (6 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/executor.rs` (2 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-core/src/error.rs` (1 edge(s))
