---
type: module
title: "spec_kit_tool"
source_path: "crates/factory-mcp-server/src/tools/spec_kit_tool.rs"
description: "MCP tool implementation for invoking Spec-Kit CLI commands, with a dynamic SpecProvider for CLI or Mock execution."
tags: [tool, mcp, spec-kit, rust]
last_verified_commit: "eeb9c38"
---

Source File: `crates/factory-mcp-server/src/tools/spec_kit_tool.rs`

# spec_kit_tool

## Component Overview
The `spec_kit_tool` module implements an MCP `Tool` trait that enables the agent to invoke the `specify` CLI. It defines a `SpecProvider` trait to allow dynamic switching between executing the real CLI (`CliSpecProvider`) and generating mock outputs (`MockSpecProvider`). This robust design ensures the tool falls back gracefully in environments where the CLI is unavailable.

## UML Diagram

```mermaid
classDiagram
    class SpecProvider {
        <<Interface>>
        +invoke(command: SpecKitCommand, args: Vec~String~) Result~String~
    }
    class MockSpecProvider {
        +specs_dir: PathBuf
        +new(specs_dir: PathBuf) MockSpecProvider
    }
    class CliSpecProvider {
        +cli_path: String
        +fallback: MockSpecProvider
        +new(cli_path: String) CliSpecProvider
    }
    class SpecKitTool {
        -provider: Arc~dyn SpecProvider~
        +new(specify_cli_path: String) SpecKitTool
        +with_provider(provider: Arc~dyn SpecProvider~) SpecKitTool
        +invoke_spec_kit(command: SpecKitCommand, args: Vec~String~) Result~String~
    }
    SpecProvider <|-- MockSpecProvider
    SpecProvider <|-- CliSpecProvider
```

## Execution Flow

```mermaid
flowchart TD
    A[SpecKitTool::call] --> B{Parse Command & Args}
    B -- Valid --> C[invoke_spec_kit]
    B -- Invalid --> D[Return Error]
    C --> E[provider.invoke]
    E -- Success --> F[Return CallToolResult Output]
    E -- Failure --> G[Return Error Output]
```
