# Feature Specification: Technical GitHub Security Audit Remediation

**Feature Branch**: `009-security-audit-remediation`

**Created**: 2026-11-02

**Status**: Draft

**Input**: User description: Comprehensive technical audit of rust_CACD_autonomous_factory addressing confirmed security defects, unverified claims, resilience gaps, secret isolation, and CI/CD supply chain hardening.

---

## Executive Summary & Objectives

This specification transforms the comprehensive technical audit findings into rigorous, measurable engineering requirements across `rust_CACD_autonomous_factory`. The objective is to eliminate confirmed critical/high-severity security vulnerabilities, establish deterministic credential isolation, enforce failsafe environment boundaries, bound asynchronous task concurrency and resource histories, and provide reproducible test evidence replacing simulated placeholders.

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - P0: Non-Serializable JIT Token Isolation & Memory Redaction (Priority: P1)

As a security-conscious autonomous factory operator,
I want JIT tokens and credentials to never enter serialized workflow inputs or logs, and to be strictly cleared from memory on drop,
So that workflow execution engines (such as Hatchet) or persistent checkpoints cannot persist, leak, clone, or log bearer secrets.

**Why this priority**:
Critical (P0) security blocker. Exposing raw bearer tokens (`Option<String>`) in `MissionInput` with `#[derive(Serialize, Deserialize, Debug, Clone)]` risks token exfiltration to databases, event buses, and log drains.

**Independent Test**:
Can be fully tested by attempting to serialize `MissionInput` or inspecting serialized workflow payloads and formatted debug logs to verify zero token leakage, and confirming that `JitToken` / secret wrappers retain zeroize-on-drop semantics without plain string conversions across application boundaries.

**Acceptance Scenarios**:
1. **Given** a `MissionInput` struct, **When** it is serialized to JSON or printed via `{:?}`, **Then** no raw JIT bearer token string is included in the output or persisted state.
2. **Given** an active mission execution requiring Vault/JIT operations, **When** tasks execute, **Then** credentials are referenced via opaque session tokens/IDs or zeroizing non-serializable context structures rather than raw bearer tokens in persistent payloads.
3. **Given** any credential memory allocation, **When** the credential reference goes out of scope, **Then** memory is securely zeroized (`ZeroizeOnDrop`) without extraneous clones surviving in memory.

---

### User Story 2 - P1: Strict Vault Repository Policy Isolation & Token Authorization (Priority: P1)

As an enterprise infrastructure operator,
I want Vault token issuance and validation to enforce real, deterministic ACL repository boundaries and comprehensive authorization metadata checks,
So that an autonomous agent operating on repository `org/repo-a` cannot read, write, or access secrets belonging to `org/repo-b`.

**Why this priority**:
High priority. The existing implementation embeds `isolated_path` into token metadata rather than verifying actual Vault ACL engine enforcement. Without end-to-end ACL boundary verification and full token claim validation, multi-repo secret isolation is unproven.

**Independent Test**:
Can be tested against Vault (or local integration mock with ACL policies) by issuing a token for `org/repo-a` and asserting unauthorized access (`403 Forbidden`) when querying `secret/data/repos/org/repo-b/*`, adjacent paths, or wildcard paths.

**Acceptance Scenarios**:
1. **Given** a token issued for repository `org/repo-a`, **When** that token attempts to access `secret/data/repos/org/repo-b/key`, **Then** the request is rejected with access denied by policy.
2. **Given** a token validation routine in `validate_token()`, **When** validating a token, **Then** it explicitly verifies: expected policy attachment, expected audience/repository match, agent identity match, non-renewability, and expiration window.
3. **Given** Vault client communication, **When** network requests are performed, **Then** an explicit request timeout and retry policy are enforced, and role tokens use safe secret types instead of raw `String`.

---

### User Story 3 - P1: Hardened Configuration, Failsafe Credentials & Lifecycle Supervision (Priority: P2)

