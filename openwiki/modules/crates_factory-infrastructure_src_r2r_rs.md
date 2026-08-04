---
type: "module-documentation"
title: "r2r.rs"
source_path: "crates/factory-infrastructure/src/r2r.rs"
description: "Detailed documentation for r2r.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: r2r.rs

**Source Path:** `crates/factory-infrastructure/src/r2r.rs`

## Overview

### Purpose
Provides implementation for r2r.rs.

### Responsibilities
* Handles logic related to r2r.

### Dependencies
* wiremock::{Mock, MockServer, ResponseTemplate}, async_trait::async_trait, super::*, wiremock::matchers::{method, path}, serde_json::json

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### R2rClient

**Overview:** Represents R2rClient.

**Public Methods:**

None.

#### HttpR2rClient

**Overview:** Represents HttpR2rClient.

**Public Methods:**

##### `new(url: String (Any), user: String (Any), pwd: String (Any)) -> Self`
Executes new.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class R2rClient {
        <<trait>>
    }
    class HttpR2rClient {
        +new(url: String:Any, user: String:Any, pwd: String:Any) Self
        -get_token() anyhow::Result<String>
        -search(query: &str:Any) anyhow::Result<String>
        -push_osr_metric(metric: &factory_core::OsrMetric:Any) anyhow::Result<()>
    }
    R2rClient <|-- HttpR2rClient : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as R2rService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-infrastructure/src`
* **Dependencies:** wiremock::{Mock, MockServer, ResponseTemplate}, async_trait::async_trait, super::*, wiremock::matchers::{method, path}, serde_json::json
