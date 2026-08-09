---
type: "module-documentation"
title: "nhi.rs"
source_path: "crates/factory-core/src/security/nhi.rs"
description: "Detailed documentation for nhi.rs"
tags: ["documentation", "ast", "openwiki"]
timestamp: "2026-08-09T06:11:32Z"
---

# File: nhi.rs

**Source Path:** `crates/factory-core/src/security/nhi.rs`

## Overview

### Purpose
Provides implementation for nhi.rs.

### Responsibilities
* Handles logic related to nhi.

### Dependencies
* base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _}, chrono::{DateTime, Utc}, ed25519_dalek::Signer, ed25519_dalek::SigningKey, rand::rngs::OsRng, serde::{Deserialize, Serialize}, super::*

### Imported modules
* None

### Exported classes
* AgentSubject, CryptographicProof, VerifiableCredential

### Exported interfaces
* None

### Exported functions
* None

## Public API

### Exported Classes / Structs / Interfaces

#### AgentSubject

**Overview:**
Why it exists:
Provides capabilities related to AgentSubject.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `allowed_namespaces` (Vec<String>): Purpose - Stores allowed_namespaces data. Constraints - Valid Vec<String>.
* `id` (String): Purpose - Stores id data. Constraints - Valid String.
* `roles` (Vec<String>): Purpose - Stores roles data. Constraints - Valid Vec<String>.

**Public Methods:**

None.

**Private Methods:**

None.

#### CryptographicProof

**Overview:**
Why it exists:
Provides capabilities related to CryptographicProof.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

Default constructor.

**Attributes:**

* `created` (DateTime<Utc>): Purpose - Stores created data. Constraints - Valid DateTime<Utc>.
* `jws` (String): Purpose - Stores jws data. Constraints - Valid String.
* `proof_purpose` (String): Purpose - Stores proof_purpose data. Constraints - Valid String.
* `proof_type` (String): Purpose - Stores proof_type data. Constraints - Valid String.
* `verification_method` (String): Purpose - Stores verification_method data. Constraints - Valid String.

**Public Methods:**

None.

**Private Methods:**

None.

#### VerifiableCredential

**Overview:**
Why it exists:
Provides capabilities related to VerifiableCredential.

What business capability it provides:
Supports core domain concepts.

How it collaborates with other classes:
Works with related entities to process logic.

**Constructor:**

##### `new(id: String (Any), issuer: String (Any), credential_subject: AgentSubject (Any))`
Parameters: id: String (Any), issuer: String (Any), credential_subject: AgentSubject (Any)
Dependencies: Inherited from context
Initialization: Sets up VerifiableCredential

**Attributes:**

* `context` (Vec<String>): Purpose - Stores context data. Constraints - Valid Vec<String>.
* `credential_subject` (AgentSubject): Purpose - Stores credential_subject data. Constraints - Valid AgentSubject.
* `credential_type` (Vec<String>): Purpose - Stores credential_type data. Constraints - Valid Vec<String>.
* `id` (String): Purpose - Stores id data. Constraints - Valid String.
* `issuance_date` (DateTime<Utc>): Purpose - Stores issuance_date data. Constraints - Valid DateTime<Utc>.
* `issuer` (String): Purpose - Stores issuer data. Constraints - Valid String.
* `proof` (Option<CryptographicProof>): Purpose - Stores proof data. Constraints - Valid Option<CryptographicProof>.

**Public Methods:**

##### `sign(signing_key: &ed25519_dalek::SigningKey (Any), key_id: &str (Any)) -> crate::error::Result<()>`

###### Description
/// Generates a JWS for the credential and attaches it to the `proof` field.

###### Inputs
* `signing_key: &ed25519_dalek::SigningKey`: type=Any, meaning=Input for signing_key: &ed25519_dalek::SigningKey, valid values=Any valid Any, optional=No, default value=None
* `key_id: &str`: type=Any, meaning=Input for key_id: &str, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: crate::error::Result<()>
Semantic meaning: Result of sign
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
let result = instance.sign();
```

##### `sign_async(signing_key: ed25519_dalek::SigningKey (Any), key_id: String (Any)) -> crate::error::Result<()>`

###### Description
/// Asynchronously signs credentials without blocking the Tokio task executor.

###### Inputs
* `signing_key: ed25519_dalek::SigningKey`: type=Any, meaning=Input for signing_key: ed25519_dalek::SigningKey, valid values=Any valid Any, optional=No, default value=None
* `key_id: String`: type=Any, meaning=Input for key_id: String, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: crate::error::Result<()>
Semantic meaning: Result of sign_async
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
let result = instance.sign_async();
```

##### `sign_batch_async(credentials: &mut [VerifiableCredential] (Any), signing_key: &ed25519_dalek::SigningKey (Any), key_id: &str (Any)) -> crate::error::Result<()>`

###### Description
/// Asynchronously batch-signs multiple credentials concurrently.

###### Inputs
* `credentials: &mut [VerifiableCredential]`: type=Any, meaning=Input for credentials: &mut [VerifiableCredential], valid values=Any valid Any, optional=No, default value=None
* `signing_key: &ed25519_dalek::SigningKey`: type=Any, meaning=Input for signing_key: &ed25519_dalek::SigningKey, valid values=Any valid Any, optional=No, default value=None
* `key_id: &str`: type=Any, meaning=Input for key_id: &str, valid values=Any valid Any, optional=No, default value=None

###### Output
Return type: crate::error::Result<()>
Semantic meaning: Result of sign_batch_async
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
let result = instance.sign_batch_async();
```

**Private Methods:**

None.

### Exported Functions

None.

## Internal architecture

```mermaid
classDiagram
    direction BT
    class AgentSubject {
    }
    class CryptographicProof {
    }
    class VerifiableCredential {
        +new(id: String:Any, issuer: String:Any, credential_subject: AgentSubject:Any) Self
        +sign(signing_key: &ed25519_dalek::SigningKey:Any, key_id: &str:Any) crate::error::Result<()>
        +sign_async(signing_key: ed25519_dalek::SigningKey:Any, key_id: String:Any) crate::error::Result<()>
        +sign_batch_async(credentials: &mut [VerifiableCredential]:Any, signing_key: &ed25519_dalek::SigningKey:Any, key_id: &str:Any) crate::error::Result<()>
    }

```

## Execution flow & Sequence explanation

```mermaid
sequenceDiagram
    autonumber
    participant Caller as Client Interface
    participant Svc as NhiService
    Caller->>Svc: execute()
    Note over Svc: Processing internal logic
    Svc-->>Caller: result

```


## Examples

```
// Example usage of nhi.rs components
import { ... } from 'crates/factory-core/src/security/nhi.rs';
```


## Cross References
* **Parent module:** `crates/factory-core/src/security`
* **Dependencies:** base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _}, chrono::{DateTime, Utc}, ed25519_dalek::Signer, ed25519_dalek::SigningKey, rand::rngs::OsRng, serde::{Deserialize, Serialize}, super::*