As a production platform reliability engineer,
I want default/fallback administrative credentials removed and background workflow processes supervised with structured cancellation,
So that misconfigured deployments fail immediately at startup and orphaned tasks do not leak resources.

**Why this priority**:
High priority. Hardcoded fallback credentials (e.g., `admin@example.com` / `admin` in R2R configuration) create critical vulnerabilities if environment variables are omitted. Background tasks (`FinOpsAgent`, Sentry monitors) lack lifecycle supervision.

**Independent Test**:
Can be tested by launching services in staging/production profiles without mandatory environment variables (asserting startup abort with clear diagnostics), and executing mission cancellation (asserting all spawned monitor tasks terminate cleanly).

**Acceptance Scenarios**:
1. **Given** a production execution environment without `R2R_SUPERUSER_EMAIL` or `R2R_SUPERUSER_PASSWORD`, **When** the application bootstraps, **Then** it fails immediately with a fatal error instead of defaulting to hardcoded fallback credentials.
2. **Given** background monitors (FinOps, Sentry), **When** spawned within a mission workflow, **Then** they are linked to a cooperative cancellation token and supervised lifecycle handle that cleanly terminates upon workflow completion or cancellation.

---

### User Story 4 - P1: Multi-Layered Security Verification & Verifiable Agent Identity (NHI) (Priority: P2)

As a security auditor,
I want code safety checks to combine AST/compiler analysis with regex heuristic tripwires, and agent identity (NHI) to verify issuer trust, nonce replay prevention, and bounded async batching,
So that generated code cannot easily evade security gates and agent credentials are cryptographic, trusted, and non-reusable.

**Why this priority**:
High priority. The existing regex scanner achieves only 57.5% detection on holdout adversarial samples and ignores data flow / AST semantics. The NHI credential verification accepts any valid Ed25519 signature regardless of issuer authority or replay.

**Independent Test**:
Can be tested by running adversarial mutation diffs against the combined security analyzer and verifying blocking of evasion techniques; and running NHI verification tests with untrusted issuer keys and replayed nonces to confirm rejection.

**Acceptance Scenarios**:
1. **Given** an agent-generated diff containing evasive patterns (obfuscation, separated loop/break constructs, unsafe primitives), **When** analyzed, **Then** the analysis pipeline flags the code using AST-aware syntax checks and toolchain gates (clippy/cargo-audit) rather than relying exclusively on single-line regex.
2. **Given** a Non-Human Identity (NHI) credential, **When** verified, **Then** verification validates: issuer trust against an authorized issuer keystore, credential validity window (`created` and expiration), and nonce uniqueness against a replay cache.
3. **Given** batch signing or verification of NHI credentials, **When** executed across large batches, **Then** concurrency is bounded via a controlled semaphore / task pool rather than unbounded `spawn_blocking`.

---

### User Story 5 - P1/P2: True Resilience, Fault Injection & Supply Chain Hardening (Priority: P3)

As a DevSecOps engineer,
I want end-to-end fault injection with real process kill signals and hardened CI/CD supply chain controls (action SHA pinning, SBOM, provenance),
So that disaster recovery is proven under real OS termination and release artifacts meet SLSA provenance standards.

**Why this priority**:
Addresses unverified claims in `fault_injection_tests.rs` (which uses in-memory mocks) and `mission-001/report.json` inconsistencies, alongside GitHub Actions supply chain security.

**Independent Test**:
Can be tested by running a realistic fault test simulating process termination between PR creation and checkpointing (verifying idempotency), and verifying CI workflow files for commit SHA action pinning and SBOM attestation generation.

