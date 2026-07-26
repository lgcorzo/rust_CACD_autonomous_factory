---
type: module
title: "update_mission_status.rs"
source_path: "crates/factory-mcp-server/src/tools/update_mission_status.rs"
description: "Documentation for crates/factory-mcp-server/src/tools/update_mission_status.rs"
tags: [rust, module]
last_verified_commit: "4cb40d0"
---

# update_mission_status.rs

Source File: `crates/factory-mcp-server/src/tools/update_mission_status.rs`

## Component Architecture

```mermaid
classDiagram
    class UpdateMissionStatusTool
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
