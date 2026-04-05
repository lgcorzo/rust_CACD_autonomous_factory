# 🦴 factory-core

The **Core Layer** of the Dark Gravity Autonomous Agent Factory. This crate contains the shared domain models, security protocols, and common kernel logic that define the system's "Ground Truth."

## 🏗️ DDD Role: Domain Layer

Following **Domain-Driven Design (DDD)**, `factory-core` is the innermost layer. It has zero dependencies on other workspace crates and contains the pure business logic and entities.

### Key Responsibilities

- **Domain Models**: Definitions for `Mission`, `Task`, `Agent`, and `Artifact`.
- **Security Protocols**: Core logic for SQL/Command injection detection and secret leak prevention.
- **Common Utilities**: Shared error types (`FactoryError`), validation logic, and primitive types.

## 🛠️ Key Components

- **`models/`**: Structs and Enums representing the factory state.
- **`security/`**: Domain-specific security rules used by the `Reviewer` agents.
- **`utils/`**: Generic helpers for UUID handling, timestamping, and logging.

## 🧪 Testing

- **Unit Tests**: High-performance tests for pure logic and protocol parsing.
- **Property-based Testing**: (Planned) Using `proptest` for protocol robustness.
