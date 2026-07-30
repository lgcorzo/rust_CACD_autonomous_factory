---
type: module
title: "run_tests.rs"
source_path: "crates/factory-mcp-server/src/tools/run_tests.rs"
description: "Documentation for crates/factory-mcp-server/src/tools/run_tests.rs"
tags: [rust, module]
last_verified_commit: "beeed91"
---

# run_tests.rs

Source File: `crates/factory-mcp-server/src/tools/run_tests.rs`

## Component Architecture

```mermaid
classDiagram
    class RunTestsTool
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
