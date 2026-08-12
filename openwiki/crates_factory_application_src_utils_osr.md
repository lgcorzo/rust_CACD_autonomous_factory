---
type: "module-documentation"
title: "osr.rs"
source_path: "crates/factory-application/src/utils/osr.rs"
description: "Detailed documentation for osr.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "dfd90f5"
---

# File: osr.rs

**Source Path:** `crates/factory-application/src/utils/osr.rs`

## Overview

### Purpose
Provides implementation for osr.rs.

### Responsibilities
* Handles logic related to osr.

### Main Workflow
* Initialization and execution of osr logic.

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
Executes calculate_osr.

#### `levenshtein_distance(a: &str (Any), b: &str (Any)) -> usize`
Executes levenshtein_distance.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class EmptyModule {
    }

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as OsrService
    Caller->>Svc: calculate_osr()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## UML

### Class Diagram
```plantuml
@startuml
class EmptyModule {
}
@enduml

```

### Package Diagram
```plantuml
@startuml
package "osr" {
  [Module Components]
}
@enduml

```

### Sequence Diagram
```plantuml
@startuml
autonumber
participant Caller as "Client Interface"
participant Svc as "OsrService"
Caller -> Svc: calculate_osr()
note over Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

### Component Diagram
```plantuml
@startuml
component "osr" as comp
component "super::*" as super::*
comp --> super::*
@enduml

```

### Dependency Graph
```plantuml
@startuml
[osr]
[osr] --> [super::*]
@enduml

```

### Call Graph
```plantuml
@startuml
[API] --> calculate_osr
[API] --> levenshtein_distance
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
