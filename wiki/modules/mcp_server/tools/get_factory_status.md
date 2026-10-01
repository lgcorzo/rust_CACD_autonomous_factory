---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-mcp-server::tools::get_factory_status — Factory Status MCP Tool"
source_path: "crates/factory-mcp-server/src/tools/get_factory_status.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-mcp-server::tools::get_factory_status — Factory Status MCP Tool."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

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

> *Related: [tools/mod.rs](mod.md) · [Tactical Design](../../../architecture/tactical_design.md)*
