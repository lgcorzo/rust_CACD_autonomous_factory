---
type: "module-documentation"
title: "zeroclaw_sast_integration.rs"
source_path: "crates/factory-application/tests/zeroclaw_sast_integration.rs"
description: "Detailed documentation for zeroclaw_sast_integration.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "ec7bd0f"
---

# File: zeroclaw_sast_integration.rs

**Source Path:** `crates/factory-application/tests/zeroclaw_sast_integration.rs`

## Overview

### Purpose
Provides implementation for zeroclaw_sast_integration.rs.

### Responsibilities
* Handles logic related to zeroclaw_sast_integration.

### Dependencies
* factory_application::agents::ZeroClawAgent, factory_infrastructure::MockMcpClient, serde_json::json, std::sync::Arc

### Imported modules
* None

### Exported classes
* None

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

### Exported Functions

None.

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
        package "tests" {
            class Module
        }
    }
}
@enduml

```

## Component Diagram

```plantuml
@startuml
component "zeroclaw_sast_integration" as Main
component "factory_application::agents::ZeroClawAgent" as factory_application__agents__ZeroClawAgent
Main --> factory_application__agents__ZeroClawAgent : uses
component "factory_infrastructure::MockMcpClient" as factory_infrastructure__MockMcpClient
Main --> factory_infrastructure__MockMcpClient : uses
component "serde_json::json" as serde_json__json
Main --> serde_json__json : uses
component "std::sync::Arc" as std__sync__Arc
Main --> std__sync__Arc : uses
@enduml

```

## Dependency Graph

```plantuml
@startuml
[zeroclaw_sast_integration]
[zeroclaw_sast_integration] --> [factory_application::agents::ZeroClawAgent]
[zeroclaw_sast_integration] --> [factory_infrastructure::MockMcpClient]
[zeroclaw_sast_integration] --> [serde_json::json]
[zeroclaw_sast_integration] --> [std::sync::Arc]
@enduml

```

## Call Graph

```plantuml
@startuml
Caller --> Module : no public API
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "Zeroclaw_sast_integrationService" as Svc
Caller -> Svc: test_zeroclaw_allows_execution_on_sast_pass()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

## Examples

```
// Example usage of zeroclaw_sast_integration.rs components
import { ... } from 'crates/factory-application/tests/zeroclaw_sast_integration.rs';
```

## Cross References
* **Parent module:** `crates/factory-application/tests`
* **Dependencies:** factory_application::agents::ZeroClawAgent, factory_infrastructure::MockMcpClient, serde_json::json, std::sync::Arc
