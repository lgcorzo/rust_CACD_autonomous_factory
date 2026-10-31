# Feature Specification: Google AX CRD Workspace Standard (ADR-008 v3.1)

**Feature Branch**: `008-crd-workspace-standard`

**Created**: 2026-10-31

**Status**: Ready for Planning — conditional on Architecture Board approval of ADR-008 v3.1

**Input**: User description: "read the https://notebook.google.com/notebook/116d5f03-2fb0-4fa7-bd7c-1c9849920abc the adr-008-google-ax-crd-workspace-standard-v3.md"

**Source of truth**: [ADR-008 v3.1](../../wiki/architecture/decisions/adr-008-google-ax-crd-workspace-standard-v3.1.md). This spec inherits its decisions (§9) and decides nothing the ADR leaves open; items the ADR marks OPEN/VERIFY are resolved in the plan.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Declarative Agent Workspace and Mission Requests (Priority: P1)

Platform SREs and agent operators declare long-lived agent workspaces in version-controlled manifests reconciled by GitOps, and request missions on them by changing a mission reference. A workspace runs many missions over its life, at most one at a time (ADR §9.1.1–9.1.2). Each mission runs against a pinned commit and pinned constitution content captured when it starts (§9.1.6), so edits made while it runs affect only the next mission. Repository and target branch cannot change for the life of the workspace (§9.1.5).

**Why this priority**: Core capability enabling GitOps-driven agent workspaces without placing per-task state on the cluster control plane.

**Independent Test**: Apply a workspace with a mission reference and verify that exactly one mission starts, pinned to a resolved commit, with no per-task custom resources created. Change the mission reference while it runs and verify the new request is deferred, reported, and started once the first mission ends.

**Acceptance Scenarios**:

1. **Given** a valid workspace manifest with a mission reference, **When** it is applied via GitOps, **Then** the cleanup finalizer is attached and one mission starts, pinned to the commit the target branch pointed to at that moment.
2. **Given** a running mission, **When** the mission reference changes, **Then** no second mission starts concurrently, the workspace reports the request as deferred, and the new mission starts after the active one reaches a terminal state, within the requeue interval.
3. **Given** a mission reference that already ran on this workspace (e.g. after a revert), **When** it is applied again, **Then** it does not run again and the workspace reports it as already executed, in a way GitOps tooling shows as failed.
4. **Given** an existing workspace, **When** its repository or target branch is changed, **Then** the change is rejected.
5. **Given** a running mission, **When** the constitution reference or resource limits are edited, **Then** the running mission is unaffected and the edit applies to the next mission.
6. **Given** a running mission, **When** its tasks execute, **Then** each runs as an ephemeral job or a durable-workflow step and no per-task custom resource is persisted.

---

### User Story 2 - Resilient Execution & Checkpoint Recovery (Priority: P2)

When an execution pod crashes, is killed, or is evicted, the mission resumes from durable checkpoints held outside the control plane. Recovery is **at-least-once**: work since the last checkpoint is lost and the in-flight step is re-executed, so steps MUST be idempotent. Checkpointed steps are not re-executed.

**Why this priority**: Bounds lost work on infrastructure failure and keeps recovery fast.

**Independent Test**: Kill a worker pod mid-mission and verify that the mission resumes from the latest checkpoint, checkpointed steps are skipped, and the interrupted step re-runs without duplicating side effects.

**Acceptance Scenarios**:

1. **Given** a mission with persisted checkpoints, **When** a worker pod is terminated, **Then** the mission resumes within the recovery-time target (SC-001).
2. **Given** a resumed mission, **When** it continues, **Then** checkpointed steps are skipped and the in-flight step re-executes idempotently.

---

### User Story 3 - Clean Workspace Deprovisioning (Priority: P3)

When a workspace is deleted, the platform cancels its runs (active and queued), waits for them to stop within a bounded time, revokes its credentials, and removes its sandboxes before the resource is purged (ADR §9.4). Runs that escape this path are found and terminated by a periodic orphan sweep (§9.5).

**Why this priority**: Prevents orphaned runs, leaked credentials, and lingering sandboxes.

