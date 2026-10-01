---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-infrastructure::cursor_store — Polling State"
source_path: "crates/factory-infrastructure/src/cursor_store.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-infrastructure::cursor_store — Polling State."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-infrastructure::cursor_store — Polling State

> **Source**: `crates/factory-infrastructure/src/cursor_store.rs`
> **Layer**: Infrastructure

---

Cursor store for persisting poller state across restarts. Tracks the last processed issue/PR/pipeline event ID per repository to avoid re-processing.

---

> *Related: [lib.rs](lib.md) · [Infrastructure Adapters](../../architecture/infrastructure_adapters.md)*
