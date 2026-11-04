# Implementation Plan: Verified Autonomous Execution

**Branch**: `011-verified-autonomous-execution` | **Date**: 2026-11-04 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/011-verified-autonomous-execution/spec.md`

## Summary

Remediate and independently verify the Dark Gravity ecosystem, beginning with the confirmed untrusted-test host-execution bypass and disconnected FinOps hard-stop state. Follow with workflow recovery and external-boundary controls, verify GitOps and runtime enforcement against live evidence, establish an independently validated evidence/state gate, and reconcile supporting repositories and public technical claims. Keep code and deployment changes in their owning repositories and report unavailable checks as unverified. A green build, existing manifest, task checkbox, or implementer summary is not runtime proof.

## Technical Context

**Language/Version**: Rust workspace with mixed Rust 2021/2024 editions; Python package supports Python 3.12; GitOps manifests use YAML; public site is static HTML/CSS/JavaScript.

**Primary Dependencies**: Existing locked Rust workspace and its Hatchet, Kafka, Kubernetes, LiteLLM, R2R, Git provider, and identity integrations; the supporting Python package's declared dependency groups and lockfiles; existing GitOps/Flux tooling. Do not introduce a new central evidence service or broad dependency set before validating existing repository patterns.

**Storage**: Existing Hatchet/PostgreSQL and Kafka services for workflow infrastructure; existing JSON evidence and audit files in the main repository. Evidence records are versioned artifacts tied to exact source revisions. A shared, authoritative FinOps stop must be reachable by every admission worker and fail closed when unavailable; select its concrete existing shared-store integration during implementation design, not as an in-process-only latch.

**Testing**: Rust formatting, Clippy, workspace unit and integration tests; sandbox-driver and Kubernetes manifest tests; Python package's configured lint, type, test, and security checks; GitOps render/schema validation; opt-in live-cluster verification; public-site link, accessibility, and measured performance checks. Use each repository's existing supported commands as confirmed during discovery.

**Target Platform**: Linux Rust services, Kubernetes/MicroK8s with gVisor, Flux-managed deployments, GitHub CI, and the static public website. Supporting repositories retain their own platforms and release processes.

**Project Type**: Cross-repository security/reliability remediation, verification tooling and evidence model, deployment validation, and technical-documentation consistency.

**Performance Goals**: Meet the measurable outcomes in `spec.md`; execute supported checks within their existing CI/runtime limits; record measured site performance rather than estimating it. No new throughput target is inferred.

**Constraints**:
- Untrusted execution fails closed and cannot use a host-shell or weaker-isolation fallback.
- Mocked results cannot count as real tests, integration checks, infrastructure checks, or live-runtime evidence.
- Requirement promotion follows `OPEN → IMPLEMENTED → TESTED → VERIFIED`, with independent evidence for the final transition.
- Runtime-verified claims require the exact source/deployment revision, environment, and observed state.
- Keep each repository's changes independently reviewable; no repository rename or public-interface change without a compatibility decision.
- Do not mark inaccessible repositories, unavailable cluster checks, or missing evidence as verified.
- Production merges remain human-controlled under the repository's HITL policy.

**Scale/Scope**: Five primary repositories: `rust_CACD_autonomous_factory`, `gitops_internal_lgcorzo`, `mlops-python-package`, `fastapi-autogen-team`, and `lgcorzo.github.io`; inspect `lince-rs`, `autonomous_factory`, and other named dependencies only if accessible. Main-repository audit deliverables are coordinated under `docs/audit/`; code and deployment work remain in the corresponding repository.

## Constitution Check

The current `.specify/memory/constitution.md` contains unfilled template placeholders, so it does not establish ratified project principles. Do not treat example principles in that file as governance. The applicable repository-level constraint is the explicit HITL policy in `AGENTS.md`: an agent must not merge a PR/MR into a default branch, and must present validation evidence for human review.

Pre-research gates:
- **Fail-closed execution**: Pass. The plan prioritizes isolation and negative tests; no weaker fallback is allowed.
- **Evidence integrity**: Pass. Verification labels are tied to the relevant evidence class and exact revision.
- **Cross-repository scope**: Pass with separation. Each change is delivered and validated in its owning repository; the main repository coordinates requirements and audit records.
- **Human approval**: Pass. Default-branch merges remain a human action.
- **Scope control**: Pass. Execute in risk-ordered, reviewable phases; do not rename repositories or add a new evidence service without a separate justified decision.

Post-design gates:
- **Fail-closed execution**: Pass. The acceptance scenarios and evidence model distinguish real execution from mocks, non-execution, and infrastructure failure.
- **Evidence integrity**: Pass. Contracts bind records to revisions and require an independent verifier for `VERIFIED`; freshness and evidence sufficiency remain verifier policy.
- **Cross-repository scope**: Pass. The data model records repository/revision and access gaps without asserting inaccessible behavior.
- **Human approval**: Pass. The plan does not authorize default-branch merges or production actions.
- **Open implementation choice**: The shared FinOps backing store remains a bounded implementation decision, not an unresolved user requirement: implementation must trace existing persistence/workflow integration and prove cross-worker consistency before selecting it.

## Project Structure

### Documentation (this feature)

```text
specs/011-verified-autonomous-execution/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── evidence-record.schema.json
│   └── verification-state.schema.json
└── tasks.md                         # Created by /speckit-tasks
```

### Source Code and Deployment Repositories

```text
rust_CACD_autonomous_factory/
├── crates/factory-mcp-server/       # Untrusted execution, MCP, webhook, sandbox
├── crates/factory-application/      # FinOps admission and durable mission behavior
├── crates/factory-infrastructure/   # Shared external-service and state integrations
├── crates/factory-cli/              # Verification command/integration if existing patterns support it
├── evidence/                        # Revision-bound test and runtime evidence
├── docs/audit/                      # Inventory, traceability, reviews, remediation reports
└── .github/workflows/               # Independent evidence and consistency gates

