---
type: module
title: "spec_kit_tasks_to_issues.rs"
source_path: "crates/factory-mcp-server/src/tools/spec_kit_tasks_to_issues.rs"
description: "Documentation for crates/factory-mcp-server/src/tools/spec_kit_tasks_to_issues.rs"
tags: [rust, module]
last_verified_commit: "beeed91"
---

# spec_kit_tasks_to_issues.rs

Source File: `crates/factory-mcp-server/src/tools/spec_kit_tasks_to_issues.rs`

## Component Architecture

```mermaid
classDiagram
    class SpecKitTasksToIssuesTool
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
