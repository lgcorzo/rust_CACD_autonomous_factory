---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-mcp-server::tools::security_review — SAST Gate"
source_path: "crates/factory-mcp-server/src/tools/security_review.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-mcp-server::tools::security_review — SAST Gate."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-11-07T14:20:00Z"
generated: "agent:okf-professional-documenter"
verified: "false"
# 2026-11-10 DG-RVP-018 A2: auto-generated "true" withdrawn; pending human review
last_verified_commit: "f1420c76"
---

# factory-mcp-server::tools::security_review — SAST Gate

> **Source**: `crates/factory-mcp-server/src/tools/security_review.rs`
> **Layer**: Interface

---

Security review tool: analyzes code diffs or task instructions for security vulnerabilities using LLM SAST (Aethelgard) and heuristic scanning, enforcing an approved threshold of score >= 8.0. Benign functional requests, documentation, and routine code without exploit payloads evaluate to score >= 8.0 approved status.

```mermaid
flowchart TD
    A["Input Diff / Task"] --> B["Heuristic Scan"]
    B --> C{"Heuristic Score >= 8.0?"}
    C -->|Yes| D["LiteLLM Gateway Query (ollama/qwen2.5:7b)"]
    C -->|No / Penalty Found| E["Reject Score < 8.0"]
    D --> F{"LLM Vulnerability Detected?"}
    F -->|No / Safe| G["Approved (Score >= 8.0)"]
    F -->|Yes / Injection| E
```

---

> *Related: [lib.rs](../lib.md) · [Tactical Design](../../../architecture/tactical_design.md)*
