---
type: "module-documentation"
title: "osr.rs"
source_path: "crates/factory-application/src/utils/osr.rs"
description: "Detailed documentation for osr.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
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

## Public API & Architecture

### Exported Classes / Structs / Interfaces

### Exported Functions

#### `calculate_osr(wiki_content: &str (Any), r2r_text: &str (Any)) -> f32`
Executes calculate_osr.

#### `levenshtein_distance(a: &str (Any), b: &str (Any)) -> usize`
Executes levenshtein_distance.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class EmptyModule {
    }

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as OsrService
    Caller->>Svc: calculate_osr()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-application/src/utils`
* **Dependencies:** super::*
