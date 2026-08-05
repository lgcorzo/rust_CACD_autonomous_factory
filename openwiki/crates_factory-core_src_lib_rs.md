---
type: "module-documentation"
title: "lib.rs"
source_path: "crates/factory-core/src/lib.rs"
description: "Detailed documentation for lib.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-05T05:55:37Z"
---

# File: lib.rs

**Source Path:** `crates/factory-core/src/lib.rs`

## Overview

### Purpose
Provides implementation for lib.rs.

### Responsibilities
* Handles logic related to lib.

### Dependencies
* serde::{Deserialize, Serialize}, chrono::{DateTime, Utc}, uuid::Uuid, std::collections::HashMap

### Imported modules
*

### Exported classes
* Metadata, Inputs, Outputs, Targets, Mission, Task, SHAPValues, FeatureImportances, SpecArtifact, OsrMetric, FinOpsTag, ComplianceReport, UserFeedbackPayload

### Exported interfaces
*

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### Metadata

**Overview:**
Why it exists:
Provides capabilities related to Metadata.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `timestamp` (DateTime<Utc>): Purpose - Stores timestamp data. Constraints - Valid DateTime<Utc>.
* `model_version` (String): Purpose - Stores model_version data. Constraints - Valid String.
* `extra` (HashMap<String, serde_json::Value>): Purpose - Stores extra data. Constraints - Valid HashMap<String, serde_json::Value>.

**Public Methods:**

None.

**Private Methods:**

None.

#### Inputs

**Overview:**
Why it exists:
Provides capabilities related to Inputs.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `input` (String): Purpose - Stores input data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

None.

#### Outputs

**Overview:**
Why it exists:
Provides capabilities related to Outputs.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `response` (String): Purpose - Stores response data. Constraints - Valid String.
* `metadata` (Metadata): Purpose - Stores metadata data. Constraints - Valid Metadata.

**Public Methods:**

None.

**Private Methods:**

None.

#### Targets

**Overview:**
Why it exists:
Provides capabilities related to Targets.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `input_target` (String): Purpose - Stores input_target data. Constraints - Valid String.
* `response` (String): Purpose - Stores response data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

None.

#### Mission

**Overview:**
Why it exists:
Provides capabilities related to Mission.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `id` (Uuid): Purpose - Stores id data. Constraints - Valid Uuid.
* `name` (String): Purpose - Stores name data. Constraints - Valid String.
* `description` (String): Purpose - Stores description data. Constraints - Valid String.
* `created_at` (DateTime<Utc>): Purpose - Stores created_at data. Constraints - Valid DateTime<Utc>.
* `tasks` (Vec<Task>): Purpose - Stores tasks data. Constraints - Valid Vec<Task>.
* `status` (MissionStatus): Purpose - Stores status data. Constraints - Valid MissionStatus.

**Public Methods:**

None.

**Private Methods:**

None.

#### MissionStatus

**Overview:**
Why it exists:
Provides capabilities related to MissionStatus.

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

#### Task

**Overview:**
Why it exists:
Provides capabilities related to Task.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `id` (Uuid): Purpose - Stores id data. Constraints - Valid Uuid.
* `mission_id` (Uuid): Purpose - Stores mission_id data. Constraints - Valid Uuid.
* `description` (String): Purpose - Stores description data. Constraints - Valid String.
* `assigned_agent` (Option<String>): Purpose - Stores assigned_agent data. Constraints - Valid Option<String>.
* `dependencies` (Vec<Uuid>): Purpose - Stores dependencies data. Constraints - Valid Vec<Uuid>.
* `status` (TaskStatus): Purpose - Stores status data. Constraints - Valid TaskStatus.

**Public Methods:**

None.

**Private Methods:**

None.

#### TaskStatus

**Overview:**
Why it exists:
Provides capabilities related to TaskStatus.

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

#### SHAPValues

