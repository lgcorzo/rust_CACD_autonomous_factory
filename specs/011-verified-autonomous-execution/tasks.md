# Tasks: Verified Autonomous Execution

**Input**: Design documents from `specs/011-verified-autonomous-execution/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, and `quickstart.md`

**Tests**: Tests are included because the specification requires independent, positive, negative, integration, and runtime verification scenarios. Run tests in the repository that owns the behavior; do not treat mocked tests as runtime evidence.

**Organization**: Tasks are grouped by user story and ordered by the implementation plan. External repositories are addressed relative to the main repository root (`../gitops_internal_lgcorzo/`, `../mlops-python-package/`, `../fastapi-autogen-team/`, and `../lgcorzo.github.io/`).

## Phase 1: Setup

**Purpose**: Establish a revision-bound, current baseline before making repository-specific claims.

- [x] T001 [P] Record branch, full commit SHA, access status, CI/test entry points, and collection time for every in-scope repository in `docs/audit/ECOSYSTEM_INVENTORY.md`; include `lince-rs` and `autonomous_factory` only if accessible.
- [x] T002 [P] Compare existing audit reports and evidence artifacts with the current source/GitOps revisions, labeling each item current, historical, contradictory, or unverified in `docs/audit/INTEGRATED_AUDIT.md`.
- [x] T003 [P] Map every FR and SC to its owning user story, acceptance condition, required evidence class, and verification source in `docs/audit/TRACEABILITY_MATRIX.md`.

## Phase 2: Foundational

**Purpose**: Set shared reporting rules that all story-level evidence and audit deliverables must follow.

- [ ] T004 [P] Document evidence classes, revision/timestamp requirements, immutable artifact references, unavailable-access handling, and the rule that manifest/CI evidence is not runtime proof in `docs/audit/README.md`.
- [ ] T005 [P] Define positive and negative checks, required access, safe execution conditions, and evidence to retain for every security-sensitive acceptance scenario in `docs/audit/SECURITY_TEST_PLAN.md`.

**Checkpoint**: Current revisions and evidence rules are recorded; story work can proceed. Keep checks marked unavailable rather than inferring a pass.

## Phase 3: User Story 1 - Safely execute untrusted work (Priority: P1) 🎯 MVP

**Goal**: Route all agent-requested untrusted execution through the approved isolation driver; accurately report real, mocked, rejected, timed-out, and infrastructure outcomes.

**Independent Test**: Exercise every untrusted execution entry point with a valid sandbox, unavailable sandbox/runtime, invalid or unsupported request, timeout, and mocked result. Confirm that there is no host-shell fallback and only an isolated real success is reported as passed.

### Tests for User Story 1

- [ ] T006 [P] [US1] Add regression tests for direct host-shell bypass, missing runner, unsupported runtime, invalid arguments, and mocked-success classification in `crates/factory-mcp-server/tests/run_tests_security_tests.rs`.
- [ ] T007 [P] [US1] Add sandbox-driver tests for isolation unavailable, runtime rejected, timeout, and actual exit-result reporting in `crates/factory-mcp-server/tests/sandbox_driver_tests.rs`.

### Implementation for User Story 1

- [ ] T008 [US1] Replace direct `sh -c` execution with the injected approved `SandboxDriver`, preserving validated executable/argument boundaries and returning an explicit not-executed or infrastructure result on failure in `crates/factory-mcp-server/src/tools/run_tests.rs`.
- [ ] T009 [US1] Remove mock-command success behavior and return distinct real-pass, real-failure, not-executed, infrastructure-error, timeout, and mocked outcomes in `crates/factory-mcp-server/src/tools/run_tests.rs`.
- [ ] T010 [P] [US1] Make dynamically created sandbox Jobs enforce the approved RuntimeClass, non-root/read-only/no-capabilities policy, bounded resources, completion deadline, and cleanup behavior, failing closed when required controls are unavailable in `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs`.
- [ ] T011 [US1] Add an opt-in end-to-end gVisor test that confirms a real test runs in the approved sandbox and that sandbox failure never falls back to the host in `crates/factory-mcp-server/tests/gvisor_integration.rs`.

**Checkpoint**: Run the focused MCP-server tests and the authorized gVisor integration test. Do not mark the story complete if live isolation was not exercised; record that evidence gap.

## Phase 4: User Story 2 - Enforce budget stops and reliable mission progress (Priority: P1)

**Goal**: Make budget hard-stop state authoritative across admission workers and preserve mission correctness through concurrent admission, duplicate delivery, retries, cancellation, and worker restart.

**Independent Test**: Activate the hard stop through the budget monitor, submit concurrent missions through the normal admission path, and exercise duplicate events, transient failures, cancellation, and worker recovery. Verify all admissions after the stop are rejected and no incomplete mission is advanced or duplicated.

### Tests for User Story 2

- [ ] T012 [P] [US2] Extend admission integration tests to activate the monitor-owned hard stop, submit concurrent requests through normal mission admission, and assert each request beginning after the stop is rejected with the observed stop-state version in `crates/factory-application/tests/finops_admission_tests.rs`.
- [ ] T013 [P] [US2] Add workflow recovery tests for duplicate event delivery, retry after transient failure, cancellation/compensation, and worker restart without duplicate side effects in `crates/factory-application/tests/workflow_recovery_tests.rs`.

### Implementation for User Story 2

- [ ] T014 [US2] Implement a shared, durable FinOps hard-stop store using the existing persistence/workflow integration; fail admission closed on read errors and test cross-worker visibility in `crates/factory-infrastructure/src/finops_stop_store.rs`.
- [ ] T015 [US2] Inject the shared hard-stop store into both budget monitoring and every mission admission path, removing independently default-constructed latches from `crates/factory-application/src/agents/finops.rs` and `crates/factory-application/src/workflows/autonomous_mission.rs`.
- [ ] T016 [US2] Make mission event handling idempotent and define persisted retry, cancellation, worker-restart, and already-running-mission behavior in `crates/factory-application/src/workflows/autonomous_mission.rs`.
- [ ] T017 [US2] Record the admission boundary, cross-worker hard-stop evidence, retry/restart scenarios, active-mission policy, and remaining gaps in `docs/audit/WORKFLOW_RELIABILITY.md`.

**Checkpoint**: Run FinOps admission and recovery integration tests against the actual admission/workflow boundary. A process-local latch test alone does not satisfy this story.

## Phase 5: User Story 3 - Protect external service boundaries (Priority: P1)

**Goal**: Enforce effective authorization for privileged routes and webhook/event-stream behavior, and provide evidence that accepted webhooks are actually dispatched.

**Independent Test**: Send authorized and unauthorized requests through the deployed boundary; test missing, malformed, invalid, oversized, replayed, and valid webhook deliveries; confirm accepted events enter mission processing and disconnected event streams are bounded and cleaned up.

### Tests for User Story 3

- [ ] T018 [P] [US3] Add webhook tests for missing production secret, invalid signature, malformed/oversized payload, replay, and valid delivery with dispatch-failure handling in `crates/factory-mcp-server/tests/webhook_security_tests.rs`.
- [ ] T019 [P] [US3] Add route-boundary and event-stream tests for unauthorized callers, bounded session resources, disconnect cleanup, and expiry in `crates/factory-mcp-server/tests/route_boundary_tests.rs`.

### Implementation for User Story 3

- [ ] T020 [US3] Require webhook authentication in protected configurations, validate payload bounds and replay state, and return success only after verifiable mission submission in `crates/factory-mcp-server/src/github_webhook.rs`.
- [ ] T021 [US3] Inventory every privileged route and enforce the intended identity/authorization boundary at the effective application/deployment edge; do not treat CORS as authentication in `crates/factory-mcp-server/src/lib.rs` and `crates/factory-mcp-server/src/main.rs`.
- [ ] T022 [US3] Bound event-stream session ownership, lifetime, and cleanup after disconnect or slow/abandoned clients in `crates/factory-mcp-server/src/lib.rs`.
- [ ] T023 [US3] Assess issuer, audience, purpose, validity, revocation, replay, rotation, secret exposure, and demonstrated assurance limits with revision-bound evidence in `docs/audit/NHI_SECURITY_REVIEW.md`.

**Checkpoint**: Run route/webhook tests and, where authorized, request tests through the deployed edge. Record any unavailable deployment-boundary checks as unverified.

## Phase 6: User Story 4 - Distinguish declared controls from enforced controls (Priority: P1)

**Goal**: Track controls separately across design, implementation, declaration, reconciliation, enforcement, and live runtime verification, tied to exact source and deployment revisions.

**Independent Test**: Compare selected sandbox controls in source, rendered manifests, Flux reconciliation, and live resources. Confirm that manifest-only, stale, or inaccessible evidence cannot produce a runtime-verified status.

### Tests and verification for User Story 4

- [ ] T024 [P] [US4] Add a manifest regression check for required RuntimeClass, namespace PSA, RBAC, network policy, image, resource, timeout, and cleanup settings in `../gitops_internal_lgcorzo/tests/test_sandbox_job_policy.sh`.
- [ ] T025 [US4] Align the GitOps sandbox Job with the approved sandbox policy and fail closed on missing required isolation controls in `../gitops_internal_lgcorzo/infrastructure/agents/sandbox-job.yaml`.
- [ ] T026 [US4] Render the agent overlay and compare its declared controls with the dynamically generated Job policy; record source revisions and rendered results in `docs/audit/GITOPS_RUNTIME_VERIFICATION.md` using `../gitops_internal_lgcorzo/infrastructure/agents/kustomization.yaml` and `crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs`.
- [ ] T027 [US4] Record current Flux source/Kustomization revisions, RuntimeClass handler, and observed namespace/network/RBAC/workload state; identify each check as unavailable if cluster access is missing in `docs/audit/GITOPS_RUNTIME_VERIFICATION.md`.
- [ ] T028 [US4] With cluster-owner authorization, execute a harmless disposable sandbox workload and safe negative network/resource checks; capture admitted workload, observed runtime handler, result, cleanup, source revision, and deployment revision in `docs/audit/GITOPS_RUNTIME_VERIFICATION.md`.

### Implementation and reporting for User Story 4

- [ ] T029 [US4] Update each control's lifecycle-stage status and attach exact evidence references, marking stale or unavailable stages unverified in `docs/audit/TRACEABILITY_MATRIX.md`.

**Checkpoint**: No control is labeled runtime-verified without live evidence for the exact inspected source/deployment revision and environment.

## Phase 7: User Story 5 - Independently verify completion with reproducible evidence (Priority: P1)

**Goal**: Validate versioned evidence, reject insufficient or stale results, and allow only an independent verifier to advance tested requirements to verified.

**Independent Test**: Validate complete evidence and reject missing, failed, mocked-only, stale-revision, unavailable, unreproducible, and self-reported-only evidence; reject forbidden status transitions.

### Tests for User Story 5

- [ ] T030 [P] [US5] Add valid/invalid fixtures for required evidence fields, revision formats, evidence class/outcome, artifact references, verifier identity, and allowed transitions against `specs/011-verified-autonomous-execution/contracts/evidence-record.schema.json` and `specs/011-verified-autonomous-execution/contracts/verification-state.schema.json` in `crates/factory-cli/tests/evidence_contract_tests.rs`.

### Implementation for User Story 5

- [ ] T031 [US5] Implement evidence-record parsing and validation for required fields, exact source/deployment revisions, evidence class, outcome, artifact references, and freshness policy using the versioned schemas in `crates/factory-cli/src/evidence.rs`.
- [ ] T032 [US5] Implement requirement-state transition enforcement, including no `IMPLEMENTED → VERIFIED` jump, independent verifier identity, and invalidation on relevant revision changes in `crates/factory-cli/src/evidence.rs` and `crates/factory-cli/src/main.rs`.
- [ ] T033 [US5] Make evidence generation invoke the contract/policy validator and fail CI on invalid, stale, failed, missing, mocked-only, or self-reported-only proof in `scripts/generate_evidence.sh`, `Makefile`, and `.github/workflows/pipeline.yml`.
- [ ] T034 [US5] Independently inspect the generated evidence against every mandatory condition and report pass/fail, evidence class, revisions, and limitations in `docs/audit/VERIFICATION_EVIDENCE_REPORT.md`.

**Checkpoint**: Re-run all negative fixtures and the existing `make evidence` CI path. A schema-valid record alone does not prove that the underlying result is sufficient.

## Phase 8: User Story 6 - Keep ecosystem documentation and public claims accurate (Priority: P2)

**Goal**: Reconcile supporting-repository reviews, public security/lifecycle claims, repository statistics, and site quality measurements to dated and inspectable evidence.

**Independent Test**: Compare canonical architecture, lifecycle, runtime, repository-statistic, and verification facts to all published pages; run contradiction checks and record measured accessibility/performance results or mark them unmeasured.

### Tests and repository reviews for User Story 6

- [ ] T035 [P] [US6] Review the current Python package revision, supported interpreter/dependency groups, packaging, lockfiles, tests, and CI commands; record reproducibility and security outcomes in `docs/audit/reviews/mlops-python-package.md` using `../mlops-python-package/pyproject.toml`, `../mlops-python-package/README.md`, and its current CI configuration.
- [ ] T036 [P] [US6] Review the Rust agent service API, Rust edition, stream behavior, LiteLLM/R2R integration, telemetry, tests, and naming compatibility; record exact revision and test outcomes in `docs/audit/reviews/fastapi-autogen-team.md` using `../fastapi-autogen-team/Cargo.toml`, `../fastapi-autogen-team/src/`, and `../fastapi-autogen-team/README.md`.
- [ ] T037 [P] [US6] Inspect `lince-rs` and `autonomous_factory` only if accessible, recording revision and evidence or explicitly marking each inaccessible without inferred findings in `docs/audit/ECOSYSTEM_INVENTORY.md`.
- [ ] T038 [P] [US6] Add tests that detect divergent canonical counts, lifecycle facts, and unsupported security/runtime claims across public pages in `../lgcorzo.github.io/tests/test_public_claims.py`.

### Implementation and reporting for User Story 6

- [ ] T039 [US6] Create a dated canonical source for public architecture/lifecycle/security claims and repository statistics, including definitions and source links, in `../lgcorzo.github.io/data/canonical-claims.json`.
- [ ] T040 [US6] Implement a deterministic checker that compares canonical facts to the rendered source pages and fails on material contradictions or missing evidence metadata in `../lgcorzo.github.io/scripts/check_public_claims.py`.
- [ ] T041 [US6] Run the canonical-claims check in the website repository's CI and ensure pull-request validation requires no privileged statistics credential in `../lgcorzo.github.io/.github/workflows/site-consistency.yml`.
- [ ] T042 [US6] Reconcile unsupported or inconsistent lifecycle, security, and dated-statistic claims to the canonical evidence source in `../lgcorzo.github.io/index.html`, `../lgcorzo.github.io/architecture.html`, `../lgcorzo.github.io/lifecycle.html`, `../lgcorzo.github.io/notebooks.html`, and `../lgcorzo.github.io/fork-policy.html`.
- [ ] T043 [US6] Measure and report metadata, structured information, accessibility, responsive behavior, links, and page performance; distinguish measured findings from unverified claims in `docs/audit/PUBLIC_SITE_REVIEW.md`.

**Checkpoint**: Run website consistency tests and the available site-quality measurements. Do not claim accessibility or performance is verified without current measurements.

## Phase 9: Polish and Cross-Cutting Concerns

**Purpose**: Integrate results, close evidence gaps where possible, and report remaining risks without overstating verification.

- [ ] T044 Regenerate the integrated findings and requirement/control mappings from the final inspected repository revisions in `docs/audit/INTEGRATED_AUDIT.md` and `docs/audit/TRACEABILITY_MATRIX.md`.
- [ ] T045 Produce the final remediation summary with P0/P1 impact, evidence, owning repository/revision, remediation status, and remaining runtime/access limitations in `docs/audit/REMEDIATION_SUMMARY.md`.
- [ ] T046 Run every safe, available scenario in `specs/011-verified-autonomous-execution/quickstart.md` and attach command output and limitations to the relevant evidence records.
- [ ] T047 Re-run affected Rust formatting, Clippy, workspace tests, repository-specific checks, GitOps rendering, evidence validation, and website consistency checks; record exact commands and revisions in `docs/audit/REMEDIATION_SUMMARY.md`.
- [ ] T048 Review all unresolved findings and evidence gaps; keep overall completion open while any P0 remains unresolved and mark inaccessible live checks as unverified in `docs/audit/REMEDIATION_SUMMARY.md`.

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependency; establish current revisions and baseline first.
- **Foundational (Phase 2)**: Depends on Setup; blocks story implementation so every team uses the same evidence and access rules.
- **User Stories (Phases 3–8)**: Depend on Setup and Foundational. Follow the plan's risk order: execution isolation, FinOps/workflow, external boundaries, runtime control evidence, independent verification, then ecosystem/public claims.
- **Polish (Phase 9)**: Depends on all selected stories; final claims must reflect their resulting evidence.

### User Story Dependencies

- **US1**: Starts after Foundational; first implementation increment and recommended MVP.
- **US2**: Starts after Foundational; independently testable, but execute after US1 in the risk-ordered plan.
- **US3**: Starts after Foundational; independently testable, but execute after US2 in the plan's sequence.
- **US4**: Starts after Foundational; its comparison of declared/generated workloads uses the US1 sandbox policy and its live verification requires an authorized environment.
- **US5**: Starts after Foundational; implement after the source/runtime evidence boundaries are established by US1–US4 so evidence policy reflects real outcomes.
- **US6**: Starts after Foundational; finalize published claims only after the technical reviews and evidence from US1–US5 are available.

### Within Each User Story

- Write the specified regression/contract tests before implementation and verify the tests fail for the intended defect.
- Implement the smallest owning-repository change that satisfies the acceptance criteria; do not bundle unrelated repositories into one change.
- Run story-specific tests before its checkpoint; separately record unit, integration, infrastructure, and live-runtime evidence.
- Tasks marked `[P]` touch distinct files and can be performed independently within their phase; unmarked tasks have an ordering or shared-file dependency.

### Parallel Opportunities

- **Setup**: T001, T002, and T003 can run in parallel because they produce separate audit artifacts.
- **Foundational**: T004 and T005 can run in parallel.
- **US1**: T006 and T007 are separate regression-test files; T010 can proceed independently from the `run_tests.rs` changes.
- **US2**: T012 and T013 cover different test files and scenarios.
- **US3**: T018 and T019 are separate test files.
- **US6**: T035, T036, T037, and T038 are independent repository/test investigations.
- Execute story phases in the plan's risk order even when their individual tasks have parallel opportunities.

## Parallel Example: User Story 1

```text
Task: T006 — add run-tests security regression cases in crates/factory-mcp-server/tests/run_tests_security_tests.rs
Task: T007 — add sandbox-driver outcome cases in crates/factory-mcp-server/tests/sandbox_driver_tests.rs
Task: T010 — align generated Job controls in crates/factory-mcp-server/src/tools/launch_sandbox_pod.rs
```

## Implementation Strategy

### MVP First (User Story 1)

1. Complete Setup and Foundational phases.
2. Implement US1 regression tests and sandbox-driver changes.
3. Run focused unit/security tests and the authorized gVisor integration test.
4. Stop and validate: no host fallback, no false mock pass, accurate outcomes, and explicit unavailable-runtime behavior.
5. Do not claim live isolation verification if the cluster test could not run.

### Incremental Delivery

1. Add US2 shared hard-stop enforcement and workflow recovery; validate the real admission path.
2. Add US3 endpoint/webhook protections and event-stream lifecycle validation.
3. Add US4 declared-to-runtime evidence with exact Flux/source revisions.
4. Add US5 independent evidence validation and CI gating.
5. Add US6 supporting-repository reviews, public-claim consistency, and measured website checks.
6. Finish with integrated audit, evidence freshness review, and explicit remaining gaps.

### Parallel Team Strategy

After Setup and Foundational work, staff can parallelize independent tasks inside a story (especially the marked regression tests and the US6 repository inspections). Preserve the phase sequence where a later task depends on an earlier sandbox policy, live observation, or evidence result. Do not let parallel activity overwrite shared audit reports; consolidate findings after the independent reviews finish.

## Notes

- `[P]` means the task touches files distinct from simultaneously marked tasks and has no incomplete prerequisite.
- `[US#]` links every story task to its specification story. Setup, Foundational, and Polish tasks intentionally have no story label.
- Run shared-cluster or destructive integration checks only with the owning operator's authorization; unavailable checks remain unverified.
- Do not merge changes to a protected default branch automatically; present revisions and validation evidence for human review.
