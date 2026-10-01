---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-mcp-server::tools::inspect_kafka_topic — Kafka Topic Inspection MCP Tool"
source_path: "crates/factory-mcp-server/src/tools/inspect_kafka_topic.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-mcp-server::tools::inspect_kafka_topic — Kafka Topic Inspection MCP Tool."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-mcp-server::tools::inspect_kafka_topic — Kafka Topic Inspection MCP Tool

> **Source**: `crates/factory-mcp-server/src/tools/inspect_kafka_topic.rs`  
> **Layer**: Interface  
> **Role**: MCP tool allowing diagnostic inspection of recent messages on factory Kafka topics (e.g. `mission-input`, `agent-thought`, `factory-telemetry`).

---

## Tool Specification

- **Tool Name**: `inspect_kafka_topic`
- **Description**: Reads the last `N` messages from a designated Kafka topic for diagnostic inspection.
- **Parameters**:
  - `topic` (string, required): Name of topic.
  - `limit` (integer, optional, default: 10): Number of messages to retrieve.

---

> *Related: [kafka.rs](../../infrastructure/kafka.md) · [Tactical Design](../../../architecture/tactical_design.md)*
