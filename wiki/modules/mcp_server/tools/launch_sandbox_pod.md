---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-mcp-server::tools::launch_sandbox_pod — K8s Job Creation"
source_path: "crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-mcp-server::tools::launch_sandbox_pod — K8s Job Creation."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-mcp-server::tools::launch_sandbox_pod — K8s Job Creation

> **Source**: `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs`
> **Layer**: Interface

---

Sandbox pod launch tool: creates gVisor-sandboxed K8s Jobs with resource constraints (30 MiB/0.25 CPU app, 20 MiB/0.10 CPU sidecar, no egress).

---

> *Related: [lib.rs](../lib.md) · [Tactical Design](../../../architecture/tactical_design.md)*
