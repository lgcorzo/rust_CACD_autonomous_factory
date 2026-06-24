# Dark Gravity: Autonomous Agent Factory

Welcome to the official wiki for **Dark Gravity**. This repository implements a **Continuous Agentic / Continuous Deployment (CA/CD)** autonomous factory following the **Onion Architecture** and **Domain-Driven Design (DDD)** standards.

The system orchestrates a specialized autonomous workforce — **Rustant** (Planner), **ZeroClaw** (Executor), **DevOps Agent** (Self-Healing), and **Documentation Agent** (Memory) — through a durable 6-phase Hatchet DAG within a Zero Trust OpenZiti dark network.

---

## Project Navigation

### 0. Context & Language
- **[GLOSSARY](GLOSSARY.md)**: The Ubiquitous Language defining our domain.
- **[BUSINESS-CONTEXT](BUSINESS-CONTEXT.md)**: Problem space, ROI, and Domain Vision.

### 1. Architecture & Design
- **[STRATEGIC-DESIGN](STRATEGIC-DESIGN.md)**: Bounded Contexts, ADRs, and High-Level Architecture.
- **[TACTICAL-DESIGN](TACTICAL-DESIGN.md)**: Crate mapping, MCP Tool Inventory, DAG Phases.

### 2. Autonomous Workforce
- **[AGENT-SPECIFICATIONS](AGENT-SPECIFICATIONS.md)**: Roles, Tools, and Agent Interface.
- **[EXPERIMENT-LIFECYCLE](EXPERIMENT-LIFECYCLE.md)**: 6-Phase DAG execution, R2R context retrieval.

### 3. Operations & Reliability
- **[INFRASTRUCTURE-ADAPTERS](INFRASTRUCTURE-ADAPTERS.md)**: Kafka, R2R, OpenZiti, Sandbox, Jira adapters.
- **[VERIFICATION-TRIAD](VERIFICATION-TRIAD.md)**: Logic, Architecture, and Security validation.
- **[PRODUCTION-OPERATIONS](PRODUCTION-OPERATIONS.md)**: K8s Deployments, KEDA Scaling, Secrets.

### 4. History & Governance
- **[EXPERIMENT-LOGS](EXPERIMENT-LOGS.md)**: Historical audit of missions and performance.
- **[Test Plan Report](Test_Plan_Report.md)**: Documentation verification results.

---

## CRG Analysis Snapshot

| Metric | Value |
| :--- | :--- |
| **Total Nodes** | 254 |
| **Total Edges** | 1,522 |
| **Communities** | 9 |
| **Files Analyzed** | 35 |
| **Languages** | Rust |
| **Embedded Nodes** | 195 |

---

## Graphify Analysis Snapshot

| Metric | Value |
| :--- | :--- |
| **Total Nodes** | 611 |
| **Edges** | 965 |
| **Communities** | 53 |
| **Last Updated** | 2026-06-23 |

---

*Last Updated: 2026-06-23 — Verified via CRG + Graphify analysis*