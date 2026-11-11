# Feature Specification: Secure gVisor Execution & Evidence Governance

**Feature Branch**: `012-secure-gvisor-execution`

**Created**: 2026-11-08

**Status**: Draft

**Input**: User description: "He encontrado actividad nueva importante: Dark Gravity ha avanzado bastante desde la última auditoría. El repositorio principal ya no está en el commit a894328; hoy, 7 de noviembre, aparecen una serie de cambios hasta el commit a71d000, incluido trabajo específico sobre ejecución nativa de Rust, imágenes de sandbox por lenguaje y bootstrap de proyectos Cargo... Mi conclusión: has hecho un salto importante en ingeniería de verificación, seguridad de APIs y fiabilidad de workflows. Pero todavía hay una contradicción crítica en el modelo de ejecución de código no confiable. No consideraría Dark Gravity listo para producción segura hasta resolverla... Prioridad 1 — P0: Eliminar el SubprocessDriver como opción implícita para código no confiable. Exigir explícitamente gVisor para todo código no confiable, también en desarrollo. Prioridad 2 — P0: Reparar el shim runsc de containerd en el nodo y ejecutar una prueba real que compile y ejecute código Rust dentro del Pod. Prioridad 3 — P1: Configurar FinOps con un almacenamiento realmente compartido y operaciones atómicas entre procesos. Prioridad 4 — P1: Regenerar toda la evidencia sobre el HEAD actual y demostrar que los registros anteriores quedan invalidados por obsolescencia. Prioridad 5 — P2: Sincronizar el registro canónico de claims con la evidencia de ejecución real y las fechas de auditoría. El siguiente hito: Verified Autonomous Execution: una misión completa, desde la generación del cambio hasta su aceptación, ejecutada en gVisor real, con presupuesto controlado y evidencia independiente vinculada al commit."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Zero-Trust Untrusted Execution Enforcement (Priority: P1)

As a security auditor and platform operator, I want all untrusted agent-generated code to strictly execute inside hardened, sandboxed gVisor runtimes across all environments (including development) so that host compromise or process escapes are completely eliminated and subprocess fallback is prohibited for untrusted code.

**Why this priority**: Resolves the open P0 blocker where `SubprocessDriver` was implicitly selected by default in development mode, violating the core zero-trust contract for untrusted code execution.

**Independent Test**: Can be independently verified by attempting to trigger code execution without gVisor or with fallback flags, confirming that untrusted code execution requests fail-safe with explicit security violations rather than falling back to host subprocesses.

**Acceptance Scenarios**:

1. **Given** an execution request with untrusted code, **When** no sandbox configuration is provided or `SANDBOX_MODE` is unset, **Then** the platform MUST reject host subprocess execution and fail-closed with a security policy error.
2. **Given** any development or testing environment, **When** untrusted test suites or compilation workloads execute, **Then** they MUST be assigned to an isolated sandbox pod enforcing runsc/gVisor runtime constraints, non-root UID/GID, read-only root filesystems, and dropped capabilities.
3. **Given** an environment where gVisor runtime cannot be initialized, **When** untrusted code execution is requested, **Then** the platform MUST refuse execution instead of silently degrading to host subprocesses.

---

### User Story 2 - Containerd Runsc Shim & Node Runtime Verification (Priority: P1)

As a cluster administrator, I want the Kubernetes containerd runtime shim on the worker node to correctly handle `runsc` (`io.containerd.runsc.v1` / `gvisor` runtimeClass) so that pods requesting sandbox isolation execute seamlessly end-to-end without runtime initialization failure.

**Why this priority**: Resolves audit defect AUD-010 where containerd misconfiguration prevented gVisor runtime pods from launching successfully on the cluster node.

**Independent Test**: Can be independently tested by deploying and executing a verification pod using `runtimeClassName: gvisor` that runs a self-contained Rust test and outputs validation evidence without crashing or encountering shim errors.

**Acceptance Scenarios**:

1. **Given** a Kubernetes node configured for sandboxing, **When** a pod specifying `runtimeClassName: gvisor` is scheduled, **Then** the pod starts under the `runsc` containerd shim.
2. **Given** an isolated Rust compilation workload, **When** submitted through the checked-in sandbox configuration, **Then** the documentation records the current conflict: the builder defaults to `512Mi`/`500m`, while the admission policy caps workload limits at `30Mi`/`250m`. The profile and RuntimeClass overhead sum to `85Mi`/`450m` limits and `60Mi`/`250m` requests; actual admission and runtime behavior are **TO CONFIRM**.

---

### User Story 3 - Multi-Process Atomic FinOps State Governance (Priority: P2)

As a financial operations owner and multi-worker orchestrator, I want FinOps budget stops and spend limits to be tracked via shared atomic storage across concurrent worker instances so that admission decisions prevent race conditions and enforce strict budget caps.

**Why this priority**: The existing FinOps implementation uses single-instance mutexes with process-local PID paths, risking split-brain budget counters or race conditions across multiple distributed worker pods.

**Independent Test**: Can be independently tested by executing concurrent budget decrement and admission requests across multiple worker processes against a shared state store, asserting strict monotonic spend tracking and zero unauthorized over-budget admissions.

**Acceptance Scenarios**:

