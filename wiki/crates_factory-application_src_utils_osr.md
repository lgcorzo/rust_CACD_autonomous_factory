---
type: module
title: "osr.rs"
source_path: "crates/factory-application/src/utils/osr.rs"
description: "Documentation for crates/factory-application/src/utils/osr.rs"
tags: [rust, module]
last_verified_commit: "17a28f4"
---

# osr.rs

Source File: `crates/factory-application/src/utils/osr.rs`

## Component Architecture

```mermaid
classDiagram
    class Empty
```

## Execution Flow

```mermaid
flowchart TD
    Start --> calculate_osr
    calculate_osr --> levenshtein_distance
    levenshtein_distance --> test_verify_osr_calculation
    test_verify_osr_calculation --> End
```
