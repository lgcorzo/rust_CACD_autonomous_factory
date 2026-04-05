# Solution Architecture Report (SAR): LLMOps Autogen Team

## 1. Architectural Style

The project follows **Domain-Driven Design (DDD)** and **Onion Architecture** to ensure modularity and maintainability.

## 2. REPOSITORY STRUCTURE & LINKS

> [!IMPORTANT]
> Map the repository layers to the architecture.

- **Root Directory**: [Root](file:///mnt/F024B17C24B145FE/Repos/llmops-python-package)
- **Application Layer**: [Application](file:///mnt/F024B17C24B145FE/Repos/llmops-python-package/src/autogen_team/application) -> Coordinators and Use Cases.
- **Domain Layer**: [Domain](file:///mnt/F024B17C24B145FE/Repos/llmops-python-package/src/autogen_team/models) -> Business Logic and Entities.
- **Infrastructure Layer**: [Infrastructure](file:///mnt/F024B17C24B145FE/Repos/llmops-python-package/src/autogen_team/infrastructure) -> Technical concerns and External Adapters.
- **Data Access Layer**: [Data Access](file:///mnt/F024B17C24B145FE/Repos/llmops-python-package/src/autogen_team/data_access) -> Persistence and Datasets.

## 3. Layer Detail

### 3.1 Domain Layer

- **Entities**: Autogen Models, Metrics, Schemas.
- **Value Objects**: Model Configurations, Evaluation Results.
- **Domain Services**: Registry Services, Model Promotion Logic.

### 3.2 Application Layer

- **Jobs/Use Cases**: TrainingJob, InferenceJob, HatchetInferenceJob, EvaluationJob, TuningJob.
- **MCP Server**: Autonomous tool coordinator (MCP Server Bootstrap).
- **DTOs**: Job Contexts, Request/Response schemas, MCP TextContent.

### 3.3 Infrastructure Layer

- **External Services**: MLflow (for tracking and registry), Hatchet (for orchestration), Kafka (optional messaging), LiteLLM (LLM provider), R2R (RAG engine).
- **Adapters**: Searchers, Splitters, Signers, Orchestration Workflows, MCP Tools (plan_mission, execute_code, run_tests, security_review, retrieve_context, index_code).

### 3.4 Data Access Layer

- **Repositories**: DatasetRepository, RegistryRepository.
- **Datasets**: Training data, Evaluation sets.

## 4. LLM Component Interactions

- **Vector Database**: R2R RAG (Knowledge Graph + Vector)
- **LLM Provider**: Autogen (Microsoft), LiteLLM (Gemini Pro)
- **Embedding Model**: Managed by R2R

## 5. Design Diagrams

> [!TIP]
> Use Mermaid for dynamic diagrams.

```mermaid
graph TD
    A[CLI/Job Trigger] --> B[Application Layer]
    M[MCP Client] --> MC[MCP Server]
    MC --> B
    B --> C[Domain Layer]
    C --> D[Model Implementations]
    B --> E[Infrastructure Layer]
    E --> F[MLflow / Registry]
    E --> I[Hatchet Orchestration]
    E --> R[R2R RAG / LiteLLM]
    B --> G[Data Access Layer]
    G --> H[Datasets]
    I --> B
    MC -- "dispatches" --> T[MCP Tools]
    T -- "validates" --> S[Sandbox]
```

---

_Template generated for Agentic workflows._
