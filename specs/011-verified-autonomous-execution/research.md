# Research: Verified Autonomous Execution

**Feature**: `011-verified-autonomous-execution`
**Date**: 2026-11-04

## Decisions

### D1. Close the untrusted execution bypass before expanding verification

- **Decision**: Treat every agent-requested test or generated-code execution as untrusted. Route it through the approved gVisor-backed sandbox and fail closed when the sandbox, required runtime, or Kubernetes controls are unavailable. Model test requests as executable plus arguments rather than shell text where the existing interface permits.
- **Rationale**: The reviewed `run_tests` implementation stores a `SandboxDriver` but invokes `sh -c` directly, and a `mock...` command returns a successful test outcome. The code-generated Kubernetes Job has security controls that differ from the GitOps template. A single isolated execution boundary and honest result types address the directly evidenced P0 failures.
- **Alternatives considered**: Preserve host subprocess execution in production; rejected because it permits untrusted code to bypass the sandbox. Continue treating mock results as successful tests; rejected because simulated success cannot prove execution.
- **Evidence**: `crates/factory-mcp-server/src/tools/run_tests.rs`, `crates/factory-mcp-server/src/lib.rs`, `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs`, and `infrastructure/agents/sandbox-job.yaml` in the GitOps checkout.

### D2. Verify controls across lifecycle stages rather than inferring enforcement

- **Decision**: Report each control's state separately as designed, implemented, declared, reconciled, enforced, and runtime-verified where applicable. Require the exact repository revision and live environment evidence for a runtime-verified result.
- **Rationale**: Current GitOps and cluster inspection confirms a restricted sandbox namespace, a NetworkPolicy, and namespaced Role/RoleBinding, while the repository's existing traceability matrix still describes those controls as missing. Manifests and CI status alone cannot prove live behavior.
- **Alternatives considered**: Reuse the earlier audit status without rechecking; rejected because it contradicts current repository and cluster state. Mark every manifest-backed control verified; rejected because declaration is not enforcement evidence.
- **Evidence**: `docs/audit/TRACEABILITY_MATRIX.md`; `infrastructure/agents/{sandboxes-namespace,sandboxes-networkpolicy,rbac,sandbox-job,kustomization}.yaml`; live `kubectl` inspection of `factory-sandboxes`; live Flux status captured during the audit.

### D3. Make the FinOps stop authoritative for every admission path

- **Decision**: Model hard-stop state as one authoritative state source shared by the budget monitor and all admission paths; admission must fail closed if that state cannot be read. Define and test the treatment of already-running missions separately from the prohibition on new admissions.
- **Rationale**: `FinOpsAgent::default()` creates a fresh latch, and the monitor and mission steps instantiate the agent independently. Merely checking a latch on a newly constructed agent cannot propagate a stop. A single authoritative source also avoids a process-local-only guarantee being mistaken for a cross-worker guarantee.
- **Alternatives considered**: Continue constructing default agents at each workflow step; rejected because each instance owns its own latch. Assume one in-process shared pointer is sufficient; rejected unless deployment topology and worker ownership prove that all monitor and admission operations share it.
- **Evidence**: `crates/factory-application/src/agents/finops.rs`; `crates/factory-application/src/workflows/autonomous_mission.rs`.

### D4. Require end-to-end admission and workflow failure tests

- **Decision**: Test budget exhaustion through the normal admission boundary and verify mission behavior for redelivery, retry, cancellation, and worker restart. Separate unit, integration, infrastructure, and live-runtime evidence.
- **Rationale**: A test that toggles the same latch directly on an isolated object does not establish that the monitor's decision reaches the workflow. Workflow correctness similarly depends on durable and distributed boundaries, not only local helper behavior.
- **Alternatives considered**: Treat existing unit coverage or task checkboxes as sufficient; rejected because they do not prove the integration and recovery scenarios required by the specification.
- **Evidence**: `crates/factory-application/src/workflows/autonomous_mission.rs`, `crates/factory-application/tests/finops_admission_tests.rs`, and the feature acceptance scenarios.

