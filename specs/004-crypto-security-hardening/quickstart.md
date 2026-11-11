# Quickstart Validation Guide: Zero-Trust Cryptographic Security Hardening

**Feature**: [spec.md](spec.md) | **Branch**: `004-crypto-security-hardening` | **Date**: 2026-09-20  
**Status**: Ready for Verification

This guide outlines runnable scenarios to validate all 5 cryptographic hardening pillars and vulnerability mitigations (VULN-01 to VULN-05) in the Dark Gravity Autonomous Factory.

---

## Prerequisites

- **Rust Toolchain**: Rust 1.85+ (`cargo`) with Tokio async runtime; exact tested patch is **TO CONFIRM**.
- **Environment Variables**:
  ```bash
  export FINOPS_TEAM="dark-gravity-ops"
  export FINOPS_EPIC="E6.3"
  export FINOPS_MICROSERVICE="factory-application"
  export FINOPS_COST_CENTER="eu-rd-grants"
  export FINOPS_MAX_DAILY_BUDGET="50.0"
  ```
- **Optional Local Vault / Wiremock**: For full integration testing against Token APIs.

---

## Validation Scenario 1: Memory Zeroization & RAM Hygiene

Run the zeroization test and benchmark. The benchmark calls `JitToken::zeroize()` on a token constructed from a 36-character ASCII string. The repository records no timing output or machine, so the timing is **TO CONFIRM**.

```bash
# Run unit tests validating zeroize on drop
cargo test --package factory-core test_zeroize -- --nocapture

# Run Criterion zeroization benchmark
cargo bench --bench zeroize_benchmark
```

**Expected Outcome**:
- Credentials overwritten with null bytes.
- Record the benchmark output and machine details; timing is **TO CONFIRM** until those results are checked in.

---

## Validation Scenario 2: Asynchronous Batch Ed25519 Signing

Verify that Non-Human Identities (NHI) sign W3C Verifiable Credentials asynchronously and in batches without blocking the Tokio runtime.

```bash
cargo test --package factory-core --lib security::nhi::tests::test_vc_async_signing_and_batch -- --nocapture
```

**Expected Outcome**:
- `sign_async` and `sign_batch_async` complete cleanly.
- `proof.verification_method` references the Ed25519 key ID.
- Throughput exceeds $100\text{ ops/sec}$.

---

## Validation Scenario 3: SAST Scoring & Critical Vulnerability Rejection

Verify that `SastScanResult::inspect_diff` enforces the $\ge 8.0/10.0$ threshold and rejects RCE, SQLi, and hardcoded credentials.

```bash
cargo test --package factory-application --lib workflows::circuit_breaker::tests::test_circuit_breaker_pass -- --nocapture
```

**Expected Outcome**:
- Clean diffs score $\ge 8.0$ and pass.
- Injected secrets, `eval()`, or `system()` calls reduce score below 8.0, flag critical vulnerabilities, and reject the patch.

---

## Validation Scenario 4: Circuit Breaker 3-Failure & Deadlock Tripping

Verify that `CircuitBreakerGuard` halts execution after 3 failed remediation attempts or upon detecting identical diff hashes.

```bash
cargo test --package factory-application --lib workflows::circuit_breaker::tests::test_circuit_breaker_retries_and_stuck -- --nocapture
```

**Expected Outcome**:
- Attempt 1 returns `Retrying { attempt: 1, max_attempts: 3 }`.
- Attempt 2 returns `Retrying { attempt: 2, max_attempts: 3 }`.
- Attempt 3 returns `AgentStuck { reason: ... }`.
- Escalation alert is generated for human architect (HITL Vertex 3).

---

## Validation Scenario 5: FinOps Vtag Header Injection & HardStop Guardrails

Check that `FinOpsAgent` injects virtual tags, detects spend acceleration, and calculates the 90% daily threshold. Runtime results and currency are **TO CONFIRM**.

```bash
cargo test --package factory-application --lib agents::finops -- --nocapture
```

**Expected Outcome**:
- `FinOpsTag` headers (`x-vtags-*`) are formatted properly.
- Velocity alerts trigger upon rapid budget consumption.
- HardStop threshold is 45.0 budget units (90% of 50.0); runtime behavior and currency are **TO CONFIRM**.
