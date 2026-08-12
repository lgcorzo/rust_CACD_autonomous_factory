---
type: "module-documentation"
title: "r2r.rs"
source_path: "crates/factory-infrastructure/src/r2r.rs"
description: "Detailed documentation for r2r.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "dfd90f5"
---

# File: r2r.rs

**Source Path:** `crates/factory-infrastructure/src/r2r.rs`

## Overview

### Purpose
Provides implementation for r2r.rs.

### Responsibilities
* Handles logic related to r2r.

### Main Workflow
* Initialization and execution of r2r logic.

### Dependencies
* async_trait::async_trait, serde_json::json, super::*, wiremock::matchers::{method, path}, wiremock::{Mock, MockServer, ResponseTemplate}

### Imported modules
* None

### Exported classes
* HttpR2rClient

### Exported interfaces
* R2rClient

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### HttpR2rClient

**Overview:**
Why it exists:
Provides capabilities related to HttpR2rClient.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new(url: String (Any), user: String (Any), pwd: String (Any))`
Parameters: url: String (Any), user: String (Any), pwd: String (Any)
Dependencies: Inherited from context
Initialization: Sets up HttpR2rClient

**Attributes:**

* `client` (reqwest::Client): Purpose - Stores client data. Constraints - Valid reqwest::Client.
* `pwd` (String): Purpose - Stores pwd data. Constraints - Valid String.
* `url` (String): Purpose - Stores url data. Constraints - Valid String.
* `user` (String): Purpose - Stores user data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

* `get_token() -> anyhow::Result<String>`: Internal helper logic.
* `push_osr_metric(metric: &factory_core::OsrMetric (Any)) -> anyhow::Result<()>`: Internal helper logic.
* `search(query: &str (Any)) -> anyhow::Result<String>`: Internal helper logic.

#### R2rClient

**Overview:**
Why it exists:
Provides capabilities related to R2rClient.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

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

```mermaid
classDiagram
    direction BT
    class HttpR2rClient {
        -get_token() anyhow::Result<String>
        +new(url: String:Any, user: String:Any, pwd: String:Any) Self
        -push_osr_metric(metric: &factory_core::OsrMetric:Any) anyhow::Result<()>
        -search(query: &str:Any) anyhow::Result<String>
    }
    R2rClient <|-- HttpR2rClient : Inheritance / Specialization
    class R2rClient {
        <<trait>>
    }

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as R2rService
    Caller->>Svc: get_token()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## UML

### Class Diagram
```plantuml
@startuml
class HttpR2rClient {
    -get_token() : anyhow::Result<String>
    +new(url: String:Any, user: String:Any, pwd: String:Any) : Self
    -push_osr_metric(metric: &factory_core::OsrMetric:Any) : anyhow::Result<()>
    -search(query: &str:Any) : anyhow::Result<String>
}
R2rClient <|-- HttpR2rClient : Inheritance
interface R2rClient <<trait>> {
}
@enduml

```

### Package Diagram
```plantuml
@startuml
package "r2r" {
  [Module Components]
}
@enduml

```

### Sequence Diagram
```plantuml
@startuml
autonumber
participant Caller as "Client Interface"
participant Svc as "R2rService"
Caller -> Svc: new()
note over Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

### Component Diagram
```plantuml
@startuml
component "r2r" as comp
component "async_trait::async_trait" as async_trait::async_trait
comp --> async_trait::async_trait
component "serde_json::json" as serde_json::json
comp --> serde_json::json
component "super::*" as super::*
comp --> super::*
component "wiremock::matchers::{method, path}" as wiremock::matchers::{method, path}
comp --> wiremock::matchers::{method, path}
component "wiremock::{Mock, MockServer, ResponseTemplate}" as wiremock::{Mock, MockServer, ResponseTemplate}
comp --> wiremock::{Mock, MockServer, ResponseTemplate}
@enduml

```

### Dependency Graph
```plantuml
@startuml
[r2r]
[r2r] --> [async_trait::async_trait]
[r2r] --> [serde_json::json]
[r2r] --> [super::*]
[r2r] --> [wiremock::matchers::{method, path}]
[r2r] --> [wiremock::{Mock, MockServer, ResponseTemplate}]
@enduml

```

### Call Graph
```plantuml
@startuml
[API] --> HttpR2rClient::new
@enduml

```

## Examples

```
// Example usage of r2r.rs components
import { ... } from 'crates/factory-infrastructure/src/r2r.rs';
```

## Cross References
* **Parent module:** `crates/factory-infrastructure/src`
* **Dependencies:** async_trait::async_trait, serde_json::json, super::*, wiremock::matchers::{method, path}, wiremock::{Mock, MockServer, ResponseTemplate}
