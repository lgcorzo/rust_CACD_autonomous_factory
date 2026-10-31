---
iso_doc_type: "Record"
iso_viewpoint: "ArchitectureDecision"
type: "architecture"
title: "ADR-008: Google AX Declarative Patterns, CRD Scope, and SSOT Reconciliation Contract (v3.1)"
description: "ADR-008 v3 (accepted) plus v3.1 amendment (proposed) resolving workspace↔mission cardinality, spec mutability, and the scope of the 30Mi/250m resource bound."
tags: ['adr', 'iso42010', 'crd', 'kube-rs', 'hatchet', 'admission']
timestamp: "2026-10-31T10:35:00+01:00"
generated: "agent:antigravity"
verified: "false"
---

# ADR-008: Adoption of Google AX Declarative Patterns, CRD Scope, and SSOT Reconciliation Contract

| Field | Value |
| :--- | :--- |
| **Status** | v3: Accepted (2026-10-30) · **v3.1 amendment (§9): Proposed — pending Architecture Board review**. §9.1.5 (R) and §9.2.5 (X) decided by the Lead System Architect on 2026-10-31. |
| **Deciders** | Dark Gravity Architecture Board, Lead System Architect & SRE Principal |
| **Target subsystems** | `gitops_internal_lgcorzo`, `factory-cli` (operator), `factory-application`, orchestrators (Hatchet), `development` (gVisor) |
| **Origin** | §1–§8 reproduce v3 as held in NotebookLM source `adr-008-google-ax-crd-workspace-standard-v3.md`. §9 is new. |
| **Consumers** | `specs/008-crd-workspace-standard/spec.md` inherits §9; it must not decide anything this ADR leaves open. |

---

## 1. Context and Problem Statement

As the Dark Gravity Autonomous Software Factory scales its CA/CD workforce, we require a Kubernetes-native declarative interface to manage multi-agent workspaces via GitOps (`gitops_internal_lgcorzo`).

Google AX (Agent Executor) introduces a pattern that decouples logical agent sessions (Actors) from physical compute (Workers), with four declarative primitives: Workspace, Task, Gateway, Model. This ADR adopts AX's conceptual patterns while rejecting its software stack, resolves CRD vs. etcd constraints, establishes a single source of truth (SSOT), and defines the operational contract of our native Rust operator (`kube-rs`).

## 2. Technical Evaluation of Google AX

- **Open source & self-hosted**: Apache 2.0; a self-hosted runtime, not a managed GCP/Vertex AI service.
- **Maturity**: active early development with frequent breaking changes; a reference architecture, not a cross-industry specification.
- **etcd design choice**: AX deliberately does not model tasks as CRDs, because millions of short-lived task objects create etcd write churn. AX stores task state in Redis and executes via its Agent Substrate runtime.

## 3. Decision Outcome

- **CRD scope**: `AgentWorkspace` (`darkgravity.io/v1alpha1`) is the **only** CRD. `AgentTask` is **not** a CRD.
- **Task execution**: atomic tasks run as `batch/v1` Jobs (`ttlSecondsAfterFinished: 60`) or as step functions in Hatchet's durable DAG.
- **Runtime**: we reject the AX codebase (Go / Redis / Agent Substrate) to avoid duplicating our stack (Hatchet, CloudNativePG PostgreSQL 16, Kafka, Rust 2024).

## 4. SSOT & Reconciliation Contract

### A. Authority & one-way status projection

- **Hatchet (on PostgreSQL CNPG) is the absolute SSOT** for mission state, step retries, diff SHA-256 hashes, and `BridgeState` checkpoints.
- **`AgentWorkspace.status` is a strictly one-way, read-only projection** (Hatchet → etcd), written for GitOps observability and FluxCD tracking. The operator **never** reads `status` as an input to operational decisions.
- **Status throttling**: patches only on major phase transitions (`Pending`, `Initializing`, `Running`, `Paused`, `Completed`, `Failed`).

### B. Reconciliation loop (`factory-cli operator` via `kube-rs`)

1. FluxCD applies the `AgentWorkspace` to etcd.
2. The operator watches specs and compares generation.
3. The dispatch layer emits an Ed25519-signed event to Kafka (mission ingestion).
4. Hatchet runs the DAG; checkpoints go to MinIO S3.
5. The operator projects major phase changes to `status`.

