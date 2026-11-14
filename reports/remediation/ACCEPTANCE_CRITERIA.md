# Acceptance Criteria

- **`F-SUP-01` (`PR-1`):** `pyproject.toml` has exact 40-char Git SHAs for `agent-framework-openai` and `mocogpt`; `poetry install --no-root` succeeds in CI.
- **`F-SUP-02` (`PR-2`):** `cargo llvm-cov --workspace` runs in CI and fails if coverage drops below measured baseline.
- **`F-SCAN-01` (`PR-3`):** All 17 historical misses pass; independent held-out `TPR >= 85%`; `FPR = 0%` across `>= 300` benign samples.
- **`F-NHI-01` (`PR-4`):** Production NHI verification fails closed if `trust_store` or `replay_cache` is missing or unavailable.
- **`F-EVID-01` (`PR-5`):** Transition to `VERIFIED` fails without valid Ed25519 signature from authorized verifier; historical records remain `HISTORICAL`.
- **`F-GIT-01` (`PR-6`):** All 67 findings classified; active secrets rotated; `gitleaks detect --source .` passes with documented `.gitleaksignore`.
- **`F-KES-01` (`PR-7`):** Consumers audited; `root.key` context documented and `SECURITY.md` updated, or repo archived.
- **`F-PORTAL-01` (`PR-8`):** Public-only inventory auto-generated from GitHub API; zero unlinked `VERIFIED`/`DEPLOYED` badges.
- **`F-OPS-01` / `F-BAK-01` (`PR-9`):** All GitOps images pinned by `@sha256:...`; backup CronJobs deployed and monthly restore test documented.
- **`F-K8S-01` (`PR-10`):** `evidence/baseline.md` regenerated via `scripts/capture_cluster_baseline.sh`; `runsc` execution and full mission `RUNTIME_VERIFIED`.