1. **Given** multiple worker pods executing tasks in parallel, **When** budget allocations and spend tracking are evaluated, **Then** spend queries and stop conditions evaluate against a centralized/shared persistent store using atomic locking or CAS (compare-and-swap) semantics.
2. **Given** a hard budget limit threshold, **When** the cumulative spend reaches the limit, **Then** all worker instances immediately reject subsequent mission admissions.

---

### User Story 4 - Evidence Gate Freshness & Non-Mock Cryptographic Validation (Priority: P2)

As a compliance officer, I want verification evidence to be bound cryptographically to the current repository commit HEAD and require genuine sandbox execution logs so that stale or simulated evidence from previous commits is automatically invalidated.

**Why this priority**: The Evidence Gate contained stale evidence from commit `1b9a51f` while HEAD has progressed, and simulated/mock drivers must not satisfy verified milestone claims.

**Independent Test**: Can be independently tested by passing evidence generated for an older commit to `factory-cli validate-evidence`, verifying that validation fails due to commit mismatch and stale timestamp detection.

**Acceptance Scenarios**:

1. **Given** evidence artifacts generated for a past commit SHA, **When** submitted to the Evidence Gate validator against the current repository state, **Then** validation MUST be rejected with a freshness/commit obsolescence failure.
2. **Given** test execution evidence generated using simulated/mock sandbox drivers, **When** evaluated for production verification certification, **Then** the Evidence Gate MUST flag and reject synthetic mock evidence in favor of attested sandbox execution results.

---

### User Story 5 - Public Claims & Documentation Synchronization (Priority: P3)

As a public observer and stakeholder, I want public claims, READMEs, and canonical claims registries to accurately reflect demonstrated, verified runtime capabilities and current audit dates so that platform documentation maintains absolute integrity.

**Why this priority**: Eliminates discrepancies between published website claims (e.g. gVisor deployment status, audit dates, zero-vulnerability guarantees) and empirically verified code capabilities.

**Independent Test**: Can be tested via automated claims consistency checks (`check_public_claims.py` and repository linters) ensuring 100% alignment between code reality, canonical JSON registries, and web presentations.

**Acceptance Scenarios**:

1. **Given** published claims in `canonical-claims.json` and web pages, **When** automated claim verification runs, **Then** all dates, execution modes, and architectural statements match real, test-backed system state.

---

### Edge Cases

- **Runtime Shim Failure**: If the node kernel or containerd daemon rejects `runsc`, the sandbox launcher must emit an explicit `InfrastructureError` event and abort the mission safely rather than continuing in an insecure state.
- **Concurrent Worker Failures in FinOps**: If the shared FinOps store becomes unreachable or locks become stale, workers must default to a fail-closed admission policy (`FailingFinOpsStopStore`) to prevent uncontrolled cost overruns.
- **Evidence SHA Drift During In-Flight PRs**: If code changes are committed while a verification run is in progress, the resulting evidence must detect the SHA mismatch upon delivery and require re-execution.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST prohibit `SubprocessDriver` execution of untrusted code in all environments, enforcing sandbox pod containment as the mandatory execution path.
- **FR-002**: System MUST fail-closed with a distinct security error whenever an untrusted execution request cannot obtain a verified sandbox environment.
- **FR-003**: System MUST support containerd `runsc` runtime integration with container hardening: UID/GID 65534, non-root user, disabled privilege escalation, dropped capabilities, read-only root filesystems, and memory limits of at least 512Mi.
- **FR-004**: System MUST store FinOps budget stops and cumulative spend records in a shared, multi-process-safe persistent backend with atomic concurrency controls.
- **FR-005**: Evidence Gate MUST enforce cryptographic commit SHA matching against repository `HEAD` and invalidate any evidence produced from prior commits or mock drivers.
- **FR-006**: System MUST produce an end-to-end Verified Autonomous Execution mission record linking the generated change, gVisor execution logs, budget accounting, and cryptographic attestation to the commit.
- **FR-007**: System MUST synchronize public claims repositories, audit dates, and website documentation with verifiable system capabilities.

### Key Entities

- **SandboxExecutionProfile**: Represents the security and container constraints for executing untrusted workloads (runtime class, memory/CPU limits, capability drops, read-only root filesystem).
- **FinOpsSharedState**: Represents the multi-process persistent ledger of spend, active budget ceilings, and hard-stop signals across distributed workers.
- **VerificationEvidencePackage**: Represents the cryptographically signed collection of commit SHA, test execution logs, artifact hashes, execution environment attestation, and verifier identity.
- **AutonomousMissionLifecycle**: Represents the end-to-end execution flow from planning, sandbox execution, validation, review, and evidence gating to delivery.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of untrusted code execution attempts outside sandbox containment are rejected with zero fallbacks to host subprocesses.
- **SC-002**: Real Rust compilation and test execution inside a gVisor/runsc sandbox completes with an exit code of 0 and zero memory exhaustion failures.
- **SC-003**: Zero budget over-allocation or race condition discrepancies across concurrent worker instances under high load.
- **SC-004**: Evidence Gate validation passes with 100% compliance against current repository `HEAD` without relying on mock or stale evidence.
- **SC-005**: Complete end-to-end autonomous mission executed with all 5 lifecycle phases succeeding under full governance and verified evidence.

## Assumptions

- MicroK8s or target Kubernetes cluster has the gVisor (`runsc`) addon/shim installed and configured in containerd.
- Shared storage or network-accessible persistent storage is available for distributed worker pods in the cluster.
- Repository CI/CD and verification tooling have access to Git metadata to compute commit SHAs and artifact checksums.
