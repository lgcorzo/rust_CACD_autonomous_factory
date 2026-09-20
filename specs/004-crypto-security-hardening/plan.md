# Implementation Plan: Zero-Trust Cryptographic Security Hardening & AI Agent Governance

**Branch**: `004-crypto-security-hardening` | **Date**: 2026-09-20 | **Spec**: [specs/004-crypto-security-hardening/spec.md](spec.md)

**Input**: Feature specification from `specs/004-crypto-security-hardening/spec.md`

---

## Summary

Implement and consolidate the enterprise Zero-Trust cryptographic security architecture, runtime sandboxing, and AI agent governance for the **Dark Gravity Autonomous Software Factory (V7.2)** based on the comprehensive security audit and vulnerability mitigations (VULN-01 to VULN-05):
1. **Kernel Sandboxing & Resource Clamping (VULN-03)**:
   - Restrict `ZeroClaw` execution containers to gVisor (`runsc`) user-space syscall containment with $\le 30\text{ MiB}$ RAM and $\le 250\text{m}$ CPU.
   - Constrain OpenZiti mTLS sidecars in GitOps manifests to $\le 20\text{ MiB}$ RAM / $100\text{m}$ CPU, paving the architectural path toward embedded `openziti-rs` transport.
   - Enforce memory sanitization (`#[derive(Zeroize, ZeroizeOnDrop)]`) ensuring secrets are wiped from RAM in $< 4.33\,\mu\text{s}$.
2. **Dark Network & Outbound Synchronization**:
   - Zero public ingress ports; all microservice traffic operates across OpenZiti mTLS 1.3 encrypted tunnels.
   - Outbound-only polling (`factory-cli poller`) every 30–60 seconds for VCS events (GitLab/GitHub), eliminating vulnerable webhooks.
3. **Non-Human Identity (NHI) & Ephemeral Vault JIT Tokens (VULN-04)**:
   - Ed25519 W3C Verifiable Credentials signed via `ed25519-dalek` v2 (`Ed25519Signature2020` / JWS) for all commits, database records, and A2A events.
   - Strict 5-minute (300 seconds) non-renewable JIT access token issuance via HashiCorp Vault (`VaultSecurityBounds`).
4. **Adaptive Velocity Circuit Breaker & Heterogeneous SAST Governance (VULN-01 & VULN-02)**:
   - LiteLLM Gateway enforces heterogeneous model routing: local fast models (`Bonsai 27B Ternario` / `Qwen 2.5 Coder 7B`) for generation vs. independent frontier model family for the SAST judge (`security_review`) with strict $\ge 8.0 / 10.0$ threshold.
   - `CircuitBreakerGuard` enhanced with SHA-256 diff-hash deadlock detection (freezing on duplicate diffs) and spend velocity alerts ($> +\$1.00 / 60\text{s}$).
   - 3 consecutive failures freeze the Hatchet DAG, revoke JIT tokens, transition to `Agent-Stuck`, and escalate to human architects (HITL Vertex 3).
5. **FinOps Virtual Tags & HardStop Guardrails (VULN-05)**:
   - HTTP header injection (`x-vtags-team: dark-gravity-ops`, `x-vtags-epic`, `x-vtags-microservice: factory-application`, `x-vtags-cost_center: eu-rd-grants`).
   - HardStop triggered at $90\%$ of daily budget ($\$45.00$ on $\$50.00$ default `FINOPS_MAX_DAILY_BUDGET`), publishing `budget-exceeded` to Kafka and pausing DAG execution.

---

## Technical Context

**Language/Version**: Rust 1.80+ (2024 Edition, Cargo workspace)  
**Primary Dependencies**: `ed25519-dalek` v2, `zeroize` 1.7+, `reqwest` (json), `serde`, `serde_json`, `tokio` (full), `async-trait`, `chrono`, `tracing`, `wiremock`, `criterion`  
**Storage**: HashiCorp Vault (JIT tokens, Ed25519 NHI keys), Kafka (topics: `agent-thought`, `mission-input`, `budget-exceeded`), pgvector (R2R GraphRAG)  
**Testing**: `cargo test --workspace`, `cargo bench --bench zeroize_benchmark`  
**Target Platform**: Linux Kubernetes cluster with gVisor (`runsc`), Cilium CNI (Deny-All egress), and OpenZiti dark overlay  
**Project Type**: Autonomous CA/CD Multi-Agent Software Factory  
**Performance Goals**: Forensic memory zeroization $< 4.33\,\mu\text{s}$; async Ed25519 batch signing $> 100\text{ ops/sec}$; circuit breaker tripping $\le 1\text{s}$  
**Constraints**: Application RAM $\le 30\text{ MiB}$, sidecar RAM $\le 20\text{ MiB}$, JIT TTL $\le 300\text{s}$, SAST pass score $\ge 8.0/10.0$, FinOps HardStop at $90\%$ budget  
**Scale/Scope**: Covers all autonomous agent lifecycles (Rustant, ZeroClaw, Auditor, DocAgent, FinOps) across repositories  

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Gate / Principle | Status | Evaluation & Compliance Notes |
| :--- | :--- | :--- |
| **I. Library-First** | PASS | Security traits (`SecurityBounds`, `SecurityValidator`), structs (`VerifiableCredential`, `JitToken`, `FinOpsTag`), and models reside cleanly in `factory-core`. |
| **II. CLI Interface** | PASS | Hardening checks, verification probes, and poller daemon operations are exposed via `factory-cli`. |
| **III. Test-First (NON-NEGOTIABLE)** | PASS | All cryptographic signing, zeroize benchmarks, diff auditing, and circuit breakers have automated unit and mock integration tests. |
| **IV. Integration Testing** | PASS | Exercises real/wiremock HashiCorp Vault token endpoints, LiteLLM `/spend/logs` telemetry, and Kafka event publishing. |
| **V. Observability & Telemetry** | PASS | Real-time spend velocity alerts, Sentry exception captures, and structured audit reports with causal tracing via `agent-thought`. |