- **Read**: `AgentWorkspace.spec`, generation metadata, Hatchet workflow status (gRPC).
- **Write**: Kafka mission-ingestion events (on new spec generation); throttled `status` patches.

### C. Finalizer `darkgravity.io/workspace-finalizer`

On deletion: (1) CANCEL active Hatchet runs for the workspace; (2) revoke Vault JIT tokens for the repository path; (3) delete ephemeral sandboxes in `development`; (4) remove the finalizer.

### D. Divergence

- **Orphaned runs**: a running workflow for a deleted CRD is force-terminated and its tokens revoked.
- **Idempotency**: reconciliation uses `metadata.generation` to avoid duplicate Kafka events on non-spec updates. *(Refined in §9.1.4.)*

## 5. Failure Modes & HA

- **Operator HA**: leader election (`coordination.k8s.io/v1` Lease); standbys stay warm.
- **Crash resilience**: Hatchet resumes from the latest `StepCheckpoint` in MinIO (`dg-factory-checkpoints`); completed mutations and tool calls are skipped.

## 6. Policy Enforcement & KPIs

### A. Hard limits

A `ValidatingAdmissionPolicy` (or Kyverno) evaluates `AgentWorkspace` CRDs and sandbox Jobs; any request or limit above `memory: 30Mi` or `cpu: 250m` is rejected before reaching etcd. *(Scope defined in §9.2.)*

### B. SRE KPIs

| Metric | Target | Measurement |
| :--- | :--- | :--- |
| MTTR | < 45 s | Pod crash/kill → Hatchet step resumption in gVisor sandbox |
| Orphaned executions | 0 active leaks | Hatchet runs without a matching CRD/finalizer |
| etcd write rate | < 5 status writes/mission | `apiserver_request_total` filtered by `darkgravity.io` patches |
| SRE triage | < 2 min | Diagnosis via `kubectl get agws` |

## 7. Declarative Specification (v3 example)

```yaml
apiVersion: darkgravity.io/v1alpha1
kind: AgentWorkspace
metadata:
  name: workspace-lince-rs
  namespace: agents
  finalizers:
    - darkgravity.io/workspace-finalizer
spec:
  repositoryUrl: "https://github.com/lgcorzo/lince-rs.git"
  targetBranch: "main"
  constitutionRef: ".specify/memory/constitution.md"
  resourceLimits:
    maxRam: "30Mi"
    maxCpu: "250m"
status:
  phase: "Running"
  activeMissionId: "real-mission-e7-001"
  hatchetWorkflowRunId: "wf-run-786625995"
  lastSyncTimestamp: "2026-10-30T14:45:00Z"
```

## 8. Summary vs. v2

| Dimension | v2 | v3 |
| :--- | :--- | :--- |
| CRD scope | `AgentTask` as CRD | `AgentWorkspace` sole CRD; tasks as Jobs / Hatchet steps |
| etcd protection | Abstract | Throttled status + Job TTL |
| Status role | Projection, no rules | Strict one-way projection; never read |
| Hard limits | Git versioning | Admission-time enforcement |
| HA | Omitted | Leader election + finalizer |
| Success criteria | Qualitative | Quantitative SRE KPIs |

---

## 9. Amendment v3.1 *(Proposed — for Architecture Board review)*

Decision status per item: **DECIDED** (confirmed by the Lead System Architect, 2026-10-31), **OPEN** (to be closed in the implementation plan), or **VERIFY** (a technical assumption not yet tested against the cluster or the deployed versions).

### 9.1 Workspace ↔ Mission

**9.1.1 Cardinality — 1:N sequential. DECIDED.** An `AgentWorkspace` is a long-lived environment that runs many missions over its lifetime, **at most one active at a time** (enforced by 9.1.4). Concurrent missions per workspace are out of scope for v1alpha1.

**9.1.2 Mission trigger — explicit `spec.missionRef`. DECIDED.** A mission is requested by setting `spec.missionRef` to a new opaque value. Edits to other fields never start a mission. This replaces "new spec generation" in §4.B, because `metadata.generation` also increments on edits that must not start missions.

