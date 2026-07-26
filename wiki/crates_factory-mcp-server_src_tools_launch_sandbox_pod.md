---
type: module
title: "launch_sandbox_pod.rs"
source_path: "crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs"
description: "Documentation for crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs"
tags: [rust, module]
last_verified_commit: "4cb40d0"
---

# launch_sandbox_pod.rs

Source File: `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs`

## Component Architecture

```mermaid
classDiagram
    class SandboxJobSpec
    class LaunchSandboxPodTool
```

## Execution Flow

```mermaid
flowchart TD
    Start --> new
    new --> default
    default --> name
    name --> description
    description --> input_schema
    input_schema --> call
    call --> End
```
