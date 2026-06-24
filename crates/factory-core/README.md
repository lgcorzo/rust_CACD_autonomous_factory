# 🦴 factory-core

The **Core Layer** of the Dark Gravity Autonomous Agent Factory. This crate contains the shared domain models, security protocols, and common kernel logic that define the system's "Ground Truth."

## 🏗️ DDD Role: Domain Layer

Following **Domain-Driven Design (DDD)**, `factory-core` is the innermost layer. It has zero dependencies on other workspace crates and contains the pure business logic and entities.

### Key Responsibilities

- **Domain Models**: Definitions for `Mission`, `Task`, `Agent`, and `Artifact` representing the ground truth domain.
- **Security Protocols**: The `SecurityValidator` trait providing domain-level interfaces for signature verification and content auditing.
- **Memory Sanitization**: Planned integration with `zeroize`-style memory clearing for sensitive data (tracked in backlog).

## 🛠️ Key Components

- **`lib.rs`**: Core domain models, including metadata containers and mission/task tracking structures.
- **`security.rs`**: Domain-level validation interfaces for auditing agent outputs and verifying signatures.
- **`error.rs`**: Unified error handling map (`FactoryError` / `Result`).

## 🧪 Testing & Verification

- **Unit Tests**: Pure logic validation, signature verification, and input sanitation verification.