**9.1.3 Activity is answered by Hatchet, never by `status`. DECIDED.** `status.activeMissionRef` is a projection only. The operator determines whether a mission is active, and whether a `missionRef` was already executed, by querying Hatchet for runs labelled with the workspace UID.

**9.1.4 Two guarantees, two keys. DECIDED (mechanism VERIFY).**

| Guarantee | Key | Enforced by | Why |
| :--- | :--- | :--- | :--- |
| A given `missionRef` runs at most once | `(metadata.uid, spec.missionRef)` | Consumer that creates the Hatchet run: create-if-absent on a unique key | After a failover two leaders may both emit; only consumer-side uniqueness closes the race. `uid` (not `name`) makes a recreated workspace start fresh. |
| At most one mission active per workspace | `metadata.uid` | Hatchet concurrency control: key = workspace UID, max 1 running, **queueing** strategy (never cancel-in-progress or cancel-newest) | The operator's check-then-emit (9.1.3) is not atomic. If `missionRef` changes twice quickly, or a failover happens before the consumer has created the first run, the operator sees no active mission and emits again. |

*VERIFY:* the concurrency strategies available in the deployed Hatchet version, and that the chosen one queues rather than drops or cancels.

*Consequence:* the "last write wins" of 9.1.5 is **best-effort**. In the race above, two `missionRef` values can both be queued and will run one after the other. The invariant that is guaranteed is "at most one active", not "only the latest runs".

**9.1.5 Spec mutability — (R) snapshot at dispatch. DECIDED.**

| Field | Rule | Enforcement |
| :--- | :--- | :--- |
| `repositoryUrl`, `targetBranch` | Immutable for the life of the workspace | CEL transition rule `self == oldSelf` |
| `constitutionRef`, `resourceLimits` | Editable; edits apply to the **next** mission only | Snapshot at dispatch (9.1.6) |
| `missionRef` | Editable; a change while a mission is active is deferred, not dispatched | Operator (9.1.7), backed by 9.1.4 |

Rejected alternative: a validating webhook that queries Hatchet. It couples every GitOps write to Hatchet availability: with fail-closed, edits are blocked during an outage; with fail-open, the rule is bypassed.

**9.1.6 The snapshot pins content, not strings. DECIDED.** `targetBranch` is a moving pointer and `constitutionRef` is a path inside the repository, so copying the strings freezes nothing. At dispatch the operator:

1. resolves `targetBranch` to a **commit SHA**;
2. records in the mission input: `commitSha`, `constitutionRef`, the SHA-256 of the constitution file content at that commit, and `resourceLimits`;
3. projects `commitSha` to `status.activeMissionCommit`.

The mission works exclusively against that commit and that constitution content. Branch-tip and constitution changes take effect at the next dispatch.

**9.1.7 Dispatching a deferred `missionRef` when the active mission ends. DECIDED.** Two triggers:

- **Event-driven (primary):** Hatchet publishes terminal-state events for workspace-labelled runs (via the existing Kafka bus or a Hatchet event hook, VERIFY). The operator maps the event to the workspace UID and enqueues a reconcile.
- **Periodic requeue (safety net):** while a workspace has `MissionAccepted=False, reason=MissionActive`, the operator requeues it at a fixed interval (proposal: 60 s, OPEN). This bounds the extra wait for a deferred `missionRef` to one interval, even if completion events are lost.

**9.1.8 Conditions. DECIDED (Flux integration VERIFY).** Conditions follow Kubernetes conventions (`type`, `status`, `reason`, `message`, `lastTransitionTime`, `observedGeneration`) and are written **only when they change** (see 9.3).

| Condition | Status / Reason | Meaning | Flux health |
| :--- | :--- | :--- | :--- |
| `MissionAccepted` | `True` / `Dispatched` | Current `missionRef` was dispatched | Healthy |
| | `False` / `MissionActive` | Deferred until the active mission ends | In progress |
| | `False` / `AlreadyExecuted` | This `missionRef` already ran under this UID (e.g. after a `git revert`); it will not run again. Use a new value to re-run. | **Failed** |
| | `Unknown` / `OrchestratorUnavailable` | Hatchet unreachable; acceptance cannot be decided | In progress |
| `Synced` | `True` / `Current` | `status` reflects Hatchet | Healthy |
| | `False` / `OrchestratorUnavailable` | Hatchet unreachable; `phase` and mission fields may be stale | In progress |
| `FinalizationBlocked` | `True` / `OrchestratorUnavailable` · `SecretsUnavailable` · `CancelTimeout` | Deletion is waiting (see 9.4) | — |

