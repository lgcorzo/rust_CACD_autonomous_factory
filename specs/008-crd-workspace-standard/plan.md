# Implementation Plan: Google AX CRD Workspace Standard (ADR-008 v3.1)

**Branch**: `008-crd-workspace-standard` | **Date**: 2026-10-31 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/008-crd-workspace-standard/spec.md` and [ADR-008 v3.1](../../wiki/architecture/decisions/adr-008-google-ax-crd-workspace-standard-v3.1.md)

---

## Summary

Implement the Kubernetes-native declarative management layer for autonomous agent workspaces defined in ADR-008 v3.1. This establishes `AgentWorkspace` (`darkgravity.io/v1alpha1`) as the sole persistent CRD, rejecting raw Google AX runtime code while adopting its conceptual separation of concerns. Atomic tasks execute as ephemeral jobs or Hatchet DAG step functions without etcd write churn. Control-plane reconciliation is implemented natively in Rust (`factory-cli operator`) using `kube-rs` with leader election and one-way status projection, backed by Kubernetes `ValidatingAdmissionPolicy` enforcing hard resource ceilings ($\le 30\text{Mi RAM}$, $\le 250\text{m CPU}$).

---

## Technical Context

**Language/Version**: Rust 2024 Edition (v1.85+)
**Primary Dependencies**:
- `kube` (0.98), `k8s-openapi` (0.24) with features `v1_30`
- `tokio` (1.43), `tracing` (0.1), `clap` (4.5)
- `hatchet-sdk` (0.1)
- `schemars` (0.8), `serde` (1.0), `serde_json` (1.0)
**Storage**:
- Hatchet Engine / PostgreSQL 16 (CNPG) as Absolute SSOT
- MinIO S3 (`dg-factory-checkpoints`) for `BridgeState` and `StepCheckpoint` persistence
**Testing**: `cargo test`, `wiremock`, `k8s-openapi` mock admission suite, integration tests in `crates/factory-cli`
**Target Platform**: Kubernetes / MicroK8s (v1.30+) Linux x86_64 with gVisor (`runsc`)
**Project Type**: Multi-crate Rust Workspace (CLI, Operator, Domain Application)
**Performance Goals**:
- MTTR $< 45\text{ s}$ on worker crash/recovery
- etcd write rate ratio $\le 6$ patches per mission lifecycle
- SRE triage speed $< 2\text{ min}$ via `kubectl get agws`
**Constraints**:
- Hard memory bound: Workload containers $\le 30\text{Mi RAM}$, $\le 250\text{m CPU}$
- Ephemeral containers prohibited in sandbox namespace (`development`)
- At most 1 active mission per workspace; idempotency key `(uid, missionRef)`

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I: Library-First**:
  - CRD domain types and reconciler business logic reside in reusable modules (`crates/factory-application` and `crates/factory-cli`).
  - Pass: Core abstractions are decoupled and testable without a live cluster.
- **Principle II: CLI Interface**:
  - The operator is exposed via `factory-cli operator --leader-election`.
  - Pass: Standardized CLI flags, structured logging (JSON/tracing), exit codes.
- **Principle III: Test-First (TDD)**:
  - Unit tests for CEL compilation, admission evaluation, and reconciler state transitions precede live cluster wiring.
  - Pass: Mocked reconciler tests will verify all phase transitions.
- **Principle IV: Integration Testing**:
  - E2E tests verify the complete loop: Manifest submission $\rightarrow$ Admission validation $\rightarrow$ Operator dispatch $\rightarrow$ Finalizer teardown.
  - Pass: Verification suite scripted in `quickstart.md`.
- **Principle V: Observability & Simplicity**:
  - `additionalPrinterColumns` expose essential status without exposing micro-step internals.
  - Pass: Standard `kubectl get agws` meets SRE diagnostic KPI.

---

## Project Structure

### Documentation (this feature)

```text
specs/008-crd-workspace-standard/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   ├── crd-agentworkspace.yaml
│   └── validating-admission-policies.yaml
├── checklists/
│   └── requirements.md  # Spec quality checklist
└── spec.md              # Feature specification
```

### Source Code Layout

```text
crates/factory-cli/
├── Cargo.toml
└── src/
    ├── main.rs                      # Adds `operator` subcommand
    └── operator/                    # Dedicated Kubernetes Operator module
        ├── mod.rs                   # Reconciler loop and leader election
        ├── crd.rs                   # AgentWorkspace CRD definition & schema generation
        ├── controller.rs            # kube::runtime::Controller implementation
        ├── finalizer.rs             # darkgravity.io/workspace-finalizer teardown logic
        ├── hatchet_sync.rs          # Hatchet gRPC client & SSOT projection mapping
        └── admission_test.rs        # In-memory CEL and schema assertion tests

crates/factory-application/
├── Cargo.toml
└── src/
    └── workspaces/                  # Shared domain contracts for workspace lifecycle
        ├── mod.rs
        └── events.rs                # Kafka mission dispatch schemas

deploy/crds/
└── agentworkspaces.darkgravity.io.yaml  # Generated CRD artifact for GitOps

deploy/policies/
└── validating-admission-policies.yaml   # Hard resource bounds enforcement
```

---

## Phases & Execution Strategy

### Phase 0: Outline & Research *(Completed)*
- Documented in `research.md`.
- Evaluated and settled Option (X) for gVisor overhead, separate VAP policies for Pods/Jobs, Hatchet `(uid, missionRef)` concurrency, and finalizer asynchronous bounded teardown.

### Phase 1: Design & Contracts *(Completed)*
- Documented in `data-model.md`, `contracts/`, and `quickstart.md`.
- Derived CRD manifest with CEL validation rules and admission policy contracts.

### Phase 2: Implementation Sequencing (for `tasks.md`)
1. **Task 1: Benchmark gVisor Overhead (Blocking)**:
   - Run diagnostic pod in gVisor to measure Sentry/Gofer memory consumption and set `RuntimeClass.overhead`.
2. **Task 2: Define CRD Rust Structs**:
   - Implement `AgentWorkspace` and `AgentWorkspaceSpec` in `crates/factory-cli/src/operator/crd.rs`.
3. **Task 3: Implement Reconciler Loop**:
   - Implement `kube::runtime::Controller` watching `AgentWorkspace` objects, handling Ed25519 dispatch, and throttled status projection.
4. **Task 4: Implement Deletion Finalizer**:
   - Implement `darkgravity.io/workspace-finalizer` asynchronous teardown sequence (cancel runs, revoke tokens, delete sandboxes).
5. **Task 5: Deploy ValidatingAdmissionPolicies**:
   - Apply CEL validation rules to test cluster and verify admission rejection suite.
6. **Task 6: Verification & SRE Telemetry**:
   - Execute test scenarios from `quickstart.md`, verifying MTTR $< 45\text{ s}$ and etcd write count $\le 6$ per mission.
