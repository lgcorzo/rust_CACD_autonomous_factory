# src-tool

## Overview

Directory-based community: factory-mcp-server/src

- **Size**: 33 nodes
- **Cohesion**: 0.1259
- **Dominant Language**: rust

## Members

| Name | Kind | File | Lines |
|------|------|------|-------|
| McpServer | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 32-200 |
| Default | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 26-30 |
| default | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 27-29 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 33-38 |
| add_tool | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 40-43 |
| register_default_tools | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 44-89 |
| handle_request | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 91-97 |
| handle_list_tools | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 99-116 |
| handle_call_tool | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 118-140 |
| sse_handler | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 142-168 |
| post_handler | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 170-186 |
| error_response | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 188-199 |
| ax_keep_alive | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 202-206 |
| test_list_tools | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 215-241 |
| test_call_tool_not_found | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 244-256 |
| test_call_tool_error_sanitization | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 259-283 |
| test_call_tool_success | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs | 286-310 |
| main | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/main.rs | 11-51 |
| JsonRpcRequest | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/protocol.rs | 4-9 |
| JsonRpcResponse | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/protocol.rs | 12-17 |
| JsonRpcError | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/protocol.rs | 20-24 |
| McpTool | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/protocol.rs | 27-31 |
| CallToolResult | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/protocol.rs | 34-37 |
| McpContent | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/protocol.rs | 41-45 |
| ExecutionResult | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 8-13 |
| SubprocessDriver | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 20-20 |
| SandboxDriver | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 94-122 |
| execute | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 95-121 |
| test_subprocess_driver_timeout | Test | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 68-74 |
| FirecrackerDriver | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 87-91 |
| Default | Class | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 81-85 |
| default | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 82-84 |
| new | Function | /mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs | 88-90 |

## Execution Flows

- **handle_request** (criticality: 0.42, depth: 1)
- **post_handler** (criticality: 0.42, depth: 2)
- **register_default_tools** (criticality: 0.36, depth: 1)
- **handle_call_tool** (criticality: 0.36, depth: 1)
- **sse_handler** (criticality: 0.36, depth: 1)

## Dependencies

### Outgoing

- `to_string` (22 edge(s))
- `Some` (11 edge(s))
- `std::env::var` (11 edge(s))
- `Box::new` (11 edge(s))
- `json` (9 edge(s))
- `clone` (7 edge(s))
- `unwrap` (6 edge(s))
- `Arc::new` (6 edge(s))
- `Ok` (6 edge(s))
- `assert` (6 edge(s))
- `McpServer::new` (5 edge(s))
- `return_const` (5 edge(s))
- `assert_eq` (5 edge(s))
- `get` (4 edge(s))
- `tracing::info` (4 edge(s))

### Incoming

- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/lib.rs` (8 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/sandbox.rs` (8 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/protocol.rs` (6 edge(s))
- `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-mcp-server/src/main.rs` (1 edge(s))
