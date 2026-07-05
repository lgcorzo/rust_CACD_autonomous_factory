# GLOSSARY: Ubiquitous Language

This document defines the common terminology used throughout the **Dark Gravity** ecosystem.

---

## Core Entities

| Term | Definition | DDD Context |
| :--- | :--- | :--- |
| **Mission** | A high-level goal or problem statement the factory must resolve. | **Aggregate Root** |
| **Task** | A unit of work within a mission, assigned to an agent. | **Entity** |
| **Phase** | A distinct stage in the Hatchet execution DAG (6 phases). | **Domain Event** |
| **Agent** | An autonomous entity with specific roles and tools (Rustant, ZeroClaw). | **Domain Service** |
| **Thought** | A structured reasoning artifact produced by an agent before taking an action. | **Value Object** |
| **Artifact** | Any tangible output produced during a mission (code, tests, PRs). | **Entity** |

## Architecture Terms

| Term | Definition |
| :--- | :--- |
| **LiteLLM** | OpenAI-compatible LLM gateway for model routing. |
| **Superpowers** | Agentic skill framework; provides `writing-plans`, `subagent-driven-development`, etc. |
| **MCP** | Model Context Protocol — JSON-RPC over SSE/HTTP for agent-tool communication. |
| **Hatchet Engine** | Durable workflow engine; orchestrates the 6-phase DAG with PostgreSQL-backed state persistence. |
| **Onion Architecture** | The layered architecture pattern organizing the 5-crate Rust workspace. |
| **CRG (code-review-graph)** | Code analysis tool for semantic search, dependency graphs, and automated wiki generation. |
| **Graphify** | Code structure extraction tool for community detection and wiki report generation. |

## Infrastructure Terms

| Term | Definition |
| :--- | :--- |
| **Sandbox** | Isolated execution environment — SubprocessDriver (local) or Firecracker (KVM micro-VM). |
| **Firecracker** | Hardware-virtualized micro-VM via KVM; uses `AF_VSOCK` for host-guest communication. |
| **OpenZiti** | Zero Trust networking overlay; mTLS tunnels with zero public ports. |
| **SecurityValidator** | Trait for signature verification and content auditing (`factory-core`), implemented by `Ed25519Validator`. |
| **SecurityBounds** | Trait for JIT token issuance and validation (`factory-core`), implemented by `VaultSecurityBounds`. |
| **Vault** | HashiCorp Vault, used to dynamically provision short-lived JIT tokens via `SecurityBounds`. |
| **R2R GraphRAG** | Graph-based Retrieval-Augmented Generation for semantic codebase memory. |

## Verification Terms

| Term | Definition |
| :--- | :--- |
| **Verification Triad** | Three-pillar validation: Logical (tests), Architectural (linters), Security (LLM-as-a-Judge). |
| **LLM-as-a-Judge** | LLM analyzing code diffs; used in `security_review` MCP tool. |

## Documentation Index

| Document | Purpose |
| :--- | :--- |
| [Home](Home.md) | Wiki Home Page |
| [Business Context](BUSINESS-CONTEXT.md) | Business goals and strategy |
| [Strategic Design](STRATEGIC-DESIGN.md) | High-level system strategy |
| [Tactical Design](TACTICAL-DESIGN.md) | Domain models and component design |
| [Agent Specs](AGENT-SPECIFICATIONS.md) | Autonomous agents details |
| [Experiment Lifecycle](EXPERIMENT-LIFECYCLE.md) | Execution and MLflow experiments |
| [Adapters](INFRASTRUCTURE-ADAPTERS.md) | Connectors and external systems |
| [Verification Triad](VERIFICATION-TRIAD.md) | Three-pillar validation strategy |
| [Production Ops](PRODUCTION-OPERATIONS.md) | Deployment and maintenance |
| [Experiment Logs](EXPERIMENT-LOGS.md) | Experiment tracking logs |
| [Test Plan Report](Test_Plan_Report.md) | Testing strategy and reports |
| [README](README.md) | Project Overview |

---

*Last updated: 2026-07-02 — Verified against actual codebase via CRG analysis*