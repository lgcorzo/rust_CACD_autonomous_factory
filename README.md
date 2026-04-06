# Dark Gravity: CACD Autonomous Agent Factory

[![CI/CD Pipeline](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/pipeline.yml/badge.svg)](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/pipeline.yml)
[![Wiki Content Sync](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/docs-to-wiki.yml/badge.svg)](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/docs-to-wiki.yml)
[![Docker Pulls](https://img.shields.io/docker/pulls/lgcorzo/dark-gravity-factory.svg)](https://hub.docker.com/r/lgcorzo/dark-gravity-factory)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

**Dark Gravity** is a high-performance, long-term agentic system serving as the core intelligence for the **CA/CD Autonomous Agent Factory**. Rebuilt from the ground up in **Rust**, it orchestrates complex multi-agent workflows with zero-trust security and durable task execution.

---

## 🏗 Standards & Architecture

This project strictly adheres to **LLMOps** and **Domain-Driven Design (DDD)** standards.

- **LLMOps Lifecycle**: We follow a robust LLM application lifecycle, including automated prompt engineering, retrieval-augmented generation (RAG) evaluation, and model-in-the-loop security audits to ensure production-grade reliability and observability.
- **DDD-based Structure**: The codebase is architected using DDD principles to maintain a clear separation of concerns. This ensures that the core domain logic remains isolated from infrastructure details and external service adapters (like Jira or R2R), allowing the system to scale and evolve without regression.

The codebase is organized as a unified Rust workspace for maximum modularity and performance.

### 1. Workspace Crate Map

- **`factory-core`**: Shared domain models, security protocols, and common kernel logic.
- **`factory-application`**: High-performance agent logic featuring **Rustant** (Architect/Planner) and **ZeroClaw** (Executor). Orchestrates specialized 6-phase Hatchet DAGs.
- **`factory-mcp-server`**: High-concurrency Model Context Protocol server updated for **SSE transport** and integrated skills (Firecracker, R2R Context Pruning).
- **`factory-infrastructure`**: Universal clients for Jira, R2R (RAG), Kafka (**agent-thought** telemetry), and OpenZiti integration.

### 2. Durable Orchestration (Phase 12 State-Loop)

Missions are orchestrated using a robust **6-phase DAG** in **Hatchet**:
`Ingestion` → `Plan (Rustant)` → `Code (ZeroClaw)` → `Validation (ZeroClaw)` → `Review (Rustant)` → `Delivery (GitOps)`.

### 3. Integrated Intelligence & Security

- **Rustant**: Leverages **R2R Graph RAG** with a custom 3-level context pruning skill for high-precision planning.
- **ZeroClaw**: Executes code in isolated **Firecracker micro-VMs** for maximum security and resource efficiency.
- **Telemetry**: Real-time agent reasoning is published to **Kafka** for auditability and monitoring via OpenWebUI.

---

## 🚀 Getting Started

### Prerequisites

- [Rust Toolchain](https://rustup.rs/) (Stable 1.75+)
- [Hatchet Server](https://github.com/hatchet-dev/hatchet)
- [MCP Connection](https://modelcontextprotocol.io/)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/lgcorzo/rust_CACD_autonomous_factory
   cd rust_CACD_autonomous_factory
   ```

2. Build the workspace:

   ```bash
   cargo build --release
   ```

### Running the Factory Worker

Start the unified worker to begin processing autonomous missions:

```bash
# Provide your MCP gateway URL
cargo run -p factory-cli -- worker --mcp-url http://localhost:8100
```

---

## 🛠 Development Workflow

We enforce high source quality through automated CI pipelines.

### Standard Commands

- **Check Style**: `cargo fmt --all -- --check`
- **Static Analysis**: `cargo clippy --workspace -- -D warnings`
- **Run Tests**: `cargo test --workspace`

## ✅ Repository Checks

The following automated checks are active in this repository to maintain high engineering standards:

### 1. CI/CD Pipeline (`pipeline.yml`)

- **Linting**: Enforces strict `rustfmt` and `clippy` (warnings-as-errors) across the entire workspace.
- **Testing**: Executes all unit and integration tests (excluding heavy smoke tests).
- **Protoc Build**: Automated Protocol Buffers compilation verification for all service definitions.
- **Docker Build**: Validates that a production-ready image can be built and pushed to Docker Hub upon merge to `main`.

### 2. Wiki Content Sync (`docs-to-wiki.yml`)

- Maintains bi-directional synchronization between the repo's `wiki/` folder and the GitHub Project Wiki.
- Ensures documentation remains consistent and discoverable.

### 3. Image Integrity & Deployment

- Production images are published under `lgcorzo/dark-gravity-factory`.
- Every release candidate is tagged with the specific `git SHA` for full auditability and rollback capability.

### 4. Code Standards

- **Edition 2024 Readiness**: The project uses modern Rust editions for safety and performance.
- **Dependency Guard**: Automated monitoring of critical libraries (Hatchet, OpenZiti, the AWS SDK).

---

## 📜 Project Documentation

For a deep-dive into the factory's mechanics, follow the documentation roadmap:

1. **[Architecture](wiki/architecture.md)** — The high-level technical blueprint and Zero Trust ZTA patterns.
2. **[Execution Flows](wiki/execution_flows.md)** — Sequence diagrams for tool execution and SSE transport.
3. **[Testing Strategy](wiki/testing_strategy.md)** — Unit, integration, and LLMOps evaluation standards.
4. **[External Integrations](wiki/integrations.md)** — Connection details for **Jira** and **R2R Graph RAG**.
5. **[Deployment Guide](wiki/deployment_guide.md)** — Production operations via Kubernetes, KEDA, and Sealed Secrets.

Access the full **[Documentation Index](wiki/README.md)** for business context and mission history.

---

*Dark Gravity - Engineering the Future of Autonomous Systems.*
