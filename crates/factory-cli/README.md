# 🛠️ factory-cli

The **Command Line Interface (CLI)** for the Dark Gravity Autonomous Agent Factory. This crate provides a unified entry point for both developers and the automated Hatchet workers.

## 🏗️ DDD Role: Interface Layer

Following **Domain-Driven Design (DDD)**, `factory-cli` is the entry point for end-user and CI system interactions with the application.

### Key Responsibilities

- **Worker Launch**: Command to start a Hatchet worker and connect to the factory's workflow engine.
- **Local Testing**: Commands for simulating mission inputs, testing MCP tools locally, and verifying integrations.
- **Management**: CLI-based inspection of mission status, Kafka history, and MinIO artifacts.

## 🛠️ Commands

- **`worker`**: Start the long-running worker process.
- **`test-tool`**: Locally invoke any MCP tool without starting the full server.
- **`mock-server`**: Run the Axum-based API mock server for development and CI.

## 🧪 Testing

- **CLI Integration Tests**: Verifying argument parsing with `clap` and core execution flows.
- **E2E (End-to-End)**: (Planned) Full mission lifecycle testing from the CLI.
