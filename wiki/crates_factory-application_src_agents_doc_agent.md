# factory-application::agents::doc_agent — DocumentationAgent

> **Source**: `crates/factory-application/src/agents/doc_agent.rs`
> **Layer**: Application

---

## Wiki Generation Flow

```mermaid
sequenceDiagram
    participant DOC as DocumentationAgent
    participant AST as AST Parser
    participant Wiki as wiki/ Directory
    participant OSR as OSR Calculator

    DOC->>AST: Parse source files for public symbols
    AST-->>DOC: Symbol list (structs, traits, functions)
    DOC->>Wiki: Generate OKF markdown per module
    DOC->>OSR: Calculate Omission/Staleness Ratio
    OSR-->>DOC: OSR metric

    alt OSR < 5%
        DOC->>DOC: ✅ Quality gate passed
    else OSR >= 5%
        DOC->>DOC: ❌ Regenerate missing docs
    end
```

## OSR (Omission/Staleness Ratio) Calculation

| Metric | Formula | Gate |
|:---|:---|:---|
| Missing Symbols | `undocumented_public_symbols / total_public_symbols` | < 5% |
| Stale Pages | `pages_not_updated_since_last_code_change / total_pages` | < 5% |
| Combined OSR | `max(missing, stale)` | < 5% |

---

> *Related: [OSR Calculator](crates_factory-application_src_utils_osr.md) · [HITL Governance](HITL-GOVERNANCE.md)*
