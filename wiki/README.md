# 🏗️ Dark Gravity — Autonomous Agent Factory (CA/CD)

Welcome to the official documentation for the **Dark Gravity Autonomous Agent Factory**. This project implements a **Continuous Agentic / Continuous Deployment (CA/CD)** pipeline following **LLMOps** and **Domain-Driven Design (DDD)** standards.

## 📖 Documentation Index

| Section | Description | Key Topics |
| :--- | :--- | :--- |
| [**Business Understanding**](business_understanding) | The "Why" and "What" | ROI, KPIs, Mission Lifecycle. |
| [**Architecture**](architecture) | Technical Blueprint | DDD Layers, Rust MCP, Rustant/ZeroClaw. |
| [**Execution Flows**](execution_flows) | How it Works | 6-Phase DAG, Sequence Diagrams, Telemetry. |
| [**Agent Specifications**](agents_specification) | Specialized Workers | **Rustant** (Architect), **ZeroClaw** (Executor). |
| [**Testing Strategy**](testing_strategy) | Quality Assurance | 95% Coverage, Firecracker, LLM-Judge. |
| [**Integrations**](integrations) | External Connectivity | **Jira**, **R2R Graph RAG**, **Kafka**. |
| [**Deployment Guide**](deployment_guide) | Operations & Scaling | K8s Manifests, Sealed Secrets. |

---

### Quick Launch

To get started with the factory, ensure you have the following prerequisites in your cluster:

1. **Hatchet Engine**: Orchestration backbone.
2. **LiteLLM Gateway**: Access to `mnimax2.5` models.
3. **R2R RAG**: Vector store for codebase context pruning.
4. **Kafka**: Telemetry sink for agent reasoning flows.
5. **OpenZiti**: Zero Trust mTLS networking.

### Core Stack

- **Agents**: Specialized Rust-native workers (**Rustant**, **ZeroClaw**).
- **Runtime**: Kubernetes (MicroK8s), Firecracker (Sandboxing).
- **Transport**: JSON-RPC over **SSE (Server-Sent Events)**.

---

## 🛠️ Repository Structure

- `crates/`: Modular Rust workspace components.
- `k8s/`: Kubernetes manifests for deployment.
- `opencode.json`: Primary configuration for the OpenCode agent.
- `wiki/`: (You are here) Comprehensive project documentation.

> [!TIP]
> Each sub-crate in `crates/` contains its own internal documentation and tests. For deep-dives into specific logic, refer to the individual crate folders.
