# Dark Gravity Audit & Verification Evidence Rules

**Feature**: `011-verified-autonomous-execution`  
**Contract Schemas**:
- `specs/011-verified-autonomous-execution/contracts/evidence-record.schema.json`
- `specs/011-verified-autonomous-execution/contracts/verification-state.schema.json`

This document establishes the shared reporting, evidence-collection, and verification rules that govern all audit artifacts in `docs/audit/` and all evidence records in `evidence/`.

---

## 1. Evidence Classes (`FR-020`, `FR-028`)

Every `EvidenceRecord` MUST specify a `verification_class` from the approved enumeration and a distinct `outcome`:

| `verification_class` | Definition & Scope | Permitted to Satisfy |
|---|---|---|
| `mock` | Simulated, stubbed, or in-memory fake execution where the real runtime/command is not invoked | Mock-behavior classification tests ONLY. **NEVER** satisfies real unit, integration, infrastructure, or runtime verification (`FR-006`). |
| `unit` | Isolated function, struct, or module test executed in-process (`cargo test`, `pytest`) | Code-level logic, boundary validation, parser/contract checks (`implemented` / `TESTED`). |
| `integration` | Multi-component or service-boundary test exercising real interfaces (HTTP router, durable store, workflow engine) | Cross-component admission, webhook authentication, route boundary, and state recovery requirements. |
| `infrastructure` | Static manifest rendering, schema validation, policy scripts (`kubectl kustomize`, `test_sandbox_job_policy.sh`) | `declared` configuration state ONLY. **NEVER** proves live cluster reconciliation or runtime enforcement (`FR-015`, `FR-027`). |
| `live_runtime` | Empirical observation of a live cluster resource, Flux reconciler revision, admission rejection, or scheduled gVisor pod execution | `reconciled`, `enforced`, and `runtime_verified` lifecycle stages (`FR-015`, `FR-016`). |
| `documentation` | Revision-bound inspection of source files, READMEs, and canonical claim registries | Documentation accuracy and inventory requirements (`FR-001`, `FR-022`–`FR-026`). |
| `performance` | Repeatable timing, payload weight, or resource profiling measurement | Measured performance claims (`FR-029`, `SC-011`). |
| `accessibility` | Automated or empirical DOM/WCAG/responsive viewport inspection | Measured accessibility and responsive layout claims (`FR-029`, `SC-011`). |

### Observed Outcomes (`FR-005`, `FR-020`)
Every record MUST classify its `outcome` as one of:
- `passed`: Real execution completed and satisfied all assertions.
- `failed`: Real execution completed and violated one or more assertions or exited non-zero.
- `not_executed`: Request was rejected by validation/policy before execution occurred.
- `infrastructure_error`: Missing binary/runner, unreachable cluster/store, or environment failure prevented execution.
- `timeout`: Workload exceeded its configured execution deadline (`activeDeadlineSeconds` or process timeout).
- `mocked`: Execution used a mock/simulated path; cannot be used as `passed` evidence for real execution.
- `unavailable`: Target repository, cluster, or external service was inaccessible at collection time.

---

## 2. Revision, Timestamp, and Immutability Rules (`FR-001`, `FR-016`, `FR-020`, `FR-021`)

1. **Exact Source Revision**:
   - Every evidence record MUST bind to a `source_revision` containing `repository` (non-empty canonical identifier) and `commit` (full 40-character hexadecimal Git SHA, `^[0-9a-f]{40}$`).
   - Short SHAs, branch names alone, or `"HEAD"` placeholders are invalid.
2. **Deployment Revision Requirement**:
   - Whenever `verification_class` is `live_runtime` (or when a claim asserts deployed/reconciled behavior), `deployment_revision` MUST be populated with the exact reconciled GitOps commit SHA or cluster resource revision.
3. **Timestamps**:
   - `collected_at` MUST be an RFC 3339 UTC timestamp (`YYYY-MM-DDTHH:MM:SSZ`).
4. **Immutable Artifact References**:
   - Each entry in `artifacts[]` MUST include a project-relative `path` (or URI) and a `description`, plus a `sha256` content digest (`^[0-9a-f]{64}$`) whenever the artifact is a file on disk.
   - Evidence records are append-only and immutable. When source or deployment revisions change, new evidence records MUST be collected; old records are retained as historical audit trail and cannot validate the newer revision (`FR-021`).

---

## 3. Requirement State Transitions & Verifier Independence (`FR-018`, `FR-019`)

1. **Allowed State Machine**:
   - `OPEN → IMPLEMENTED → TESTED → VERIFIED`
   - Any invalidation on revision change transitions `VERIFIED → OPEN` (or `TESTED → OPEN`) with `invalidated_at` recorded.
   - Direct transition `IMPLEMENTED → VERIFIED` is **strictly forbidden** (`FR-018`).
2. **Independent Verification**:
   - A requirement may transition to `VERIFIED` only when `verified_by.id` is non-empty and **distinct from** `implementation_agent_id` (`FR-019`).
   - The verifier must inspect the underlying source revision, test output, and artifact digests rather than trusting task checkboxes or agent summaries.

---

## 4. Handling Unavailable Access & Avoiding False Runtime Proof (`FR-002`, `FR-015`, `FR-027`)

1. **Manifest / CI Evidence Is Not Runtime Proof**:
   - Passing `cargo test`, green GitHub Actions workflows, or clean `kubectl kustomize` output establishes `implemented`, `TESTED`, or `declared` status ONLY.
   - A control MUST NOT be marked `reconciled`, `enforced`, or `runtime_verified` without `live_runtime` evidence collected from the target cluster at the inspected source and deployment revisions (`SC-004`).
2. **Unavailable Repositories or Environments**:
   - When a repository (such as `lince-rs`) or live cluster (`server.internal.lgcorzo`) is unreachable or unauthorized, record `access_status: not_accessible` or `outcome: unavailable` with explicit `limitations`.
   - Never infer code contents, silently skip a required stage, or substitute a mocked pass for an unavailable environment.
