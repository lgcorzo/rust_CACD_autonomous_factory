# 🦴 factory-core

The **Core Layer** of the Dark Gravity Autonomous Agent Factory. This crate contains the shared domain models, security protocols, and common kernel logic that define the system's "Ground Truth."

## 🏗️ DDD Role: Domain Layer

Following **Domain-Driven Design (DDD)**, `factory-core` is the innermost layer. It has zero dependencies on other workspace crates and contains the pure business logic and entities.

### Key Responsibilities

- **Domain Models**: Definitions for `Mission`, `Task`, `Agent`, and `Artifact` representing the ground truth domain.
- **Non-Human Identity (NHI) Security**: Core logic and structures for Ed25519-signed Verifiable Credentials (VC) to establish cryptographic trust between autonomous workers.
- **Memory Sanitization**: Zero-trust memory zeroing using the `zeroize` crate to securely wipe private keys and API tokens from RAM within 4.33 microseconds (verified via Criterion benchmarks).
- **Security Protocols**: The `SecurityValidator` trait providing domain-level interfaces for signature verification and content auditing.

## 🛠️ Key Components

- **`lib.rs`**: Core domain models, including metadata containers and mission/task tracking structures.
- **`security.rs`**: Domain-level validation interfaces for auditing agent outputs and verifying signatures.
- **`error.rs`**: Unified error handling map (`FactoryError` / `Result`).

## 🧪 Testing & Verification

- **Unit Tests**: Pure logic validation, signature verification, and input sanitation verification.
- **Memory Purge Benchmarks**: Automated micro-benchmarking using Criterion to verify that `zeroize` cleans sensitive structures in under 4.33 microseconds.
- **Cryptographic Integration**: Verification of Ed25519 key-pair checks and signature verification flows.
