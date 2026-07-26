---
type: module
title: "index_code.rs"
source_path: "crates/factory-mcp-server/src/tools/index_code.rs"
description: "Documentation for crates/factory-mcp-server/src/tools/index_code.rs"
tags: [rust, module]
last_verified_commit: "4cb40d0"
---

# index_code.rs

Source File: `crates/factory-mcp-server/src/tools/index_code.rs`

## Component Architecture

```mermaid
classDiagram
    class IndexCodeTool
```

## Execution Flow

```mermaid
flowchart TD
    Start --> new
    new --> name
    name --> description
    description --> input_schema
    input_schema --> call
    call --> test_index_code_tool_missing_content
    test_index_code_tool_missing_content --> End
```