*Hysteresis:* `Synced` and `OrchestratorUnavailable` flip only after the failure persists for a minimum window (proposal: 3 consecutive failed probes over ≥ 60 s, OPEN), so transient errors do not cause writes.

*VERIFY:* Flux Kustomization `healthCheckExprs` (CEL expressions for `current`, `inProgress`, `failed`) is available in the deployed Flux version, and is configured so that `AlreadyExecuted` is reported as failed and `MissionActive` / `OrchestratorUnavailable` as in progress. If it is unavailable, fall back to a summary `Ready` condition that kstatus understands.

**9.1.9 Status printer columns. DECIDED.** `kubectl get agws` shows `PHASE`, `MISSION` (`activeMissionRef`), `ACCEPTED` (`MissionAccepted` reason), `SYNCED`, `RUN` (Hatchet run ID), and `AGE`. This supports the triage KPI without exposing micro-step state.

### 9.2 Scope of the 30Mi / 250m Bound

**9.2.1 Per container, all container kinds. DECIDED.** For every entry in `containers` and `initContainers` (including restartable sidecar init containers), `requests` and `limits` for both `cpu` and `memory` **must be declared**, with `limits.memory ≤ 30Mi`, `limits.cpu ≤ 250m`, and `requests ≤ limits`.

**9.2.2 Ephemeral containers forbidden in sandbox namespaces. DECIDED.** Kubernetes does not allow resource requirements on ephemeral containers, so "declare limits" cannot apply to them. Admission denies `pods/ephemeralcontainers` in `development`.

**9.2.3 Mandatory declaration, no defaulting. DECIDED.** No `LimitRange` with `default` or `defaultRequest` in sandbox namespaces. Defaulting runs in mutating admission, before validation, so a manifest without limits would pass after being auto-filled. A `LimitRange` with `max` only is allowed as defence in depth.

**9.2.4 Aggregate per-pod cap. DECIDED (CEL feasibility VERIFY).** Cap: **30Mi / 250m** for the workload, excluding sandbox runtime overhead (9.2.5).

Kubernetes computes pod resources as `max(init-phase peak, steady state)`, where:

- steady state = Σ app containers + Σ sidecar init containers;
- init-phase peak = max over each non-sidecar init container *i* of (*i* + Σ sidecars started before *i*).

The exact init peak depends on declaration order. The policy uses the **conservative upper bound** max(non-sidecar init) + Σ all sidecars. This may reject a pod Kubernetes would accept, but it never accepts one that exceeds the cap.

*Mechanism:* CEL list `sum()` / `max()` over `map(...)` avoids the need for a fold (§9.5). Memory converts with `asInteger()`. CPU like `250m` is not an integer, so it is converted to integer millicores via `asApproximateFloat()`. *VERIFY on MicroK8s* before falling back to Kyverno or a webhook.

**9.2.5 gVisor overhead — (X) excluded. DECIDED.** The 30Mi / 250m bound applies to workload containers. Sandbox overhead is declared in `RuntimeClass.overhead`, applied by the RuntimeClass admission controller to `pod.spec.overhead`, counted by the scheduler and quota, and budgeted separately.

**Blocking plan task:** measure steady-state and peak RSS/CPU of the gVisor sandbox (runsc Sentry + Gofer) for a representative idle and active task, then set `RuntimeClass.overhead` from the measurement. No figure is asserted until measured.

**9.2.6 Enforcement points. DECIDED (versions VERIFY).**

| Target | Mechanism | Purpose |
| :--- | :--- | :--- |
| `AgentWorkspace.spec.resourceLimits` | CEL rules in the CRD schema | Independent of admission-policy availability |
| Pods in `development` (including operator-created) and `pods/ephemeralcontainers` | Validating admission policy (pods), `failurePolicy: Fail` | Catches manifest errors **and operator bugs** |
| Jobs in `development` | Separate validating admission policy (jobs) | Rejects a bad Job at creation instead of at each pod |

