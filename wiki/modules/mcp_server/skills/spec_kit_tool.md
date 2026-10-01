---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "Spec Kit MCP Tool — Comprehensive Documentation"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for Spec Kit MCP Tool — Comprehensive Documentation."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# Spec Kit MCP Tool — Comprehensive Documentation

> **Purpose**: Bridges MCP protocol to Spec-Kit SDD workflow for autonomous mission planning and execution.

---

## Tool Interface

| Field | Value |
|:---|:---|
| **MCP Tool Name** | `spec_kit_tool` |
| **Category** | Planning |
| **Authorized Agents** | RustantAgent |
| **Input** | Goal description + codebase context |
| **Output** | SDD artifacts (spec.md, plan.md, tasks.md) |

## SDD Workflow Bridge

```mermaid
sequenceDiagram
    participant Agent as RustantAgent
    participant MCP as spec_kit_tool
    participant SpecKit as Spec Kit CLI

    Agent->>MCP: invoke("spec_kit_tool", {goal, context})
    MCP->>SpecKit: speckit init
    MCP->>SpecKit: speckit specify --goal "{goal}"
    MCP->>SpecKit: speckit plan
    MCP->>SpecKit: speckit tasks
    SpecKit-->>MCP: tasks.md content
    MCP->>MCP: parse_sdd_tasks(content)
    MCP-->>Agent: SddMissionPlan with SddTaskItems
```

---

> *Related: [RustantAgent](../../application/agents/rustant.md) · [Tactical Design](../../../architecture/tactical_design.md)*
