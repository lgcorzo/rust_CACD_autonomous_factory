---
type: "module-documentation"
title: "r2r.rs"
source_path: "crates/factory-infrastructure/src/r2r.rs"
description: "Detailed documentation for r2r.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "ec7bd0f"
---

# File: r2r.rs

**Source Path:** `crates/factory-infrastructure/src/r2r.rs`

## Overview

### Purpose
Provides implementation for r2r.rs.

### Responsibilities
* Handles logic related to r2r.

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
No description provided.

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
* `map_stacktrace_to_ast(stacktrace: &str (Any)) -> anyhow::Result<String>`: Internal helper logic.
* `push_osr_metric(metric: &factory_core::OsrMetric (Any)) -> anyhow::Result<()>`: Internal helper logic.
* `search(query: &str (Any)) -> anyhow::Result<String>`: Internal helper logic.

#### R2rClient

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
class HttpR2rClient {
    -get_token() : anyhow::Result<String>
    -map_stacktrace_to_ast(stacktrace: &str:Any) : anyhow::Result<String>
    +new(url: String:Any, user: String:Any, pwd: String:Any) : Self
    -push_osr_metric(metric: &factory_core::OsrMetric:Any) : anyhow::Result<()>
    -search(query: &str:Any) : anyhow::Result<String>
}
R2rClient <|-- HttpR2rClient : extends/implements
interface R2rClient {
}
@enduml

```

## Package Diagram

```plantuml
@startuml
package "crates" {
    package "factory-infrastructure" {
        package "src" {
            class Module
        }
    }
}
@enduml

```

## Component Diagram

```plantuml
@startuml
component "r2r" as Main
component "async_trait::async_trait" as async_trait__async_trait
Main --> async_trait__async_trait : uses
component "serde_json::json" as serde_json__json
Main --> serde_json__json : uses
component "super::*" as super___
Main --> super___ : uses
component "wiremock::matchers::{method, path}" as wiremock__matchers___method__path_
Main --> wiremock__matchers___method__path_ : uses
component "wiremock::{Mock, MockServer, ResponseTemplate}" as wiremock___Mock__MockServer__ResponseTemplate_
Main --> wiremock___Mock__MockServer__ResponseTemplate_ : uses
@enduml

```

## Dependency Graph

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

## Call Graph

```plantuml
@startuml
Caller --> HttpR2rClient::new
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "R2rService" as Svc
Caller -> Svc: get_token()
note right of Svc: Processing internal logic
Svc --> Caller: result
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
