---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-mcp-server::sandbox — gVisor Pod Lifecycle"
source_path: "crates/factory-mcp-server/src/sandbox.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-mcp-server::sandbox — gVisor Pod Lifecycle."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-mcp-server::sandbox — gVisor Pod Lifecycle

> **Source**: `crates/factory-mcp-server/src/sandbox.rs`
> **Layer**: Interface

---

gVisor sandbox pod lifecycle management: creation, health monitoring, resource enforcement, and cleanup of K8s Jobs with RuntimeClass=gvisor.

---

> *Related: [lib.rs](lib.md) · [Tactical Design](../../architecture/tactical_design.md)*
