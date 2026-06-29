# 🤖 factory-application

The **Application Layer** of the Dark Gravity Autonomous Agent Factory. This crate orchestrates the high-level workflows and agentic missions.

## 🏗️ DDD Role: Application Layer

Following **Domain-Driven Design (DDD)**, `factory-application` uses services from the **Core** and **Infrastructure** layers to implement business workflows. It contains the "Brains" of the factory.

### Key Responsibilities

- **Durable Orchestration (Hatchet DAG)**: Directing the 6-phase state loop (`Ingestion` → `Plan (Rustant)` → `Code (ZeroClaw)` → `Validation (ZeroClaw)` → `Review (Rustant)` → `Delivery (GitOps)`) using Hatchet's orchestrator.
- **State Checkpointing**: Persisting step checkpoints (`StepCheckpoint`) to Hatchet's PostgreSQL backend, enabling crash-resilient mission execution and automated state recovery.
- **Agent Orchestration**: Coordinating Rustant (planning) and ZeroClaw (execution) agents with structured context passing and state management.
- **Task Decomposition**: Parsing complex, high-level Jira requirements into direct, dependency-resolved task graphs.

## 🛠️ Key Components

- **`agents/`**: Core agent logic:
  - **`rustant.rs`**: The architect and planner agent utilizing semantic context pruning.
  - **`zeroclaw.rs`**: The code generator and executor agent operating in isolated environments.
- **`workflows/`**: Hatchet step definitions and orchestration hooks:
  - **`autonomous_mission.rs`**: The primary 6-phase mission orchestrator managing state transitions.
  - **`develop_task.rs`**: Granular task execution, validation loops, and git delivery handlers.

## 🧪 Testing & Verification

- **State Transition Testing**: Validating branching logic, failure scenarios, and rollback paths within the mission state machine.
- **Agent Coordination Testing**: Verifying that Rustant and ZeroClaw correctly pass context and respond to state transitions within the mission lifecycle.
- **Checkpoint Recovery Verification**: Unit tests asserting that transient failures trigger automatic resume workflows from the last successfully stored `StepCheckpoint`.
