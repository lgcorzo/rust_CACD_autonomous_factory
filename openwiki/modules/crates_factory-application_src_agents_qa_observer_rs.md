---
type: "module-documentation"
title: "qa_observer.rs"
source_path: "crates/factory-application/src/agents/qa_observer.rs"
description: "Detailed documentation for qa_observer.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: qa_observer.rs

**Source Path:** `crates/factory-application/src/agents/qa_observer.rs`

## Overview

### Purpose
Provides implementation for qa_observer.rs.

### Responsibilities
* Handles logic related to qa_observer.

### Dependencies
* uuid::Uuid, async_trait::async_trait, serde_json::Value, crate::Agent, factory_infrastructure::{GitlabClient, HttpGitlabClient, HttpSentryClient, SentryClient}, std::time::Duration, crate::workflows::autonomous_mission::MissionInput, hatchet_sdk::{Hatchet, Runnable}

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### QAObserverAgent

**Overview:** Represents QAObserverAgent.

**Public Methods:**

##### `new(sentry_url: String (Any), sentry_token: String (Any), sentry_project: String (Any), gitlab_url: String (Any), gitlab_token: String (Any), gitlab_project: String (Any), hatchet: Hatchet (Any)) -> Self`
Executes new.

##### `monitor_crashes() -> anyhow::Result<()>`
Executes monitor_crashes.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class QAObserverAgent {
        -default() Self
        +new(sentry_url: String:Any, sentry_token: String:Any, sentry_project: String:Any, gitlab_url: String:Any, gitlab_token: String:Any, gitlab_project: String:Any, hatchet: Hatchet:Any) Self
        +monitor_crashes() anyhow::Result<()>
        -name() String
        -execute(_task_description: &str:Any) anyhow::Result<Value>
    }
    Default <|-- QAObserverAgent : Inheritance / Specialization
    Agent <|-- QAObserverAgent : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as Qa_observerService
    Caller->>Svc: default()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-application/src/agents`
* **Dependencies:** uuid::Uuid, async_trait::async_trait, serde_json::Value, crate::Agent, factory_infrastructure::{GitlabClient, HttpGitlabClient, HttpSentryClient, SentryClient}, std::time::Duration, crate::workflows::autonomous_mission::MissionInput, hatchet_sdk::{Hatchet, Runnable}
