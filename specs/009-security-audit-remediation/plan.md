# Implementation Plan: Technical Security Audit Remediation

**Branch**: `009-security-audit-remediation` | **Date**: 2026-11-02 | **Spec**: [spec.md](file:///mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/specs/009-security-audit-remediation/spec.md)

**Input**: Feature specification from `specs/009-security-audit-remediation/spec.md`

## Summary

Remediate security vulnerabilities and verify architectural claims highlighted during technical GitHub audit:
1. Remove JIT token serialization and memory exposure from `MissionInput`.
2. Enforce verified Vault ACL isolation and strict token authorization claims.
3. Eliminate default/fallback credentials and supervise background tasks via structured cancellation tokens.
4. Complement regex tripwires with AST parsing, bound NHI concurrency, and enforce issuer trust / nonce replay checks.
5. Provide realistic fault injection recovery tests, normalize evidence metrics, and pin GitHub Actions to full commit SHAs.

## Technical Context

**Language/Version**: Rust 1.80+ (Workspace edition 2021)

**Primary Dependencies**: `tokio` (1.x), `serde`, `zeroize` (1.8+), `ed25519-dalek` (2.x), `syn` (2.x), `reqwest` (0.12+)

**Storage**: Checkpoint storage (S3 / persistent volume), Vault kv-v2 secret engine

**Testing**: `cargo test --workspace`, simulated fault injection & Vault mock ACL harness

**Target Platform**: Linux (x86_64 / aarch64), Kubernetes container environment with gVisor sandbox runtime

**Project Type**: Autonomous Factory Orchestrator & CLI

**Performance Goals**: Bounded batch cryptographic verification (<50ms for 100 credentials with bounded worker pool), zero secret heap retention after drop

**Constraints**: Zero secret leakage in serialization or logs; fail-closed security boundary

**Scale/Scope**: 5 workspace crates (`factory-core`, `factory-application`, `factory-infrastructure`, `factory-cli`, `factory-mcp-server`)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I: Self-contained & modular remediation**: Passes. Each module (`security`, `vault`, `workflows`, `nhi`) maintains strict encapsulation.
- **Principle II: Security by Default**: Passes. Plaintext tokens removed, missing credentials trigger fatal errors, Vault ACLs strictly checked.
- **Principle III: Test-First & Evidence Verification**: Passes. Specific independent tests created for serialization safety, ACL isolation, replay prevention, and process recovery.
- **Principle IV: Human-In-The-Loop Merge**: Passes. All changes delivered on branch `009-security-audit-remediation` with strict manual merge requirement.

## Project Structure

### Documentation (this feature)

```text
specs/009-security-audit-remediation/
├── spec.md              # Feature specification
├── plan.md              # Implementation plan (this file)
├── research.md          # Phase 0 analysis & architectural decisions
├── data-model.md        # Phase 1 domain entities & schemas
├── quickstart.md        # Phase 1 verification commands
└── contracts/           # API/SPI contracts
```

### Source Code Impact

```text
crates/
├── factory-core/
│   └── src/
│       ├── security.rs             # Multi-layer AST + regex analyzer, JitToken zeroize verification
│       └── security/nhi.rs         # Issuer trust store, nonce replay cache, bounded batch async
├── factory-infrastructure/
│   └── src/
│       └── vault.rs                # Vault client timeout, policy validation, ACL boundary enforcement
├── factory-application/
│   ├── src/
│   │   └── workflows/
│   │       ├── autonomous_mission.rs  # Remove jit_token from MissionInput, supervisor cancellation
│   │       └── circuit_breaker.rs     # Bounded diff history, persistence support
│   └── tests/
│       └── fault_injection_tests.rs   # Realistic process recovery and PR idempotency tests
evidence/
└── missions/mission-001/
    └── report.json                 # Normalized resource metrics & real hash references
.github/
└── workflows/
    └── pipeline.yml                # Commit SHA action pinning & supply chain hardening
```

## Phase Strategy & Timeline

- **Phase 0 (Research & Feasibility)**: Complete. Analyzed attack surface, state persistence hazards, and remediation mechanics.
- **Phase 1 (Design & Contracts)**: Complete. Created data models, credential reference patterns, and quickstart verification suite.
- **Phase 2 (Implementation Execution - via `/speckit-tasks`)**:
  - Step 1: P0 JIT token decoupling and zeroize-on-drop wrapper in `factory-application` & `factory-core`.
  - Step 2: Vault ACL verification, authorization checks, and timeout configuration in `factory-infrastructure`.
  - Step 3: Hardened configuration bootstrapping and task supervision in `factory-application`.
  - Step 4: AST-enhanced code verification, NHI issuer trust, and bounded async batching in `factory-core`.
  - Step 5: Fault injection idempotency tests, report metric normalization, and CI SHA pinning.
