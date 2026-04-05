# 🏗️ Dark Gravity — Autonomous Agent Factory (CA/CD)

Welcome to the official documentation for the **Dark Gravity Autonomous Agent Factory**. This project implements a **Continuous Agentic / Continuous Deployment (CA/CD)** pipeline following **LLMOps** and **Domain-Driven Design (DDD)** standards.

## 📖 Documentation Index

| Section | Description | Key Topics |
| :--- | :--- | :--- |
| [**Business Understanding**](business_understanding.md) | The "Why" and "What" | ROI, KPIs, Mission Lifecycle. |
| [**Architecture**](architecture.md) | Technical Blueprint | DDD Layers, Rust MCP, OpenCode. |
| [**Execution Flows**](execution_flows.md) | How it Works | Sequence Diagrams, Agent Loops. |
| [**Testing Strategy**](testing_strategy.md) | Quality Assurance | 95% Coverage, WireMock, LLM-Judge. |
| [**Integrations**](integrations.md) | External Connectivity | **Jira**, **R2R Graph RAG**, LiteLLM. |
| [**Deployment Guide**](deployment_guide.md) | Operations & Scaling | K8s Manifests, Sealed Secrets. |

---

### Quick Launch

To get started with the factory, ensure you have the following prerequisites in your cluster:

1. **Hatchet Engine**: Orchestration backbone.
2. **LiteLLM Gateway**: Access to Gemini/MiniMax models.
3. **R2R RAG**: Vector store for codebase context.
4. **OpenZiti**: Zero Trust networking.

### Core Stack

- **Languages**: Rust (MCP Server), Go (OpenCode).
- **Runtime**: Kubernetes (MicroK8s), Firecracker (Sandboxing).
- **Transport**: JSON-RPC over HTTP/SSE.

---

## 🛠️ Repository Structure

- `crates/`: Modular Rust workspace components.
- `k8s/`: Kubernetes manifests for deployment.
- `opencode.json`: Primary configuration for the OpenCode agent.
- `wiki/`: (You are here) Comprehensive project documentation.

> [!TIP]
> Each sub-crate in `crates/` contains its own internal documentation and tests. For deep-dives into specific logic, refer to the individual crate folders.
