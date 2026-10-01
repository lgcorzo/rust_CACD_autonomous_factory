---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-infrastructure::pipeline_classifier — Error Taxonomy"
source_path: "crates/factory-infrastructure/src/pipeline_classifier.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-infrastructure::pipeline_classifier — Error Taxonomy."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-infrastructure::pipeline_classifier — Error Taxonomy

> **Source**: `crates/factory-infrastructure/src/pipeline_classifier.rs`
> **Layer**: Infrastructure

---

CI/CD pipeline error classifier mapping raw error logs to ErrorCategory enum variants. Uses pattern matching and optional LLM-assisted classification for ambiguous errors.

---

> *Related: [lib.rs](lib.md) · [Infrastructure Adapters](../../architecture/infrastructure_adapters.md)*
