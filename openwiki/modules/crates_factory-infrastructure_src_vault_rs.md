---
type: "module-documentation"
title: "vault.rs"
source_path: "crates/factory-infrastructure/src/vault.rs"
description: "Detailed documentation for vault.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-04T05:41:34Z"
---

# File: vault.rs

**Source Path:** `crates/factory-infrastructure/src/vault.rs`

## Overview

### Purpose
Provides implementation for vault.rs.

### Responsibilities
* Handles logic related to vault.

### Dependencies
* super::*, wiremock::{Mock, MockServer, ResponseTemplate}, wiremock::matchers::{header, method, path}, factory_core::security::{JitToken, SecurityBounds}, reqwest::Client, async_trait::async_trait, serde_json::json

## Public API & Architecture

### Exported Classes / Structs / Interfaces

#### VaultSecurityBounds

**Overview:** Represents VaultSecurityBounds.

**Public Methods:**

##### `new(vault_addr: String (Any), role_token: String (Any)) -> Self`
Executes new.

### Exported Functions

None.

## Internal Architecture & Execution Flow

```mermaid
classDiagram
    direction BT
    class VaultSecurityBounds {
        +new(vault_addr: String:Any, role_token: String:Any) Self
        -validate_token(token: &JitToken:Any) factory_core::error::Result<bool>
        -issue_jit_token(audience: &str:Any) factory_core::error::Result<JitToken>
    }
    SecurityBounds <|-- VaultSecurityBounds : Inheritance / Specialization

```

### Sequence Explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as VaultService
    Caller->>Svc: new()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```

## Cross References
* **Parent module:** `crates/factory-infrastructure/src`
* **Dependencies:** super::*, wiremock::{Mock, MockServer, ResponseTemplate}, wiremock::matchers::{header, method, path}, factory_core::security::{JitToken, SecurityBounds}, reqwest::Client, async_trait::async_trait, serde_json::json
