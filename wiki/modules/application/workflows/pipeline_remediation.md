---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-application::workflows::pipeline_remediation — CI/CD Auto-Fix"
source_path: "crates/factory-application/src/workflows/pipeline_remediation.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-application::workflows::pipeline_remediation — CI/CD Auto-Fix."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-application::workflows::pipeline_remediation — CI/CD Auto-Fix

> **Source**: `crates/factory-application/src/workflows/pipeline_remediation.rs`
> **Layer**: Application

---

## CI/CD Pipeline Auto-Remediation Flow

```mermaid
flowchart TD
    FAIL["PipelineFailureEvent"] --> CLASS["ErrorCategory Classification"]
    CLASS --> CODE["CodeCompilation"] & LINT["LintViolation"] & TEST["TestFailure"] & SEC["SecurityAudit"] & INFRA["InfrastructureBuild"]

    CODE --> FIX["ZeroClawAgent: Fix compilation error"]
    LINT --> FIX
    TEST --> FIX
    SEC --> AUDIT["AuditorAgent: Security review"]
    INFRA --> SKIP["Skip: Not remediable"]

    FIX --> VALIDATE["Run tests in sandbox"]
    AUDIT --> VALIDATE
    VALIDATE --> PR["Create fix PR"]

    style FAIL fill:#f44336,stroke:#c62828,color:#fff
    style PR fill:#4CAF50,stroke:#2E7D32,color:#fff
```

---

> *Related: [autonomous_mission.rs](autonomous_mission.md)*
