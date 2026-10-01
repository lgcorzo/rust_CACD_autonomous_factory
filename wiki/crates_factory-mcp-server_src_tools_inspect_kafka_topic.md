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

> *Related: [kafka.rs](crates_factory-infrastructure_src_kafka.md) · [Tactical Design](TACTICAL-DESIGN.md)*
