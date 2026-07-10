# Dark Gravity: Autonomous Agent Factory

Welcome to the official wiki for **Dark Gravity**. This repository implements a **Continuous Agentic / Continuous Deployment (CA/CD)** autonomous factory following the **Onion Architecture** and **Domain-Driven Design (DDD)** standards.

The system orchestrates a specialized autonomous workforce — **Rustant** (Planner), **ZeroClaw** (Executor), **DevOps Agent** (Self-Healing), and **Documentation Agent** (Memory) — through a durable 6-phase Hatchet DAG within a Zero Trust OpenZiti dark network.

---

## Project Navigation

### 0. Context & Language
- **[GLOSSARY](GLOSSARY)**: The Ubiquitous Language defining our domain.
- **[BUSINESS-CONTEXT](BUSINESS-CONTEXT)**: Problem space, ROI, and Domain Vision.

### 1. Architecture & Design
- **[STRATEGIC-DESIGN](STRATEGIC-DESIGN)**: Bounded Contexts, ADRs, and High-Level Architecture.
- **[TACTICAL-DESIGN](TACTICAL-DESIGN)**: Crate mapping, MCP Tool Inventory, DAG Phases.

### 2. Autonomous Workforce
- **[AGENT-SPECIFICATIONS](AGENT-SPECIFICATIONS)**: Roles, Tools, and Agent Interface.
- **[EXPERIMENT-LIFECYCLE](EXPERIMENT-LIFECYCLE)**: 6-Phase DAG execution, R2R context retrieval.

### 3. Operations & Reliability
- **[INFRASTRUCTURE-ADAPTERS](INFRASTRUCTURE-ADAPTERS)**: Kafka, R2R, OpenZiti, Sandbox, Jira adapters.
- **[VERIFICATION-TRIAD](VERIFICATION-TRIAD)**: Logic, Architecture, and Security validation.
- **[PRODUCTION-OPERATIONS](PRODUCTION-OPERATIONS)**: K8s Deployments, KEDA Scaling, Secrets.

### 4. History & Governance
- **[EXPERIMENT-LOGS](EXPERIMENT-LOGS)**: Historical audit of missions and performance.
- **[Test Plan Report](Test_Plan_Report)**: Documentation verification results.

---

## CRG Analysis Snapshot

| Metric | Value |
| :--- | :--- |
| **Total Nodes** | 425 |
| **Total Edges** | 3,363 |
| **Communities** | 79 |
| **Files Analyzed** | 65 |
| **Languages** | Rust |
| **Embedded Nodes** | 425 |

---

## Graphify Analysis Snapshot

| Metric | Value |
| :--- | :--- |
| **Total Nodes** | 1,066 |
| **Edges** | 1,587 |
| **Communities** | 79 |
| **Last Updated** | 2026-07-10 |

---

*Last Updated: 2026-07-10 — Verified via CRG + Graphify analysis*