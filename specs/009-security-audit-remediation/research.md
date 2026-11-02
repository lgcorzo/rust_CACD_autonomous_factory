# Research & Analysis: Technical Security Audit Remediation

**Feature**: `009-security-audit-remediation`
**Date**: 2026-11-02

## 1. JIT Token & Credential Lifecycle Remediation

### Problem
`crates/factory-application/src/workflows/autonomous_mission.rs` contains:
```rust
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MissionInput {
    // ...
    #[serde(default)]
    pub jit_token: Option<String>,
}
```
Hatchet or workflow runners serialize `MissionInput` across workflow step checkpoints. This leaks bearer tokens to state stores, telemetry logs via `{:?}`, and clones secrets into heap copies.

### Architectural Solution
1. **Opaque Token Reference**: Replace raw string tokens with an opaque reference/session ID (`Option<SessionCredentialRef>`) or separate execution context (`TaskExecutionContext`).
2. **Encapsulated Zeroizing Token**: Store raw bearer tokens exclusively inside a zeroizing wrapper (`JitToken` with `zeroize::ZeroizeOnDrop`) held in-memory within worker context, NOT in serialized Hatchet workflow payloads.
3. **Custom Debug & Serialization**: Implement explicit `Debug` redact filters (`"[REDACTED_JIT_TOKEN]"`) and prevent `serde::Serialize` from serializing active credentials.

---

## 2. Vault Multi-Repository ACL Isolation & Authorization

### Problem
In `crates/factory-infrastructure/src/vault.rs`:
```rust
let isolated_path = format!("secret/data/repos/{}/*", repo);
let repo_policy = format!("repo-{}", repo.replace('/', "-"));
```
`isolated_path` is passed only in token metadata (`"meta": { ... }`), which does not enforce access boundaries inside Vault unless matching Vault ACL policies are explicitly configured and attached. Furthermore, `validate_token()` checks only TTL/expiration, omitting policy, audience, and agent ID assertions.

### Architectural Solution
1. **Deterministic ACL Boundary Verification**: Ensure Vault policy rules explicitly match path boundaries: `path "secret/data/repos/<sanitized_repo>/*" { capabilities = ["create", "read", "update", "delete", "list"] }` and deny access to other paths.
2. **Comprehensive Validation Checks**: Update `validate_token` to inspect:
   - Associated policy names (must include `repo-<sanitized_repo>`).
   - Token metadata claims (`audience == repo`, `agent_id == expected_agent_id`).
   - Expiration and non-renewability.
3. **Hardened Client**: Add explicit HTTP timeouts (`Duration::from_secs(10)`), retry backoff, and wrap `role_token` into a secure type implementing `ZeroizeOnDrop`.

---

## 3. Configuration & Supervisor Management

### Problem
1. Fallback credentials in `crates/factory-application/src/workflows/autonomous_mission.rs`:
   ```rust
   let r2r_user = std::env::var("R2R_SUPERUSER_EMAIL").unwrap_or_else(|_| "admin@example.com".to_string());
   let r2r_pwd = std::env::var("R2R_SUPERUSER_PASSWORD").unwrap_or_else(|_| "admin".to_string());
   ```
2. Unsupervised background tasks spawned via `tokio::spawn(async move { ... })` for FinOps and Sentry without cancellation tokens or lifecycle supervision.

### Architectural Solution
1. **Strict Environment Enforcement**: Enforce mandatory variables in production profiles (`Environment::Production` or when `FACTORY_ENV != "development"`), failing startup immediately with explicit error.
2. **Structured Lifecycle & Cancellation**: Pass `tokio_util::sync::CancellationToken` and supervise spawned tasks via a join handle tracker / task registry that cancels tasks upon mission completion or workflow shutdown.

---

## 4. Multi-Layer Security Verification & NHI Hardening

### Problem
1. Heuristic regex in `crates/factory-core/src/security.rs` achieved only 57.5% detection on holdout adversarial samples and only checks added lines.
2. `crates/factory-core/src/security/nhi.rs` verifies Ed25519 signatures with an externally supplied key but does not verify issuer trust, expiration window of `created`, or nonce replay.
3. Unbounded concurrency in `sign_batch_async` / `verify_batch_async` using unbounded `tokio::task::spawn_blocking`.

### Architectural Solution
1. **Defense-in-Depth Verification Pipeline**: Combine regex tripwires with AST parsing (`syn` for Rust diffs) and toolchain validation checks (`cargo clippy`, `cargo audit`).
2. **Trust & Replay Prevention**:
   - Introduce an `IssuerKeystore` / trust store to validate verifying keys.
   - Implement a bounded `NonceReplayCache` with TTL tracking.
   - Enforce timestamp validity window checks on `created`.
3. **Bounded Concurrency**: Bound batch operations using `tokio::sync::Semaphore` (e.g. limit concurrency to `num_cpus * 2`).

---

## 5. Resilience & Supply Chain Integrity

### Problem
1. `tests/fault_injection_tests.rs` only tests in-memory mock storage and does not verify recovery under true failure between PR creation and checkpointing.
2. `evidence/missions/mission-001/report.json` contains conflicting resource numbers (1.85 cores vs 0.25 limit) and placeholder commit SHA (`a1b2c3d4e5f6...`).
3. GitHub Actions in `.github/workflows/pipeline.yml` use unpinned branch tags (`actions/checkout@v4`).

### Architectural Solution
1. **Idempotency & Kill-Recovery Verification**: Implement tests verifying that if a mission fails after creating a pull request but before writing a checkpoint, re-execution detects the existing PR without duplicate creation.
2. **Evidence Schema Normalization**: Split metric reporting into `worker_host_metrics` vs `sandbox_gvisor_metrics`, and replace synthetic placeholders with verifiable hashes.
3. **Supply Chain Pinning**: Pin all GitHub Actions in `.github/workflows/` to 40-character commit SHAs.
