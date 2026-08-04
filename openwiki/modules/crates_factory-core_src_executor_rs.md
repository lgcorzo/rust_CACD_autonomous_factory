---
type: "module-documentation"
title: "executor.rs"
source_path: "crates/factory-core/src/executor.rs"
description: "Detailed documentation for executor.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: executor.rs

**Source Path:** `crates/factory-core/src/executor.rs`

## Overview

### Purpose
Provides implementation for executor.rs.

### Responsibilities
* Handles logic related to executor.

### Dependencies
* std::path::PathBuf, crate::error::FactoryError, async_trait::async_trait

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### SurgicalPatch

**Overview:** Represents SurgicalPatch.

**Public Methods:**

None.

#### ExecutionResult

**Overview:** Represents ExecutionResult.

**Public Methods:**

None.

#### CodeSurgeryExecutor

**Overview:** Represents CodeSurgeryExecutor.

**Public Methods:**

None.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class SurgicalPatch {
    }
    class ExecutionResult {
    }
    class CodeSurgeryExecutor {
        <<trait>>
    }

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as ExecutorService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-core/src`
* **Dependencies:** std::path::PathBuf, crate::error::FactoryError, async_trait::async_trait
