---
iso_doc_type: "Description"
iso_viewpoint: "DeploymentView"
type: "operations"
title: "Experiment Logs — Telemetry Schema"
description: "ISO 42010 DeploymentView / ISO 15289 Description documentation for Experiment Logs — Telemetry Schema."
tags: ['iso42010', 'okf', 'operations', 'deployment']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# Experiment Logs — Telemetry Schema

> **Purpose**: Experiment log format, telemetry schema, and dashboard integration.

---

## Log Format

```json
{
  "mission_id": "uuid",
  "phase": "Plan|Code|Validation|Review|Delivery",
  "agent": "RustantAgent|ZeroClawAgent|AuditorAgent",
  "timestamp": "2026-10-01T12:00:00Z",
  "duration_ms": 1500,
  "status": "success|failure|timeout",
  "metrics": {
    "tokens_used": 5420,
    "files_modified": 3,
    "tests_passed": 12,
    "tests_failed": 0,
    "sast_score": 9.5
  }
}
```

## Dashboard Integration

| Metric | Source | Dashboard |
|:---|:---|:---|
| Mission throughput | Hatchet API | Grafana: Missions/hour |
| Token spend | LiteLLM /spend/logs | Grafana: FinOps panel |
| SAST gate pass rate | AuditorAgent | Grafana: Security panel |
| Circuit breaker trips | Aethelgard | Grafana: Reliability panel |

---

> *Related: [Experiment Lifecycle](../architecture/runtime_sequences.md) · [Compliance & Audit](../security/compliance_audit.md)*
