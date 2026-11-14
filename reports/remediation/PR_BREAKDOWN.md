# PR Breakdown

| PR ID | Repo | Finding(s) | Objective | Dependencies | Effort | Priority |
|---|---|---|---|---|---|---|
| **PR-1** | `llmops-python-package` | `F-SUP-01` | Pin `agent-framework-openai` and `mocogpt` to immutable Git SHAs + regenerate `poetry.lock` | None | XS | P0 |
| **PR-2** | `rust_CACD_autonomous_factory` | `F-SUP-02` | Measure `cargo llvm-cov` baseline and add progressive floor in `pipeline.yml` | None | S | P1 |
| **PR-3** | `rust_CACD_autonomous_factory` | `F-SCAN-01` | Hybrid DLP scanner + independent dev/held-out/benign (>=300) corpora | None | M | P1 |
| **PR-4** | `rust_CACD_autonomous_factory` | `F-NHI-01` | Enforce non-optional `trust_store` & `replay_cache` in NHI prod paths | None | M | P1 |
| **PR-5** | `rust_CACD_autonomous_factory` | `F-EVID-01` | Mandatory Ed25519 verification for new `VERIFIED` transitions | None | M | P1 |
| **PR-6** | `rust_CACD_autonomous_factory` | `F-GIT-01` | Classify 67 historical Gitleaks findings + documented `.gitleaksignore` | Owner review of active secrets | S | P1 |
| **PR-7** | `kes` | `F-KES-01` | Audit consumers/upstream, document `root.key` context, update `SECURITY.md` (or archive) | None | S | P1 |
| **PR-8** | `lgcorzo.github.io` | `F-PORTAL-01` | Automated public-only GitHub API inventory + evidence-linked badges | None | S | P2 |
| **PR-9** | `gitops_internal_lgcorzo` | `F-OPS-01`, `F-BAK-01` | Enforce `@sha256:...` image digests + PostgreSQL/MinIO backup CronJobs | None | M | P1 |
| **PR-10** | `rust_CACD_autonomous_factory` | `F-K8S-01` | Regenerate `evidence/baseline.md` & verify `gVisor/runsc` + real mission | K8s cluster available | M | P0 |
