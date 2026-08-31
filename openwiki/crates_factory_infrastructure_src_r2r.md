---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "r2r.rs"
source_path: "crates/factory-infrastructure/src/r2r.rs"
description: "Detailed documentation for r2r.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-31T05:39:54Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "7bf01b8"
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

##### `new(url (String), user (String), pwd (String))`
Parameters: url (String), user (String), pwd (String)
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

* `get_token(self (Self)) -> anyhow::Result<String>`: Internal helper logic.
* `map_stacktrace_to_ast(self (Self), stacktrace (&str)) -> anyhow::Result<String>`: Internal helper logic.
* `push_osr_metric(self (Self), metric (&factory_core::OsrMetric)) -> anyhow::Result<()>`: Internal helper logic.
* `search(self (Self), query (&str)) -> anyhow::Result<String>`: Internal helper logic.

#### R2rClient

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

None.

**Public Methods:**

##### `map_stacktrace_to_ast(self (Self), stacktrace (&str)) -> anyhow::Result<String>`

###### Description
No description provided.

###### Inputs
* `self`: type=Self, meaning=Input for self, valid values=Any valid Self, optional=No, default value=None
* `stacktrace`: type=&str, meaning=Input for stacktrace, valid values=Any valid &str, optional=No, default value=None

###### Output
Return type: anyhow::Result<String>
Semantic meaning: Result of map_stacktrace_to_ast
Possible null values: Conditional
Exceptions: None handled explicitly

###### Side Effects
Database updates: None
File operations: None
Network calls: None
Cache: None
State changes: Updates internal variables

###### Complexity
Time Complexity: O(1) mostly
Space Complexity: O(1) mostly

###### Example
```
let result = instance.map_stacktrace_to_ast();
```

##### `push_osr_metric(self (Self), metric (&factory_core::OsrMetric)) -> anyhow::Result<()>`

###### Description
No description provided.

###### Inputs
* `self`: type=Self, meaning=Input for self, valid values=Any valid Self, optional=No, default value=None
* `metric`: type=&factory_core::OsrMetric, meaning=Input for metric, valid values=Any valid &factory_core::OsrMetric, optional=No, default value=None

###### Output
Return type: anyhow::Result<()>
Semantic meaning: Result of push_osr_metric
Possible null values: Conditional
Exceptions: None handled explicitly

###### Side Effects
Database updates: None
File operations: None
Network calls: None
Cache: None
State changes: Updates internal variables

###### Complexity
Time Complexity: O(1) mostly
Space Complexity: O(1) mostly

###### Example
```
let result = instance.push_osr_metric();
```

##### `search(self (Self), query (&str)) -> anyhow::Result<String>`

###### Description
No description provided.

###### Inputs
* `self`: type=Self, meaning=Input for self, valid values=Any valid Self, optional=No, default value=None
* `query`: type=&str, meaning=Input for query, valid values=Any valid &str, optional=No, default value=None

###### Output
Return type: anyhow::Result<String>
Semantic meaning: Result of search
Possible null values: Conditional
Exceptions: None handled explicitly

###### Side Effects
Database updates: None
File operations: None
Network calls: None
Cache: None
State changes: Updates internal variables

###### Complexity
Time Complexity: O(1) mostly
Space Complexity: O(1) mostly

###### Example
```
let result = instance.search();
```

**Private Methods:**

None.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class HttpR2rClient {
    -get_token(self: Self) anyhow::Result<String>
    -map_stacktrace_to_ast(self: Self, stacktrace: &str) anyhow::Result<String>
    +new(url: String, user: String, pwd: String) Self
    -push_osr_metric(self: Self, metric: &factory_core::OsrMetric) anyhow::Result<()>
    -search(self: Self, query: &str) anyhow::Result<String>
}
R2rClient <|-- HttpR2rClient : extends/implements
interface R2rClient {
    +map_stacktrace_to_ast(self: Self, stacktrace: &str) anyhow::Result<String>
    +push_osr_metric(self: Self, metric: &factory_core::OsrMetric) anyhow::Result<()>
    +search(self: Self, query: &str) anyhow::Result<String>
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
Caller --> R2rClient::map_stacktrace_to_ast
Caller --> R2rClient::push_osr_metric
Caller --> R2rClient::search
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
