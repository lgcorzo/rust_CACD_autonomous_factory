# 📈 EXPERIMENT-LIFECYCLE: Mission Execution

This document maps the journey of a mission through the factory, detailing the **6-Phase DAG** and the **MLflow Experiment Trace**.

---

## 🛤️ The 6-Phase DAG

Every mission follows a deterministic path of execution to ensure durability and quality.

```mermaid
sequenceDiagram
    participant GH as External Source (GitHub/GitLab)
    participant Hatchet as Hatchet Engine
    participant RU as Rustant (Planner)
    participant ZC as ZeroClaw (Executor)
    participant LLM as LiteLLM (minimax-m2.7:cloud)
    participant R2R as R2R (Context)

    GH->>Kafka: New Mission (Input Topic)
    Kafka->>Hatchet: Trigger Workflow
    Hatchet->>RU: Phase 1: Planning
    RU->>LLM: What is the strategy?
    LLM-->>RU: Step 1, Step 2, Step 3
    Hatchet->>ZC: Phase 2: Implementation
    ZC->>R2R: Get relevant code snippets
    R2R-->>ZC: Context matches
    ZC->>LLM: Generate Code & Tests
    LLM-->>ZC: Rust Code + Unit Tests
    Hatchet->>ZC: Phase 3: Validation (Sandbox)
    ZC->>Sandbox: Execute cargo test
    Sandbox-->>ZC: ALL PASS
    Hatchet->>RU: Phase 4: Review (Security/Arch)
    RU->>LLM: Audit this code
    LLM-->>RU: Approved
    Hatchet->>Delivery: Phase 5: Delivery (PR Create)
    Delivery->>GH: Open Pull Request
    Hatchet->>Kafka: Phase 6: Completion (Audit Log)
```

---

## 📊 MLOps Integration: The Experiment Trace

We treat every mission as an **MLflow Run** to track performance and regressions.

| Phase | Metric Logged | Artifact Saved |
| :--- | :--- | :--- |
| **Ingestion** | `is_valid_mission` | Metadata JSON |
| **Planning** | `token_usage`, `strategy_score` | `plan.md` |
| **Execution** | `implement_latency` | `diff.patch` |
| **Validation** | `test_pass_rate` | `test_logs.txt` |
| **Review** | `security_risk_score` | `review_report.pdf` |
| **Delivery** | `success_rate` | PR URL |

### Feedback Loop
If any phase fails, the reason and context are captured and published to the **R2R Feedback Cluster**, allowing future missions with similar patterns to avoid the same pitfalls.

---

## 🌩️ KEDA & Scalability

The factory scales based on mission demand (message lag in the input topic).

```mermaid
graph LR
    K[Kafka mission-input] -->|Queue Lag| KE[KEDA ScaledObject]
    KE -->|Scale| D[OpenCode Agent Pods]
```
