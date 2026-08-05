---
type: "module-documentation"
title: "lib.rs"
source_path: "crates/factory-infrastructure/src/lib.rs"
description: "Detailed documentation for lib.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-05T05:55:37Z"
---

# File: lib.rs

**Source Path:** `crates/factory-infrastructure/src/lib.rs`

## Overview

### Purpose
Provides implementation for lib.rs.

### Responsibilities
* Handles logic related to lib.

### Dependencies
* pub r2r::{HttpR2rClient, R2rClient}, pub s3::AwsS3Storage, pub aethalgard::MockAethalgardClient, pub kafka::{KafkaClient, RdKafkaClient}, pub mcp_client::{McpClient, McpHttpClient, McpSseClient}, pub r2r::MockR2rClient, pub aethalgard::{AethalgardClient, HttpAethalgardClient}, pub kafka::{KafkaClient, RdKafkaClient, SimpleMockKafkaClient}, pub gitlab::MockGitlabClient, pub gitlab::{GitlabClient, GitlabIssue, HttpGitlabClient}, pub ziti::{OpenZitiIdentity, ZitiIdentity}, pub jira::MockJiraClient, pub jira::{HttpJiraClient, JiraClient}, pub mcp_client::MockMcpClient, pub sentry::MockSentryClient, pub sentry::{CrashEvent, HttpSentryClient, SentryClient}, pub ziti::MockZitiIdentity

### Imported modules
*

### Exported classes
*

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

```mermaid
classDiagram
    direction BT
    class S3Storage {
        <<trait>>
    }

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as LibService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Examples

```
// Example usage of lib.rs components
import { ... } from 'crates/factory-infrastructure/src/lib.rs';
```

## Cross References
* **Parent module:** `crates/factory-infrastructure/src`
* **Dependencies:** pub r2r::{HttpR2rClient, R2rClient}, pub s3::AwsS3Storage, pub aethalgard::MockAethalgardClient, pub kafka::{KafkaClient, RdKafkaClient}, pub mcp_client::{McpClient, McpHttpClient, McpSseClient}, pub r2r::MockR2rClient, pub aethalgard::{AethalgardClient, HttpAethalgardClient}, pub kafka::{KafkaClient, RdKafkaClient, SimpleMockKafkaClient}, pub gitlab::MockGitlabClient, pub gitlab::{GitlabClient, GitlabIssue, HttpGitlabClient}, pub ziti::{OpenZitiIdentity, ZitiIdentity}, pub jira::MockJiraClient, pub jira::{HttpJiraClient, JiraClient}, pub mcp_client::MockMcpClient, pub sentry::MockSentryClient, pub sentry::{CrashEvent, HttpSentryClient, SentryClient}, pub ziti::MockZitiIdentity
