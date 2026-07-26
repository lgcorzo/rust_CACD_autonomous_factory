---
type: module
title: "plan_mission.rs"
source_path: "crates/factory-mcp-server/src/tools/plan_mission.rs"
description: "Documentation for crates/factory-mcp-server/src/tools/plan_mission.rs"
tags: [rust, module]
last_verified_commit: "4cb40d0"
---

# plan_mission.rs

Source File: `crates/factory-mcp-server/src/tools/plan_mission.rs`

## Component Architecture

```mermaid
classDiagram
    class PlanMissionTool
```

## Execution Flow

```mermaid
flowchart TD
    Start --> new
    new --> name
    name --> description
    description --> input_schema
    input_schema --> call
    call --> End
```
