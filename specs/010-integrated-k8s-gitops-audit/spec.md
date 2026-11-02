# Feature Specification: Dark Gravity Integrated Kubernetes & GitOps Audit & Remediation

**Feature ID**: `010-integrated-k8s-gitops-audit`  
**Status**: `Draft / Clarified`  
**Date**: `2026-11-02`  
**Target Repositories**:
- Application: `https://github.com/lgcorzo/rust_CACD_autonomous_factory` (Branch: `010-integrated-k8s-gitops-audit`)
- GitOps / Source of Truth: `https://github.com/lgcorzo/gitops_internal_lgcorzo` (Local clone: `/mnt/F024B17C24B145FE/Repos/gitops_internal_lgcorzo`)
- Target Environment: Local Kubernetes Cluster (`server.internal.lgcorzo`, microk8s v1.31, with `RuntimeClass` `gvisor` using handler `runsc`)

---

## 1. Executive Summary & Mission
Perform an evidence-driven audit and hardening remediation across the autonomous software factory project (`rust_CACD_autonomous_factory`) and its Kubernetes GitOps deployment repository (`gitops_internal_lgcorzo`).

The objective is to establish that Dark Gravity cannot execute untrusted code on the host, enforces strict gVisor kernel-level boundary isolation (`RuntimeClass: gvisor`), denies unauthorized network access by default, bounds resources and execution timeouts, secures identities and credentials in Git and at runtime, and ensures that test/mission results cannot be fabricated by fallback mocks or missing executables.

---

## 2. Core Clarified Decisions & Working Rules

1. **Dual-Repository Audit & Source of Truth**:
   - `gitops_internal_lgcorzo` is the authoritative source of truth for the Kubernetes deployment and runtime isolation posture.
   - Application code changes and GitOps manifests will be synchronized across both repositories.
2. **Distinct Categorization of Guarantees**:
   Every security property must be explicitly differentiated across:
   - Implemented in application code (Rust).
   - Configured in GitOps (YAML/Kustomize/Flux).
   - Enforced by Kubernetes admission/kernel (Pod Security Admission, gVisor, NetworkPolicy).
   - Verified in live cluster (`server.internal.lgcorzo`).
   - Demonstrated by reproducible automated tests.
3. **Execution Methodology**:
   - **Phase 0 (Baseline & Traceability)**: Full static inspection of both repositories, construction of the traceability matrix (`docs/audit/TRACEABILITY_MATRIX.md`), threat model (`docs/audit/THREAT_MODEL.md`), initial findings report (`docs/audit/INTEGRATED_AUDIT.md`), and execution plan without modifying production cluster resources.
   - **Phases 1–5 (Incremental Remediation)**: Execution in small, reviewable stages prioritized by severity:
     - Priority 1: Eliminating host subprocess execution & shell bypasses in `crates/factory-mcp-server` (P0).
     - Priority 2: Mandatory gVisor runtime class enforcement with fail-closed semantics (P0).
     - Priority 3: Hardening `run_tests` to prevent false positive successes and missing executable illusions (P0).
     - Priority 4: Hardening Kubernetes RBAC, ServiceAccounts, and NetworkPolicies (P0/P1).
     - Priority 5: FinOps mission admission hard-stop integration (P1).
     - Priority 6: Hardening MCP API, SSE cleanup, and webhook authentication (P1).
     - Priority 7: Reproducible security tests and verifiable evidence generation (P1/P2).
     - Priority 8: CI/CD & Flux validation gates (P1/P2).

---

## 3. Scope of Audited Application Components & Requirements

### 3.1 Sandbox Execution (`crates/factory-mcp-server/src/sandbox.rs`, `tools/launch_sandbox_pod.rs`)
- **P0 Finding**: The local subprocess driver allows executing arbitrary Python/TypeScript directly on the host machine.
- **Requirement**:
  - In production mode, untrusted code execution MUST NOT use local subprocesses.
  - Production execution must dispatch exclusively to an isolated Kubernetes Job using `runtimeClassName: gvisor`.
  - A missing or unsupported runtime class must trigger a fail-closed error immediately. No fallback to `runc` or host subprocess.
  - Consolidate sandbox configuration to eliminate conflicting environment variables (`SANDBOX_MODE` vs `USE_GVISOR`).

### 3.2 Test Execution (`crates/factory-mcp-server/src/tools/run_tests.rs`)
- **P0 Finding**: `run_tests` uses `sh -c` on the host, bypassing the sandbox driver. Missing test executables can fabricate successful test passes.
- **Requirement**:
  - All test commands must execute inside the designated sandbox.
  - Remove direct host shell execution.
  - Return explicit outcomes (`passed`, `failed`, `not_executed`, `infrastructure_error`, `timeout`). Tests that cannot execute must never satisfy an acceptance gate.

### 3.3 Kubernetes Job Hardening (`launch_sandbox_pod.rs`, GitOps Job manifests)
- **Requirements**:
  - Explicit `runtimeClassName: gvisor`.
  - Dedicated sandbox execution namespace (e.g., `factory-sandboxes`).
  - Dedicated ServiceAccount with `automountServiceAccountToken: false`.
  - SecurityContext: `allowPrivilegeEscalation: false`, `capabilities.drop: ["ALL"]`, `seccompProfile: RuntimeDefault`, `readOnlyRootFilesystem: true` (where compatible), non-root UID.
  - CPU, memory, ephemeral storage limits, execution timeouts, and `ttlSecondsAfterFinished` for automatic cleanup.
  - Immutable image references (digests or pinned tags).

### 3.4 MCP Server, SSE & Webhook Security
- **Requirements**:
  - Enforce authentication/authorization on MCP tool execution and webhooks.
  - Replay protection and HMAC signature verification for GitHub webhooks.
  - Bounded queues for SSE channels and deterministic session cleanup on client disconnection.

### 3.5 FinOps Mission Admission Gate
- **Requirements**:
  - Connect the FinOps budget hard-stop latch to mission admission. No new missions may start once the budget threshold is hit.

---

## 4. GitOps & FluxCD Audit Scope (`gitops_internal_lgcorzo`)
- Validate Flux Kustomizations, HelmReleases, and Pod Security Standards (Restricted).
- Verify default-deny Ingress and Egress NetworkPolicies for the sandbox namespace:
  - Block access to Kubernetes API server, cloud/host metadata (169.254.169.254), internal control-plane services, and Vault admin endpoints.
- Verify SOPS secret encryption at rest and absence of plaintext secrets.
- Verify RBAC least-privilege matrix: sandbox workloads must not possess permissions to create Pods, Jobs, Secrets, or RoleBindings.

---

## 5. Verification & Deliverables
1. `docs/audit/INTEGRATED_AUDIT.md`: Executive audit summary, findings ranked P0–P3.
2. `docs/audit/TRACEABILITY_MATRIX.md`: Capability -> Rust Code -> GitOps Resource -> Kubernetes Enforcement -> Cluster Evidence -> Status (PASS/PARTIAL/FAIL/NOT_VERIFIED).
3. `docs/audit/THREAT_MODEL.md`: Assets, threat actors, attack vectors, mitigations.
4. `docs/audit/REMEDIATION_PLAN.md`: Implementation tasks and dependency graph.
5. `docs/audit/SECURITY_TEST_PLAN.md`: Reproducible security test commands and assertions.
6. Passing automated quality gates in both repositories.
