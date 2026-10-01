# factory-mcp-server::tools::deep_research_tool — Deep Research MCP Tool

> **Source**: `crates/factory-mcp-server/src/tools/deep_research_tool.rs`  
> **Layer**: Interface  
> **Role**: MCP tool enabling agents to initiate autonomous deep research workflows over Hatchet.

---

## Tool Specification

- **Tool Name**: `deep_research`
- **Description**: Triggers a multi-step background research task analyzing architecture or domain questions against R2R knowledge stores.
- **Parameters**:
  - `query` (string, required): The target question or research topic.
  - `job_id` (string, optional): Specific correlation ID.

---

> *Related: [tools/mod.rs](crates_factory-mcp-server_src_tools_mod.md) · [Tactical Design](TACTICAL-DESIGN.md)*
