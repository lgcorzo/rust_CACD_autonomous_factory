# Quickstart & Verification Guide: Security Audit Remediation

**Feature**: `009-security-audit-remediation`
**Date**: 2026-11-02

## 1. Verify JIT Token Absence in Serialized Workflow

Run the test asserting `MissionInput` payload serialization contains no raw secret keys or tokens:

```bash
cargo test -p factory-application workflows::autonomous_mission::tests::test_mission_input_serialization_safety
```

Expected output:
- Serialized JSON passes inspection without bearer token strings or sensitive keywords.
- `format!("{:?}", mission_input)` displays redacted values.

---

## 2. Verify Multi-Repository Vault Isolation

Execute Vault integration test asserting isolation across repositories:

```bash
cargo test -p factory-infrastructure vault::tests::test_cross_repository_acl_isolation
```

Expected behavior:
- Token generated for `org/repo-a` returns `403 Forbidden` / policy violation when querying secrets under `secret/data/repos/org/repo-b/*`.
- Token validation fails if audience or repo policy mismatch.

---

## 3. Verify Production Profile Fail-Safe Startup

Run configuration validation without required environment variables:

```bash
FACTORY_ENV=production cargo test -p factory-application tests::test_production_startup_fails_without_credentials
```

Expected behavior:
- Application startup aborts with explicit error if `R2R_SUPERUSER_EMAIL` or `R2R_SUPERUSER_PASSWORD` are missing.

---

## 4. Run NHI Verification with Replay & Trust Checks

Execute NHI suite verifying issuer trust and replay defense:

```bash
cargo test -p factory-core security::nhi::tests::test_nhi_issuer_trust_and_replay_rejection
```

Expected behavior:
- Untrusted issuer keys rejected.
- Repeated nonce invocation rejected with replay detection error.

---

## 5. Verify Supply Chain Action Pinning

Verify that all `.github/workflows/*.yml` files contain full 40-character commit SHAs:

```bash
grep -E "uses: [a-zA-Z0-9_\-\/]+@" .github/workflows/*.yml
```

Expected behavior:
- Every action reference ends with `@<40-char-sha>` (e.g. `uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11`).
