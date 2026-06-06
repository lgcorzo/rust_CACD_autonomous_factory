# EXPERIMENT-LIFECYCLE: Mission Execution

This document maps the journey of a mission through the factory, detailing the **6-Phase DAG** (Ingestion + 5 execution phases) orchestrated by Hatchet Engine.

---

## The 6-Phase DAG

Every mission follows a deterministic path of execution to ensure durability and quality.

```mermaid
sequenceDiagram
    participant Ext as "External Source (GitHub/GitLab/Jira)"
    participant Kafka as "Kafka: mission-input"
    participant Hatchet as "Hatchet Engine"
    participant RU as "Rustant (PO Agent)"
    participant ZC as "ZeroClaw (Developer)"
    participant LLM as "LiteLLM Gateway"
    participant R2R as "R2R GraphRAG"
    participant Sandbox as "gVisor/Firecracker Sandbox"
    participant DevOps as "DevOps Agent"
    participant Doc as "Documentation Agent"
    participant GH as "GitHub PR"

    Ext->>Kafka: New Mission (Protobuf)
    Kafka->>Hatchet: Phase 0: Ingestion Trigger
    Hatchet->>RU: Phase 1: Planning
    RU->>R2R: retrieve_context
    R2R-->>RU: Codebase context
    RU->>RU: Spec-Kit Pipeline
    Note over RU: /speckit-constitution → /speckit-specify → /speckit-plan → /speckit-tasks
    RU-->>Hatchet: Plan artifacts (spec.md, plan.md, tasks.md)
    Hatchet->>ZC: Phase 2: Implementation
    ZC->>Sandbox: execute_code
    Sandbox-->>ZC: Code + tests
    Hatchet->>ZC: Phase 3: Validation
    ZC->>Sandbox: run_tests (cargo test)
    Sandbox-->>ZC: Test results
    Hatchet->>RU: Phase 4: Security Review
    RU->>RU: security_review (OWASP SAST + LLM-as-a-Judge)
    RU-->>Hatchet: Approved / Rejected
    Hatchet->>GH: Phase 5: Delivery (create_pull_request)
    Hatchet->>Kafka: Completion → agent-thought / mission-artifact
    alt CI/CD Fails
        DevOps->>DevOps: Aethelgard Auto-Remediation Loop
        DevOps->>R2R: Query historical fixes
        DevOps->>ZC: Apply patch
    end
    alt Merge to main
        Doc->>Doc: deepwiki-rs AST delta sync
        Doc->>R2R: Upsert embeddings
        Doc->>Doc: Superpowers Wiki regeneration (OSR < 5%)
    end
```

---

## Phase Details

| Phase | Agent | Tool | Description |
| :--- | :--- | :--- | :--- |
| **0. Ingestion** | Hatchet | Kafka consumer | External trigger → `mission-input` topic |
| **1. Planning** | Rustant | `plan_mission`, `retrieve_context`, Spec-Kit | Decompose goal into spec-driven tasks |
| **2. Implementation** | ZeroClaw | `execute_code` | Generate code in gVisor/Firecracker sandbox |
| **3. Validation** | ZeroClaw | `run_tests` | Execute `cargo test` in sandbox |
| **4. Security Review** | Rustant | `security_review` | OWASP SAST + LLM-as-a-Judge ≥ 8.0/10 |
| **5. Delivery** | Hatchet | `create_pull_request` | Create GitHub PR with mission artifacts |

---

## Telemetry & MLOps

| Stream | Topic | Content |
| :--- | :--- | :--- |
| Agent Thoughts | `agent-thought` | Reasoning chains, phase transitions |
| Mission Artifacts | `mission-artifact` | Delivery summaries, PR URLs |
| Cost Attribution | Vtags (StackSpend/Finout) | Per-Epic LLM spend tracking |

---

## Closed-Loop QA

When production errors occur, the **DevOps Agent**:

1. Polls Sentry API every 15 minutes for new exceptions.
2. Grades severity (auto-filter benign warnings).
3. Maps exception to responsible microservice via R2R GraphRAG.
4. Creates prioritized backlog issue tagged `autonomous-plan`.

---

## KEDA & Scalability

```mermaid
graph LR
    K["Kafka mission-input"] -->|Queue Lag| KE["KEDA ScaledObject"]
    KE -->|Scale 0→10| D["OpenCode Agent Pods"]
```

- **Min Replicas**: 0 (Scale to zero when idle)
- **Max Replicas**: 10
- **Trigger**: Kafka lag threshold > 1
