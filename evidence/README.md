# Evidence classes

Evidence class names describe the kind of artifact. They do not establish a general assurance level. Read each artifact's source, scope, and limitations before relying on it.

| Class | What it can show | What it cannot show |
|---|---|---|
| `unit_test` | The behavior asserted by a unit test in the recorded run and code revision. | Integration behavior, live deployment state, or behavior outside the tested cases. |
| `integration_test` | The behavior exercised across the components and environment named by the test. | Live cluster behavior unless the recorded test actually ran against that cluster. |
| `ci_execution` | The commands and outcomes in a continuous integration (CI) run when its workflow, run, and commit are identified. | Deployment or runtime state not observed by those commands. |
| `live_runtime` | The state directly captured from a running system at the recorded time, when raw output and provenance are available. | State at other times, unobserved components, or behavior beyond the captured operations. |
| `external_verification` | The specific material independently examined by a named party and the result they recorded. | A broader certification or assurance beyond that examination's stated scope. |
| `mock` | Simulated inputs and behavior in the named mock or fixture. | Production behavior, a live system, or external effects. |

## Rules

1. Use `Designed`, `Implemented`, or `Tested` for project status. Do not treat generated stamps or authored narratives as independent review.
2. Keep requested values separate from observed values.
3. Keep historical evidence. Add a dated superseded note that identifies the replacement or explains what remains unconfirmed.
4. A live capture must include enough raw output and provenance to identify what was observed and when.
5. The existing `evidence/baseline.md` is unverified and pending regeneration. It is historical material, not proof of current runtime state.

## Known limits

The security and reliability reports in this directory are authored scenario descriptions. Their contents do not by themselves establish a reproducible CI run or current live-cluster state. The repository does not include benchmark output or the machine details for the zeroization benchmark.
