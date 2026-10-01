---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-mcp-server::tools::security_review — SAST Gate"
source_path: "crates/factory-mcp-server/src/tools/security_review.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-mcp-server::tools::security_review — SAST Gate."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-mcp-server::tools::security_review — SAST Gate

> **Source**: `crates/factory-mcp-server/src/tools/security_review.rs`
> **Layer**: Interface

---

Security review tool: runs Semgrep SAST scan on code diffs and evaluates against the score >= 8.0 gate.

---

> *Related: [lib.rs](../lib.md) · [Tactical Design](../../../architecture/tactical_design.md)*
