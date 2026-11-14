# Risk Register

| Risk ID | Description | Likelihood | Impact | Mitigation |
|---|---|---|---|---|
| **R-01** | MicroK8s API server (`127.0.0.1:16443`) remains unavailable, blocking `RUNTIME_VERIFIED` | High | High | Recover MicroK8s or provision alternative K8s cluster with `runsc` |
| **R-02** | Active credential among 67 historical Gitleaks findings | Medium | Critical | Classify all 67 findings and rotate any active secret before `.gitleaksignore` |
| **R-03** | False positives in hybrid DLP scanner blocking legitimate PRs | Medium | Medium | Validate against `>= 300` benign samples with `FPR = 0%` |
| **R-04** | Multi-replica scaling before PostgreSQL NHI/FinOps/SSE migration | Low | High | Enforce `replicas: 1` in v1 GitOps manifests until Postgres migration completes |
