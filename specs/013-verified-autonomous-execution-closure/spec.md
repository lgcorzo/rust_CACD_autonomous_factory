# Feature Specification: DG-RVP-013: Verified Autonomous Execution Closure

**Feature Branch**: `013-verified-autonomous-execution-closure`

**Created**: 2026-11-09

**Status**: Draft

**Input**: User description: "DG-RVP-013: Verified Autonomous Execution Closure - Close all remaining critical security, runtime, FinOps and evidence-governance gaps identified in the latest independent audit."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Unified Fail-Closed Execution Security Policy (Priority: P0)

As an SRE and Platform Security Engineer, I want the autonomous factory system to enforce a single, unified, fail-closed execution security policy (`ExecutionSecurityPolicy`) across all environments, so that untrusted code is never executed directly on the host and cannot bypass sandbox isolation through conflicting environment flags.

**Why this priority**: Preventing untrusted code execution on the host is the fundamental security boundary of the autonomous factory. Without a unified fail-closed policy, configuration conflicts (such as `USE_GVISOR=false` or `SANDBOX_MODE=subprocess`) could expose host infrastructure.

**Independent Test**: Can be independently verified by submitting untrusted workloads under conflicting, missing, or insecure configuration combinations (e.g. `USE_GVISOR=false`, `SANDBOX_MODE=subprocess`, unknown environment) and verifying that the system rejects execution with a typed security error and never falls back to host execution.

**Acceptance Scenarios**:

1. **Given** an untrusted mission task execution request, **When** the environment is configured with `USE_GVISOR=false` or `SANDBOX_MODE=subprocess`, **Then** the policy engine rejects the bypass and enforces gVisor isolation (or fails closed if gVisor is unavailable).
2. **Given** any workload in a production environment, **When** any host subprocess execution is requested, **Then** admission rejects the request unconditionally.
3. **Given** an unknown, invalid, or missing policy configuration, **When** any execution is requested, **Then** the policy evaluation fails closed immediately without scheduling any job or subprocess.

---

### User Story 2 - Real MicroK8s gVisor Runtime Repair & Execution (Priority: P0)

As a Kubernetes Runtime Security Engineer, I want the cluster's containerd runtime handler and MicroK8s configuration to properly resolve and execute the gVisor shim (`io.containerd.runsc.v1`), so that real untrusted code compiles and executes inside a gVisor-backed sandbox Pod, returns exit code 0, and captures truthful stdout/stderr.

**Why this priority**: A declared `RuntimeClass: gvisor` that fails containerd handler resolution or silently uses default runc is a false security assertion. Demonstrating real gVisor execution is non-negotiable for zero-trust claims.

**Independent Test**: Can be independently verified by deploying a disposable sandbox workload to MicroK8s using `runtimeClassName: gvisor`, verifying execution in the gVisor sandbox kernel, and asserting exit code, stdout/stderr propagation, and resource cleanup.

**Acceptance Scenarios**:

1. **Given** a Kubernetes Job using `runtimeClassName: gvisor`, **When** the Pod is scheduled on the MicroK8s node, **Then** the container runtime launches under the `io.containerd.runsc.v1` handler without timing out or failing containerd unpack validation.
2. **Given** a compiling and passing test workload, **When** executed inside the gVisor sandbox, **Then** it completes normally with exit code 0 and accurate stdout logs.
3. **Given** a workload that violates security policy (e.g., attempts network egress, host file access, or exceeds memory/time deadlines), **When** executed, **Then** the gVisor sandbox enforces the constraint, terminates the job appropriately, and records a clean failure.

---

### User Story 3 - Observed Fact-Based Runtime Attestation (Priority: P0)

As an Independent Verification Engineer, I want execution attestations to reflect independently observed runtime facts (actual node, Pod UID, container ID, container image digest, runtime handler) rather than echoing static configuration, so that audit records cannot be spoofed by test doubles or configuration defaults.

**Why this priority**: Misrepresenting configuration as observed runtime reality undermines the trustworthiness of audit reports and public claims.

**Independent Test**: Can be independently verified by comparing the generated attestation payload against Kubernetes API query results for the target Pod and ensuring simulated/mock runs cannot produce a `LIVE_RUNTIME` verified attestation.

**Acceptance Scenarios**:

1. **Given** a completed sandbox execution, **When** the attestation record is produced, **Then** it populates node identifier, Pod UID, container ID, image digest, runtime handler, and exit code directly from observed cluster state.
2. **Given** an unobserved or simulated execution (such as an in-memory test double), **When** evaluated by the runtime evidence verifier, **Then** it is marked as unobserved/mock and rejected for `LIVE_RUNTIME` certification.
3. **Given** any discrepancy between requested policy and observed container execution (e.g., digest mismatch or missing runtime handler), **When** verified, **Then** the verification gate marks the execution as failed.

---

### User Story 4 - FinOps Concurrency & Distributed Consistency (Priority: P1)

As a FinOps and SRE Engineer, I want budget checks and the hard-stop latch to operate with atomic, versioned consistency across concurrent processes and nodes, so that concurrent mission submissions cannot overspend or bypass budget limits.

**Why this priority**: Budget enforcement must be strictly atomic; concurrent worker processes must observe consistent lock targets and respect budget ceilings without race conditions.

**Independent Test**: Can be independently verified by launching concurrent admission requests against a nearly exhausted budget and verifying that once the budget is depleted or the latch is set, no further workload is admitted.

**Acceptance Scenarios**:

