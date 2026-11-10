# Evidence Integrity & Independent Verification Report

**Standard**: Dark Gravity Evidence Governance Standard v2.0 (DG-RVP-014)  
**Evaluated At**: 2026-11-09T23:40:00Z  
**Verifier Identity**: `factory-cli-independent-verifier` (role: `independent_verifier`)  
**Status**: `VERIFIED`

---

## 1. Scope & Objective

The objective of this report is to certify that:
1. No capability can transition to `VERIFIED` without qualifying evidence matching its exact requirement class.
2. A lower-level test class (`UNIT`, `COMPONENT`, `INTEGRATION`) can never satisfy a higher-level requirement (`KUBERNETES_ADMISSION`, `LIVE_RUNTIME`, `CHAOS`).
3. Evidence records are strictly bound to Git commit SHAs, RFC3339 timestamps, verified environment descriptors, and cryptographic artifact hashes (`sha256`).
4. Evidence generation executes real commands, captures stdout/stderr/exit codes, and never synthesizes passing results.

---

## 2. Evidence Classification Hierarchy

Enforced in `crates/factory-cli/src/evidence.rs`:

| Class | Level | Permitted Environment | Allowed Tooling | Mandatory Artifacts | Max Age |
|---|:---:|---|---|---|:---:|
| `UNIT` | L1 | `local`, `ci`, `cluster` | `cargo-test`, `pytest` | Source or test output | 90 days |
| `COMPONENT` | L2 | `local`, `ci`, `cluster` | `cargo-test`, `pytest` | Test execution logs | 90 days |
| `INTEGRATION` | L3 | `local`, `ci`, `cluster` | `cargo-test`, `pytest` | Multi-crate integration logs | 90 days |
| `KUBERNETES_ADMISSION` | L4 | `cluster`, `staging`, `production` | `kubectl`, admission webhooks | Cluster API responses / denials | 30 days |
| `LIVE_RUNTIME` | L5 | `cluster`, `staging`, `production` | In-pod probes, runtime watchers | Structured telemetry (`.json`, `.log`) | 30 days |
| `SECURITY_NEGATIVE` | L5 | `cluster`, `staging`, `production` | In-pod isolation probes | In-pod denial outputs, audit logs | 90 days |
| `CHAOS` | L6 | `cluster`, `staging` | Chaos Mesh, fault injectors | Cluster telemetry, resilience logs | 30 days |
| `MANUAL_REVIEW` | — | Any | Human auditor | Signed audit document / cryptographic signature | 180 days |

---

## 3. Mandatory Negative Gate Tests

Ten automated regression tests in `crates/factory-cli/tests/evidence_contract_tests.rs` verify that the Evidence Gate rejects false-positive verifications:
1. **Neg-01**: `LIVE_RUNTIME` record backed only by `cargo test` -> Rejected.
2. **Neg-02**: `LIVE_RUNTIME` record with environment `local` -> Rejected.
3. **Neg-03**: `LIVE_RUNTIME` record backed only by Markdown -> Rejected.
4. **Neg-04**: Evidence record with tampered or mismatched artifact SHA-256 -> Rejected.
5. **Neg-05**: Evidence record exceeding maximum allowed age -> Rejected.
6. **Neg-06**: Record with missing exit code claiming `passed` -> Rejected.
7. **Neg-07**: Record with missing or non-independent verifier -> Rejected.
8. **Neg-08**: Requirement claiming `VERIFIED` with empty `evidence_refs` -> Rejected.
9. **Neg-09**: Requirement `FR-008` (`LIVE_RUNTIME`) satisfied with a `UNIT` evidence record -> Rejected.
10. **Neg-10**: Record claiming outcome `passed` with non-zero exit code (`101`) -> Rejected.

All 10 negative test cases fail-closed as required.
