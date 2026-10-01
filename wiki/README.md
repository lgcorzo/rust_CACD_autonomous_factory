---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "Dark Gravity — Autonomous Agent Factory (CA/CD)"
description: "ISO 42010 ComponentView / ISO 15289 Description documentation for Dark Gravity — Autonomous Agent Factory (CA/CD)."
tags: ['iso42010', 'okf']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# Dark Gravity — Autonomous Agent Factory (CA/CD)

Welcome to the official documentation for the **Dark Gravity Autonomous Agent Factory**. This project implements a **Continuous Agentic / Continuous Deployment (CA/CD)** pipeline following **Domain-Driven Design (DDD)** and **Onion Architecture** standards.

---

## Documentation Index

| Section | Description | Key Topics |
| :--- | :--- | :--- |
| [**Business Understanding**](architecture/business_context.md) | The "Why" and "What" | ROI, KPIs, Workforce, Mission Lifecycle. |
| [**Architecture**](architecture/strategic_design.md) | Technical Blueprint | DDD Layers, ADRs, Bounded Contexts, Zero Trust. |
| [**Execution Flows**](architecture/runtime_sequences.md) | How it Works | 6-Phase DAG, Agent Orchestration, Mission Lifecycle. |
| [**Agent Specifications**](architecture/agent_specifications.md) | Specialized Workers | **Rustant** (Planner), **ZeroClaw** (Executor), **DevOps**, **Documentation**. |
| [**Testing Strategy**](security/verification_triad.md) | Quality Assurance | Logical, Architectural, Security Validation. |
| [**Integrations**](architecture/infrastructure_adapters.md) | External Connectivity | **Kafka**, **R2R GraphRAG**, **OpenZiti**, **Jira**, **S3**, **Sentry**. |
| [**Deployment Guide**](operations/production_operations.md) | Operations & Scaling | GitOps, K8s Manifests, KEDA, Sealed Secrets. |
| [**User Manual**](operations/user_manual.md) | User Guides | Project onboarding, setup, and usage. |

---

### Core Stack

| Component | Technology |
| :--- | :--- |
| **Language** | Rust (edition 2021/2024) |
| **Async Runtime** | Tokio (full features) |
| **HTTP Server** | Axum 0.7 |
| **Serialization** | Serde + Serde JSON |
| **Workflow Engine** | Hatchet SDK (Rust) |
| **MCP Protocol** | JSON-RPC over SSE |
| **LLM Gateway** | LiteLLM (OpenAI-compatible) |
| **Vector Store** | R2R GraphRAG + pgvector |
| **Event Bus** | Confluent Kafka (rdkafka) |
| **Zero Trust** | OpenZiti (mTLS 1.3) |
| **Sandbox** | Firecracker micro-VM |
| **Container Orchestration** | Kubernetes + KEDA |
| **Secrets** | Bitnami SealedSecrets |
| **CI/CD** | GitHub Actions |

---

## Repository Structure

- `crates/`: Modular Rust workspace (5 crates: core, application, mcp-server, infrastructure, cli).
- `k8s/`: Kubernetes manifests for deployment.
- `.agents/skills/`: Superpowers skills for Documentation Agent.
- `wiki/`: (You are here) Comprehensive project documentation.

---

## CRG Analysis Summary

Based on `code-review-graph` analysis of the codebase:

- **Total Nodes**: 254 (functions, structs, traits, tests)
- **Total Edges**: 1,522 (calls, dependencies, data flows)
- **Communities**: 9 (agents, workflows, tools, clients, mission, CLI, tests, skills, src)
- **Files Analyzed**: 35 Rust files across 5 crates

### Key Communities (CRG)

| Community | Crate | Nodes | Purpose |
|-----------|-------|-------|---------|
| `tools-tool` | factory-mcp-server | 68 | MCP tools (plan_mission, execute_code, retrieve_context, etc.) |
| `src-client` | factory-infrastructure | 42 | Service clients (Jira, R2R, Kafka, MCP, S3, Ziti, Vault) |
| `agents-agent` | factory-application | 15 | Rustant (Planner) & ZeroClaw (Executor) agents |
| `workflows-mission` | factory-application | 6 | Hatchet workflows (mission, task orchestration) |
| `src-mission` | factory-core | 12 | Domain models (Mission, Task, Metadata, SecurityValidator, SecurityBounds) |
| `src-tool` | factory-mcp-server | 33 | Sandbox, protocol, protocol handlers |
| `skills-context` | .agents/skills | 11 | Superpowers skill definitions |
| `src-cli` | factory-cli | 3 | CLI entry point |
| `tests-security` | tests | 4 | Security integration tests |

---

## Getting Started

1. **Prerequisites**: Rust 1.75+, Hatchet server, LiteLLM gateway
2. **Build**: `cargo build --release --workspace`
3. **Run Worker**: `cargo run -p factory-cli -- worker --mcp-url http://localhost:8100`

---

*Last updated: 2026-07-02 — Generated from CRG analysis + source verification*