---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-mcp-server::tools::deep_research_tool — Deep Research MCP Tool"
source_path: "crates/factory-mcp-server/src/tools/deep_research_tool.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-mcp-server::tools::deep_research_tool — Deep Research MCP Tool."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

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

> *Related: [tools/mod.rs](mod.md) · [Tactical Design](../../../architecture/tactical_design.md)*
