---
type: module
title: "adk_driver.rs"
source_path: "crates/factory-application/src/bridge/adk_driver.rs"
description: "Documentation for crates/factory-application/src/bridge/adk_driver.rs"
tags: [rust, module]
last_verified_commit: "4cb40d0"
---

# adk_driver.rs

Source File: `crates/factory-application/src/bridge/adk_driver.rs`

## Component Architecture

```mermaid
classDiagram
    class NativeADKDriver
```

## Execution Flow

```mermaid
flowchart TD
    Start --> apply_patch
    apply_patch --> verify_syntax
    verify_syntax --> End
```
