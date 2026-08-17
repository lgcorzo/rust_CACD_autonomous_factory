---
type: "module-documentation"
title: "sentry.rs"
source_path: "crates/factory-infrastructure/src/sentry.rs"
description: "Detailed documentation for sentry.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "198b215"
---

# File: sentry.rs

**Source Path:** `crates/factory-infrastructure/src/sentry.rs`

## Overview

### Purpose
Provides implementation for sentry.rs.

### Responsibilities
* Handles logic related to sentry.

### Dependencies
* async_trait::async_trait, serde::{Deserialize, Serialize}, serde_json::json, super::*, wiremock::matchers::{header, method, path, query_param}, wiremock::{Mock, MockServer, ResponseTemplate}

### Imported modules
* None

### Exported classes
* CrashEvent, HttpSentryClient

### Exported interfaces
* SentryClient

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### CrashEvent

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

* `culprit` (Option<String>): Purpose - Stores culprit data. Constraints - Valid Option<String>.
* `event_id` (String): Purpose - Stores event_id data. Constraints - Valid String.
* `level` (String): Purpose - Stores level data. Constraints - Valid String.
* `message` (String): Purpose - Stores message data. Constraints - Valid String.
* `project` (String): Purpose - Stores project data. Constraints - Valid String.
* `timestamp` (String): Purpose - Stores timestamp data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

None.

#### HttpSentryClient

**Overview:**
No description provided.

**Constructor:**

##### `new(url: String (Any), api_token: String (Any))`
Parameters: url: String (Any), api_token: String (Any)
Dependencies: Inherited from context
Initialization: Sets up HttpSentryClient

**Attributes:**

* `api_token` (String): Purpose - Stores api_token data. Constraints - Valid String.
* `client` (reqwest::Client): Purpose - Stores client data. Constraints - Valid reqwest::Client.
* `url` (String): Purpose - Stores url data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

* `fetch_recent_crashes(project: &str (Any), since_minutes: u64 (Any)) -> anyhow::Result<Vec<CrashEvent>>`: Internal helper logic.

#### SentryClient

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

None.

**Public Methods:**

None.

**Private Methods:**

None.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class CrashEvent {
}
class HttpSentryClient {
    -fetch_recent_crashes(project: &str:Any, since_minutes: u64:Any) : anyhow::Result<Vec<CrashEvent>>
    +new(url: String:Any, api_token: String:Any) : Self
}
SentryClient <|-- HttpSentryClient : extends/implements
interface SentryClient {
}
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "SentryService" as Svc
Caller -> Svc: execute()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```



## Examples

```
// Example usage of sentry.rs components
import { ... } from 'crates/factory-infrastructure/src/sentry.rs';
```



## Cross References
* **Parent module:** `crates/factory-infrastructure/src`
* **Dependencies:** async_trait::async_trait, serde::{Deserialize, Serialize}, serde_json::json, super::*, wiremock::matchers::{header, method, path, query_param}, wiremock::{Mock, MockServer, ResponseTemplate}
