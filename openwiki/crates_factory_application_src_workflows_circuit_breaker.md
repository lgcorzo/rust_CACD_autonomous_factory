---
type: "module-documentation"
title: "circuit_breaker.rs"
source_path: "crates/factory-application/src/workflows/circuit_breaker.rs"
description: "Detailed documentation for circuit_breaker.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "198b215"
---

# File: circuit_breaker.rs

**Source Path:** `crates/factory-application/src/workflows/circuit_breaker.rs`

## Overview

### Purpose
Provides implementation for circuit_breaker.rs.

### Responsibilities
* Handles logic related to circuit_breaker.

### Dependencies
* factory_core::security::SastScanResult, serde::{Deserialize, Serialize}, super::*

### Imported modules
* None

### Exported classes
* CircuitBreakerGuard

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### CircuitBreakerGuard

**Overview:**
No description provided.

**Constructor:**

##### `new(max_attempts: u32 (Any), min_safety_score: f32 (Any))`
Parameters: max_attempts: u32 (Any), min_safety_score: f32 (Any)
Dependencies: Inherited from context
Initialization: Sets up CircuitBreakerGuard

**Attributes:**

* `current_attempt` (u32): Purpose - Stores current_attempt data. Constraints - Valid u32.
* `max_attempts` (u32): Purpose - Stores max_attempts data. Constraints - Valid u32.
* `min_safety_score` (f32): Purpose - Stores min_safety_score data. Constraints - Valid f32.

**Public Methods:**

##### `evaluate_diff(diff: &str (Any)) -> (CircuitBreakerStatus, SastScanResult)`

###### Description
/// Evaluates the code diff against the security threshold and tracks attempts.

###### Inputs
* `diff: &str`: type=Any, meaning=Input for diff: &str, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: (CircuitBreakerStatus, SastScanResult)
Semantic meaning: Result of evaluate_diff
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
let result = instance.evaluate_diff();
```

##### `format_stuck_alert(repo: &str (Any), pr_number: u64 (Any), reason: &str (Any)) -> String`

###### Description
/// Generates human architect escalation message when stuck.

###### Inputs
* `repo: &str`: type=Any, meaning=Input for repo: &str, valid values=Any valid Any, optional=No, default value=None
* `pr_number: u64`: type=Any, meaning=Input for pr_number: u64, valid values=Any valid Any, optional=No, default value=None
* `reason: &str`: type=Any, meaning=Input for reason: &str, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: String
Semantic meaning: Result of format_stuck_alert
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
let result = instance.format_stuck_alert();
```

**Private Methods:**

* `default() -> Self`: Internal helper logic.

#### CircuitBreakerStatus

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
class CircuitBreakerGuard {
    -default() : Self
    +evaluate_diff(diff: &str:Any) : (CircuitBreakerStatus, SastScanResult)
    +format_stuck_alert(repo: &str:Any, pr_number: u64:Any, reason: &str:Any) : String
    +new(max_attempts: u32:Any, min_safety_score: f32:Any) : Self
}
Default <|-- CircuitBreakerGuard : extends/implements
enum CircuitBreakerStatus {
}
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "Circuit_breakerService" as Svc
Caller -> Svc: default()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```



## Examples

```
// Example usage of circuit_breaker.rs components
import { ... } from 'crates/factory-application/src/workflows/circuit_breaker.rs';
```



## Cross References
* **Parent module:** `crates/factory-application/src/workflows`
* **Dependencies:** factory_core::security::SastScanResult, serde::{Deserialize, Serialize}, super::*
