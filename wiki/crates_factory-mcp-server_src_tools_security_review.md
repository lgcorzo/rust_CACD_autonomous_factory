---
type: module
title: "security_review.rs"
source_path: "crates/factory-mcp-server/src/tools/security_review.rs"
description: "Documentation for crates/factory-mcp-server/src/tools/security_review.rs"
tags: [rust, module]
last_verified_commit: "beeed91"
---

# security_review.rs

Source File: `crates/factory-mcp-server/src/tools/security_review.rs`

## Component Architecture

```mermaid
classDiagram
    class SecurityReviewTool
```

## Execution Flow

```mermaid
flowchart TD
    Start --> new
    new --> default
    default --> name
    name --> description
    description --> input_schema
    input_schema --> call
    call --> End
```
