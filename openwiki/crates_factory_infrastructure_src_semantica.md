---
type: "module-documentation"
title: "semantica.rs"
source_path: "crates/factory-infrastructure/src/semantica.rs"
description: "Detailed documentation for semantica.rs"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "9c1db1c"
---

# File: semantica.rs

**Source Path:** `crates/factory-infrastructure/src/semantica.rs`

## Overview

### Purpose
Provides implementation for semantica.rs.

### Responsibilities
* Handles logic related to semantica.

### Dependencies
* async_trait::async_trait, serde::{Deserialize, Serialize}, super::*

### Imported modules
* None

### Exported classes
* Conflict, DecisionRecord, HttpSemanticaClient, MissionPlan, ProvenanceReport

### Exported interfaces
* SemanticaClient

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### Conflict

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

* `conflict_id` (String): Purpose - Stores conflict_id data. Constraints - Valid String.
* `description` (String): Purpose - Stores description data. Constraints - Valid String.
* `rule_violated` (String): Purpose - Stores rule_violated data. Constraints - Valid String.
* `severity` (String): Purpose - Stores severity data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

None.

#### DecisionRecord

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

* `agent_id` (String): Purpose - Stores agent_id data. Constraints - Valid String.
* `ast_node_ids` (Vec<String>): Purpose - Stores ast_node_ids data. Constraints - Valid Vec<String>.
* `decision_id` (String): Purpose - Stores decision_id data. Constraints - Valid String.
* `mission_id` (String): Purpose - Stores mission_id data. Constraints - Valid String.
* `reasoning` (String): Purpose - Stores reasoning data. Constraints - Valid String.
* `timestamp` (String): Purpose - Stores timestamp data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

None.

#### HttpSemanticaClient

**Overview:**
No description provided.

**Constructor:**

##### `new(endpoint: String (Any), nhi_identity: Option<String> (Any))`
Parameters: endpoint: String (Any), nhi_identity: Option<String> (Any)
Dependencies: Inherited from context
Initialization: Sets up HttpSemanticaClient

**Attributes:**

* `client` (reqwest::Client): Purpose - Stores client data. Constraints - Valid reqwest::Client.
* `endpoint` (String): Purpose - Stores endpoint data. Constraints - Valid String.
* `nhi_identity` (Option<String>): Purpose - Stores nhi_identity data. Constraints - Valid Option<String>.

**Public Methods:**

None.

**Private Methods:**

* `detect_conflicts(plan: &MissionPlan (Any)) -> anyhow::Result<Vec<Conflict>>`: Internal helper logic.
* `record_decision(record: &DecisionRecord (Any)) -> anyhow::Result<()>`: Internal helper logic.
* `verify_provenance(patch_id: &str (Any)) -> anyhow::Result<ProvenanceReport>`: Internal helper logic.

#### MissionPlan

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

* `constitution_rules` (Vec<String>): Purpose - Stores constitution_rules data. Constraints - Valid Vec<String>.
* `mission_id` (String): Purpose - Stores mission_id data. Constraints - Valid String.
* `proposed_tasks` (Vec<String>): Purpose - Stores proposed_tasks data. Constraints - Valid Vec<String>.
* `spec_content` (String): Purpose - Stores spec_content data. Constraints - Valid String.
* `title` (String): Purpose - Stores title data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

None.

#### ProvenanceReport

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

* `causal_chain` (Vec<String>): Purpose - Stores causal_chain data. Constraints - Valid Vec<String>.
* `is_valid` (bool): Purpose - Stores is_valid data. Constraints - Valid bool.
* `patch_id` (String): Purpose - Stores patch_id data. Constraints - Valid String.
* `policy_violations` (Vec<String>): Purpose - Stores policy_violations data. Constraints - Valid Vec<String>.

**Public Methods:**

None.

**Private Methods:**

None.

#### SemanticaClient

**Overview:**
No description provided.

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

```plantuml
@startuml
class Conflict {
}
class DecisionRecord {
}
class HttpSemanticaClient {
    -detect_conflicts(plan: &MissionPlan:Any) : anyhow::Result<Vec<Conflict>>
    +new(endpoint: String:Any, nhi_identity: Option<String>:Any) : Self
    -record_decision(record: &DecisionRecord:Any) : anyhow::Result<()>
    -verify_provenance(patch_id: &str:Any) : anyhow::Result<ProvenanceReport>
}
SemanticaClient <|-- HttpSemanticaClient : extends/implements
class MissionPlan {
}
class ProvenanceReport {
}
interface SemanticaClient {
}
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "SemanticaService" as Svc
Caller -> Svc: execute()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```


## Examples

```
// Example usage of semantica.rs components
import { ... } from 'crates/factory-infrastructure/src/semantica.rs';
```


## Cross References
* **Parent module:** `crates/factory-infrastructure/src`
* **Dependencies:** async_trait::async_trait, serde::{Deserialize, Serialize}, super::*
