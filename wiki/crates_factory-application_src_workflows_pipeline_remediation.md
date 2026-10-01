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

> *Related: [autonomous_mission.rs](crates_factory-application_src_workflows_autonomous_mission.md)*
