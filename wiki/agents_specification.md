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

- **Workspace Management**: Standardized file manipulation and git integration.
- **Firecracker Sandbox**: Orchestrates isolated micro-VMs (KVM) for secure code execution.

---

## 👻 GravityRunner (The Direct Action Runner)

GravityRunner is a specialized agent that operates directly within the GitHub ecosystem as a self-hosted runner. It is designed for environment-sensitive tasks and direct PR management.

### GravityRunner Responsibilities

- **Direct PR Correction**: Applying trivial fixes (linting, formatting) directly to PR branches via GitHub API.
- **Workflow Orchestration**: Triggering and monitoring GitHub Actions to validate code within the standard CI environment.
- **Environment Mirroring**: Running checks in an environment that exactly matches the production target.

### GravityRunner Skills

- **GitHub App Integration**: Authenticated via scoped GitHub Apps for maximum security.
- **Direct Commitment**: Bypasses traditional Git clones for direct file-level updates via API when appropriate.
- **Pod Security**: Runs in a highly restricted Kubernetes pod with micro-segmented network access.

---

## 📊 Summary of Agent Capabilities

| Feature | Rustant (Architect) | ZeroClaw (Executor) | GravityRunner (Runner) |
| :--- | :---: | :---: | :---: |
| **Hatchet Action** | `plan_mission`, `review_mission` | `code_task`, `validate_task` | `fix_pr`, `run_workflow` |
| **Logic Engine** | Planning / Security | Coding / Testing | PR Management / CI |
| **Infrastructure** | R2R / Vector Store | Sandbox / Firecracker | GitHub Runner / Pod |
| **Observability** | Agent-Thought (Kafka) | Agent-Thought (Kafka) | Agent-Thought (Kafka) |
| **Isolation** | Containerized | Micro-VM (KVM) | Secured Pod |

---

## 📈 MLOps 2026: Experiment Lifecycle

Every mission is an experiment. Agents are integrated with **MLflow** via the `factory-infrastructure` layer:

1. **Rustant**: Logs the **Strategy Score** and **Token Usage** per plan.
2. **ZeroClaw**: Logs **Test Pass Rate** and **Execution Latency** for every task.
3. **GravityRunner**: Logs **Action Success Rate** and **PR Self-Correction** frequency.

Failures at any phase are caught and stored in the **R2R Feedback Loop** to improve future planning precision.