gitops_internal_lgcorzo/
└── infrastructure/agents/           # Sandbox Job, namespace, NetworkPolicy and RBAC

mlops-python-package/
└── pyproject.toml and existing package/tests/CI

fastapi-autogen-team/
└── Cargo.toml, src/, tests/, and Kubernetes deployment

lgcorzo.github.io/
└── index.html, architecture.html, lifecycle.html, notebooks.html,
    fork-policy.html, assets/, and site validation workflow
```

**Structure Decision**: This is not a single-crate feature. Use the existing five-crate Rust workspace for application changes, and keep each supporting repository's implementation and CI within that repository. Use the feature directory for shared contracts, validation guidance, and the task plan. Inaccessible repositories receive inventory entries only; do not create speculative changes for them.

## Execution Strategy

Each phase ends with evidence and a go/no-go review. A P0 execution-isolation failure blocks any claim of verified autonomous execution. A phase may report work complete only for its own acceptance conditions; later phases do not retroactively imply earlier runtime verification.

### Phase 0 — Current-state inventory and evidence baseline

- Inspect current HEAD, default branch, tests, CI, dependencies, security configuration, and documentation for every accessible in-scope repository.
- Record full commit IDs, collection time, and access status in `docs/audit/ECOSYSTEM_INVENTORY.md`.
- Compare current source, specs, GitOps, available live state, and public claims; classify claims as observed, stale, unsupported, or requiring live verification.
- Record known P0/P1 findings and evidence gaps without copying stale status from earlier audit files.
- Gate: every conclusion is tied to a current source or explicitly marked unverified.

### Phase 1 — P0 execution isolation and truthful test results

- Remove direct host-shell execution from all untrusted execution paths; ensure the injected approved driver is actually used.
- Replace shell-string interpretation at the execution boundary with a validated structured request.
- Fail closed if Kubernetes, the required gVisor RuntimeClass, or the execution image is unavailable.
- Align dynamically generated workloads with the effective GitOps security/resource policy, including timeouts and cleanup.
- Correct supported-language behavior and ensure unsupported or invalid runtime requests cannot pass.
- Separate mocked results from real execution reports.
- Add positive and negative tests at unit, integration, and actual sandbox boundaries where the test environment permits.
- Gate: zero untrusted host-execution paths; sandbox-unavailable, missing-runtime, invalid-command, and mocked-only cases cannot yield a real pass.

### Phase 2 — FinOps hard stop and mission reliability

- Trace budget monitor creation and every mission admission path, including process/worker topology.
- Implement one shared authoritative stop state reachable by all admission workers; fail closed if it cannot be read.
- Define and test hard-stop semantics for new admissions and already-running work.
- Exercise the normal admission boundary after a spend threshold is reached.
- Review idempotency, retries, redelivery, cancellation, compensation, and worker restart recovery.
- Produce `docs/audit/WORKFLOW_RELIABILITY.md` with scenarios, evidence classes, results, and remaining gaps.
- Gate: the integration scenario proves that after hard stop takes effect no new admission is accepted; failure/retry cases do not duplicate or falsely advance a mission.

### Phase 3 — MCP, webhook, and identity boundaries

- Map effective authentication/authorization for all exposed routes, inspecting both application and deployment edge configuration.
- Bound event-stream resources and verify disconnect cleanup and expiry.
- Require webhook authentication in protected environments; reject missing/invalid credentials, invalid or oversized payloads, and replayed deliveries.
- Verify that an accepted webhook is actually submitted to mission processing.
- Review issuer, audience, purpose, validity, revocation, replay, rotation, secret exposure, and Vault behavior.
- Produce `docs/audit/NHI_SECURITY_REVIEW.md`; document boundary limitations rather than claiming cryptographic assurance by implication.
- Gate: unauthorized/replayed inputs are rejected; successful webhook acceptance has dispatch evidence; all route exposures have a documented effective trust boundary.

### Phase 4 — GitOps and live Kubernetes verification

- Review Flux source/Kustomization revisions and the declared sandbox namespace, RuntimeClass, PSA, RBAC, network, resource, ingress, secret, and image controls.
- Validate rendered manifests and admission behavior.
- Inspect the live runtime class handler and reconciled objects; run a harmless sandbox verification workload and appropriate negative network/resource tests when the cluster allows it.
- Record source and deployment revisions separately and preserve unavailable checks as unverified.
- Produce `docs/audit/GITOPS_RUNTIME_VERIFICATION.md`.
- Gate: runtime claims are backed by live observations for the exact revision; no YAML-only check is presented as runtime enforcement.

### Phase 5 — Independent verification and evidence gate

- Finalize versioned contracts for evidence records and requirement-state transitions.
- Implement validation for required fields, evidence class, revision matching, artifact integrity/references, exit outcomes, and freshness.
- Enforce `OPEN → IMPLEMENTED → TESTED → VERIFIED` with no direct implementation-to-verified transition.
- Keep the verifier independent from the implementation agent and make failed, stale, missing, mocked-only, or self-reported-only evidence block promotion.
- Add tests for both accepted evidence and each rejection condition, and integrate the gate into protected CI workflows.
- Produce `docs/audit/VERIFICATION_EVIDENCE_REPORT.md`.
- Gate: the required negative cases are rejected by automated tests; successful promotion is reproducible and reviewer-auditable.

### Phase 6 — Supporting repositories

- Review Python dependency groups, interpreter compatibility, package installation, training/serving separation, lockfiles, tests, and security checks.
- Review the agent-service API, Rust edition, stream behavior, LiteLLM/R2R integrations, telemetry, tests, and naming implications. Do not rename automatically.
- Inspect `lince-rs` and `autonomous_factory` only if accessible; document inaccessible status otherwise.
- Keep changes in their repository and require compatibility tests for public behavior changes.
- Produce a repository-specific review in `docs/ENGINEERING_REVIEW.md` or a linked audit artifact, with exact inspected revisions.
- Gate: each accessible repository has a revision-bound review and reproducible validation outcome; no public-interface or rename change proceeds without an approved migration decision.

### Phase 7 — Public website and ecosystem consistency

- Reconcile lifecycle and security claims to the current source, GitOps, and verified runtime evidence.
- Establish a dated source and method for repository/fork statistics and detect contradictions across pages.
- Connect lab-notebook claims to repository, commit, test, original result, and limitations.
- Assess metadata, structured data, accessibility, responsive behavior, links, and measured performance using repeatable checks.
- Add CI consistency checks for selected canonical facts and evidence freshness.
- Gate: no unsupported security/runtime/performance claim is presented as verified; public statistics agree or state their source/date and scope.

### Phase 8 — Final integrated audit

- Re-run affected CI, integration, deployment, documentation, and live-runtime validations at the resulting revisions.
- Regenerate the inventory, integrated audit, traceability matrix, security test plan, and remediation summary.
- Produce `docs/audit/REMEDIATION_SUMMARY.md`; report remaining P0/P1 findings, access gaps, and per-repository commits.
- Gate: unresolved P0 findings prevent overall completion; missing cluster access, tests, or evidence remain explicitly unverified.

## Complexity Tracking

| Scope item | Justification | Simpler alternative rejected because |
|---|---|---|
| Coordinating five repositories | The requirements explicitly cover the application, its deployment, a Python package, the agent service, and the public site; each has separate ownership and release evidence. | Treating the main repository as representative would leave runtime configuration and published claims unverified. Changes remain separated by repository to control complexity. |
| Independent evidence gate | Existing task status and audit prose can diverge from implementation; evidence promotion is a cross-cutting trust requirement. | Relying on agent summaries or existing green CI would not prove acceptance at the execution/runtime boundary. |