### D5. Enforce external-boundary behavior at the effective deployment edge

- **Decision**: Inspect both service-level routes and the deployed ingress/network boundary. Require missing webhook credentials to fail closed in protected environments; confirm accepted events are actually submitted to mission processing. Treat CORS only as browser-origin policy, not caller authentication.
- **Rationale**: The current webhook skips signature validation when its secret is empty and returns `mission_triggered` without a visible dispatch in the handler. Whether other endpoints are reachable or protected cannot be concluded from the application router alone.
- **Alternatives considered**: Infer public exposure from binding to all interfaces; rejected because in-cluster services or ingress may constrain access. Infer mission enqueue from the HTTP response; rejected because the response is not a processing acknowledgement.
- **Evidence**: `crates/factory-mcp-server/src/main.rs`, `crates/factory-mcp-server/src/lib.rs`, and `crates/factory-mcp-server/src/github_webhook.rs`; deployed ingress and cluster policy must be inspected during implementation.

### D6. Use versioned, machine-readable evidence with explicit verification classes

- **Decision**: Define a versioned evidence record contract and machine-readable validation rules that identify requirement, source commit, collection time, environment, verification method, command or test, exit status, outcome class, and artifact references. Keep mocked, unit, integration, infrastructure, and live-runtime evidence distinct. Reject stale or incomplete evidence for requirements that require stronger evidence.
- **Rationale**: Existing repository status and benchmark records are JSON, but the reviewed status record is tied to an older commit and the baseline contains abbreviated image digests. A versioned record is compatible with current artifact conventions and can be validated in CI without introducing a new runtime database as a prerequisite.
- **Alternatives considered**: Treat prose reports as the source of truth; rejected because fields cannot be reliably checked for completeness or freshness. Require a new central evidence service immediately; rejected for the initial increment because it expands operational scope before the record contract and verifier are validated.
- **Evidence**: `evidence/verified_status.json`, `evidence/benchmarks/benchmark_summary.json`, `evidence/baseline.md`, and `.github/workflows/pipeline.yml`. Existing status is a simple commit-bound JSON summary; the workflow already runs formatting, Clippy, and workspace tests, but no versioned evidence schema or independent promotion validator was found in the inspected paths.

### D7. Require independent promotion and prohibit self-reported verification

- **Decision**: A requirement progresses through `OPEN → IMPLEMENTED → TESTED → VERIFIED`. Only a verifier independent of the implementing agent may promote to `VERIFIED`, using source, test output, configuration, commit identity, and any required runtime evidence. There is no direct `IMPLEMENTED → VERIFIED` transition.
- **Rationale**: The audit found completed task checkboxes that conflict with the current source behavior. Independent inspection of evidence and exact revision is necessary to distinguish an implementation claim from demonstrated acceptance.
- **Alternatives considered**: Let the implementing agent self-verify; rejected because it does not address the identified self-reporting risk. Require live-cluster evidence for every requirement; rejected because many unit-level requirements do not need or benefit from live-cluster execution.
- **Evidence**: `specs/010-integrated-k8s-gitops-audit/tasks.md`, current source behavior, and FR-017–FR-021.

### D8. Preserve repository boundaries and phase changes by risk

- **Decision**: Coordinate from the Dark Gravity feature plan but implement and validate changes in their owning repositories. Prioritize P0 execution isolation, then FinOps/workflow and external boundaries, then GitOps/runtime evidence and the verification gate, and finally supporting repositories and public documentation.
- **Rationale**: The main factory, GitOps, Python package, agent service, and website have independent release and CI boundaries. A single cross-repository unreviewed change would make causality and rollback harder. The current agent service is Rust despite its historical repository name; renaming is explicitly out of scope without an approved migration.
- **Alternatives considered**: Merge repositories to simplify planning; rejected because repository count is not evidence of architectural duplication. Rename the agent service automatically; rejected because repository URLs, workflows, consumers, and deployment references may depend on the existing name.
- **Evidence**: Local checkout boundaries and the feature's human-in-the-loop merge assumption.

