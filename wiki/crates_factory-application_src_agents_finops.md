# factory-application::agents::finops — FinOpsAgent

> **Source**: `crates/factory-application/src/agents/finops.rs`
> **Layer**: Application

---

## UML Class Diagram

```mermaid
classDiagram
    class FinOpsAgent {
        -String litellm_base_url
        -String api_key
        -Client client
        -FinOpsTag tag
        +inject_vtags(request) RequestBuilder
        +evaluate_spend_delta(current, previous, max) BudgetEvaluation
        +dispatch_budget_exceeded_event(kafka, mission, spend, max) Result
        +monitor_budget() Result
    }
    class BudgetEvaluation {
        <<enumeration>>
        Healthy: spend, velocity
        VelocityAnomaly: spend, velocity, alert
        HardStop: spend, velocity, max_budget, alert
    }
    class FinOpsTag {
        +String team
        +String epic
        +String microservice
        +String environment
        +String cost_center
    }

    FinOpsAgent --> BudgetEvaluation
    FinOpsAgent --> FinOpsTag
```

## Budget Enforcement Sequence

```mermaid
sequenceDiagram
    participant FIN as FinOpsAgent
    participant LLM as LiteLLM /spend/logs
    participant Kafka as Kafka
    participant Ops as Operations Team

    loop Every 60s (with exponential backoff on failure)
        FIN->>LLM: GET /spend/logs?epic={epic}
        LLM-->>FIN: Spend data (total_spend or array)
        FIN->>FIN: evaluate_spend_delta(current, previous, $50)

        alt Healthy (spend < 90%, velocity < $1/min)
            FIN->>FIN: Log healthy status
        else VelocityAnomaly (velocity > $1/min)
            FIN->>Ops: WARN: Anomaly detected
        else HardStop (spend >= 90% of $50)
            FIN->>Kafka: Publish budget-exceeded event
            FIN->>Ops: ERROR: HARDSTOP TRIGGERED
        end
    end
```

## Circuit Breaker Backoff

| Consecutive Failures | Behavior | Interval |
|:---|:---|:---|
| 0 | Normal polling | 60s |
| 1-4 | ERROR log + exponential backoff | 120s → 240s → 480s |
| 5+ | WARN log (downgraded) | Capped at 900s |

---

> *Related: [AuditorAgent](crates_factory-application_src_agents_auditor.md) · [Compliance & Audit](COMPLIANCE-AUDIT.md)*
