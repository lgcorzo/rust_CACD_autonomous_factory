---
iso_doc_type: "Description"
iso_viewpoint: "ComponentView"
type: "module"
title: "cursor_store.rs"
source_path: "crates/factory-infrastructure/src/cursor_store.rs"
description: "Detailed documentation for cursor_store.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-25T05:53:44Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "e2707de"
---

# File: cursor_store.rs

**Source Path:** `crates/factory-infrastructure/src/cursor_store.rs`

## Overview

### Purpose
Provides implementation for cursor_store.rs.

### Responsibilities
* Handles logic related to cursor_store.

### Dependencies
* async_trait::async_trait, chrono::Utc, factory_core::PollerSyncCursor, std::collections::{HashMap, HashSet}, std::sync::Arc, super::*, tokio::sync::RwLock

### Imported modules
* None

### Exported classes
* InMemoryCursorStore, PostgresCursorStore

### Exported interfaces
* CursorStore

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### CursorStore

**Overview:**
No description provided.

**Constructor:**

Default constructor.

**Attributes:**

None.

**Public Methods:**

##### `get_cursor(source_key (&str)) -> anyhow::Result<Option<PollerSyncCursor>>`

###### Description
No description provided.

###### Inputs
* `source_key`: type=&str, meaning=Input for source_key, valid values=Any valid &str, optional=No, default value=None

###### Output
Return type: anyhow::Result<Option<PollerSyncCursor>>
Semantic meaning: Result of get_cursor
Possible null values: Conditional
Exceptions: None handled explicitly

###### Side Effects
Database updates: None
File operations: None
Network calls: None
Cache: None
State changes: Updates internal variables

###### Complexity
Time Complexity: O(1) mostly
Space Complexity: O(1) mostly

###### Example
```
let result = instance.get_cursor();
```

##### `is_event_processed(source_key (&str), event_hash (&str)) -> anyhow::Result<bool>`

###### Description
No description provided.

###### Inputs
* `source_key`: type=&str, meaning=Input for source_key, valid values=Any valid &str, optional=No, default value=None
* `event_hash`: type=&str, meaning=Input for event_hash, valid values=Any valid &str, optional=No, default value=None

###### Output
Return type: anyhow::Result<bool>
Semantic meaning: Result of is_event_processed
Possible null values: Conditional
Exceptions: None handled explicitly

###### Side Effects
Database updates: None
File operations: None
Network calls: None
Cache: None
State changes: Updates internal variables

###### Complexity
Time Complexity: O(1) mostly
Space Complexity: O(1) mostly

###### Example
```
let result = instance.is_event_processed();
```

##### `mark_event_processed(source_key (&str), event_hash (&str)) -> anyhow::Result<()>`

###### Description
No description provided.

###### Inputs
* `source_key`: type=&str, meaning=Input for source_key, valid values=Any valid &str, optional=No, default value=None
* `event_hash`: type=&str, meaning=Input for event_hash, valid values=Any valid &str, optional=No, default value=None

###### Output
Return type: anyhow::Result<()>
Semantic meaning: Result of mark_event_processed
Possible null values: Conditional
Exceptions: None handled explicitly

###### Side Effects
Database updates: None
File operations: None
Network calls: None
Cache: None
State changes: Updates internal variables

###### Complexity
Time Complexity: O(1) mostly
Space Complexity: O(1) mostly

###### Example
```
let result = instance.mark_event_processed();
```

##### `save_cursor(cursor (&PollerSyncCursor)) -> anyhow::Result<()>`

###### Description
No description provided.

###### Inputs
* `cursor`: type=&PollerSyncCursor, meaning=Input for cursor, valid values=Any valid &PollerSyncCursor, optional=No, default value=None

###### Output
Return type: anyhow::Result<()>
Semantic meaning: Result of save_cursor
Possible null values: Conditional
Exceptions: None handled explicitly

###### Side Effects
Database updates: None
File operations: None
Network calls: None
Cache: None
State changes: Updates internal variables

###### Complexity
Time Complexity: O(1) mostly
Space Complexity: O(1) mostly

###### Example
```
let result = instance.save_cursor();
```

**Private Methods:**

None.

#### InMemoryCursorStore

**Overview:**
No description provided.

**Constructor:**

##### `new()`
Parameters:
Dependencies: Inherited from context
Initialization: Sets up InMemoryCursorStore

**Attributes:**

* `cursors` (Arc<RwLock<HashMap<String, PollerSyncCursor>>>): Purpose - Stores cursors data. Constraints - Valid Arc<RwLock<HashMap<String, PollerSyncCursor>>>.
* `processed_events` (Arc<RwLock<HashMap<String, HashSet<String>>>>): Purpose - Stores processed_events data. Constraints - Valid Arc<RwLock<HashMap<String, HashSet<String>>>>.

**Public Methods:**

None.

**Private Methods:**

* `get_cursor(source_key (&str)) -> anyhow::Result<Option<PollerSyncCursor>>`: Internal helper logic.
* `is_event_processed(source_key (&str), event_hash (&str)) -> anyhow::Result<bool>`: Internal helper logic.
* `mark_event_processed(source_key (&str), event_hash (&str)) -> anyhow::Result<()>`: Internal helper logic.
* `save_cursor(cursor (&PollerSyncCursor)) -> anyhow::Result<()>`: Internal helper logic.

