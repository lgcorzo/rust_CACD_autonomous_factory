# Dark Gravity: CACD Autonomous Agent Factory

[![CI/CD Pipeline](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/pipeline.yml/badge.svg)](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/pipeline.yml)
[![Wiki Content Sync](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/docs-to-wiki.yml/badge.svg)](https://github.com/lgcorzo/rust_CACD_autonomous_factory/actions/workflows/docs-to-wiki.yml)
[![Docker Pulls](https://img.shields.io/docker/pulls/lgcorzo/dark-gravity-factory.svg)](https://hub.docker.com/r/lgcorzo/dark-gravity-factory)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

**Dark Gravity** is a high-performance, long-term agentic system serving as the core intelligence for the **CA/CD Autonomous Agent Factory**. Rebuilt from the ground up in **Rust**, it orchestrates complex multi-agent workflows with zero-trust security and durable task execution.

---

## 🏗 Standards & Architecture

This project strictly adheres to **LLMOps** and **Domain-Driven Design (DDD)** standards, elevated to the production-grade **Version 7 (V7) Architecture** for autonomous operations.

- **LLMOps Lifecycle**: We follow a robust LLM application lifecycle, including automated prompt engineering, retrieval-augmented generation (RAG) evaluation, and model-in-the-loop security audits to ensure production-grade reliability and observability.
- **DDD-based Structure**: The codebase is architected using DDD principles to maintain a clear separation of concerns. This ensures that the core domain logic remains isolated from infrastructure details and external service adapters (like Jira or R2R), allowing the system to scale and evolve without regression.

The codebase is organized as a unified Rust workspace for maximum modularity and performance.

### 1. Workspace Crate Map

- **`factory-core`**: Innermost domain layer. Manages domain entities (`Mission`, `Task`), a `SecurityValidator` trait for agent output auditing, and unified error handling via `FactoryError`.
- **`factory-application`**: Workflow orchestration layer. Leverages **Rustant** (Planner) and **ZeroClaw** (Executor) to process task workflows. Maintains durably checkpointed state via `StepCheckpoint`s using Hatchet's PostgreSQL backend.
- **`factory-mcp-server`**: Presentation layer. Axum-based Model Context Protocol server exposing tools over Server-Sent Events (SSE) transport with isolated Firecracker execution interfaces.
- **`factory-infrastructure`**: Adapter layer. Concrete clients for Jira, R2R (RAG), MinIO/AWS S3, OpenZiti (Zero Trust overlay networks), and Confluent Cloud Kafka (via `rdkafka` client).
- **`factory-cli`**: Interface entry point. Contains the Hatchet worker initializer and Kafka telemetry querying utilities.

### 2. Durable Orchestration (Phase 12 State-Loop)

Missions are orchestrated using a robust **6-phase DAG** in **Hatchet** with database-backed `StepCheckpoint` recovery:
`Ingestion` → `Plan (Rustant)` → `Code (ZeroClaw)` → `Validation (ZeroClaw)` → `Review (Rustant)` → `Delivery (GitOps)`.

### 3. Integrated Intelligence & Security

- **Rustant**: Leverages **R2rClient** for semantic code search and context retrieval to inform specification design and planning.
- **ZeroClaw**: Compiles and executes code in secure **Firecracker micro-VMs** via a sandboxed executor driver.
- **Zero Trust Mesh**: Agent-to-agent communication is fully encrypted and bound to identity namespaces using **OpenZiti mTLS 1.3** overlays.
- **Telemetry**: Reasoning logs are published to **Confluent Cloud Kafka** topics (`mission-input`, `agent-thought`, `mission-artifact`) for real-time observability.

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

1. **[Strategic Design](wiki/STRATEGIC-DESIGN.md)** — Bounded contexts, Onion Architecture layers, and Zero Trust ZTA patterns.
2. **[Execution Flows & Lifecycle](wiki/EXPERIMENT-LIFECYCLE.md)** — Hatchet 6-phase DAG logic, step checkpointing, and real-time Kafka telemetry.
3. **[Verification Triad](wiki/VERIFICATION-TRIAD.md)** — Strict testing strategy (logic, architecture, and security) with sandboxed execution.
4. **[Infrastructure Adapters](wiki/INFRASTRUCTURE-ADAPTERS.md)** — Production connectors for Jira, R2R Graph RAG, and live rdkafka Kafka adapters.
5. **[Production Operations](wiki/PRODUCTION-OPERATIONS.md)** — Operational scaling, Kubernetes, KEDA, OpenZiti overlays, and secure memory management.

Access the full **[Documentation Index](wiki/README.md)** for business context and mission history.

---

*Dark Gravity - Engineering the Future of Autonomous Systems.*
