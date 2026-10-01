---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-application::agents::auditor — AuditorAgent"
source_path: "crates/factory-application/src/agents/auditor.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-application::agents::auditor — AuditorAgent."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

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

> *Related: [FinOpsAgent](finops.md) · [Security Architecture](../../../security/security_architecture.md)*