#### PostgresCursorStore

**Overview:**
No description provided.

**Constructor:**

##### `new(database_url (String))`
Parameters: database_url (String)
Dependencies: Inherited from context
Initialization: Sets up PostgresCursorStore

**Attributes:**

* `database_url` (String): Purpose - Stores database_url data. Constraints - Valid String.
* `fallback_store` (InMemoryCursorStore): Purpose - Stores fallback_store data. Constraints - Valid InMemoryCursorStore.

**Public Methods:**

None.

**Private Methods:**

* `get_cursor(source_key (&str)) -> anyhow::Result<Option<PollerSyncCursor>>`: Internal helper logic.
* `is_event_processed(source_key (&str), event_hash (&str)) -> anyhow::Result<bool>`: Internal helper logic.
* `mark_event_processed(source_key (&str), event_hash (&str)) -> anyhow::Result<()>`: Internal helper logic.
* `save_cursor(cursor (&PollerSyncCursor)) -> anyhow::Result<()>`: Internal helper logic.

### Exported Functions

None.

## Internal architecture

```plantuml
@startuml
interface CursorStore {
    +get_cursor(source_key: &str) anyhow::Result<Option<PollerSyncCursor>>
    +is_event_processed(source_key: &str, event_hash: &str) anyhow::Result<bool>
    +mark_event_processed(source_key: &str, event_hash: &str) anyhow::Result<()>
    +save_cursor(cursor: &PollerSyncCursor) anyhow::Result<()>
}
class InMemoryCursorStore {
    -get_cursor(source_key: &str) anyhow::Result<Option<PollerSyncCursor>>
    -is_event_processed(source_key: &str, event_hash: &str) anyhow::Result<bool>
    -mark_event_processed(source_key: &str, event_hash: &str) anyhow::Result<()>
    +new() Self
    -save_cursor(cursor: &PollerSyncCursor) anyhow::Result<()>
}
CursorStore <|-- InMemoryCursorStore : extends/implements
class PostgresCursorStore {
    -get_cursor(source_key: &str) anyhow::Result<Option<PollerSyncCursor>>
    -is_event_processed(source_key: &str, event_hash: &str) anyhow::Result<bool>
    -mark_event_processed(source_key: &str, event_hash: &str) anyhow::Result<()>
    +new(database_url: String) Self
    -save_cursor(cursor: &PollerSyncCursor) anyhow::Result<()>
}
CursorStore <|-- PostgresCursorStore : extends/implements
@enduml

```

## Package Diagram

```plantuml
@startuml
package "crates" {
    package "factory-infrastructure" {
        package "src" {
            class Module
        }
    }
}
@enduml

```

## Component Diagram

```plantuml
@startuml
component "cursor_store" as Main
component "async_trait::async_trait" as async_trait__async_trait
Main --> async_trait__async_trait : uses
component "chrono::Utc" as chrono__Utc
Main --> chrono__Utc : uses
component "factory_core::PollerSyncCursor" as factory_core__PollerSyncCursor
Main --> factory_core__PollerSyncCursor : uses
component "std::collections::{HashMap, HashSet}" as std__collections___HashMap__HashSet_
Main --> std__collections___HashMap__HashSet_ : uses
component "std::sync::Arc" as std__sync__Arc
Main --> std__sync__Arc : uses
component "super::*" as super___
Main --> super___ : uses
component "tokio::sync::RwLock" as tokio__sync__RwLock
Main --> tokio__sync__RwLock : uses
@enduml

```

## Dependency Graph

```plantuml
@startuml
[cursor_store]
[cursor_store] --> [async_trait::async_trait]
[cursor_store] --> [chrono::Utc]
[cursor_store] --> [factory_core::PollerSyncCursor]
[cursor_store] --> [std::collections::{HashMap, HashSet}]
[cursor_store] --> [std::sync::Arc]
[cursor_store] --> [super::*]
[cursor_store] --> [tokio::sync::RwLock]
@enduml

```

## Call Graph

```plantuml
@startuml
Caller --> CursorStore::get_cursor
Caller --> CursorStore::is_event_processed
Caller --> CursorStore::mark_event_processed
Caller --> CursorStore::save_cursor
Caller --> InMemoryCursorStore::new
Caller --> PostgresCursorStore::new
@enduml

```

## Execution flow & Sequence explanation

```plantuml
@startuml
autonumber
participant "Client Interface" as Caller
participant "Cursor_storeService" as Svc
Caller -> Svc: get_cursor()
note right of Svc: Processing internal logic
Svc --> Caller: result
@enduml

```

## Examples

```
// Example usage of cursor_store.rs components
import { ... } from 'crates/factory-infrastructure/src/cursor_store.rs';
```

## Cross References
* **Parent module:** `crates/factory-infrastructure/src`
* **Dependencies:** async_trait::async_trait, chrono::Utc, factory_core::PollerSyncCursor, std::collections::{HashMap, HashSet}, std::sync::Arc, super::*, tokio::sync::RwLock
