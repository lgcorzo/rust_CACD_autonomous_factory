# Implementation Plan: DG-RVP-013: Verified Autonomous Execution Closure

**Branch**: `013-verified-autonomous-execution-closure` | **Date**: 2026-11-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/013-verified-autonomous-execution-closure/spec.md`

## Summary

Remediate and close all remaining critical security, runtime, FinOps, and evidence-governance gaps identified in the DG-RVP-012/013 audits.
The engineering approach:
1. Establish a single unified `ExecutionSecurityPolicy` in `crates/factory-core` and enforce it strictly across all execution entry points in `crates/factory-mcp-server`, eliminating all unsafe host fallbacks.
2. Correct the MicroK8s containerd configuration to properly bind the `runsc` handler to `io.containerd.runsc.v1`, validating real gVisor pod execution and resource isolation.
3. Transform `SandboxAttestation` to record truthful, independently observed cluster facts (Pod UID, container ID, image digest, runtime handler) queried directly from the Kubernetes API.
4. Align FinOps storage locking to use consistent `.lock` targets for reads and writes, adding atomic transactional budget reservations with fail-closed semantics.
5. Synchronize GitOps manifests, validate evidence records against the current commit SHA, and align public portfolio claims on `lgcorzo.github.io`.

## Technical Context

**Language/Version**: Rust 1.85.1 (2024 edition) / Python 3.12  
**Primary Dependencies**: `kube 5.0.0`, `k8s-openapi 0.29.0`, `axum 0.8`, `tokio 1.0`, `fs2 0.4`, `serde_json`, `chrono`  
**Storage**: Kubernetes API (`Job`/`Pod` status), MinIO S3, Local atomic JSON stop store with fs2 mutual exclusion  
**Testing**: `cargo test --workspace` (with `-j 1`), `pytest`, live disposable Kubernetes sandbox probes  
**Target Platform**: MicroK8s Kubernetes on Linux (`server.internal.lgcorzo`), containerd 2.4.2, gVisor (`runsc` / `containerd-shim-runsc-v1`)  
**Project Type**: Autonomous Software Development Factory (Multi-Crate Rust Workspace)  
**Performance Goals**: Sandbox spin-up & test execution < 15s; FinOps lock acquisition < 100ms; zero token waste  
**Constraints**: Fail-closed security; dropped capabilities `["ALL"]`; `runAsNonRoot: true` (`65534`); read-only root filesystems; strict HITL merge policy  
**Scale/Scope**: 4 repositories (`rust_CACD_autonomous_factory`, `gitops_internal_lgcorzo`, `lgcorzo.github.io`, `fastapi-autogen-team`)

## Constitution Check

- [x] **Test-First**: Unit and negative security tests authored before modifying execution and storage paths.
- [x] **Library-First**: Policy logic encapsulated in `factory-core::security_policy` with clear typed contracts.
- [x] **Observability**: Attestations capture observed runtime telemetry without synthetic mocks.
- [x] **Governance**: Strict separation of duties, Human-In-The-Loop merge policy upheld.

## Project Structure

### Documentation (this feature)

```text
specs/013-verified-autonomous-execution-closure/
├── spec.md                   # Feature specification
├── plan.md                   # This implementation plan
├── research.md               # Phase 0 research findings
├── data-model.md             # Phase 1 data models & state diagrams
├── contracts/                # Phase 1 interface contracts
│   └── execution-contracts.md
├── quickstart.md             # Phase 1 validation runbook
├── verification-matrix.md    # Control-to-test mapping matrix
└── tasks.md                  # Phase 2 implementation task list (generated via /speckit-tasks)
```

### Source Code Impact

```text
crates/
├── factory-core/
│   └── src/
│       ├── lib.rs
│       └── security_policy.rs        # NEW: Unified ExecutionSecurityPolicy & violations
├── factory-mcp-server/
│   └── src/
│       ├── sandbox.rs                # Update: Integrate policy & observed attestation
│       └── tools/
│           └── launch_sandbox_pod.rs # Update: Strict gVisor enforcement & Pod inspection
├── factory-infrastructure/
│   └── src/
│       └── finops_stop_store.rs      # Update: Unified lock targets & atomic reserve
└── factory-cli/
    └── src/
        └── main.rs
```

## Phases

- **Phase 0 (Research)**: Completed. Evaluated containerd shim binding, lock inconsistency, and policy models.
- **Phase 1 (Design & Contracts)**: Completed. Authored `data-model.md`, `contracts/execution-contracts.md`, `quickstart.md`, and `verification-matrix.md`.
- **Phase 2 (Implementation & Tasks)**: Execution of task breakdown in `tasks.md` via `/speckit-tasks` and `/speckit-implement`.
