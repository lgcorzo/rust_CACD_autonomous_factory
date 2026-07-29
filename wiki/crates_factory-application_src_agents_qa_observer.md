---
type: module
title: "qa_observer.rs"
source_path: "crates/factory-application/src/agents/qa_observer.rs"
description: "Documentation for crates/factory-application/src/agents/qa_observer.rs"
tags: [rust, module]
last_verified_commit: "17a28f4"
---

# qa_observer.rs

Source File: `crates/factory-application/src/agents/qa_observer.rs`

## Component Architecture

```mermaid
classDiagram
    class QAObserverAgent
```

## Execution Flow

```mermaid
flowchart TD
    Start --> default
    default --> new
    new --> monitor_crashes
    monitor_crashes --> name
    name --> execute
    execute --> End
```
