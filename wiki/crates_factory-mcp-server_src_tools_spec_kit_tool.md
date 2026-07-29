---
type: module
title: "spec_kit_tool.rs"
source_path: "crates/factory-mcp-server/src/tools/spec_kit_tool.rs"
description: "Documentation for crates/factory-mcp-server/src/tools/spec_kit_tool.rs"
tags: [rust, module]
last_verified_commit: "17a28f4"
---

# spec_kit_tool.rs

Source File: `crates/factory-mcp-server/src/tools/spec_kit_tool.rs`

## Component Architecture

```mermaid
classDiagram
    class SpecKitCommand {
        <<enumeration>>
    }
    class SpecProvider {
        <<trait>>
    }
    class MockSpecProvider
    class CliSpecProvider
    class SpecKitTool
```

## Execution Flow

```mermaid
flowchart TD
    Start --> invoke
    invoke --> new
    new --> default
    default --> invoke
    invoke --> new
    new --> invoke
    invoke --> new
    new --> with_provider
    with_provider --> invoke_spec_kit
    invoke_spec_kit --> name
    name --> description
    description --> input_schema
    input_schema --> call
    call --> test_mock_spec_provider
    test_mock_spec_provider --> test_cli_spec_provider_fallback
    test_cli_spec_provider_fallback --> test_spec_kit_tool_mock_mode
    test_spec_kit_tool_mock_mode --> End
```