### D9. Treat current site measurements and statistics as evidence-bearing inputs

- **Decision**: Reconcile architecture/lifecycle and security claims against current source and deployment state. Require a common source, definition, and collection date for repeated repository statistics. Measure performance and accessibility rather than judging them by source inspection.
- **Rationale**: The site and README contain differing public repository and fork counts, and the architecture pages make runtime-security claims that need evidence from actual execution paths. Static metadata inspection cannot substantiate measured performance or accessibility.
- **Alternatives considered**: Pick the latest-looking number manually; rejected because the data-source date and counting method would remain ambiguous. Claim page performance/accessibility from HTML review; rejected because those are runtime/user-experience properties.
- **Evidence**: `lgcorzo.github.io/README.md`, `index.html`, `fork-policy.html`, and `architecture.html`; current owner/source for statistics and measurement tooling remain to be confirmed during that repository's review.

## Research findings and implementation constraints

- **Evidence format and verifier integration**: Existing evidence includes JSON status/benchmark files and narrative reports, but no repository-wide versioned evidence schema or independent promotion validator was found in the inspected paths. `evidence/verified_status.json` is tied to commit `93024823eaa7d8322e32aff177daa481ea49a17a` and must be treated as historical, not current proof. The plan therefore introduces versioned JSON Schemas and a verifier test suite before choosing a CI entry point; do not assume the old `verified` boolean is sufficient.
- **CI validation patterns**: The main repository's current workflow already runs `cargo fmt --all -- --check`, workspace Clippy, and workspace tests. Reuse those gates and add schema/evidence checks to the existing CI workflow rather than adding a second CI system. Repository-specific workflows and checks must be verified at their then-current revisions during implementation.
- **Live Kubernetes claims**: The previous audit observed a restricted `factory-sandboxes` namespace, NetworkPolicy, Role/RoleBinding, and Flux-managed resources. That historical observation supports a baseline only. A new runtime-verification result must read the live Flux/source revision, RuntimeClass handler, admitted workload state, and workload behavior; a successful Kustomize render or manifest inspection is declaration evidence only. If cluster access or a harmless workload test is unavailable, the result stays unverified.
- **Supporting repositories**: The prior inspection captured `fastapi-autogen-team` Rust test commands (`cargo test --test unit_tests` and `cargo test --test integration_tests`) and its README documents ignored Kubernetes smoke tests. The Python package README describes pytest but does not establish a single canonical command in the inspected excerpt. Confirm each repository's current default branch, revision, CI, and command before relying on it; never run destructive smoke tests against shared infrastructure without authorization.
- **Website statistics**: Use one canonical dated source and counting definition for repeated statistics. A public-PR workflow must not need a privileged credential to validate checked-in snapshots; authenticated refresh, if required, is a separately protected action. The data source and automation are to be selected after inspecting the site's current workflow and access constraints.

## Decisions intentionally deferred to implementation

- **Durable FinOps state**: The required contract is shared, authoritative state observed by every admission worker, with fail-closed reads. Choose the backing store only after tracing existing Hatchet/PostgreSQL and worker integration, and test cross-worker visibility and restart behavior. A process-local latch is unacceptable unless topology and lifecycle evidence prove all monitor and admission operations share it.
- **Already-running missions at hard stop**: Prevent new admissions as soon as the stop takes effect. Do not forcibly terminate in-progress work without an explicit cancellation/compensation policy and owner approval.
- **Unavailable environments**: Current live-runtime evidence is time-bound. Repeat against the exact release/deployment revision; if credentials, cluster capabilities, or a safe test target are unavailable, record the check as unavailable and leave the requirement unverified.
