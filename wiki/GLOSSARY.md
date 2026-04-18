# 📖 GLOSSARY: Ubiquitous Language

This document defines the common terminology used throughout the **[PROJECT_NAME]** ecosystem. Adhering to this language ensures consistency between business requirements, system architecture, and agent reasoning.

---

## 🏗️ Core Entities

| Term | Definition | DDD Context |
| :--- | :--- | :--- |
| **Mission** | A high-level goal or problem statement (e.g., a GitHub Issue) that the factory must resolve. | **Aggregate Root** |
| **Phase** | A distinct logical stage in the mission execution DAG (e.g., Planning, Coding, Validation). | **Domain Event** |
| **Agent** | An autonomous entity with specific roles and tools (e.g., Rustant, ZeroClaw). | **Domain Service** |
| **Thought** | A structured reasoning artifact produced by an agent before taking an action. | **Value Object** |
| **Artifact** | Any tangible output produced during a mission (Code, Tests, PRs, Reports). | **Entity** |

## 🛠️ Infrastructure Terms

| Term | Definition |
| :--- | :--- |
| **Tool** | A specific MCP (Model Context Protocol) capability exposed to an agent (e.g., `read_file`, `execute_test`). |
| **Sandbox** | A secured, isolated environment (e.g., Firecracker MicroVM) where untrusted code is executed. |
| **Backbone** | The durable orchestration engine (Hatchet) that manages mission state and retries. |
| **Adapter** | An infrastructure-layer component that connects the factory to external services (GitHub, Kafka). |

## 📈 MLOps Terms

| Term | Definition |
| :--- | :--- |
| **Experiment** | A single mission execution tracked in **MLflow**. |
| **Telemetry** | Real-time streams of agent thoughts and system metrics published to **Kafka**. |
| **Verification Triad** | The three-pillar validation strategy: Logical, Architectural, and Security. |
