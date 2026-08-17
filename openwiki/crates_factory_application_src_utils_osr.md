---
type: "module-documentation"
title: "osr.rs"
source_path: "crates/factory-application/src/utils/osr.rs"
description: "Detailed documentation for osr.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "198b215"
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
* None

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
No description provided.

#### `levenshtein_distance(a: &str (Any), b: &str (Any)) -> usize`
No description provided.

## Internal architecture

```plantuml
@startuml
class EmptyModule {
}
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "OsrService" as Svc
Caller -> Svc: calculate_osr()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```



## Examples

```
// Example usage of osr.rs components
import { ... } from 'crates/factory-application/src/utils/osr.rs';
```



## Cross References
* **Parent module:** `crates/factory-application/src/utils`
* **Dependencies:** super::*
