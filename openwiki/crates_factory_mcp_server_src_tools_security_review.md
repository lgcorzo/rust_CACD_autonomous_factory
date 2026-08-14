---
type: "module-documentation"
title: "security_review.rs"
source_path: "crates/factory-mcp-server/src/tools/security_review.rs"
description: "Detailed documentation for security_review.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "7982a81"
---

# File: security_review.rs

**Source Path:** `crates/factory-mcp-server/src/tools/security_review.rs`

## Overview

### Purpose
Provides implementation for security_review.rs.

### Responsibilities
* Handles logic related to security_review.

### Dependencies
* async_openai::Client, async_openai::config::OpenAIConfig, async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionResponseFormat, ChatCompletionResponseFormatType,
    CreateChatCompletionRequestArgs,
}, async_trait::async_trait, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, serde_json::{json, Value}, std::env, super::*

### Imported modules
* None

### Exported classes
* SecurityReviewTool

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### SecurityReviewTool

**Overview:**
Why it exists:
Provides capabilities related to SecurityReviewTool.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new()`
Parameters:
Dependencies: Inherited from context
Initialization: Sets up SecurityReviewTool

**Attributes:**

* `client` (Client<OpenAIConfig>): Purpose - Stores client data. Constraints - Valid Client<OpenAIConfig>.
* `model` (String): Purpose - Stores model data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

* `call(params: Value (Any)) -> anyhow::Result<CallToolResult>`: Internal helper logic.
* `default() -> Self`: Internal helper logic.
* `description() -> String`: Internal helper logic.
* `heuristic_scan(diff: &str (Any)) -> (f64, Vec<String>)`: Internal helper logic.
* `input_schema() -> Value`: Internal helper logic.
* `name() -> String`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class SecurityReviewTool {
    -call(params: Value:Any) : anyhow::Result<CallToolResult>
    -default() : Self
    -description() : String
    -heuristic_scan(diff: &str:Any) : (f64, Vec<String>)
    -input_schema() : Value
    -name() : String
    +new() : Self
}
Default <|-- SecurityReviewTool : Inheritance / Specialization
Tool <|-- SecurityReviewTool : Inheritance / Specialization
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant Caller as "Client Interface"
participant Svc as "Security_reviewService"
Caller -> Svc : call()
note over Svc : Processing internal logic
Svc --> Caller : result
@enduml

```

## Examples

```
// Example usage of security_review.rs components
import { ... } from 'crates/factory-mcp-server/src/tools/security_review.rs';
```

## Cross References
* **Parent module:** `crates/factory-mcp-server/src/tools`
* **Dependencies:** async_openai::Client, async_openai::config::OpenAIConfig, async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionResponseFormat, ChatCompletionResponseFormatType,
    CreateChatCompletionRequestArgs,
}, async_trait::async_trait, crate::protocol::{CallToolResult, McpContent}, crate::tools::Tool, serde_json::{json, Value}, std::env, super::*
