# v0.3.0 release notes — proposed

This proposal avoids claims about a live cluster or third-party certification. The repository does not establish that the current checked-in files match the `v0.3.0` tag; release-specific inclusion and test results are **TO CONFIRM**.

## Included in the checked-in repository

- The sandbox configuration uses the Kubernetes `RuntimeClass` named `gvisor` with the `runsc` handler.
- The Custom Resource Definition (CRD) and admission policies cap declared workload limits at `30Mi` memory and `250m` CPU. The checked-in sandbox profile declares a worker and sidecar, and the RuntimeClass adds `35Mi` memory and `100m` CPU overhead. Together, the configured limits account for `85Mi` and `450m`; configured requests account for `60Mi` and `250m`. These are configuration sums, not measurements.
- The workspace includes a GitHub webhook route at `POST /webhooks/github`. No Service or Ingress is declared in the checked-in `deploy/` or `config/` directories, so external exposure is **TO CONFIRM**.
- The source includes a Criterion benchmark that calls `JitToken::zeroize()` on a token constructed from a 36-character ASCII string. The repository records no benchmark output or machine; timing is **TO CONFIRM**.

## CI checks configured

The pipeline workflow configures `cargo audit -D warnings`, `cargo fmt --all -- --check`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace -- --skip smoke`, and the evidence-contract test. It also runs the spec-artifact and wiki-export checks. This describes configured jobs, not their results for the `v0.3.0` tag.

## Known limitations

- The sandbox job builder defaults to `512Mi` memory and `500m` CPU, which conflicts with the declared workload cap.
- The repository does not establish the active cluster namespace inventory, installed container network interface (CNI), Kafka provider, or external webhook exposure.
- These notes make no claim of third-party certification.
