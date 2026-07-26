---
type: module
title: "execute_code.rs"
source_path: "crates/factory-mcp-server/src/tools/execute_code.rs"
description: "Documentation for crates/factory-mcp-server/src/tools/execute_code.rs"
tags: [rust, module]
last_verified_commit: "4cb40d0"
---

# execute_code.rs

Source File: `crates/factory-mcp-server/src/tools/execute_code.rs`

## Component Architecture

```mermaid
classDiagram
    class ExecuteCodeTool
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
