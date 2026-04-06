# 🤖 Agent Specifications: Rustant & ZeroClaw

The Dark Gravity Factory operates with specialized agents designed for high-performance and secure mission execution.

## 🏛️ Rustant (The Architect)

Rustant is the primary intelligence behind the factory's planning and review phases. It specializes in high-level strategy and security auditing.

### Rustant Responsibilities

- **Mission Planning**: Decomposing high-level goals into actionable task sequences.
- **Context Pruning**: Analyzing R2R Graph RAG results to extract precise patterns.
- **Security Review**: Auditing code artifacts for architectural violations and vulnerabilities.

### Rustant Skills

- **R2R Integration**: Direct interface with the vector store for context retrieval.
- **Strategic Reasoning**: Optimized for the `mnimax2.5` model.
- **Thought Publishing**: Real-time telemetry via Kafka.

---

## 🛠️ ZeroClaw (The Executor)

ZeroClaw is the "muscle" of the system, responsible for implementing, testing, and validating code changes.

### ZeroClaw Responsibilities

- **Code Implementation**: Translating task descriptions into high-quality code.
- **Unit/Integration Testing**: Generating and running test suites with structured feedback.
- **System Validation**: Ensuring the solution meets the goal requirements.

### ZeroClaw Skills

- **Firecracker Sandbox**: Orchestrates isolated micro-VMs (KVM) for secure code execution.
- **SSE Transport**: Handles long-running execution sessions without timeouts.
- **Workspace Management**: Standardized file manipulation and git integration.

---

## 📊 Summary of Agent Capabilities

| Feature | Rustant (Architect) | ZeroClaw (Executor) |
| :--- | :---: | :---: |
| **Hatchet Action** | `plan_mission`, `review_mission` | `code_task`, `validate_task` |
| **Logic Engine** | Planning / Security | Coding / Testing |
| **Infrastructure** | R2R / Vector Store | Sandbox / Firecracker |
| **Observability** | Agent-Thought (Kafka) | Agent-Thought (Kafka) |
| **Isolation** | Containerized | Micro-VM (KVM) |
