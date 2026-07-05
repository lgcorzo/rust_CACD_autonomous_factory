# skills-context

## Overview

Directory-based community: factory-mcp-server/src/skills

- **Size**: 11 nodes
- **Cohesion**: 0.1000
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| ContextSkill | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/skills/context.rs | 5-33 |
| prune_context | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/skills/context.rs | 6-24 |
| format_for_llm | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/skills/context.rs | 26-32 |
| test_prune_context_no_pruning | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/skills/context.rs | 40-44 |
| test_prune_context_with_newline | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/skills/context.rs | 47-53 |
| test_prune_context_without_newline | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/skills/context.rs | 56-60 |
| test_prune_context_empty | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/skills/context.rs | 63-66 |
| test_prune_context_max_zero | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/skills/context.rs | 69-72 |
| test_format_for_llm | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/skills/context.rs | 75-82 |
| test_prune_context_unicode_boundary | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/skills/context.rs | 85-90 |
| test_prune_context_unicode_invalid_boundary | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/skills/context.rs | 93-98 |

## Execution Flows

No execution flows pass through this community.

## Dependencies

### Outgoing

- `assert_eq` (10 edge(s))
- `to_string` (3 edge(s))
- `json` (1 edge(s))
- `len` (1 edge(s))
- `is_char_boundary` (1 edge(s))
- `rfind` (1 edge(s))

### Incoming

- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/skills/context.rs` (10 edge(s))
