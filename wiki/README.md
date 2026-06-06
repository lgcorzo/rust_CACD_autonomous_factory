# Dark Gravity — Autonomous Agent Factory (CA/CD)

Welcome to the official documentation for the **Dark Gravity Autonomous Agent Factory**. This project implements a **Continuous Agentic / Continuous Deployment (CA/CD)** pipeline following **Spec-Driven Development (SDD)**, **Domain-Driven Design (DDD)**, and **Onion Architecture** standards.

---

## Documentation Index

| Section | Description | Key Topics |
| :--- | :--- | :--- |
| [**Business Understanding**](BUSINESS-CONTEXT) | The "Why" and "What" | ROI, KPIs, Workforce, Mission Lifecycle. |
| [**Architecture**](STRATEGIC-DESIGN) | Technical Blueprint | DDD Layers, ADRs, Bounded Contexts, Zero Trust. |
| [**Execution Flows**](EXPERIMENT-LIFECYCLE) | How it Works | 6-Phase DAG, Spec-Kit Pipeline, Aethelgard Loop. |
| [**Agent Specifications**](AGENT-SPECIFICATIONS) | Specialized Workers | **Rustant** (PO), **ZeroClaw** (Dev), **DevOps**, **Documentation**. |
| [**Testing Strategy**](VERIFICATION-TRIAD) | Quality Assurance | Logical, Architectural, Security Validation. |
| [**Integrations**](INFRASTRUCTURE-ADAPTERS) | External Connectivity | **Kafka**, **R2R GraphRAG**, **OpenZiti**, **Jira**, **S3**, **Sentry**. |
| [**Deployment Guide**](PRODUCTION-OPERATIONS) | Operations & Scaling | GitOps (FluxCD), K8s Manifests, KEDA, Sealed Secrets. |

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
| **SDD Framework** | GitHub Spec-Kit |
| **Skill Framework** | Superpowers (obra/superpowers) |
| **LLM Gateway** | LiteLLM (OpenAI-compatible) |
| **Vector Store** | R2R GraphRAG + pgvector |
| **Event Bus** | Confluent Kafka (rdkafka) |
| **Zero Trust** | OpenZiti (mTLS 1.3) |
| **Sandbox** | gVisor + Firecracker micro-VM |
| **Container Orchestration** | Kubernetes + KEDA |
| **Secrets** | Bitnami SealedSecrets |
| **CI/CD** | GitHub Actions |

---

## Repository Structure

- `crates/`: Modular Rust workspace (5 crates: core, application, mcp-server, infrastructure, cli).
- `k8s/`: Kubernetes manifests for deployment.
- `.agents/skills/`: Superpowers skills for Documentation Agent.
- `wiki/`: (You are here) Comprehensive project documentation.
