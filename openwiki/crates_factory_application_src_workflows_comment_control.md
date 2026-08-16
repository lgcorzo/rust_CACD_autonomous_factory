---
type: "module-documentation"
title: "comment_control.rs"
source_path: "crates/factory-application/src/workflows/comment_control.rs"
description: "Detailed documentation for comment_control.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "e48839f"
---

# File: comment_control.rs

**Source Path:** `crates/factory-application/src/workflows/comment_control.rs`

## Overview

### Purpose
Provides implementation for comment_control.rs.

### Responsibilities
* Handles logic related to comment_control.

### Dependencies
* chrono::Utc, crate::agents::{RustantAgent, ZeroClawAgent}, factory_core::{PRCommentEvent, PRDirective}, factory_infrastructure::McpClient, factory_infrastructure::aethalgard::AethalgardClient, factory_infrastructure::aethalgard::MockAethalgardClient, factory_infrastructure::github::GithubClient, factory_infrastructure::github::{GithubComment, GithubUser, MockGithubClient}, factory_infrastructure::gitlab::GitlabClient, factory_infrastructure::mcp_client::MockMcpClient, factory_infrastructure::r2r::MockR2rClient, factory_infrastructure::r2r::R2rClient, serde::{Deserialize, Serialize}, std::sync::Arc, super::*

### Imported modules
* None

### Exported classes
* CommentControlInput, CommentControlOutput, CommentControlService

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### CommentControlInput

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

* `event` (PRCommentEvent): Purpose - Stores event data. Constraints - Valid PRCommentEvent.

**Public Methods:**

None.

**Private Methods:**

None.

#### CommentControlOutput

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

* `comment_posted` (bool): Purpose - Stores comment_posted data. Constraints - Valid bool.
* `directive_type` (String): Purpose - Stores directive_type data. Constraints - Valid String.
* `response_body` (String): Purpose - Stores response_body data. Constraints - Valid String.
* `status` (String): Purpose - Stores status data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

None.

#### CommentControlService

**Overview:**
No description provided.

**Constructor:**

##### `new(github_client: Option<Arc<dyn GithubClient>> (Any), gitlab_client: Option<Arc<dyn GitlabClient>> (Any), mcp_client: Arc<dyn McpClient> (Any), r2r_client: Arc<dyn R2rClient> (Any), aethalgard_client: Arc<dyn AethalgardClient> (Any))`
Parameters: github_client: Option<Arc<dyn GithubClient>> (Any), gitlab_client: Option<Arc<dyn GitlabClient>> (Any), mcp_client: Arc<dyn McpClient> (Any), r2r_client: Arc<dyn R2rClient> (Any), aethalgard_client: Arc<dyn AethalgardClient> (Any)
Dependencies: Inherited from context
Initialization: Sets up CommentControlService

**Attributes:**

* `_mcp_client` (Arc<dyn McpClient>): Purpose - Stores _mcp_client data. Constraints - Valid Arc<dyn McpClient>.
* `github_client` (Option<Arc<dyn GithubClient>>): Purpose - Stores github_client data. Constraints - Valid Option<Arc<dyn GithubClient>>.
* `gitlab_client` (Option<Arc<dyn GitlabClient>>): Purpose - Stores gitlab_client data. Constraints - Valid Option<Arc<dyn GitlabClient>>.
* `rustant_agent` (Arc<RustantAgent>): Purpose - Stores rustant_agent data. Constraints - Valid Arc<RustantAgent>.
* `zeroclaw_agent` (Arc<ZeroClawAgent>): Purpose - Stores zeroclaw_agent data. Constraints - Valid Arc<ZeroClawAgent>.

**Public Methods:**

##### `handle_directive(input: &CommentControlInput (Any)) -> anyhow::Result<CommentControlOutput>`

###### Description
No description provided.

###### Inputs
* `input: &CommentControlInput`: type=Any, meaning=Input for input: &CommentControlInput, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: anyhow::Result<CommentControlOutput>
Semantic meaning: Result of handle_directive
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
let result = instance.handle_directive();
```

**Private Methods:**

None.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
class CommentControlInput {
}
class CommentControlOutput {
}
class CommentControlService {
    +handle_directive(input: &CommentControlInput:Any) : anyhow::Result<CommentControlOutput>
    +new(github_client: Option<Arc<dyn GithubClient>>:Any, gitlab_client: Option<Arc<dyn GitlabClient>>:Any, mcp_client: Arc<dyn McpClient>:Any, r2r_client: Arc<dyn R2rClient>:Any, aethalgard_client: Arc<dyn AethalgardClient>:Any) : Self
}
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "Comment_controlService" as Svc
Caller -> Svc: execute()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

## Examples

```
// Example usage of comment_control.rs components
import { ... } from 'crates/factory-application/src/workflows/comment_control.rs';
```

## Cross References
* **Parent module:** `crates/factory-application/src/workflows`
* **Dependencies:** chrono::Utc, crate::agents::{RustantAgent, ZeroClawAgent}, factory_core::{PRCommentEvent, PRDirective}, factory_infrastructure::McpClient, factory_infrastructure::aethalgard::AethalgardClient, factory_infrastructure::aethalgard::MockAethalgardClient, factory_infrastructure::github::GithubClient, factory_infrastructure::github::{GithubComment, GithubUser, MockGithubClient}, factory_infrastructure::gitlab::GitlabClient, factory_infrastructure::mcp_client::MockMcpClient, factory_infrastructure::r2r::MockR2rClient, factory_infrastructure::r2r::R2rClient, serde::{Deserialize, Serialize}, std::sync::Arc, super::*
