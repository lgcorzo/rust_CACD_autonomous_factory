# src-tool

## Overview

Directory-based community: factory-mcp-server/src

- **Size**: 36 nodes
- **Cohesion**: 0.0988
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| handle_feedback | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/feedback_route.rs | 11-80 |
| McpServer | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 34-262 |
| Default | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 28-32 |
| default | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 29-31 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 35-40 |
| add_tool | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 42-45 |
| register_default_tools | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 46-126 |
| handle_request | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 128-159 |
| handle_list_tools | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 161-178 |
| handle_call_tool | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 180-202 |
| sse_handler | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 204-230 |
| post_handler | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 232-248 |
| error_response | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 250-261 |
| ax_keep_alive | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 264-268 |
| test_list_tools | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 277-303 |
| test_call_tool_not_found | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 306-318 |
| test_call_tool_error_sanitization | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 321-345 |
| test_call_tool_success | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 348-372 |
| main | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/main.rs | 11-55 |
| JsonRpcRequest | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/protocol.rs | 4-11 |
| JsonRpcResponse | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/protocol.rs | 14-22 |
| JsonRpcError | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/protocol.rs | 25-29 |
| McpTool | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/protocol.rs | 33-37 |
| CallToolResult | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/protocol.rs | 41-45 |
| McpContent | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/protocol.rs | 49-53 |
| ExecutionResult | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 8-13 |
| execute_surgery | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 19-27 |
| NativeSurgerySandboxDriver | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 30-32 |
| SandboxDriver | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 122-175 |
| execute | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 123-174 |
| execute_surgery | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 40-46 |
| SubprocessDriver | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 49-49 |
| SandboxMode | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 114-117 |
| GvisorK8sDriver | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 119-119 |
| test_subprocess_driver_timeout | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 182-188 |
| main | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/scratch.rs | 5-17 |

## Execution Flows

- **handle_request** (criticality: 0.42, depth: 1)
- **post_handler** (criticality: 0.42, depth: 2)
- **register_default_tools** (criticality: 0.36, depth: 1)
- **handle_call_tool** (criticality: 0.36, depth: 1)
- **sse_handler** (criticality: 0.36, depth: 1)

## Dependencies

### Outgoing

- `to_string` (35 edge(s))
- `std::env::var` (23 edge(s))
- `Box::new` (15 edge(s))
- `Some` (14 edge(s))
- `unwrap_or_else` (11 edge(s))
- `clone` (10 edge(s))
- `json` (10 edge(s))
- `Arc::new` (9 edge(s))
- `unwrap` (8 edge(s))
- `tracing::info` (6 edge(s))
- `Ok` (6 edge(s))
- `assert` (6 edge(s))
- `map` (5 edge(s))
- `McpServer::new` (5 edge(s))
- `return_const` (5 edge(s))

### Incoming

- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs` (10 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs` (8 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/protocol.rs` (6 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/feedback_route.rs` (1 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/main.rs` (1 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/scratch.rs` (1 edge(s))
