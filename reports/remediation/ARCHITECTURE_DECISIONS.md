# Architecture Decisions (Approved)

- **ADR-REM-01 (Tenancy):** Single-tenant per cluster in v1; explicit organization/workspace context in contracts; no misleading optional `tenant_id`.
- **ADR-REM-02 (NHI & FinOps Persistence):** Single-node/1-replica in v1; shared storage traits targeting independent PostgreSQL schemas/roles (no Hatchet internal tables, no Redis/Valkey, no POSIX RWX as sole consistency guarantee).
- **ADR-REM-03 (SSE Scalability):** 1 replica in-memory in v1 (documented limitation); multi-replica requires PostgreSQL shared session registry with `Last-Event-ID` and crash recovery.
- **ADR-REM-04 (Evidence Trust):** Organizational Ed25519 trust root; private key outside repo; independent trust store; canonical serialization, domain separation, anti-replay, key rotation/revocation. Historical evidence remains `HISTORICAL`/`IMPLEMENTED`.
- **ADR-REM-05 (GitOps Images):** Mandatory `@sha256:...` digest + commit tag for all production containers, sidecars, and init containers, validated in CI.
- **ADR-REM-06 (Backups & DR):** PostgreSQL RPO <= 15m / RTO <= 4h (daily + WAL + PITR); MinIO RPO <= 1h / RTO <= 4h (versioning + independent copy); monthly sample restore + quarterly full DR test.
- **ADR-REM-07 (Supply Chain):** Immutable SHAs + `poetry.lock` (`llmops-python-package`), `cargo deny` + `cargo llvm-cov` progressive floor (`rust_CACD_autonomous_factory`); SBOM/Cosign/SLSA deferred.
- **ADR-REM-08 (Gitleaks):** No `git filter-repo` history rewrite; classify all 67 findings, rotate any active secrets, document `.gitleaksignore` by fingerprint.
- **ADR-REM-09 (Portal):** Public-only automated inventory via GitHub API; `VERIFIED` and `DEPLOYED` badges must link to real, dated evidence.
