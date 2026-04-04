# Dark Gravity: CACD Autonomous Agent Factory

[![Check CI](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/check.yml/badge.svg)](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/check.yml)
[![License](https://img.shields.io/github/license/lgcorzo/rust_CACD_autonomous_factory)](https://github.com/lgcorzo/rust_CACD_autonomous_factory/blob/main/LICENSE)

**Dark Gravity** is a high-performance, long-term agentic system serving as the core intelligence for the **CA/CD Autonomous Agent Factory**. Rebuilt from the ground up in **Rust**, it orchestrates complex multi-agent workflows with zero-trust security and durable task execution.

---

## 🎯 Architecture & Guiding Principles

The codebase follows strict **Domain-Driven Design (DDD)** and is organized as a unified Rust workspace for maximum modularity and performance.

### 1. Workspace Crate Map
- **`factory-core`**: Shared domain models, security protocols, and common kernel logic.
- **`factory-application`**: Core agent logic (`Coder`, `Reviewer`, etc.) and Hatchet-driven functional workflows.
- **`factory-mcp-server`**: High-concurrency Model Context Protocol server providing standardized tools to agents.
- **`factory-infrastructure`**: Universal clients for MCP, S3, Kafka, and OpenZiti integration with built-in mocking support.
- **`factory-cli`**: The unified binary entry point for workers and management.

### 2. Durable Orchestration
Missions are orchestrated using **Hatchet** (via `hatchet-sdk` v0.2.7), ensuring that long-running agentic tasks are persistent, parallelized (fan-out/fan-in), and resilient to failures.

### 3. Native Security Review
Built-in security agents perform automated OWASP Top 10 scanning, SQL/Command injection detection, and secret leak prevention over every generated diff.

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

## 📜 Documentation
- [Rust Migration Plan](.artifacts/migration_plan_rust.md)
- [Testing Implementation](.artifacts/task_testing_implementation.md)
- [Final Walkthrough](.artifacts/walkthrough_rust_migration.md)

---
*Dark Gravity - Engineering the Future of Autonomous Systems.*
