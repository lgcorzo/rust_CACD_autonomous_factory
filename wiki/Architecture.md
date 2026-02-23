# System Architecture (Domain-Driven Design)

The project follows a **Domain-Driven Design (DDD)** architecture, ensuring code is modular, scalable, and easy to test. The application is divided into distinct layers and domains.

## High-Level Structure

The high-level directory structure corresponds to the DDD layers:

- **`application/`**: Contains specific use cases and application logic (e.g., jobs for training, inference).
- **`domain/`** (Implemented as separate top-level domains):
  - **`models/`**: Defines the machine learning models and their behaviors.
  - **`evaluation/`**: Contains metrics and evaluation logic.
  - **`registry/`**: Handles model registration and versioning.
- **`infrastructure/`**: Contains technical concerns, such as I/O, messaging, and framework adapters.
- **`data_access/`**: Handles data retrieval, storage, and dataset management.

## Layers Description

### 1. Application Layer (`src/autogen_team/application`)

This layer coordinates the application's activities. It does not contain business rules or state, but delegates to the domain objects.

- **Jobs**: Defines the workflow for tasks like `TrainingJob`, `InferenceJob`, etc.
- **Agents**: Specialized worker classes (Planner, Coder, Tester, Reviewer) built for the autonomous factory.
- **MCP Server**: Coordinates autonomous AI tools (mission planning, code execution, RAG) over SSE. Prompts are externalized to YAML for dynamic behavior tuning.
- **Workflows**: Hatchet-based durable workflows (e.g., `autonomous_mission`) managing the DAG of tasks.
- **Orchestration**: The `invoke all` task coordinates the sequence of training, inference, and service startup.

### 1b. Deployment & Scalability (`k8s/`)

The application deployment is managed via Kubernetes manifests designed for a GitOps flow.

- **Workers**: OpenCode pods deployed as deployment resources natively integrated with Hatchet.
- **Autoscaling**: KEDA `ScaledObject` resources dynamically scale OpenCode workers based on Kafka topic lag (`mission-input` and `agent-thought`).
- **Services**: The MCP Server runs as a persistent service inside the `agents` namespace, allowing OpenCode to invoke tools securely.

### 2. Domain Layers

These layers represent the core business logic and rules.

- **Models (`src/autogen_team/models`)**:
  - Defines the `Model` interfaces and concrete implementations (e.g., `BaselineAutogenModel`).
  - Encapsulates model configuration, prediction, and explanation logic.

- **Evaluation (`src/autogen_team/evaluation`)**:
  - Contains metrics (e.g., `AutogenMetric`) for assessing model performance.

- **Registry (`src/autogen_team/registry`)**:
  - Manages the lifecycle of models (registering, loading, promoting).

### 3. Data Access Layer (`src/autogen_team/data_access`)

- **Repositories**: Interfaces and implementations for accessing data sources (datasets, configs).
- **Adapters**: Technical implementations for specific data backends.

### 4. Infrastructure Layer (`src/autogen_team/infrastructure`)

- **Services**: External services integration (e.g., MLflow).
- **Messaging**: Kafka or other messaging system integrations.
- **Utils**: shared utilities like signers, searchers, and splitters.

## Dependency Rule

Dependencies point **inwards** or towards the center of the architecture (Domain). Infrastructure depends on the Domain, not strictly vice-versa (via dependency inversion).