**Independent Test**: Delete a workspace with an active mission and verify ordered teardown. Repeat with the orchestration engine and the secrets service unavailable, and verify the bounded behaviour below.

**Acceptance Scenarios**:

1. **Given** a workspace with an active mission, **When** deletion is requested, **Then** runs are cancelled, awaited until terminal (bounded), credentials revoked, sandboxes deleted, and only then the finalizer removed.
2. **Given** cancellation does not complete within the bound, **When** the bound expires, **Then** credentials are still revoked, sandboxes deleted, and the remaining run is terminated by the orphan sweep.
3. **Given** the secrets service is unavailable, **When** deletion is requested, **Then** deletion waits and reports why, and proceeds once revocation succeeds or the credential lifetime has elapsed.
4. **Given** a run whose workspace no longer exists, **When** the orphan sweep runs, **Then** the run is cancelled and its credentials revoked within the orphan time bound (SC-002).

---

### User Story 4 - Resource Bound Enforcement at Admission (Priority: P4)

Every workspace and every sandbox workload must stay within the resource bound, enforced before anything is persisted (ADR §9.2). The bound applies per container (including init and sidecar containers) and to the pod's workload total. Requests and limits must be declared explicitly, never defaulted. Ephemeral containers are not allowed in sandbox namespaces. Sandbox runtime overhead is excluded from the bound and budgeted separately.

**Why this priority**: Enforces governance and protects node stability, including against platform bugs.

**Independent Test**: Submit workspaces, jobs, and pods that are compliant, over the bound per container, over the bound in aggregate, missing declarations, or adding ephemeral containers, and verify that only compliant ones are admitted.

**Acceptance Scenarios**:

1. **Given** a container whose limits exceed the bound, or whose requests exceed its limits, **When** submitted, **Then** it is rejected before persistence.
2. **Given** a pod whose containers are each within the bound but whose workload total exceeds it, **When** submitted, **Then** it is rejected.
3. **Given** a container with missing requests or limits, **When** submitted, **Then** it is rejected and not defaulted.
4. **Given** a request to add an ephemeral container to a sandbox pod, **When** submitted, **Then** it is rejected.
5. **Given** a workspace declaring zero, negative, or over-bound resource limits, **When** submitted, **Then** it is rejected.
6. **Given** a fully compliant workload, **When** submitted, **Then** it is admitted.

---

### Edge Cases

