# Non-Human Identity (NHI), Credential & External Boundary Security Review

**Feature**: `011-verified-autonomous-execution`  
**User Story**: User Story 3 — Protect external service boundaries (`FR-011`–`FR-014`)  
**Updated At (UTC)**: `2026-11-06T17:38:00Z`

---

## 1. Privileged HTTP Route Inventory & Effective Authorization Boundary (`FR-011`, `FR-013`, `T021`, `T022`)

| Route | HTTP Method | Purpose | Effective Authentication & Authorization Boundary | Session / Resource Bounds |
|---|---|---|---|---|
| `/mcp` | `POST` | JSON-RPC tool invocation & discovery (`run_tests`, `execute_code`, `launch_sandbox_pod`, etc.) | Enforced by `require_privileged_auth` middleware in `crates/factory-mcp-server/src/lib.rs` (`Authorization: Bearer <MCP_API_BEARER_TOKEN>`). When `session_id` query param is present, caller's `X-MCP-Client-Id` must match `SessionRecord.owner_id` (`403 Forbidden` on mismatch). CORS `Origin` header is **never** treated as authentication. | Bounded JSON body; per-session message forwarding uses `try_send` over a bounded 32-slot channel. |
| `/sse` | `GET` | Server-Sent Events (SSE) stream establishment | Enforced by `require_privileged_auth` middleware (`401 Unauthorized` without valid bearer token). | Bounded to `max_sessions` (default `64`, returns `429 Too Many Requests` when full), per-session 32-slot `mpsc::channel`, `session_ttl` (`300s` default), and automatic `SessionDisconnectGuard` cleanup on stream drop. |
| `/feedback` | `POST` | Human-in-the-Loop feedback injection | Enforced by `require_privileged_auth` middleware (`401 Unauthorized` without valid bearer token) plus input control-character sanitization in `feedback_route.rs`. | Bounded JSON payload. |
| `/webhooks/github` | `POST` | Inbound GitHub issue / comment webhook | Enforced by `handle_github_webhook` in `crates/factory-mcp-server/src/github_webhook.rs`: mandatory `GITHUB_WEBHOOK_SECRET` in protected mode (`401` if unset), HMAC-SHA256 `X-Hub-Signature-256` verification (`401`), unique `X-GitHub-Delivery` replay cache (`409 Conflict`), and verifiable `WebhookMissionDispatcher` receipt (`502 Bad Gateway` on dispatch failure). | Enforced payload limit `max_webhook_payload_bytes` (`256 KiB` default; `413 Payload Too Large` on excess). |
| `/health` | `GET` | Liveness / readiness probe | Public read-only status endpoint returning `{"status": "ok"}`. | Stateless constant response. |

---

## 2. Non-Human Identity (NHI) & Credential Assessment (`FR-014`, `T023`)

Inspected implementation files:
- `crates/factory-core/src/security/nhi.rs` (`VerifiableCredential`, `IssuerTrustStore`, `NonceReplayCache`)
- `crates/factory-core/src/security/mod.rs` (`CredentialSessionRef`, `Zeroize`)
- `crates/factory-infrastructure/src/vault.rs` (`VaultClient`, repository-scoped JIT tokens)

| Assessment Dimension | Implementation Mechanism | Verification Evidence (`unit` / `integration`) | Demonstrated Assurance Limits |
|---|---|---|---|
| **Issuer Trust** | `IssuerTrustStore` (`InMemoryTrustStore`) binds an issuer DID (e.g., `did:darkgravity:authority`) to a specific Ed25519 `VerifyingKey`. `verify_with_trust_and_replay` returns `VerificationOutcome::UntrustedIssuer` if the issuer/key pair is not registered (`nhi.rs:413-418`). | `crates/factory-core/src/security/nhi.rs` (`test_nhi_issuer_trust_and_replay_rejection`) | Tested with in-memory trust store. Distributed PKI / Spire / Vault PKI root rotation across cluster nodes is not runtime-verified in this test suite. |
| **Audience & Purpose** | `AgentSubject` encodes `id`, `roles`, and `allowed_namespaces` (e.g., `["factory-sandbox"]`). `CryptographicProof` binds `proof_purpose: "assertionMethod"` and `verification_method` inside RFC 8785 JCS canonicalized signing input (`nhi.rs:151-172`). Vault ACL tokens bind strictly to a single `repository` (`vault.rs`). | `crates/factory-infrastructure/tests/vault_isolation_tests.rs`, `crates/factory-application/tests/mission_input_security_tests.rs` | `VerifiableCredential` does not yet enforce a separate top-level `aud` URI field beyond `AgentSubject.allowed_namespaces` and `proof_purpose`. |
| **Temporal Validity** | `VerifiableCredential` checks `expiration_date` against `now` (`nhi.rs:352-356`), returning `VerificationOutcome::Expired`. Vault JIT tokens enforce a 5-minute (`300s`) non-renewable TTL (`vault.rs`). | `vault::tests::test_vault_expired_token_rejected`, `vault::tests::test_vault_renewable_token_rejected` | Relies on synchronized system clocks (`Utc::now()`) across issuer and verifier. |
| **Revocation** | `CredentialSessionRef` and Vault JIT tokens support explicit revocation on circuit-breaker trip (`autonomous_mission.rs` revokes JIT token when deadlock/max-retries trips). | `crates/factory-application/tests/circuit_breaker_tests.rs` | `VerifiableCredential` JWS proofs are self-contained; revocation before `expiration_date` depends on `NonceReplayCache` or session-store invalidation rather than an online CRL/OCSP responder. |
| **Replay Protection** | `NonceReplayCache` tracks seen `proof.nonce` values within a configurable TTL window (`nhi.rs:65-99`) and returns `VerificationOutcome::ReplayedNonce` on reuse. Webhook deliveries track `X-GitHub-Delivery` IDs (`github_webhook.rs`). | `nhi::tests::test_nhi_issuer_trust_and_replay_rejection`, `tests/webhook_security_tests.rs` | `NonceReplayCache` and `seen_webhook_deliveries` are process-memory caches; cross-pod replay protection requires a shared Redis/PostgreSQL backing store in multi-replica deployments. |
| **Rotation & Secret Exposure** | Raw secret tokens are wrapped in `ZeroizeOnDrop` types (`factory-core/src/security/mod.rs`), `MissionInput` passes only opaque `CredentialSessionRef` handles over workflow serialization, and `Debug` implementations redact `[VERIFIABLE_CREDENTIAL]` (`autonomous_mission.rs:75`). | `crates/factory-application/tests/mission_input_security_tests.rs`, `evidence/security-tests.md` | Memory zeroization guarantees apply to Rust-managed heap buffers after drop, not OS swap or external process dumps. |

