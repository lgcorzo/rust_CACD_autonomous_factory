---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "Module: factory-core::error"
source_path: "crates/factory-core/src/error.rs"
description: "Centralized FactoryError enumeration and Result type alias."
tags: ["factory_core", "error", "thiserror"]
last_verified_commit: "main"
timestamp: "2026-07-31T16:35:00Z"
---

# Module Specification: `factory-core::error`

* **Source File Reference:** `crates/factory-core/src/error.rs` (Lines: L1-L45)
* **Upstream Dependencies:** Standard library `thiserror` crate
* **Downstream Consumers:** All workspace crates

---

## 1. Architectural Role & Responsibilities

Encapsulates error handling across all factory components using the `FactoryError` enum and standard `Result<T, FactoryError>` alias.

---

## 2. Error Enumeration Diagram

```mermaid
classDiagram
    class FactoryError {
        <<enumeration>>
        Security(String)
        Executor(String)
        SyntaxError(String)
        Io(std::io::Error)
        Internal(String)
    }
```
