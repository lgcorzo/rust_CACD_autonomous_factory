# 🤖 AGENT-SPECIFICATIONS: Autonomous Workforce

This document specifies the roles, responsibilities, and evaluation metrics for the **[PROJECT_NAME]** autonomous agents.

---

## 🧭 The Planning Agent (e.g., Rustant)

The Planner is the "Captain" of the mission. It is responsible for architectural integrity and strategic decomposition.

### Responsibilities
- **Mission Planning**: Decomposing high-level goals into actionable task sequences.
- **Context Pruning**: Analyzing RAG results to extract precise patterns.
- **Architectural Review**: Auditing code artifacts for alignment with the **Strategic Design**.

### Evaluation (MLEval Metrics)
- **Strategy Score**: Accuracy of the plan against the domain rules.
- **Token Efficiency**: Minimizing calls while maintaining plan quality.

---

## 🛠️ The Execution Agent (e.g., ZeroClaw)

The Executor is the "Muscle" of the system, responsible for implementation and functional validation.

### Responsibilities
- **Code Implementation**: Translating task descriptions into high-quality source code.
- **Verification**: Generating and running automated test suites in a secured sandbox.
- **Self-Correction**: Iterating on code based on test failure feedback.

### Evaluation (MLEval Metrics)
- **Test Pass Rate**: Percentage of generated tests that pass.
- **Execution Latency**: Time taken to implement and verify a task.

---

## 👻 The Delivery Agent (e.g., GravityRunner)

The Delivery agent is the "Ghost in the Machine" that interfaces with version control systems and CI/CD pipelines.

### Responsibilities
- **PR Management**: Creating and updating Pull Requests with structured descriptions.
- **Action Triggering**: Initiating `workflow_dispatch` calls to verify the build in the real environment.
- **Direct Commit Fixes**: Applying emergency patches to commits directly from the cluster.

### Evaluation (MLEval Metrics)
- **Action Success Rate**: Reliability of external API calls.
- **Correction Frequency**: Number of self-correction cycles needed for a merge-ready PR.

---

## 📊 Evaluation Hierarchy

| Agent | Core LLM Model | Isolation Level | Telemetry Stream |
| :--- | :--- | :--- | :--- |
| **Planner** | `minimax-m2.7:cloud` | Containerized | Agent-Thought (Kafka) |
| **Executor** | `minimax-m2.7:cloud` | Micro-VM (KVM) | Agent-Thought (Kafka) |
| **Delivery** | `minimax-m2.7:cloud` | Secured Pod | Agent-Thought (Kafka) |

---

## 📈 MLOps 2026: Experiment Lifecycle

Every mission is an experiment. Agents log their activities to **MLflow** via the `Infrastructure Layer`. Failures at any phase are ingested into the **Feedback Registry** to allow the system to learn from historical mistakes.
