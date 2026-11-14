# Implementation Plan — Dark Gravity & LGCorzo Ecosystem

**Baseline Commit:** `a667a90cfc432457e3c748ee5945af2b2e6f4646` (`main`)  
**Maturity Scale:** `IMPLEMENTED` -> `CI_VERIFIED` -> `RUNTIME_VERIFIED` -> `PRODUCTION_READY`

## Objectives
1. Enforce independent Ed25519 verification for new `VERIFIED` evidence transitions while preserving `HISTORICAL` records.
2. Prepare NHI, FinOps, and SSE contracts for multi-replica PostgreSQL evolution while operating single-node/1-replica in v1.
3. Harden Supply Chain (`llmops-python-package` immutable Git SHAs, Rust `cargo llvm-cov` baseline, GitOps `@sha256:...` digests).
4. Improve DLP/Adversarial Scanner to `TPR >= 85%` and `FPR = 0%` on an independent held-out corpus with `>= 300` benign samples.
5. Resolve historical Gitleaks classification, `lgcorzo/kes` consumer audit, and `lgcorzo.github.io` public-only automated inventory.
6. Unblock Kubernetes runtime verification (`gVisor/runsc`, `evidence/baseline.md` regeneration, end-to-end mission) once cluster access is restored.
