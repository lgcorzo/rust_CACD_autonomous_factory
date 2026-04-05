# Dark Gravity: CACD Autonomous Agent Factory

[![Check CI](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/check.yml/badge.svg)](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/check.yml)
[![License](https://img.shields.io/github/license/lgcorzo/rust_CACD_autonomous_factory)](https://github.com/lgcorzo/rust_CACD_autonomous_factory/blob/main/LICENSE)

**Dark Gravity** is a high-performance, long-term agentic system serving as the core intelligence for the **CA/CD Autonomous Agent Factory**. Rebuilt from the ground up in **Rust**, it orchestrates complex multi-agent workflows with zero-trust security and durable task execution.

---

## 🏗 Standards & Architecture

This project strictly adheres to **LLMOps** and **Domain-Driven Design (DDD)** standards.

- **LLMOps Lifecycle**: We follow a robust LLM application lifecycle, including automated prompt engineering, retrieval-augmented generation (RAG) evaluation, and model-in-the-loop security audits to ensure production-grade reliability and observability.
- **DDD-based Structure**: The codebase is architected using DDD principles to maintain a clear separation of concerns. This ensures that the core domain logic remains isolated from infrastructure details and external service adapters (like Jira or R2R), allowing the system to scale and evolve without regression.

The codebase is organized as a unified Rust workspace for maximum modularity and performance.

### 1. Workspace Crate Map

- **`factory-core`**: Shared domain models, security protocols, and common kernel logic.
- **`factory-application`**: Core agent logic (`Coder`, `Reviewer`, etc.) and Hatchet-driven functional workflows.
- **`factory-mcp-server`**: High-concurrency Model Context Protocol server providing standardized tools to agents.
- **`factory-infrastructure`**: Universal clients for Jira, R2R (RAG), S3, Kafka, and OpenZiti integration with built-in mocking support and 95%+ test coverage.
- **`factory-cli`**: The unified binary entry point for workers and management.

### 2. Durable Orchestration

Missions are orchestrated using **Hatchet** (via `hatchet-sdk` v0.2.7), ensuring that long-running agentic tasks are persistent, parallelized (fan-out/fan-in), and resilient to failures.

### 3. Native Security Review & External Intelligence

Built-in security agents perform automated OWASP Top 10 scanning, while integration with **Jira** and **R2R Graph RAG** provides deep contextual awareness and project tracking capabilities.

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

### CI/CD Status

Automated checks are run on every Pull Request. See the [CI Pipeline Design](.artifacts/ci_pipeline_design.md) for detailed verification standards.

---

## 📜 Project Documentation

For a deep-dive into the factory's mechanics, follow the documentation roadmap:

1. **[Architecture](docs/architecture.md)** — The high-level technical blueprint and Zero Trust ZTA patterns.
2. **[Execution Flows](docs/execution_flows.md)** — Sequence diagrams for tool execution and SSE transport.
3. **[Testing Strategy](docs/testing_strategy.md)** — Unit, integration, and LLMOps evaluation standards.
4. **[External Integrations](docs/integrations.md)** — Connection details for **Jira** and **R2R Graph RAG**.
5. **[Deployment Guide](docs/deployment_guide.md)** — Production operations via Kubernetes, KEDA, and Sealed Secrets.

Access the full **[Documentation Index](docs/README.md)** for business context and mission history.

---

*Dark Gravity - Engineering the Future of Autonomous Systems.*
