---
type: "module-documentation"
title: "lib.rs"
source_path: "crates/factory-infrastructure/src/lib.rs"
description: "Detailed documentation for lib.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "7982a81"
---

# File: lib.rs

**Source Path:** `crates/factory-infrastructure/src/lib.rs`

## Overview

### Purpose
Provides implementation for lib.rs.

### Responsibilities
* Handles logic related to lib.

### Dependencies
* pub aethalgard::MockAethalgardClient, pub aethalgard::{AethalgardClient, HttpAethalgardClient}, pub github::MockGithubClient, pub github::{GithubClient, GithubIssue, HttpGithubClient}, pub gitlab::MockGitlabClient, pub gitlab::{GitlabClient, GitlabIssue, HttpGitlabClient}, pub jira::MockJiraClient, pub jira::{HttpJiraClient, JiraClient}, pub kafka::{KafkaClient, RdKafkaClient, SimpleMockKafkaClient}, pub kafka::{KafkaClient, RdKafkaClient}, pub mcp_client::MockMcpClient, pub mcp_client::{McpClient, McpHttpClient, McpSseClient}, pub r2r::MockR2rClient, pub r2r::{HttpR2rClient, R2rClient}, pub s3::AwsS3Storage, pub semantica::MockSemanticaClient, pub semantica::{
    Conflict, DecisionRecord, HttpSemanticaClient, MissionPlan, ProvenanceReport, SemanticaClient,
}, pub sentry::MockSentryClient, pub sentry::{CrashEvent, HttpSentryClient, SentryClient}, pub ziti::MockZitiIdentity, pub ziti::{OpenZitiIdentity, ZitiIdentity}

### Imported modules
* None

### Exported classes
* None

### Exported interfaces
* S3Storage

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### S3Storage

**Overview:**
Why it exists:
Provides capabilities related to S3Storage.

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

```plantuml
@startuml
interface S3Storage {
}
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant Caller as "Client Interface"
participant Svc as "LibService"
Caller -> Svc : execute()
note over Svc : Processing internal logic
Svc --> Caller : result
@enduml

```

## Examples

```
// Example usage of lib.rs components
import { ... } from 'crates/factory-infrastructure/src/lib.rs';
```

## Cross References
* **Parent module:** `crates/factory-infrastructure/src`
* **Dependencies:** pub aethalgard::MockAethalgardClient, pub aethalgard::{AethalgardClient, HttpAethalgardClient}, pub github::MockGithubClient, pub github::{GithubClient, GithubIssue, HttpGithubClient}, pub gitlab::MockGitlabClient, pub gitlab::{GitlabClient, GitlabIssue, HttpGitlabClient}, pub jira::MockJiraClient, pub jira::{HttpJiraClient, JiraClient}, pub kafka::{KafkaClient, RdKafkaClient, SimpleMockKafkaClient}, pub kafka::{KafkaClient, RdKafkaClient}, pub mcp_client::MockMcpClient, pub mcp_client::{McpClient, McpHttpClient, McpSseClient}, pub r2r::MockR2rClient, pub r2r::{HttpR2rClient, R2rClient}, pub s3::AwsS3Storage, pub semantica::MockSemanticaClient, pub semantica::{
    Conflict, DecisionRecord, HttpSemanticaClient, MissionPlan, ProvenanceReport, SemanticaClient,
}, pub sentry::MockSentryClient, pub sentry::{CrashEvent, HttpSentryClient, SentryClient}, pub ziti::MockZitiIdentity, pub ziti::{OpenZitiIdentity, ZitiIdentity}
