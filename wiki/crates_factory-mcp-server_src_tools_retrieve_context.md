---
type: module
title: "retrieve_context.rs"
source_path: "crates/factory-mcp-server/src/tools/retrieve_context.rs"
description: "Documentation for crates/factory-mcp-server/src/tools/retrieve_context.rs"
tags: [rust, module]
last_verified_commit: "17a28f4"
---

# retrieve_context.rs

Source File: `crates/factory-mcp-server/src/tools/retrieve_context.rs`

## Component Architecture

```mermaid
classDiagram
    class RetrieveContextTool
    class ManualMockR2rClient
```

## Execution Flow

```mermaid
flowchart TD
    Start --> new
    new --> name
    name --> description
    description --> input_schema
    input_schema --> call
    call --> search
    search --> push_osr_metric
    push_osr_metric --> test_retrieve_context_tool_success
    test_retrieve_context_tool_success --> test_retrieve_context_tool_failure
    test_retrieve_context_tool_failure --> End
```
