# Verification Quickstart

**Feature**: `011-verified-autonomous-execution`

This guide is the validation path for implementation and review. A passing build alone does not establish runtime enforcement or independent verification. Run commands from the owning repository, record its full commit, and attach inspectable output to the relevant requirement.

## Prerequisites

- Rust 1.85+ for edition 2024 crates; no `rust-toolchain.toml` or `rust-version` pin is checked in, so the exact tested patch is **TO CONFIRM**.
- `cargo` and access to the locked dependencies.
- For GitOps checks: `kustomize`; for live checks: authorized read-only `kubectl` access to the target cluster.
- For tests requiring Kubernetes, Hatchet, PostgreSQL, Kafka, or external services: the test-specific environment and credentials documented by the owning repository. Do not point destructive smoke tests at shared infrastructure.
- A reviewer who is not the implementing agent for any `VERIFIED` promotion.

## 1. Validate Rust workspace baseline

From `rust_CACD_autonomous_factory/`:

```sh
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

Expected: all commands exit successfully. These checks cover source quality and tests, not sandbox execution in a live cluster. Include the commit SHA and raw command output in the evidence record.

## 2. Exercise untrusted execution and result classification

Run the new execution-boundary unit/integration tests added by the implementation against these cases:

1. A valid supported request with the approved sandbox ready: confirm the result is from the sandbox, includes the runtime identity and exit outcome, and has no host-execution path.
2. Missing/unavailable Kubernetes, required RuntimeClass, policy, or execution image: confirm no host/weak-isolation fallback occurs and the outcome is `not_executed` or `infrastructure_error`.
3. Unsupported runtime, invalid arguments, timeout, and resource exhaustion: confirm the request is rejected or accurately classified and never reported as passed.
4. Mock-only execution: confirm its evidence is class `mock`/outcome `mocked` and cannot satisfy real execution requirements.

Expected: only a real successful run under the approved isolation boundary can satisfy real-execution acceptance. Unit-level mocks are useful for control flow but do not replace sandbox integration evidence.

## 3. Validate GitOps declaration and (when authorized) live reconciliation

From `gitops_internal_lgcorzo/`:

```sh
kustomize build infrastructure/agents
kubectl get runtimeclass gvisor -o yaml
kubectl get namespace factory-sandboxes -o yaml
kubectl -n factory-sandboxes get networkpolicy,role,rolebinding
```

Expected: the render succeeds and live objects match the intended source revision. The first command proves only rendered declarations. Record Flux source and Kustomization revisions and compare them with the live objects. Do not mark runtime enforcement verified from manifest output alone.

With cluster-owner authorization and a safe disposable target, run the approved harmless sandbox verification workload and its negative network/resource checks. Capture the admitted Job/Pod, observed runtime handler, completion result, and cleanup. If any capability or authorization is unavailable, record `unavailable` and leave the corresponding requirement unverified; do not create an alternate weaker test.

## 4. Validate mission admission and workflow recovery

Run the application integration tests using the repository's test harness. The tests must activate the authoritative budget hard stop and submit concurrent requests through the normal mission-admission boundary, then exercise duplicate delivery, retry, cancellation, and worker restart.

Expected: every request beginning after the stop takes effect is rejected; the evidence identifies the shared stop-state version. Duplicate/retried events do not duplicate side effects or advance incomplete work. Results must include any already-running mission behavior and compensation limitations.

## 5. Validate evidence contract and independent promotion

Validate fixtures against [`evidence-record.schema.json`](contracts/evidence-record.schema.json) and [`verification-state.schema.json`](contracts/verification-state.schema.json) using the schema validator wired into CI. The implementation must provide positive and negative fixtures; check that it rejects:

- missing requirement/revision/timestamp/method/artifact references;
- failed, mocked-only, stale, unavailable, or unreproducible evidence offered as proof;
- a state jump from `IMPLEMENTED` directly to `VERIFIED`;
- a verifier who is the same principal as the implementer;
- evidence tied to a source/deployment revision other than the one being verified.

Expected: valid records can be parsed, but a schema-valid record is not automatically sufficient for verification. The independent verifier checks all mandatory acceptance conditions and evidence classes before advancing `TESTED → VERIFIED`.

## 6. Validate supporting repositories and published claims

Run each repository's own current CI commands at its recorded revision. The Rust agent-service README documents:

```sh
cargo test --test unit_tests
cargo test --test integration_tests
```

The Python package and website commands must be taken from their current workflow/configuration rather than inferred from historical README text. Record compatibility, security, accessibility, and performance results separately. Reconcile public lifecycle/security facts and repository statistics to one dated canonical source; do not use unmeasured or historical results as current proof.

## Evidence to retain

For each requirement, preserve the complete record described in [`data-model.md`](data-model.md): requirement ID, exact source and (when applicable) deployment revisions, UTC collection time, environment, evidence class, command/test identity, outcome/exit result, artifacts and content hashes where available, collector, and limitations. Keep historical records immutable. On source/deployment changes, invalidate affected verification states and re-run the required checks.
