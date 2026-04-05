# 🔌 factory-infrastructure

The **Infrastructure Layer** of the Dark Gravity Autonomous Agent Factory. This crate contains the concrete implementations (Adapters) for interacting with external services.

## 🏗️ DDD Role: Infrastructure Layer

Following **Domain-Driven Design (DDD)**, `factory-infrastructure` translates domain intents into specific technical outputs. It provides the "Hands" for the factory agents.

### Key Responsibilities

- **Service Adapters**: Concrete implementations for **Jira**, **R2R (Graph RAG)**, **LiteLLM**, **Kafka**, and **MinIO**.
- **Client Traits**: Trait definitions for internal mocking and dependency injection.
- **Security**: Handling OIDC authentication, API tokens, and SSE transport details.

## 🛠️ Integrated Clients

- **`JiraClient`**: JQL search and mission status transitions.
- **`R2rClient`**: Graph RAG context retrieval and document indexing.
- **`S3Client`**: Workspace artifact persistence (via MinIO).
- **`KafkaClient`**: Mission event production and consumption.

## 🧪 Testing & Reliability

- **95% Code Coverage Target**: Mandatory for all infrastructure adapters.
- **Structural Mocking**: Using **`wiremock`** for full API simulation and **`mockall`** for trait-based unit tests.
- **Resilience**: Built-in retry logic and circuit breakers for external API calls.
