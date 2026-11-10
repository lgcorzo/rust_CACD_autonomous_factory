# Implementation Plan — Evidence Integrity & Independent Verification Closure

**Program:** DG-RVP-014  
**Date:** 2026-11-10  
**Status:** Completed  

## 1. Objective
Remediate the verification gaps and false-positive evidence issues identified in the Dark Gravity factory:
1. Enforce strict Evidence Gate classification (`UNIT`, `COMPONENT`, `INTEGRATION`, `KUBERNETES_ADMISSION`, `LIVE_RUNTIME`, etc.).
2. Prevent unit tests or markdown files from certifying `LIVE_RUNTIME`.
3. Separate sandbox attestation into requested, observed, and verified layers.
4. Harden FinOps store with fail-closed amount validation and multi-process concurrency testing.
5. Provide 10 mandatory negative tests in `crates/factory-cli/tests/evidence_contract_tests.rs`.

## 2. Architecture & Design Decisions
- `crates/factory-cli/src/evidence.rs`: Add `EvidenceClass` enum with `max_age_days()`, `satisfies()` matrix, and strict validation.
- `crates/factory-mcp-server/src/sandbox.rs`: Create `RequestedSandboxSpec`, `ObservedRuntimeFacts`, and `VerifiedSandboxChecks`.
- `crates/factory-infrastructure/src/finops_stop_store.rs`: Add rejection for negative numbers, `NaN`, and `Infinity`.
- `.github/workflows/pipeline.yml`: Add `evidence-integrity` job running `factory-cli validate-evidence`.

## 3. Verification
- `cargo test -p factory-cli --test evidence_contract_tests` (8 passed)
- `cargo test -p factory-mcp-server --test launch_sandbox_pod_tests` (4 passed)
- `cargo test -p factory-infrastructure --test finops_concurrency_test` (5 passed)
- `cargo run -p factory-cli -- validate-evidence` (clean pass)

