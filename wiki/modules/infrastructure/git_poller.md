---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-infrastructure::git_poller — Repository Change Detection"
source_path: "crates/factory-infrastructure/src/git_poller.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-infrastructure::git_poller — Repository Change Detection."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-infrastructure::git_poller — Repository Change Detection

> **Source**: `crates/factory-infrastructure/src/git_poller.rs`
> **Layer**: Infrastructure

---

Git poller for detecting new commits, branch changes, and tag updates across monitored repositories. Feeds the PollerDaemonService with change events.

---

> *Related: [lib.rs](lib.md) · [Infrastructure Adapters](../../architecture/infrastructure_adapters.md)*