1. **Given** multiple worker instances evaluating mission admission simultaneously, **When** budget state is updated, **Then** all reads and writes synchronize on the exact same lock target with versioned state updates.
2. **Given** an active hard-stop latch or an exhausted budget, **When** concurrent admission requests arrive, **Then** all requests are rejected fail-closed.
3. **Given** a storage failure or corrupted state file, **When** admission is requested, **Then** the FinOps controller fails closed and denies execution.

---

### User Story 5 - Evidence Governance & Public Claims Alignment (Priority: P1)

As a Compliance and Verification Auditor, I want the Evidence Gate state machine (`OPEN` -> `IMPLEMENTED` -> `TESTED` -> `VERIFIED`) to enforce strict commit linkage, rejection of stale/mocked evidence for runtime claims, and synchronization with the public portfolio website, so that public claims never overstate verified status.

**Why this priority**: Public transparency and truthful claim verification are vital to the integrity of Dark Gravity. Any divergence between actual cluster verification and published claims is unacceptable.

**Independent Test**: Can be independently verified by running `scripts/check_public_claims.py` and `tests/test_public_claims.py` against the evidence ledger, ensuring that claims marked `verified` have matching fresh evidence tied to the exact Git commit SHA.

**Acceptance Scenarios**:

1. **Given** an evidence record linked to an older Git commit SHA, **When** evaluated against the current repository state, **Then** the evidence gate marks the requirement as needing re-verification.
2. **Given** a public claim on `lgcorzo.github.io` regarding sandbox isolation or runtime security, **When** validated by the claims checker, **Then** the status reflects actual verified evidence (`designed`, `implemented`, `tested`, `deployed`, or `runtime_verified`).
3. **Given** a change to source code, **When** the Evidence Gate runs, **Then** it updates the evidence ledger with exact SHA-256 hashes and commit references.

---

### Edge Cases

- What happens when containerd or the MicroK8s host service is restarted while worker pods are running? (Worker pods must reconnect gracefully to the Hatchet gRPC broker and resume task polling without deadlocking).
- How does the system handle transient Sentry 429 rate limit errors or unavailable LiteLLM endpoints during mission execution? (Agents must log warnings, avoid crash-looping, and continue with safe defaults or graceful degradation).
- What happens if a sandbox container crashes before writing standard output? (The MCP tool must detect the container exit code, pull Kubernetes termination logs, report failure, and prevent misleading success attestations).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a unified `ExecutionSecurityPolicy` evaluating execution environment, workload trust level, required isolation, and admission eligibility before any code execution.
- **FR-002**: System MUST reject any attempt to execute untrusted code directly on the host across all environments (including development).
- **FR-003**: System MUST configure and verify the MicroK8s containerd runtime handler for gVisor (`io.containerd.runsc.v1`) to run real workloads with dropped capabilities and non-root execution.
- **FR-004**: System MUST capture truthful, independently observed runtime facts in execution attestations (Pod UID, container ID, image digest, runtime handler, exit code) and reject mock/synthetic data for live runtime verification.
- **FR-005**: System MUST enforce atomic, versioned consistency on FinOps budget storage and latch targets across all reading and writing processes, failing closed on storage errors.
- **FR-006**: System MUST enforce the verification evidence lifecycle (`OPEN` -> `IMPLEMENTED` -> `TESTED` -> `VERIFIED`), binding evidence records to the exact repository HEAD SHA.
- **FR-007**: System MUST validate that declarative GitOps manifests in `gitops_internal_lgcorzo` match runtime sandbox specifications and reflect the active cluster state.
- **FR-008**: System MUST ensure public claims on `lgcorzo.github.io` strictly mirror verified evidence status and never claim runtime verification without live evidence.
- **FR-009**: Supporting repositories (`mlops-python-package`, `fastapi-autogen-team`) MUST maintain consistent security claims, clean CI results, and precise documentation without unsupported absolute statements.

### Key Entities

- **ExecutionSecurityPolicy**: Central policy entity dictating permitted sandbox drivers, trust levels, runtime class requirements, and fail-closed evaluation rules.
- **RuntimeAttestation**: Verifiable evidence document recording observed cluster metadata, container digests, runtime handlers, timestamps, and cryptographic hashes of executed commands and outputs.
- **FinOpsBudgetState**: Versioned financial ledger tracking spend, limits, velocity, latch states, and reservation status under mutual exclusion.
- **EvidenceRecord**: Structured audit artifact containing commit SHA, requirement ID, test classification, command logs, exit code, and verification identity.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of untrusted execution requests are governed by `ExecutionSecurityPolicy` with 0 reachable paths to uncontained host subprocess execution.
- **SC-002**: A real Rust test workload completes inside a gVisor-backed Kubernetes Pod in MicroK8s with exit code 0, returning valid attestation with `io.containerd.runsc.v1` and dropped capabilities.
- **SC-003**: 100% of concurrent FinOps admission tests under budget exhaustion reject over-admission without race conditions or lock inconsistency.
- **SC-004**: All 24+ evidence records in the evidence ledger are validated against the exact repository HEAD SHA with 0 schema or hash discrepancies.
- **SC-005**: `scripts/check_public_claims.py` and website test suites pass with 100% claim consistency between `data/canonical-claims.json`, site HTML, and the evidence ledger.

## Assumptions

- The target MicroK8s cluster is reachable on `server.internal.lgcorzo` with administrative access for containerd and Kubernetes inspection.
- The `gvisork8s` driver in `crates/factory-mcp-server` communicates with the local Kubernetes API via the cluster service account or standard kubeconfig.
- Autonomous factory workers operate in namespace `agents` and launch ephemeral sandboxes in namespace `factory-sandboxes`.
- Public portfolio changes are published via `lgcorzo.github.io` on GitHub Pages subject to human-in-the-loop merge review.

