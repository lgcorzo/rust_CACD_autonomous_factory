# 🤖 factory-application

The **Application Layer** of the Dark Gravity Autonomous Agent Factory. This crate orchestrates the high-level workflows and agentic missions.

## 🏗️ DDD Role: Application Layer

Following **Domain-Driven Design (DDD)**, `factory-application` uses services from the **Core** and **Infrastructure** layers to implement business workflows. It contains the "Brains" of the factory.

### Key Responsibilities

- **Workflow Orchestration**: Directing the flow for **Coder**, **Reviewer**, and **Security** missions.
- **Hatchet Servicing**: Concrete implementation of Hatchet worker tasks and parallel fan-out logic.
- **Task Decomposition**: Translating Jira-born goals into actionable task DAGs for the agents.

## 🛠️ Key Components

- **`agents/`**: Specialized logic for **Coder**, **Reviewer**, **Tester**, and **Architect**.
- **`missions/`**: Definition of the **autonomous-mission** workflow and its failure modes.
- **`workflows/`**: Integration points with external schedulers and triggers.

## 🧪 Testing

- **Business Logic Tests**: Mocking infrastructure dependencies to verify workflow branching.
- **BPMN Verification**: (Planned) Visual validation of the mission state machine.
