# Phase 0: Research & Technical Resolutions

**Feature**: `008-crd-workspace-standard` (Google AX CRD Workspace Standard - ADR-008 v3.1)
**Date**: 2026-10-31

This document consolidates findings, technical decisions, and validation strategies for resolving all items marked OPEN, VERIFY, and deferred in ADR-008 v3.1 §9.

---

## 1. gVisor Sandbox Overhead & Resource Bounds (ADR §9.2.5)

### Context & Problem
ADR-008 v3.1 sets a strict workload resource ceiling of **30Mi RAM / 250m CPU** for agent execution containers. The Lead System Architect selected Option (X): the 30Mi / 250m limit applies strictly to the workload containers, while sandbox runtime overhead (`runsc` Sentry + Gofer) is budgeted separately via Kubernetes `RuntimeClass.overhead` and applied to `pod.spec.overhead`.

### Findings
- **`runsc` Overhead Mechanics**: When a pod uses `runtimeClassName: gvisor`, the container runtime allocates a sandbox process (Sentry) and a virtual filesystem Gofer. In Linux x86_64 environments, an idle `runsc` sandbox consumes ~15–25 MiB RSS baseline memory, depending on buffer configurations, plus minimal CPU (~5–15m).
- **Measurement Protocol**: A diagnostic task executes a bare-metal/MicroK8s test using a baseline gVisor pod running an idle loop and tool execution.
- **Budget Decision**:
  - Workload Container Limit: `memory: 30Mi`, `cpu: 250m` (enforced via CEL & admission).
  - `RuntimeClass.overhead`: Set to `memory: 35Mi`, `cpu: 100m` (initial conservative baseline pending cluster telemetry).
  - Total Pod Footprint (Pod Scheduled Request): `65Mi RAM`, `350m CPU`.

---

## 2. Kubernetes Admission & CEL Evaluation (ADR §9.2.4, §9.2.6, §9.6)

### Context & Problem
We need to validate both CRD schemas and Pod/Job manifests at the Kubernetes API admission boundary using `ValidatingAdmissionPolicy` (VAP) and CRD OpenAPI v3 `x-kubernetes-validations` CEL rules.

### Findings
- **Kubernetes Version**: Modern MicroK8s (v1.30+) includes `ValidatingAdmissionPolicy` as GA.
- **CEL Quantities & Arithmetic**:
  - In CRDs (`x-kubernetes-validations`): `isQuantity(self.maxRam)` and `quantity(self.maxRam).compareTo(quantity('30Mi')) <= 0` are fully supported since k8s 1.29+. String lengths MUST be bounded (`maxLength: 16`) to satisfy CEL compilation cost budgets.
  - In VAP: Memory integer conversion `quantity(...).asInteger()` yields exact bytes (e.g., `30Mi = 31457280`). CPU represented as millicores (e.g., `250m`) is converted using `int(quantity(c.resources.limits['cpu']).asApproximateFloat() * 1000.0 + 0.5)` to provide exact millicore comparisons without float drift.
- **Policy Separation**: Two separate VAP definitions (`dg-sandbox-bounds-pods` and `dg-sandbox-bounds-jobs`) prevent type-checking errors (`spec.template.spec` vs `spec`) on admission review.
- **Ephemeral Containers**: Explicitly denied by checking `!has(variables.s.ephemeralContainers) || size(variables.s.ephemeralContainers) == 0`.

---

## 3. Hatchet Engine Concurrency, SSOT & Deduplication (ADR §9.1.3, §9.1.4, §9.1.7)

### Context & Problem
Guarantee that a workspace executes at most one active mission at a time, that every `(workspace_uid, mission_ref)` runs at most once, and that deferred mission requests are automatically resumed upon mission completion.

### Findings
- **Deduplication Key**: In Hatchet SDK, workflow run dispatch supports an `idempotency_key`. We set `idempotency_key = format!("{}:{}", workspace_uid, mission_ref)`. Any subsequent dispatch with the identical key returns the existing workflow run rather than launching a duplicate.
- **Workspace Concurrency Limiter**: Hatchet provides workflow concurrency control. We assign a concurrency key based on `workspace_uid` with `max_runs: 1` and `limit_strategy: ConcurrencyLimitStrategy::Queue`. This ensures that if a second mission is queued, it does not cancel or drop the in-flight run.
- **Completion Triggers**:
  - Event-Driven: Hatchet publishes workflow terminal events (`COMPLETED`, `FAILED`, `CANCELLED`) to Kafka topic `mission-status` or gRPC stream.
  - Level-Triggered Poller: Operator runs a background reconciliation loop with a 60-second tick interval to catch missed events.

---

## 4. GitOps FluxCD Integration & Status Projection (ADR §9.1.8, §9.3)

### Context & Problem
Expose workspace state clearly for GitOps (`kubectl get agws`) while keeping etcd write rate throttled to $\le 6$ patches per mission.

### Findings
- **Fields in `AgentWorkspace.status`**:
  - `phase`: `Pending` | `Initializing` | `Running` | `Paused` | `Completed` | `Failed`
  - `activeMissionRef`: string
  - `activeMissionCommit`: 40-char git commit SHA
  - `hatchetWorkflowRunId`: string
  - `observedGeneration`: integer
  - `conditions`: List of `metav1::Condition` with types `MissionAccepted`, `Synced`, `FinalizationBlocked`
- **FluxCD Health Check**: Flux Kustomizations evaluate `healthChecks` using standard kstatus rules or `healthCheckExprs`:
  - `MissionAccepted=False, reason=AlreadyExecuted` $\rightarrow$ Failed / Unhealthy.
  - `MissionAccepted=False, reason=MissionActive` $\rightarrow$ In Progress.
  - `Synced=True, phase=Completed` $\rightarrow$ Current / Ready.
- **Write Coalescing**: The operator accumulates changes in-memory during a reconciliation pass and issues a single JSON Merge Patch to `/status` only if calculated fields differ from the remote object. Periodic heartbeats are forbidden.

---

## 5. Teardown & Finalizer Asynchrony (ADR §9.4, §9.5)

### Context & Problem
Prevent hung finalizers during deletions when external systems (Vault, Hatchet) experience downtime or latency.

### Findings
- **Teardown Sequence**:
  1. Operator receives object with `metadata.deletionTimestamp != null`.
  2. Issues `HatchetClient::cancel_workflow(active_run_id)`.
  3. Waits up to `T_cancel = 300s` for terminal state.
  4. Revokes Vault dynamic tokens for repo path. If Vault is down, checks token TTL (`T_token = 3600s`); if expired, proceeds safely.
  5. Deletes sandbox Pods/Jobs in `development` matching label `darkgravity.io/workspace-uid`.
  6. Removes finalizer string `darkgravity.io/workspace-finalizer`.
- **Orphan Sweeper**: Operator background task runs every 120s, querying active Hatchet runs matching label `darkgravity.io/workspace-uid` and cross-referencing live `AgentWorkspace` objects in Kubernetes. Unmatched runs $> 600s$ old are terminated.

---

## 6. Implementation Architecture in Rust (`crates/factory-cli`)

### Context & Problem
Where should the operator code reside?

### Findings
- The repository already has `crates/factory-cli` (CLI entry point) and `crates/factory-application` (domain orchestration).
- `crates/factory-mcp-server` already depends on `kube` (0.98) and `k8s-openapi` (0.24).
- We implement the CRD types and operator in a new module in `factory-cli` (e.g., `crates/factory-cli/src/operator/`) exposing `factory-cli operator --leader-election`, sharing Kubernetes types and Hatchet clients from existing workspace crates.

