---
type: module
title: "github_webhook.rs"
source_path: "crates/factory-mcp-server/src/github_webhook.rs"
description: "Documentation for crates/factory-mcp-server/src/github_webhook.rs"
tags: [rust, module]
last_verified_commit: "b4fd4b79"
---

# github_webhook.rs

Source File: `crates/factory-mcp-server/src/github_webhook.rs`

## Component Architecture

```mermaid
classDiagram
    class MissionDispatchReceipt
    class WebhookMissionDispatcher {
        <<trait>>
    }
    class DefaultLocalMissionDispatcher
    class GithubWebhookUser
    class GithubWebhookIssue
    class GithubWebhookRepository
    class GithubWebhookPayload
```

## Execution Flow

```mermaid
flowchart TD
    Start --> End
```
