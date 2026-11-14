# Execution Roadmap

1. **Wave 1 (Parallel, No Blockers):** `PR-1` (`llmops-python-package`), `PR-2` (`cargo llvm-cov`), `PR-7` (`kes` audit), `PR-8` (`lgcorzo.github.io` inventory).
2. **Wave 2 (Core Security & Trust):** `PR-3` (Hybrid DLP Scanner), `PR-4` (NHI non-optional controls), `PR-5` (Ed25519 Evidence Trust).
3. **Wave 3 (Secrets & GitOps):** `PR-6` (67 Gitleaks classification + `.gitleaksignore`), `PR-9` (GitOps `@sha256:...` digests + backup CronJobs).
4. **Wave 4 (Runtime Verification — Unblock K8s):** `PR-10` (MicroK8s/K8s recovery, `scripts/capture_cluster_baseline.sh`, `gVisor/runsc` + full mission `RUNTIME_VERIFIED`).
