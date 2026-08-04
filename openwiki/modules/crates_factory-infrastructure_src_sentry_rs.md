---
type: "module-documentation"
title: "sentry.rs"
source_path: "crates/factory-infrastructure/src/sentry.rs"
description: "Detailed documentation for sentry.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: sentry.rs

**Source Path:** `crates/factory-infrastructure/src/sentry.rs`

## Overview

### Purpose
Provides implementation for sentry.rs.

### Responsibilities
* Handles logic related to sentry.

### Dependencies
* wiremock::{Mock, MockServer, ResponseTemplate}, super::*, wiremock::matchers::{header, method, path, query_param}, serde_json::json, async_trait::async_trait, serde::{Deserialize, Serialize}

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### CrashEvent

**Overview:** Represents CrashEvent.

**Public Methods:**

None.

#### SentryClient

**Overview:** Represents SentryClient.

**Public Methods:**

None.

#### HttpSentryClient

**Overview:** Represents HttpSentryClient.

**Public Methods:**

##### `new(url: String (Any), api_token: String (Any)) -> Self`
Executes new.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class CrashEvent {
    }
    class SentryClient {
        <<trait>>
    }
    class HttpSentryClient {
        +new(url: String:Any, api_token: String:Any) Self
        -fetch_recent_crashes(project: &str:Any, since_minutes: u64:Any) anyhow::Result<Vec<CrashEvent>>
    }
    SentryClient <|-- HttpSentryClient : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as SentryService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-infrastructure/src`
* **Dependencies:** wiremock::{Mock, MockServer, ResponseTemplate}, super::*, wiremock::matchers::{header, method, path, query_param}, serde_json::json, async_trait::async_trait, serde::{Deserialize, Serialize}