- **Two mission requests in quick succession, or operator failover before a run exists**: at most one mission is active. Both requests may run one after the other, so "only the latest runs" is best-effort (ADR §9.1.4).
- **Lost completion event**: a deferred mission still starts, within one requeue interval (§9.1.7).
- **Orchestration engine unavailable**: the workspace reports its projected status as possibly stale and new mission requests as undecidable, without flapping on transient errors (§9.1.8).
- **Rapid workflow micro-steps**: produce no status writes; only major transitions do (§9.3).
- **Teardown dependencies unavailable**: deletion waits with a reported reason, within the bounds of §9.4.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The platform MUST register `AgentWorkspace` (`darkgravity.io/v1alpha1`) as the only persistent custom resource for agent workspaces.
- **FR-002**: The platform MUST NOT model tasks as custom resources; tasks run as ephemeral jobs with automatic cleanup or as durable-workflow steps.
- **FR-003**: The orchestration engine MUST be the single source of truth for mission state, retries, and checkpoints.
- **FR-004**: Workspace status MUST be a one-way projection from the orchestration engine. The platform MUST NOT use status to decide anything, including whether a mission is active or was already executed; those questions are answered by the orchestration engine.
- **FR-005**: A workspace MUST run many missions over its life, at most one active at a time. A mission MUST be requested only by changing the mission reference.
- **FR-006**: Each mission reference MUST run at most once per workspace instance, including across operator failover. At most one mission per workspace MUST be active, enforced by the orchestration engine and not only by a check in the operator.
- **FR-007**: A mission reference changed while a mission is active MUST be deferred and reported. It MUST be started after the active mission ends, triggered by the completion event and, if that is lost, by a periodic re-check.
- **FR-008**: At mission start the platform MUST resolve the target branch to a commit and record that commit, the constitution content hash at that commit, and the resource limits in the mission input. The mission MUST use only these.
- **FR-009**: Repository and target branch MUST be immutable for the life of a workspace. Constitution reference and resource limits MAY change and MUST apply only to subsequent missions.
- **FR-010**: Workspace status MUST expose phase, active mission reference, pinned commit, run identifier, observed generation, and conditions covering mission acceptance (dispatched / deferred / already executed / undecidable), synchronisation with the orchestration engine, and blocked finalization, each with a reason and message.
- **FR-011**: Status writes MUST occur only on major phase or condition transitions, coalesced into one write per transition. No periodic timestamp refresh and no no-op writes are allowed.
- **FR-012**: Mission dispatch events MUST be authenticated, and consumers MUST verify them before acting.
- **FR-013**: Deletion MUST cancel active and queued runs, await their terminal state within a bounded time, revoke credentials even if that bound expires, delete sandboxes, and only then release the workspace. Unavailable dependencies MUST be reported and handled within the bounds of ADR §9.4.
- **FR-014**: A periodic sweep MUST detect runs without a corresponding workspace and cancel them and revoke their credentials.
- **FR-015**: The platform MUST enforce the resource bound (memory 30Mi, CPU 250m) per container (including init and sidecar containers) and on the pod's workload total. Requests and limits are mandatory, requests MUST NOT exceed limits, and nothing is defaulted. Ephemeral containers are denied in sandbox namespaces. Workspace limits MUST be positive and within the bound. Sandbox runtime overhead is excluded and budgeted separately.
- **FR-016**: Resource enforcement MUST apply both to workspace declarations and to all sandbox jobs and pods, including those created by the platform itself.
- **FR-017**: Only one operator replica MUST reconcile at a time; standbys take over on leader failure.
- **FR-018**: Crash recovery MUST resume from durable checkpoints without re-executing checkpointed steps, with at-least-once semantics for the in-flight step.

### Key Entities *(include if feature involves data)*

- **AgentWorkspace**: Long-lived declarative environment with repository, target branch, constitution reference, resource limits, and mission reference; status projects phase, active mission, pinned commit, run ID, and conditions.
- **Mission**: Unit of autonomous work on a workspace, identified by (workspace instance, mission reference); its state lives only in the orchestration engine. Its input carries the pinned commit, constitution hash, and resource limits.
- **StepCheckpoint**: Durable intermediate state used to resume a mission.
- **WorkspaceFinalizer**: Deletion guard running the bounded teardown sequence.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: **Recovery time**: after a worker pod crash or kill, mission step execution resumes within 45 seconds. The split of that budget across failure detection, rescheduling, sandbox start, and checkpoint restore is set in the plan.
- **SC-002**: **Orphans**: no run survives more than N minutes without a corresponding workspace (N set in the plan; ADR proposal: 10).
- **SC-003**: **Control-plane write load**: status writes divided by missions reaching a terminal phase, over a rolling 7-day window, is ≤ 6. Each mission produces ≤ 4 + 2·H status writes, where H is its number of human-review cycles.
- **SC-004**: **Triage**: an operator determines a mission's state, acceptance, and synchronisation from the standard command-line listing in under 2 minutes.
- **SC-005**: **Admission enforcement**: 100% of non-compliant manifests are rejected before persistence: over-bound per container or in aggregate, missing declarations, requests above limits, ephemeral containers, and invalid workspace limits.
- **SC-006**: **Single active mission**: in fault-injection tests (rapid mission-reference changes, operator failover), no workspace ever has more than one active mission, and no mission reference runs twice.

## Assumptions

- ADR-008 v3.1 §9 is approved by the Architecture Board. If the Board changes a decision, this spec is updated before tasks are generated.
- Items ADR-008 marks VERIFY (orchestration-engine concurrency semantics, GitOps health-check support, admission/CEL capabilities of the cluster version) are confirmed in the plan; documented fallbacks apply if not.
- Sandbox runtime overhead is measured as the first plan task and budgeted outside the 30Mi / 250m bound.
- Credentials are issued just-in-time with a short, bounded lifetime.
- Concrete technologies (signature scheme, admission mechanism, CLI, orchestration and storage products) are specified in the ADR and the plan, not here.