---

## Project Structure

### Documentation (this feature)

```text
specs/004-crypto-security-hardening/
├── spec.md              # Feature specification
├── plan.md              # This implementation plan
├── research.md          # Phase 0 research findings & architectural decisions
├── data-model.md        # Phase 1 data models & entity relationships
├── quickstart.md        # Phase 1 runnable validation scenarios
├── contracts/
│   ├── circuit_breaker_contract.json # JSON Schema for CircuitBreakerGuard evaluations
│   ├── sast_evaluation_contract.json # JSON Schema for SAST judge outputs
│   └── finops_tag_contract.json     # JSON Schema for LiteLLM Virtual Tags & budgets
└── checklists/
    └── requirements.md  # Specification quality checklist (16/16 pass)
```

### Source Code Impact (repository crates)

```text
crates/
├── factory-core/
│   └── src/
│       ├── security.rs           # SecurityBounds, SastScanResult, JitToken (ZeroizeOnDrop)
│       ├── security/nhi.rs       # VerifiableCredential, async batch Ed25519 signing
│       └── lib.rs                # FinOpsTag metadata structure
├── factory-infrastructure/
│   └── src/
│       ├── vault.rs              # VaultSecurityBounds (5m JIT tokens, namespace isolation)
│       ├── security_validator.rs # Ed25519Validator implementation
│       └── ziti.rs               # OpenZiti mTLS wrapper & openziti-rs migration interface
├── factory-application/
│   └── src/
│       ├── agents/
│       │   ├── finops.rs         # FinOpsAgent (Spend velocity alert >$1/60s, HardStop 90%)
│       │   └── zeroclaw.rs       # ZeroClaw agent constrained to gVisor & JIT tokens
│       └── workflows/
│           ├── circuit_breaker.rs# CircuitBreakerGuard (3 attempts, diff-hash deadlock check)
│           └── autonomous_mission.rs # Hatchet DAG freeze & HITL escalation
└── factory-cli/
    └── src/
        └── main.rs               # Poller daemon and diagnostic verification commands
```

---

## Proposed Changes by Component

### 1. `factory-core`
- **`crates/factory-core/src/security.rs`**:
  - Maintain `SastScanResult::inspect_diff` rules enforcing $\ge 8.0/10.0$ threshold and detecting RCE, SQLi, and secrets.
  - Verify and benchmark `JitToken` with `#[derive(Zeroize, ZeroizeOnDrop)]`.
- **`crates/factory-core/src/security/nhi.rs`**:
  - Support async batch signing (`sign_batch_async`) for high-throughput A2A communication.

### 2. `factory-infrastructure`
- **`crates/factory-infrastructure/src/vault.rs`**:
  - `VaultSecurityBounds`: Enforce repository and agent path isolation (`secret/data/repos/<repo>/*`).
  - Set JIT token issuance TTL strictly to 5 minutes (`ttl: "5m"`, `renewable: false`).
- **`crates/factory-infrastructure/src/ziti.rs`**:
  - Expose transport configuration for dark overlay mTLS 1.3.

### 3. `factory-application`
- **`crates/factory-application/src/workflows/circuit_breaker.rs`**:
  - Enhance `CircuitBreakerGuard` with diff-hash history tracking to detect consecutive duplicate diffs (deadlock) and trip immediately.
  - Integrate spend velocity alert signals.
  - Format structured escalation alert for human architect (HITL Vertex 3).
- **`crates/factory-application/src/agents/finops.rs`**:
  - `FinOpsAgent`: Query `/spend/logs` every 60 seconds with Vtags (`x-vtags-*`).
  - Trip spend velocity anomaly alert when delta $> +\$1.00 / 60\text{s}$.
  - Enforce HardStop when spend $\ge 90\%$ of `FINOPS_MAX_DAILY_BUDGET` ($\$45.00$), triggering Kafka `budget-exceeded` event and freezing active DAG runs.

---

## Verification & Test Matrix

| Pillar / Mitigation | Test Method | Target Metric | Command |
| :--- | :--- | :--- | :--- |
| **RAM Zeroize** | Criterion Benchmark | $< 4.33\,\mu\text{s}$ | `cargo bench --bench zeroize_benchmark` |
| **Async Ed25519 Signing** | Tokio Unit Test | $> 100\text{ ops/sec}$ | `cargo test --package factory-core test_vc_async_signing_and_batch` |
| **SAST $\ge 8.0$ Gate** | Unit Test Diff Audit | Score $\ge 8.0$, 0 Critical | `cargo test --package factory-application test_circuit_breaker_pass` |
| **Circuit Breaker & Deadlock** | State Machine Test | Trips on 3 retries or duplicate hash | `cargo test --package factory-application test_circuit_breaker_retries_and_stuck` |
| **FinOps 90% HardStop** | Agent Budget Test | Halts at $90\%$ daily spend | `cargo test --package factory-application agents::finops` |
| **Full Workspace Verification**| Workspace Cargo Test | Zero compile errors, all pass | `cargo test --workspace` |

---

## Complexity Tracking

*No unjustified architectural violations detected. All components adhere to the project constitution.*