*VERIFY:* `ValidatingAdmissionPolicy` GA needs Kubernetes ≥ 1.30. CEL `quantity()` and the list `sum`/`max` functions must be available in the MicroK8s version. Otherwise use Kyverno. Mutating injectors (e.g. mesh sidecars) must not target `development`.

### 9.3 Status Write Budget *(replaces the etcd KPI in §6.B)*

**What counts.** A *status write* is any `PATCH` or `UPDATE` to `agentworkspaces/status`. Writes to the main resource (finalizer add/remove, spec edits) are counted separately and excluded from the ratio.

**Coalescing rules.**

- Phase, conditions, `observedGeneration`, `activeMissionRef`, `activeMissionCommit`, and the run ID are written **in the same patch** as the transition that causes them.
- `lastSyncTimestamp` is replaced by per-condition `lastTransitionTime` and is **never refreshed periodically**.
- No-op patches (computed status equal to current) are suppressed.

**Expected writes.**

| Source | Writes |
| :--- | :--- |
| Mission without human review: `Pending` → `Initializing` → `Running` → `Completed`/`Failed` | 4 |
| Each human-review cycle (`Running` → `Paused` → `Running`) | +2 |
| Deferred `missionRef` (`MissionAccepted=False/MissionActive`) | +1 |
| Spec edit not starting a mission (`observedGeneration` change) | +1 |
| Orchestrator outage episode (`Synced` False, then True) | +2 |

**KPI (revised, Board approval required).** Ratio = status writes / missions reaching a terminal phase, over a rolling 7-day window. Target **≤ 6**.

- Numerator: `apiserver_request_total{group="darkgravity.io", resource="agentworkspaces", subresource="status", verb=~"PATCH|UPDATE"}`.
- Denominator: from Hatchet terminal-run metrics.
- Supporting per-mission diagnostic: writes per mission ≤ 4 + 2·H (H = human-review cycles), checked in the e2e suite rather than from apiserver metrics, which cannot be attributed to a single mission.

### 9.4 Finalizer: Asynchronous Teardown *(refines §4.C)*

Cancelling a Hatchet run is not instantaneous. On `deletionTimestamp`:

1. **Cancel.** Request cancellation of all non-terminal runs labelled with the workspace UID, including queued runs (9.1.4).
2. **Await terminal state**, bounded by `T_cancel` (proposal: 5 min, OPEN). Re-check on requeue; do not block the reconcile loop.
3. **Revoke credentials.** Revoke Vault JIT tokens for the workspace **even if `T_cancel` expired**: a run that is still alive loses access.
4. **Delete sandboxes.** Delete Jobs and Pods in `development` labelled with the UID, using foreground propagation.
5. **Remove the finalizer** once revocation has succeeded and the sandboxes are gone.

**Failure handling.**

| Dependency down | Behaviour | Bound |
| :--- | :--- | :--- |
| Hatchet | Keep the finalizer; `FinalizationBlocked=True/OrchestratorUnavailable`; retry with exponential backoff (cap 5 min) | After `T_cancel`, continue with steps 3–5; the orphan sweep (9.5) terminates the run when Hatchet returns |
| Vault | Keep the finalizer; `FinalizationBlocked=True/SecretsUnavailable`; retry with backoff | Tokens have a short TTL (≤ `T_token`, OPEN). Once `T_token` has elapsed since the last issuance, tokens are expired by construction and the finalizer may proceed |
| MinIO | Not on the teardown path. Checkpoint retention is a bucket lifecycle policy, not a finalizer step | — |

**Manual override:** a documented runbook for removing the finalizer by hand, with an audit event, for cases outside these bounds.

### 9.5 Orphan Sweep *(refines §4.D and the KPI in §6.B)*

A periodic sweep (proposal: every 2 min, OPEN) lists non-terminal Hatchet runs labelled with a workspace UID and compares them with the existing `AgentWorkspace` UIDs. Orphans are cancelled and their tokens revoked.

