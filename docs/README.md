# 🏗️ Dark Gravity — Autonomous Agent Factory (CA/CD)

Welcome to the official documentation for the **Dark Gravity Autonomous Agent Factory**. This project implements a **Continuous Agentic / Continuous Deployment (CA/CD)** pipeline, where an autonomous workforce transforms high-level mission requests into production-ready code, running entirely within a Zero Trust Kubernetes cluster.

## 📖 Documentation Index

| Section | Description | Key Topics |
| :--- | :--- | :--- |
| [**Business Understanding**](business_understanding.md) | The "Why" and "What" | ROI, KPIs, Mission Lifecycle, Problem Statement. |
| [**Architecture**](architecture.md) | Technical Blueprint | System Components, Rust MCP Server, OpenCode Workers. |
| [**Execution Flows**](execution_flows.md) | How it Works | UML Sequence Diagrams, Tool Invocations, Verification Loops. |
| [**Testing Strategy**](testing_strategy.md) | Quality Assurance | Unit, Integration, and LLM-specific Evaluation. |
| [**Deployment Guide**](deployment_guide.md) | Operations & Scaling | K8s Manifests, Sealed Secrets, KEDA Scaling. |

---

## 🚀 Quick Launch

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
- `docs/`: (You are here) Comprehensive project documentation.

> [!TIP]
> Each sub-crate in `crates/` contains its own internal documentation and tests. For deep-dives into specific logic, refer to the individual crate folders.
