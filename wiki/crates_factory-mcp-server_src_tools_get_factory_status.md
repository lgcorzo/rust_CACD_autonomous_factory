# factory-mcp-server::tools::get_factory_status — Factory Status MCP Tool

> **Source**: `crates/factory-mcp-server/src/tools/get_factory_status.rs`  
> **Layer**: Interface  
> **Role**: MCP tool enabling agents and external orchestrators to query real-time health, active sandbox pods, and queue depths.

---

## Tool Specification

- **Tool Name**: `get_factory_status`
- **Description**: Returns JSON system metrics including Kafka broker status, R2R reachability, active Hatchet DAG runs, and sandbox count.
- **Parameters**: None.

---

> *Related: [tools/mod.rs](crates_factory-mcp-server_src_tools_mod.md) · [Tactical Design](TACTICAL-DESIGN.md)*