**Acceptance Scenarios**:
1. **Given** a mission execution where the worker creates a PR and is terminated before checkpoint commit, **When** the workflow resumes, **Then** it idempotently detects the existing PR without creating duplicate PRs or orphan state.
2. **Given** GitHub Actions workflows, **When** third-party actions are invoked, **Then** they are pinned to full immutable commit SHAs.
3. **Given** mission reports and evidence artifacts (`report.json`), **When** generated, **Then** resource metrics clearly distinguish worker vs. gVisor sandbox limits, and commit SHAs / delivery URLs reference verified, reproducible entities.

---

## Requirements *(mandatory)*

### Functional Requirements

1. **JIT Token Remediation**:
   - `MissionInput` MUST NOT contain `pub jit_token: Option<String>` with `Serialize`/`Deserialize`.
   - Workflows MUST pass credential identifiers or non-serialized task execution contexts that hold zeroizing types (`JitToken`).
   - `Debug` representations of all credential-handling structs MUST redact secret values.

2. **Vault Secret Isolation**:
   - The Vault client MUST enforce explicit request timeouts and safe role token encapsulation.
   - Token validation MUST verify policy attachment matching `repo-<sanitized-repo-name>`, audience claim, and agent identity.
   - Vault integration tests MUST verify that cross-repository secret paths return permission denied.

3. **Failsafe Environment & Supervised Processes**:
   - Application configuration MUST enforce required secrets in non-development profiles and fail startup on missing credentials.
   - All background tasks spawned during mission execution MUST bind to structured cancellation tokens (e.g. `tokio_util::sync::CancellationToken`) and be monitored for graceful shutdown.

4. **Multi-Stage Security & Identity (NHI)**:
   - Security verification of agent diffs MUST supplement regex heuristics with AST parsing and compiler validation gates.
   - NHI verification MUST validate issuer trustworthiness, temporal bounds, and enforce nonce replay prevention.
   - Batch cryptographic routines MUST limit concurrency via bounded semaphores.

5. **Circuit Breaker Persistence & Bounded State**:
   - Diff history tracking in `circuit_breaker.rs` MUST enforce a maximum bounded size (capacity limit) to avoid memory unbounded growth.
   - State checkpoints MUST support restoring circuit breaker attempt history across worker reboots.

6. **Supply Chain & Evidence Integrity**:
   - GitHub Actions workflows MUST pin action references to immutable commit SHAs.
   - Mission reports MUST define distinct fields for host, worker, and gVisor container resource metrics and link verified artifact SHAs.

---

## Non-Functional & Operational Requirements

- **Security Compliance**: Zero plaintext secrets in memory dumps, logs, or serialized payloads; zeroize on drop.
- **Fail-Secure Architecture**: System components must fail closed on missing policies, missing credentials, or expired tokens.
- **Performance & Stability**: Concurrency bounds must prevent worker thread exhaustion under heavy cryptographic or task-processing loads.
- **Auditability**: All security-critical decisions, policy checks, and workflow milestones must emit structured, redacted audit events.

---

## Out of Scope *(non-goals)*

- Rewriting Hatchet's internal engine architecture; integration must adapt to Hatchet's payload design safely.
- Replacing Ed25519 cryptographic primitives with alternate signature algorithms.
- Full custom static analysis compiler from scratch; leverage existing Rust tooling (`syn`, `clippy`, `cargo-audit`) alongside AST heuristics.

---

## Success Criteria & Verification Evidence

1. **Clean Serialization**: Inspection of serialized mission payloads confirms 0 bytes of sensitive tokens.
2. **Deterministic Vault Tests**: Integration tests prove repository boundary ACL denial (`repo-a` cannot read `repo-b`).
3. **Zero Hardcoded Fallbacks**: Test suite confirms application panic/exit on absent environment secrets in production mode.
4. **Adversarial Security Evaluation**: Holdout adversarial detection rate improves beyond regex baseline through AST checks.
5. **Idempotent Recovery**: Worker kill-recovery test succeeds without duplicate external side effects.
6. **Supply Chain Pinning**: 100% of GitHub Actions in `.github/workflows/` pinned to SHA-256 commit hashes.