**KPI (revised):** no orphaned run survives longer than **N minutes** (proposal: 10, OPEN), replacing "0 active leaks", which is not measurable without a sweep.

### 9.6 Example Rules (for discussion; not validated against the cluster)

**CRD schema (excerpt).** Strings are bounded (`maxLength`) to keep CEL cost estimates within budget, and quantities must be strictly positive (a negative value would otherwise satisfy `<= 30Mi`).

```yaml
spec:
  type: object
  required: [repositoryUrl, targetBranch, constitutionRef, resourceLimits]
  x-kubernetes-validations:
    - rule: "self.repositoryUrl == oldSelf.repositoryUrl"
      message: "repositoryUrl is immutable (ADR-008 §9.1.5)"
    - rule: "self.targetBranch == oldSelf.targetBranch"
      message: "targetBranch is immutable (ADR-008 §9.1.5)"
  properties:
    repositoryUrl:   { type: string, maxLength: 512 }
    targetBranch:    { type: string, maxLength: 255 }
    constitutionRef: { type: string, maxLength: 512 }
    missionRef:      { type: string, maxLength: 128 }
    resourceLimits:
      type: object
      required: [maxRam, maxCpu]
      properties:
        maxRam: { type: string, maxLength: 16 }
        maxCpu: { type: string, maxLength: 16 }
      x-kubernetes-validations:
        - rule: >-
            isQuantity(self.maxRam) &&
            quantity(self.maxRam).isGreaterThan(quantity('0')) &&
            quantity(self.maxRam).compareTo(quantity('30Mi')) <= 0
          message: "maxRam must be a quantity in (0, 30Mi]"
        - rule: >-
            isQuantity(self.maxCpu) &&
            quantity(self.maxCpu).isGreaterThan(quantity('0')) &&
            quantity(self.maxCpu).compareTo(quantity('250m')) <= 0
          message: "maxCpu must be a quantity in (0, 250m]"
```

**Pods policy.** Split from Jobs so each `podSpec` type-checks. Rules that read quantities are guarded by `declared`, so a missing field produces the declaration message rather than an evaluation error.

```yaml
apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicy
metadata:
  name: dg-sandbox-bounds-pods
spec:
  failurePolicy: Fail
  matchConstraints:
    resourceRules:
      - apiGroups: [""]
        apiVersions: ["v1"]
        operations: ["CREATE", "UPDATE"]
        resources: ["pods", "pods/ephemeralcontainers"]
  variables:
    - name: s
      expression: "object.spec"
    - name: inits
      expression: "has(variables.s.initContainers) ? variables.s.initContainers : []"
    - name: sidecars
      expression: "variables.inits.filter(c, has(c.restartPolicy) && c.restartPolicy == 'Always')"
    - name: plainInits
      expression: "variables.inits.filter(c, !has(c.restartPolicy) || c.restartPolicy != 'Always')"
    - name: all
      expression: "variables.s.containers + variables.inits"
    - name: declared
      expression: >-
        variables.all.all(c, has(c.resources) &&
          has(c.resources.requests) && has(c.resources.limits) &&
          ['cpu', 'memory'].all(r, r in c.resources.requests && r in c.resources.limits))
    # memory in bytes (int); cpu in millicores (int, rounded)
    - name: memApp
      expression: "([0] + variables.s.containers.map(c, quantity(c.resources.limits['memory']).asInteger())).sum()"
    - name: memSide
      expression: "([0] + variables.sidecars.map(c, quantity(c.resources.limits['memory']).asInteger())).sum()"
    - name: memInitMax
      expression: "([0] + variables.plainInits.map(c, quantity(c.resources.limits['memory']).asInteger())).max()"
    - name: cpuApp
      expression: "([0] + variables.s.containers.map(c, int(quantity(c.resources.limits['cpu']).asApproximateFloat() * 1000.0 + 0.5))).sum()"
    - name: cpuSide
      expression: "([0] + variables.sidecars.map(c, int(quantity(c.resources.limits['cpu']).asApproximateFloat() * 1000.0 + 0.5))).sum()"
    - name: cpuInitMax
      expression: "([0] + variables.plainInits.map(c, int(quantity(c.resources.limits['cpu']).asApproximateFloat() * 1000.0 + 0.5))).max()"
  validations:
    - expression: "!has(variables.s.ephemeralContainers) || size(variables.s.ephemeralContainers) == 0"
      message: "ephemeral containers are not allowed in sandbox namespaces (ADR-008 §9.2.2)"
    - expression: "variables.declared"
      message: "every container must declare cpu/memory requests and limits (ADR-008 §9.2.1)"
    - expression: >-
        !variables.declared || variables.all.all(c,
          quantity(c.resources.limits['memory']).compareTo(quantity('30Mi')) <= 0 &&
          quantity(c.resources.limits['cpu']).compareTo(quantity('250m')) <= 0 &&
          quantity(c.resources.requests['memory']).compareTo(quantity(c.resources.limits['memory'])) <= 0 &&
          quantity(c.resources.requests['cpu']).compareTo(quantity(c.resources.limits['cpu'])) <= 0)
      message: "container limits must be <= 30Mi / 250m and requests <= limits (ADR-008 §9.2.1)"
    - expression: >-
        !variables.declared ||
        (variables.memApp + variables.memSide <= 31457280 &&
         variables.memInitMax + variables.memSide <= 31457280)
      message: "pod workload memory (steady or init peak, upper bound) exceeds 30Mi (ADR-008 §9.2.4)"
    - expression: >-
        !variables.declared ||
        (variables.cpuApp + variables.cpuSide <= 250 &&
         variables.cpuInitMax + variables.cpuSide <= 250)
      message: "pod workload cpu (steady or init peak, upper bound) exceeds 250m (ADR-008 §9.2.4)"
---
apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicyBinding
metadata:
  name: dg-sandbox-bounds-pods
spec:
  policyName: dg-sandbox-bounds-pods
  validationActions: [Deny]
  matchResources:
    namespaceSelector:
      matchLabels:
        kubernetes.io/metadata.name: development
```

