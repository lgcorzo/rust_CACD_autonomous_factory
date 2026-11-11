---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-mcp-server::tools::launch_sandbox_pod — Kubernetes Job Creation"
source_path: "crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-mcp-server::tools::launch_sandbox_pod — Kubernetes Job Creation."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "false"
# 2026-11-10 DG-RVP-018 A2: auto-generated "true" withdrawn; pending human review
last_verified_commit: "fbdc5848"
---

# factory-mcp-server::tools::launch_sandbox_pod — Kubernetes Job Creation

> **Source**: `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs`
> **Layer**: Interface

---

Sandbox job builder defaults: memory limit `512Mi`, memory request `64Mi`, CPU limit `500m`, and CPU request `50m`. This conflicts with the admission cap of `30Mi`/`250m`. The builder requests RuntimeClass `gvisor` when gVisor is required. Network policy enforcement is **TO CONFIRM**.

---

> *Related: [lib.rs](../lib.md) · [Tactical Design](../../../architecture/tactical_design.md)*
