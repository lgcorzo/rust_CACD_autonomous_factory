---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-application::agents::doc_agent — DocumentationAgent"
source_path: "crates/factory-application/src/agents/doc_agent.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-application::agents::doc_agent — DocumentationAgent."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

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

> *Related: [OSR Calculator](../utils/osr.md) · [HITL Governance](../../../security/hitl_governance.md)*
