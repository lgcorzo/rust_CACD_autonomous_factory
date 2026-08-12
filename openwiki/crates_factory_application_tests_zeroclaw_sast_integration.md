---
type: "module-documentation"
title: "zeroclaw_sast_integration.rs"
source_path: "crates/factory-application/tests/zeroclaw_sast_integration.rs"
description: "Detailed documentation for zeroclaw_sast_integration.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "dfd90f5"
---

# File: zeroclaw_sast_integration.rs

**Source Path:** `crates/factory-application/tests/zeroclaw_sast_integration.rs`

## Overview

### Purpose
Provides implementation for zeroclaw_sast_integration.rs.

### Responsibilities
* Handles logic related to zeroclaw_sast_integration.

### Main Workflow
* Initialization and execution of zeroclaw_sast_integration logic.

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
    participant Svc as Zeroclaw_sast_integrationService
    Caller->>Svc: test_zeroclaw_allows_execution_on_sast_pass()
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
package "zeroclaw_sast_integration" {
  [Module Components]
}
@enduml

```

### Sequence Diagram
```plantuml
@startuml
autonumber
participant Caller as "Client Interface"
participant Svc as "Zeroclaw_sast_integrationService"
Caller -> Svc: test_zeroclaw_allows_execution_on_sast_pass()
note over Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

### Component Diagram
```plantuml
@startuml
component "zeroclaw_sast_integration" as comp
component "factory_application::agents::ZeroClawAgent" as factory_application::agents::ZeroClawAgent
comp --> factory_application::agents::ZeroClawAgent
component "factory_infrastructure::MockMcpClient" as factory_infrastructure::MockMcpClient
comp --> factory_infrastructure::MockMcpClient
component "serde_json::json" as serde_json::json
comp --> serde_json::json
component "std::sync::Arc" as std::sync::Arc
comp --> std::sync::Arc
@enduml

```

### Dependency Graph
```plantuml
@startuml
[zeroclaw_sast_integration]
[zeroclaw_sast_integration] --> [factory_application::agents::ZeroClawAgent]
[zeroclaw_sast_integration] --> [factory_infrastructure::MockMcpClient]
[zeroclaw_sast_integration] --> [serde_json::json]
[zeroclaw_sast_integration] --> [std::sync::Arc]
@enduml

```

### Call Graph
```plantuml
@startuml
[API] --> [No Public API]
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
