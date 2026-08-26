---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "osr.rs"
source_path: "crates/factory-application/src/utils/osr.rs"
description: "Detailed documentation for osr.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-26T06:00:08Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "e2707de"
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

#### `calculate_osr(wiki_content (&str), r2r_text (&str)) -> f32`
No description provided.

#### `levenshtein_distance(a (&str), b (&str)) -> usize`
No description provided.

## Internal architecture

```plantuml
@startuml
class EmptyModule {
}
@enduml

```

## Package Diagram

```plantuml
@startuml
package "crates" {
    package "factory-application" {
        package "src" {
            package "utils" {
                class Module
            }
        }
    }
}
@enduml

```

## Component Diagram

```plantuml
@startuml
component "osr" as Main
component "super::*" as super___
Main --> super___ : uses
@enduml

```

## Dependency Graph

```plantuml
@startuml
[osr]
[osr] --> [super::*]
@enduml

```

## Call Graph

```plantuml
@startuml
Caller --> calculate_osr
Caller --> levenshtein_distance
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
