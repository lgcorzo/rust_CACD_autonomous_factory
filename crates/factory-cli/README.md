# 🛠️ factory-cli

The **Command Line Interface (CLI)** for the Dark Gravity Autonomous Agent Factory. This crate provides a unified entry point for both developers and the automated Hatchet workers.

## 🏗️ DDD Role: Interface Layer

Following **Domain-Driven Design (DDD)**, `factory-cli` is the entry point for end-user and CI system interactions with the application.

### Key Responsibilities

- **Worker Execution**: Initializing the long-running Hatchet worker to listen for and execute autonomous agent workflows.
- **Dependency Integration**: Configuring client paths for the Model Context Protocol (MCP) server, R2R Graph RAG server, and live Confluent Kafka bootstrap brokers.
- **Telemetry Auditing**: Verifying telemetry pipelines by querying and reading events directly from Kafka topics.

## 🛠️ Commands

- **`worker`**: Run the worker process with custom endpoints:
  ```bash
  cargo run -p factory-cli -- worker \
    --mcp-url http://localhost:8100 \
    --r2r-url http://localhost:8000 \
    --kafka-brokers localhost:9092
  ```
- **Telemetry Querying**: Auditing active missions and agent thinking patterns via Confluent Kafka CLI tools or scripts:
  - **Query Mission Inputs**: Consuming from the `mission-input` topic to verify initial task specs.
  - **Query Agent Thoughts**: Tracking real-time agent reasoning by reading the `agent-thought` topic.
  - **Query Mission Artifacts**: Reading the `mission-artifact` topic to verify final generated files and metadata.

## 🧪 Testing & Verification

- **Command Parsing Tests**: Validating `clap` arguments, defaults, and error output patterns.
- **Telemetry Loop Verification**: E2E testing of message generation on `agent-thought` and consumer consumption logic.
- **Worker Workflow Integration**: Confirming that registered workflows (`mission-workflow` and `develop-task-workflow`) register correctly with the local Hatchet engine instance.
