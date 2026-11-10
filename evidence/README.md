# Evidence classes

Every evidence file must declare one class. A claim is only as strong as its class.

| Class | Meaning | Can support `VERIFIED`? |
|---|---|---|
| `unit_test` | Result of a unit test run in CI or locally. | No |
| `integration_test` | Result of a test spanning several components, not a live cluster. | No |
| `ci_execution` | Log or artifact of a CI workflow run, linked by run ID and commit. | No (build/test facts only) |
| `live_runtime` | Raw observation captured from a running cluster (`kubectl ... -o json`), with timestamp and SHA-256. | Yes, with an independent verifier |
| `external_verification` | Checked by a party other than the author (reviewer, third-party scan). | Yes |
| `mock` | Simulated or hand-written data. | Never |

## Rules
1. Authors mark work `IMPLEMENTED` or `TESTED`. Only a human reviewer sets anything higher.
2. Never copy a *requested* value into an *observed* field.
3. Old evidence is not deleted. A dated note records what supersedes it and why.
4. Live captures come from `scripts/capture_cluster_baseline.sh`, run by a human.
5. `evidence/baseline.md` is `UNVERIFIED: PENDING REGENERATION` until such a capture replaces it.

## Known limits
- **Secret-leak / diff detector:** blocks 23 of 40 held-out adversarial samples (57.5%), 0 of 20 benign samples (measured 2026-11-10, `crates/factory-application/tests/adversarial_suite.rs`). CI enforces this as a floor that may only rise. It is a **secondary control**. The primary controls are the gVisor sandbox and the egress policy.
