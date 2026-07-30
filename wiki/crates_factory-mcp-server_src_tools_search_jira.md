---
type: module
title: "search_jira.rs"
source_path: "crates/factory-mcp-server/src/tools/search_jira.rs"
description: "Documentation for crates/factory-mcp-server/src/tools/search_jira.rs"
tags: [rust, module]
last_verified_commit: "beeed91"
---

# search_jira.rs

Source File: `crates/factory-mcp-server/src/tools/search_jira.rs`

## Component Architecture

```mermaid
classDiagram
    class SearchJiraTool
    class ManualMockJiraClient
```

## Execution Flow

```mermaid
flowchart TD
    Start --> new
    new --> name
    name --> description
    description --> input_schema
    input_schema --> call
    call --> search_issues
    search_issues --> test_search_jira_tool_success
    test_search_jira_tool_success --> test_search_jira_tool_failure
    test_search_jira_tool_failure --> End
```
