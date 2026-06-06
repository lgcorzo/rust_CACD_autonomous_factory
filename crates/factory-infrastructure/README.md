# 🔌 factory-infrastructure

The **Infrastructure Layer** of the Dark Gravity Autonomous Agent Factory. This crate contains the concrete implementations (Adapters) for interacting with external services.

## 🏗️ DDD Role: Infrastructure Layer

Following **Domain-Driven Design (DDD)**, `factory-infrastructure` translates domain intents into specific technical outputs. It provides the "Hands" for the factory agents.

### Key Responsibilities

- **Service Adapters**: Concrete implementations for **Jira**, **R2R (Graph RAG)**, **LiteLLM**, **Kafka (rdkafka)**, and **MinIO / AWS S3**.
- **Zero-Trust Network Overlay**: Integration with **OpenZiti** to establish mTLS 1.3 encrypted overlay connections for secure, identity-based communication between agents.
- **Telemetry & Metrics**: Publishing agent reasoning state flows and pushing Operational Sovereignty Ratio (OSR) metrics via `push_osr_metric` for telemetry analytics.
- **SSE Transport**: Managing Model Context Protocol client connections over Server-Sent Events (SSE).

## 🛠️ Integrated Clients

- **`JiraClient`**: Querying backlogs, managing issue transitions, and syncing comments.
- **`R2rClient`**: Performing semantic code search and context pruning via Graph RAG.
- **`AwsS3Storage`**: Persisting generated artifacts in MinIO/AWS S3.
- **`KafkaClient` (rdkafka)**: High-throughput Kafka client producing events to Confluent Cloud topics (`mission-input`, `agent-thought`, `mission-artifact`).
- **`OpenZitiIdentity`**: Securing client networks via mTLS 1.3 certificates.

## 🧪 Testing & Reliability

- **95% Code Coverage Target**: Mandatory for all production-grade infrastructure adapters.
- **Mocking & Integration**: Using **`wiremock`** for HTTP APIs, **`mockall`** for trait mocking, and test containers for integration validation.
- **Telemetry Verification**: Verifying metric pipelines (`push_osr_metric` and reasoning logs) are published reliably under high throughput.
