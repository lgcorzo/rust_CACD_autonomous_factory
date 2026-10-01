# factory-application::agents::qa_observer — QAObserverAgent

> **Source**: `crates/factory-application/src/agents/qa_observer.rs`
> **Layer**: Application

---

## Quality Gate Enforcement

```mermaid
flowchart TD
    INPUT["Code Changes"] --> UNIT["Unit Tests"]
    INPUT --> LINT["Clippy Lints"]
    INPUT --> SAST["SAST Scan"]

    UNIT --> GATE{"All Pass?"}
    LINT --> GATE
    SAST --> GATE

    GATE -->|Yes| PASS["✅ Quality Gate Passed"]
    GATE -->|No| FAIL["❌ Quality Gate Failed"]

    style PASS fill:#4CAF50,stroke:#2E7D32,color:#fff
    style FAIL fill:#f44336,stroke:#c62828,color:#fff
```

---

> *Related: [AuditorAgent](crates_factory-application_src_agents_auditor.md) · [Verification Triad](VERIFICATION-TRIAD.md)*
