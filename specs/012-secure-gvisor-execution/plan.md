# Implementation Plan: Secure gVisor Execution & Evidence Governance

**Branch**: `012-secure-gvisor-execution` | **Date**: 2026-11-08 | **Spec**: [specs/012-secure-gvisor-execution/spec.md](specs/012-secure-gvisor-execution/spec.md)

**Input**: Feature specification from `specs/012-secure-gvisor-execution/spec.md`

## Summary

Remediate the 5 key audit findings from the 2026-11-07 review:
1. **P0**: Remove `SubprocessDriver` as an implicit default for untrusted code execution; enforce sandboxed execution (`GvisorK8sDriver`) across all environments with strict fail-closed security policies.
2. **P0**: Verify and guarantee the containerd `runsc` shim / `gvisor` RuntimeClass end-to-end on the Kubernetes node, ensuring real Rust compilation and testing pass in sandbox pods with 512Mi RAM.
3. **P1**: Upgrade FinOps stop storage from single-instance in-memory Mutex to multi-process atomic persistent locking across distributed worker replicas.
4. **P1**: Enforce commit SHA freshness and reject synthetic mock attestations in `factory-cli validate-evidence`, regenerating evidence for the current `HEAD`.
5. **P2**: Synchronize `canonical-claims.json`, public documentation, and audit verification dates.

## Technical Context

**Language/Version**: Rust 1.80+ (2021 Edition)  
**Primary Dependencies**: `tokio`, `axum`, `kube`, `k8s-openapi`, `serde`, `serde_json`, `fs2` (or file locking primitives), `sha2`  
**Storage**: JSON-backed state store with atomic file locking (`fs2`) or Kubernetes shared volumes  
**Testing**: `cargo test -p factory-mcp-server`, `cargo test -p factory-application`, `cargo test -p factory-cli`  
**Target Platform**: Linux (Ubuntu 22.04 LTS x86_64), Kubernetes (MicroK8s v1.31 with containerd + gVisor `runsc`)  
**Project Type**: Microservices architecture with CLI, MCP server (`factory-mcp-server`), background workers (`factory-worker`), and orchestration workflows (`Hatchet`)  
**Performance Goals**: Sandbox container boot < 10s, compilation < 20s, zero OOM kill on standard library tests  
**Constraints**: Single compilation jobs restricted to `cargo -j 1` on local host; memory limit >= 512Mi on sandbox pods; zero tolerance for host execution of untrusted code  
**Scale/Scope**: 5 core audit workstreams touching `factory-mcp-server`, `factory-application`, `factory-cli`, and documentation  

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **I. Library-First & Modularity**: PASS. Changes isolated within existing crates (`factory-mcp-server`, `factory-application`, `factory-cli`).
- **II. CLI Interface**: PASS. Evidence validation and execution verification accessible via `factory-cli validate-evidence`.
- **III. Test-First & Verification**: PASS. TDD unit tests and integration tests planned for each component.
- **IV. Zero-Trust Security**: PASS. Removes insecure subprocess fallback for untrusted code execution.

## Project Structure

### Documentation (this feature)

```text
specs/012-secure-gvisor-execution/
├── spec.md              # Feature specification
├── plan.md              # This file (/speckit-plan output)
├── research.md          # Research findings & architectural decisions
├── data-model.md        # Entities & state transitions
├── quickstart.md        # Validation scenarios & commands
├── contracts/           # API and CLI contracts
│   └── sandbox-and-evidence-contracts.md
└── checklists/
    └── requirements.md
```

### Source Code Layout

```text
crates/
├── factory-mcp-server/
│   ├── src/
│   │   ├── lib.rs                   # Driver instantiation & zero-trust default selection
│   │   ├── sandbox.rs               # Hardened SandboxDriver trait & Subprocess fail-closed logic
│   │   └── tools/
│   │       ├── launch_sandbox_pod.rs # gVisor RuntimeClass, 512Mi limits, securityContext
│   │       └── run_tests.rs          # Test runner command sanitization & attestation emission
│   └── tests/
│       └── sandbox_driver_tests.rs  # Zero-trust enforcement tests
├── factory-application/
│   ├── src/
│   │   └── agents/
│   │       └── finops.rs            # Atomic multi-process FinOps stop store
│   └── tests/
│       └── finops_admission_tests.rs
├── factory-cli/
│   ├── src/
│   │   ├── evidence.rs              # SHA freshness check & non-mock attestation gate
│   │   └── main.rs
│   └── tests/
│       └── evidence_tests.rs
└── docs/
    └── audit/
        ├── VERIFICATION_EVIDENCE.json # Fresh evidence bound to HEAD
        └── VERIFICATION_EVIDENCE_REPORT.md
```

## Phase 0: Outline & Research
- Completed. Documented in `specs/012-secure-gvisor-execution/research.md`.

## Phase 1: Design & Contracts
- Completed. Documented in:
  - `data-model.md`
  - `contracts/sandbox-and-evidence-contracts.md`
  - `quickstart.md`
- Agent context (`AGENTS.md`) updated.
