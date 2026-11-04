# Data Model: Verified Autonomous Execution

**Feature**: `011-verified-autonomous-execution`

This model defines the audit and verification records required by the feature. It describes the contract, not a commitment to introduce a new runtime database. Versioned JSON Schemas in `contracts/` define the evidence and requirement-state interchange formats; implementation must preserve repository boundaries and existing persistence patterns.

## Entities

### Requirement

An independently verifiable unit of expected behavior, sourced from a functional requirement or measurable success criterion.

| Field | Meaning | Validation |
|---|---|---|
| `requirement_id` | Stable source identifier, e.g. `FR-003` or `SC-001` | Unique within the feature; matches `(FR\|SC)-NNN` |
| `statement` | Required behavior or measurable outcome | Non-empty; do not paraphrase away acceptance conditions |
| `acceptance_conditions` | Conditions that must all hold before verification | At least one; each has a stable identifier |
| `required_evidence_classes` | Minimum evidence types needed for this requirement | Must exclude `mock` as sufficient verification |
| `status` | Current governed completion status | `OPEN`, `IMPLEMENTED`, `TESTED`, `VERIFIED` |
| `evidence_refs` | IDs of evidence records considered for this requirement | Each reference resolves; failed/stale/insufficient evidence cannot promote status |

### Evidence Record

An immutable observation or test result associated with one requirement and exact source revision(s). Evidence records capture what happened; they do not themselves assert that a requirement is verified.

| Field | Meaning | Validation |
|---|---|---|
| `evidence_id` | Globally unique evidence identifier | UUID |
| `schema_version` | Version of this record contract | `1.0` for this contract |
| `requirement_id` | Requirement or success criterion addressed | Must resolve to a known requirement |
| `collected_at` | UTC timestamp when the observation was collected | RFC 3339 date-time |
| `source_revision` | Repository identity and exact source commit | Non-empty repository and full commit ID |
| `deployment_revision` | Optional exact reconciled/runtime revision | Required when the evidence claim is about deployed/runtime behavior |
| `environment` | Environment identity and kind | E.g. CI, local, staging, production, cluster; stable enough to interpret |
| `verification_class` | Evidence strength/type | `mock`, `unit`, `integration`, `infrastructure`, `live_runtime`, `documentation`, `performance`, or `accessibility` |
| `method` | Tool and command/test identity used to gather evidence | Non-empty; reproducibility limitations are explicit |
| `outcome` | Observed result, including non-execution | `passed`, `failed`, `not_executed`, `infrastructure_error`, `timeout`, `mocked`, or `unavailable` |
| `exit_code` | Process exit status, if applicable | Integer or `null` when the method has no process exit code |
| `artifacts` | Logs, reports, manifests, screenshots, or other supporting evidence | Each reference is inspectable; include a content digest when available |
| `collector` | Actor/tool that collected the evidence | Identifiable; collection is distinct from verification |
| `limitations` | Known scope, access, freshness, or reproducibility limits | Required when the record cannot establish all relevant claims |

Evidence class and outcome are separate dimensions. For example, a mocked run is `verification_class: mock` and `outcome: mocked`; neither `passed` text from a log nor an exit code makes mock evidence equivalent to a real execution. An unavailable check is a record of a gap, not evidence that a control passed.

### Verification State

The current state and append-only transition history for a requirement. State changes are evaluated by policy, not by accepting an implementer's declaration.

| Field | Meaning | Validation |
|---|---|---|
| `requirement_id` | Requirement whose status is tracked | Must resolve |
| `status` | Current state | One of the four states above |
| `transitions` | Ordered state changes with actor, timestamp, and reason | Each edge must be permitted; history is append-only |
| `implementation_agent_id` | Principal responsible for implementation, when known | Used to establish verifier independence |
| `verified_by` | Independent verifier identity and role | Required at `VERIFIED`; distinct from implementer |
| `evidence_refs` | Records used to evaluate the state | Must satisfy each acceptance condition and evidence-class requirement |
| `invalidated_at` | Optional timestamp when a source/revision change invalidated a prior verification | On invalidation, re-open and re-evaluate against current revisions |

Permitted normal transitions are `OPEN → IMPLEMENTED → TESTED → VERIFIED`. `IMPLEMENTED → VERIFIED` is forbidden. When source or deployment changes invalidate the evidence basis, the requirement returns to `OPEN` with an explicit reason and new verification history; the earlier `VERIFIED` result remains in the audit trail but is no longer current.

### Repository Snapshot

A point-in-time account of a repository included in the ecosystem review.

| Field | Meaning |
|---|---|
| `repository` | Canonical repository name/URL |
| `observed_at` | Snapshot collection time |
| `access_status` | `accessible` or `not_accessible` |
| `revision` | Exact branch and commit, present only when accessible |
| `review_status` | `not_started`, `in_progress`, `reviewed`, or `blocked` |
| `validation_results` | Repository-specific CI/test/security outcomes and evidence IDs |
| `limitations` | Missing access or incomplete checks |

An inaccessible snapshot must not include inferred revision-level findings.

### Security Control

A named protection and its evidence at distinct lifecycle stages: `designed`, `implemented`, `declared`, `reconciled`, `enforced`, and `runtime_verified`. Each stage records its own state and evidence references; later stages cannot be inferred from earlier ones. Runtime verification requires source and deployment revisions, environment, observation time, and observed state.

### Execution Result

The outcome of a requested untrusted execution. It records the request identity, approved isolation/runtime, execution class, actual result, limits, and cleanup. Outcomes are `passed`, `failed`, `not_executed`, `infrastructure_error`, `timeout`, or `mocked`. Only a real `passed` outcome from an approved isolation boundary may satisfy a real-execution acceptance condition.

### Mission

A unit of autonomous work with a stable mission/event identity, admission decision, idempotency key, lifecycle state, retry count, and cancellation/compensation details. Admission records the authoritative budget-stop decision and state version used. A hard-stop decision prevents new admission; treatment of work already admitted is recorded separately.

### Audit Finding

A reviewable claim with stable ID, severity, confidence, affected repository/revision, description, evidence references, limitations, remediation status, and owner. Findings distinguish current observations from historical or unverified claims.

## Relationships and integrity rules

- A `Requirement` has zero or more `Evidence Record`s and one current `Verification State`.
- An `Evidence Record` may be considered for one or more conditions only when its `requirement_id`, revisions, class, outcome, and scope match those conditions.
- A `Verification State` references records and requirements; dangling references invalidate the transition.
- A `Repository Snapshot` supplies revision context for source-backed evidence and audit findings.
- A `Security Control` may be linked to multiple requirements and repository snapshots; each lifecycle-stage assertion references evidence independently.
- A `Mission` admission references the authoritative budget-stop observation used at the admission boundary.
- An `Audit Finding` references evidence; an unsupported claim or inaccessible source is explicitly marked as a limitation, not converted into a pass.
- Evidence is immutable. Corrections produce new records and invalidate affected verification states instead of rewriting history.
- Verification freshness is evaluated against the current relevant source/deployment revision and the requirement's policy; a universal age threshold is not inferred by this model.
