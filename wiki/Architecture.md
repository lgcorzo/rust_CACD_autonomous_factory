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
