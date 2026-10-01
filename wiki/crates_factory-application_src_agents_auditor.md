# factory-application::agents::auditor — AuditorAgent

> **Source**: `crates/factory-application/src/agents/auditor.rs`
> **Layer**: Application

---

## Security Review Flow

```mermaid
sequenceDiagram
    participant AUD as AuditorAgent
    participant Hatchet as Hatchet API
    participant LLM as LiteLLM
    participant Agent as Target Agent

    AUD->>Hatchet: GET /api/v1/workflows/{id}/runs
    Hatchet-->>AUD: Failed DAG run logs
    AUD->>AUD: Filter status == "FAILED"
    AUD->>LLM: Analyze failures + recommend fixes
    LLM-->>AUD: JSON array of recommendations
    Note right of AUD: type: prompt_adjustment | tool_modification<br/>target_agent / target_tool<br/>recommendation text
    AUD->>AUD: evaluate_prompts(targets, recommendations)
    AUD->>LLM: Propose optimized system prompt
    LLM-->>AUD: New system prompt string
    AUD-->>Agent: Apply prompt adjustment
```

## SAST Gate Integration

The AuditorAgent coordinates with `SastScanResult` to enforce the security gate:

| Check | Gate Threshold |
|:---|:---|
| SAST Score | >= 8.0 / 10.0 |
| Critical Vulnerabilities | Must be 0 |
| Hardcoded Secrets | Must be 0 |

---

> *Related: [FinOpsAgent](crates_factory-application_src_agents_finops.md) · [Security Architecture](SECURITY-ARCHITECTURE.md)*