**Overview:**
Why it exists:
Provides capabilities related to SHAPValues.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `sample` (String): Purpose - Stores sample data. Constraints - Valid String.
* `explanation` (String): Purpose - Stores explanation data. Constraints - Valid String.
* `shap_value` (f32): Purpose - Stores shap_value data. Constraints - Valid f32.

**Public Methods:**

None.

**Private Methods:**

None.

#### FeatureImportances

**Overview:**
Why it exists:
Provides capabilities related to FeatureImportances.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `feature` (String): Purpose - Stores feature data. Constraints - Valid String.
* `importance` (f32): Purpose - Stores importance data. Constraints - Valid f32.

**Public Methods:**

None.

**Private Methods:**

None.

#### SpecArtifact

**Overview:**
Why it exists:
Provides capabilities related to SpecArtifact.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `id` (Uuid): Purpose - Stores id data. Constraints - Valid Uuid.
* `name` (String): Purpose - Stores name data. Constraints - Valid String.
* `data` (serde_json::Value): Purpose - Stores data data. Constraints - Valid serde_json::Value.

**Public Methods:**

None.

**Private Methods:**

None.

#### OsrMetric

**Overview:**
Why it exists:
Provides capabilities related to OsrMetric.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `mission_id` (String): Purpose - Stores mission_id data. Constraints - Valid String.
* `osr_value` (f32): Purpose - Stores osr_value data. Constraints - Valid f32.
* `wiki_commit_sha` (String): Purpose - Stores wiki_commit_sha data. Constraints - Valid String.
* `timestamp` (DateTime<Utc>): Purpose - Stores timestamp data. Constraints - Valid DateTime<Utc>.

**Public Methods:**

None.

**Private Methods:**

None.

#### FinOpsTag

**Overview:**
Why it exists:
Provides capabilities related to FinOpsTag.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `team` (String): Purpose - Stores team data. Constraints - Valid String.
* `epic` (String): Purpose - Stores epic data. Constraints - Valid String.
* `microservice` (String): Purpose - Stores microservice data. Constraints - Valid String.
* `environment` (String): Purpose - Stores environment data. Constraints - Valid String.
* `cost_center` (String): Purpose - Stores cost_center data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

None.

#### ComplianceReport

**Overview:**
Why it exists:
Provides capabilities related to ComplianceReport.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `report_id` (Uuid): Purpose - Stores report_id data. Constraints - Valid Uuid.
* `status` (String): Purpose - Stores status data. Constraints - Valid String.
* `findings` (Vec<String>): Purpose - Stores findings data. Constraints - Valid Vec<String>.

**Public Methods:**

None.

**Private Methods:**

None.

#### UserFeedbackPayload

**Overview:**
Why it exists:
Provides capabilities related to UserFeedbackPayload.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `user_id` (String): Purpose - Stores user_id data. Constraints - Valid String.
* `session_id` (Option<String>): Purpose - Stores session_id data. Constraints - Valid Option<String>.
* `feedback_text` (String): Purpose - Stores feedback_text data. Constraints - Valid String.
* `sentiment` (String): Purpose - Stores sentiment data. Constraints - Valid String.
* `metadata` (Option<serde_json::Value>): Purpose - Stores metadata data. Constraints - Valid Option<serde_json::Value>.

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
    class Metadata {
    }
    class Inputs {
    }
    class Outputs {
    }
    class Targets {
    }
    class Mission {
    }
    class MissionStatus {
        <<enumeration>>
    }
    class Task {
    }
    class TaskStatus {
        <<enumeration>>
    }
    class SHAPValues {
    }
    class FeatureImportances {
    }
    class SpecArtifact {
    }
    class OsrMetric {
    }
    class FinOpsTag {
    }
    class ComplianceReport {
    }
    class UserFeedbackPayload {
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
import { ... } from 'crates/factory-core/src/lib.rs';
```

## Cross References
* **Parent module:** `crates/factory-core/src`
* **Dependencies:** serde::{Deserialize, Serialize}, chrono::{DateTime, Utc}, uuid::Uuid, std::collections::HashMap
