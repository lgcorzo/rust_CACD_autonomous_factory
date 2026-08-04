---
type: "module-documentation"
title: "jira.rs"
source_path: "crates/factory-infrastructure/src/jira.rs"
description: "Detailed documentation for jira.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: jira.rs

**Source Path:** `crates/factory-infrastructure/src/jira.rs`

## Overview

### Purpose
Provides implementation for jira.rs.

### Responsibilities
* Handles logic related to jira.

### Dependencies
* serde_json::json, super::*, wiremock::matchers::{method, path, query_param}, wiremock::{Mock, MockServer, ResponseTemplate}, async_trait::async_trait

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### JiraClient

**Overview:** Represents JiraClient.

**Public Methods:**

None.

#### HttpJiraClient

**Overview:** Represents HttpJiraClient.

**Public Methods:**

##### `new(url: String (Any), username: String (Any), api_token: String (Any)) -> Self`
Executes new.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class JiraClient {
        <<trait>>
    }
    class HttpJiraClient {
        +new(url: String:Any, username: String:Any, api_token: String:Any) Self
        -search_issues(query: &str:Any) anyhow::Result<String>
    }
    JiraClient <|-- HttpJiraClient : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as JiraService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-infrastructure/src`
* **Dependencies:** serde_json::json, super::*, wiremock::matchers::{method, path, query_param}, wiremock::{Mock, MockServer, ResponseTemplate}, async_trait::async_trait
