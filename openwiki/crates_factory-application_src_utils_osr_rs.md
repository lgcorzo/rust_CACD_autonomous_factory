---
type: "module-documentation"
title: "osr.rs"
source_path: "crates/factory-application/src/utils/osr.rs"
description: "Detailed documentation for osr.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-06T17:55:58Z"
---

# File: osr.rs

**Source Path:** `crates/factory-application/src/utils/osr.rs`

## Overview

### Purpose
Provides implementation for osr.rs.

### Responsibilities
* Handles logic related to osr.

### Dependencies
* super::*

### Imported modules
*

### Exported classes
* None

### Exported interfaces
* None

### Exported functions
* calculate_osr, levenshtein_distance

## Public API

### Exported Classes / Structs / Interfaces

### Exported Functions

#### `calculate_osr(wiki_content: &str (Any), r2r_text: &str (Any)) -> f32`
Executes calculate_osr.

#### `levenshtein_distance(a: &str (Any), b: &str (Any)) -> usize`
Executes levenshtein_distance.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class EmptyModule {
    }

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as OsrService
    Caller->>Svc: calculate_osr()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```


## Examples

```
// Example usage of osr.rs components
import { ... } from 'crates/factory-application/src/utils/osr.rs';
```


## Cross References
* **Parent module:** `crates/factory-application/src/utils`
* **Dependencies:** super::*
