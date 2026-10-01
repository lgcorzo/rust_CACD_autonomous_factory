# Test Plan & Report — Dark Gravity Factory

> **Purpose**: Test strategy covering unit, integration, functional, and SAST security testing.

---

## Test Pyramid

```mermaid
graph TB
    E2E["E2E / Functional Tests<br/>run_functional_suite"]
    INT["Integration Tests<br/>Cross-crate + API"]
    UNIT["Unit Tests<br/>Per-module #[test]"]
    SAST["SAST Security Gate<br/>SastScanResult"]

    E2E --> INT
    INT --> UNIT
    UNIT --> SAST

    style UNIT fill:#4CAF50,stroke:#2E7D32,color:#fff
    style SAST fill:#f44336,stroke:#c62828,color:#fff
```

## Crate Test Coverage

| Crate | Unit Tests | Integration | SAST |
|:---|:---:|:---:|:---:|
| `factory-core` | ✅ | — | — |
| `factory-application` | ✅ | ✅ | ✅ |
| `factory-infrastructure` | ✅ | ✅ | — |
| `factory-mcp-server` | ✅ | ✅ | ✅ |
| `factory-cli` | ✅ | — | — |

## Commands

```bash
# Run all tests
cargo test --workspace

# Run with coverage
cargo llvm-cov --workspace --html

# Run functional suite
cargo run --bin run_functional_suite
```

---

> *Related: [Verification Triad](VERIFICATION-TRIAD.md) · [QAObserverAgent](crates_factory-application_src_agents_qa_observer.md)*
