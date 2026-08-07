---
type: "module-documentation"
title: "spec_kit_tool.rs"
source_path: "crates/factory-mcp-server/src/tools/spec_kit_tool.rs"
description: "Detailed documentation for spec_kit_tool.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-07T06:39:28Z"
---

# File: spec_kit_tool.rs

**Source Path:** `crates/factory-mcp-server/src/tools/spec_kit_tool.rs`

## Overview

### Purpose
Provides implementation for spec_kit_tool.rs.

### Responsibilities
* Handles logic related to spec_kit_tool.

### Dependencies
* async_trait::async_trait, crate::protocol::CallToolResult, crate::tools::Tool, serde::{Deserialize, Serialize}, serde_json::{json, Value}, std::sync::Arc, super::*

### Imported modules
*

### Exported classes
* CliSpecProvider, MockSpecProvider, SpecKitTool

### Exported interfaces
* SpecProvider

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### CliSpecProvider

**Overview:**
Why it exists:
Provides capabilities related to CliSpecProvider.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new(cli_path: String (Any))`
Parameters: cli_path: String (Any)
Dependencies: Inherited from context
Initialization: Sets up CliSpecProvider

**Attributes:**

* `cli_path` (String): Purpose - Stores cli_path data. Constraints - Valid String.
* `fallback` (MockSpecProvider): Purpose - Stores fallback data. Constraints - Valid MockSpecProvider.

**Public Methods:**

None.

**Private Methods:**

* `invoke(command: SpecKitCommand (Any), args: Vec<String> (Any)) -> anyhow::Result<String>`: Internal helper logic.

#### MockSpecProvider

**Overview:**
Why it exists:
Provides capabilities related to MockSpecProvider.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new(specs_dir: std::path::PathBuf (Any))`
Parameters: specs_dir: std::path::PathBuf (Any)
Dependencies: Inherited from context
Initialization: Sets up MockSpecProvider

**Attributes:**

* `specs_dir` (std::path::PathBuf): Purpose - Stores specs_dir data. Constraints - Valid std::path::PathBuf.

**Public Methods:**

None.

**Private Methods:**

* `default() -> Self`: Internal helper logic.
* `invoke(command: SpecKitCommand (Any), _args: Vec<String> (Any)) -> anyhow::Result<String>`: Internal helper logic.

#### SpecKitCommand

**Overview:**
Why it exists:
Provides capabilities related to SpecKitCommand.

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

#### SpecKitTool

**Overview:**
Why it exists:
Provides capabilities related to SpecKitTool.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new(specify_cli_path: String (Any))`
Parameters: specify_cli_path: String (Any)
Dependencies: Inherited from context
Initialization: Sets up SpecKitTool

**Attributes:**

* `provider` (Arc<dyn SpecProvider>): Purpose - Stores provider data. Constraints - Valid Arc<dyn SpecProvider>.

**Public Methods:**

##### `with_provider(provider: Arc<dyn SpecProvider> (Any)) -> Self`

###### Description
Executes with_provider.

###### Inputs
* `provider: Arc<dyn SpecProvider>`: type=Any, meaning=Input for provider: Arc<dyn SpecProvider>, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: Self
Semantic meaning: Result of with_provider
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
let result = instance.with_provider();
```

##### `invoke_spec_kit(command: SpecKitCommand (Any), args: Vec<String> (Any)) -> anyhow::Result<String>`

###### Description
Executes invoke_spec_kit.

###### Inputs
* `command: SpecKitCommand`: type=Any, meaning=Input for command: SpecKitCommand, valid values=Any valid Any, optional=No, default value=None
* `args: Vec<String>`: type=Any, meaning=Input for args: Vec<String>, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: anyhow::Result<String>
Semantic meaning: Result of invoke_spec_kit
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
let result = instance.invoke_spec_kit();
```

**Private Methods:**

* `name() -> String`: Internal helper logic.
* `description() -> String`: Internal helper logic.
* `input_schema() -> Value`: Internal helper logic.
* `call(params: Value (Any)) -> anyhow::Result<CallToolResult>`: Internal helper logic.

#### SpecProvider

**Overview:**
Why it exists:
Provides capabilities related to SpecProvider.

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
    class CliSpecProvider {
        +new(cli_path: String:Any) Self
        -invoke(command: SpecKitCommand:Any, args: Vec<String>:Any) anyhow::Result<String>
    }
    SpecProvider <|-- CliSpecProvider : Inheritance / Specialization
    class MockSpecProvider {
        +new(specs_dir: std::path::PathBuf:Any) Self
        -default() Self
        -invoke(command: SpecKitCommand:Any, _args: Vec<String>:Any) anyhow::Result<String>
    }
    Default <|-- MockSpecProvider : Inheritance / Specialization
    SpecProvider <|-- MockSpecProvider : Inheritance / Specialization
    class SpecKitCommand {
        <<enumeration>>
    }
    class SpecKitTool {
        +new(specify_cli_path: String:Any) Self
        +with_provider(provider: Arc<dyn SpecProvider>:Any) Self
        +invoke_spec_kit(command: SpecKitCommand:Any, args: Vec<String>:Any) anyhow::Result<String>
        -name() String
        -description() String
        -input_schema() Value
        -call(params: Value:Any) anyhow::Result<CallToolResult>
    }
    Tool <|-- SpecKitTool : Inheritance / Specialization
    class SpecProvider {
        <<trait>>
    }

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Spec_kit_toolService
    Caller->>Svc: new()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Examples

```
// Example usage of spec_kit_tool.rs components
import { ... } from 'crates/factory-mcp-server/src/tools/spec_kit_tool.rs';
```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** async_trait::async_trait, crate::protocol::CallToolResult, crate::tools::Tool, serde::{Deserialize, Serialize}, serde_json::{json, Value}, std::sync::Arc, super::*
