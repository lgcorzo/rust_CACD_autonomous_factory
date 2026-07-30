---
type: module
title: "lib.rs"
source_path: "crates/factory-core/src/lib.rs"
description: "Documentation for crates/factory-core/src/lib.rs"
tags: [rust, module]
last_verified_commit: "beeed91"
---

# lib.rs

Source File: `crates/factory-core/src/lib.rs`

## Component Architecture

```mermaid
classDiagram
    class Metadata
    class Inputs
    class Outputs
    class Targets
    class Mission
    class MissionStatus {
        <<enumeration>>
    }
    class Task
    class TaskStatus {
        <<enumeration>>
    }
    class SHAPValues
    class FeatureImportances
    class SpecArtifact
    class OsrMetric
    class FinOpsTag
    class ComplianceReport
    class UserFeedbackPayload
```

## Execution Flow

```mermaid
flowchart TD
    Start --> End
```