**Jobs policy.** `dg-sandbox-bounds-jobs` is identical except that it matches `batch/v1 jobs` and sets `variables.s` to `object.spec.template.spec`. The ephemeral-container rule is omitted, since pod templates cannot declare them.

*VERIFY:* `isGreaterThan`, list `sum()` and `max()`, and `asApproximateFloat()` are available. The `+ 0.5` rounding gives exact millicores for values declared at millicore precision.

### 9.7 Consequences for v3 Text

- §4.B: "on new spec generation" → "on a `spec.missionRef` with no Hatchet run under key `(uid, missionRef)`". Dispatch carries the snapshot of 9.1.6.
- §4.C: superseded by 9.4.
- §4.D: idempotency superseded by 9.1.4; orphan handling by 9.5.
- §6.A: scoped by 9.2.
- §6.B: the etcd KPI is replaced by 9.3, the orphan KPI by 9.5, and the triage KPI is supported by 9.1.9. The MTTR budget split (failure detection / rescheduling / sandbox start / checkpoint restore) is OPEN for the plan.
- §7 example: add `spec.missionRef`. Status becomes `phase`, `activeMissionRef`, `activeMissionCommit`, `hatchetWorkflowRunId`, `observedGeneration`, `conditions[]`; `lastSyncTimestamp` is removed.

### 9.8 Open and Verify Items

| ID | Item | Type | Blocks |
| :--- | :--- | :--- | :--- |
| 9.2.5 | Measure gVisor sandbox overhead; set `RuntimeClass.overhead` | OPEN | **Plan (first task)** |
| 9.1.4 | Hatchet concurrency strategy queues, never cancels or drops | VERIFY | Plan |
| 9.1.7 | Source of terminal-run events; requeue interval | VERIFY / OPEN | Plan |
| 9.1.8 | Flux `healthCheckExprs` availability; hysteresis window | VERIFY / OPEN | Plan |
| 9.2.4, 9.2.6, 9.6 | CEL functions and VAP GA on MicroK8s; Kyverno fallback | VERIFY | Plan |
| 9.4 | `T_cancel`, `T_token` | OPEN | Plan |
| 9.5 | Sweep interval, N | OPEN | Plan |
| 9.7 | MTTR 45 s budget split | OPEN | Plan |
| — | Event signature verification owner and key rotation | OPEN | Plan |
