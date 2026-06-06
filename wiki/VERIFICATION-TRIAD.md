# VERIFICATION-TRIAD: Quality Assurance

This document defines the **Verification Triad** standard for the **Dark Gravity** autonomous factory: Logical, Architectural, and Security verification.

---

## The Hierarchy of Truth

Our strategy ensures artifact quality through three simultaneous validation pillars.

| Pillar | Focus | Key Tooling |
| :--- | :--- | :--- |
| **Logical** | Functional correctness of the code. | `cargo test`, Sandbox (gVisor/Firecracker) |
| **Architectural** | Alignment with DDD patterns and Bounded Contexts. | Rustant, `clippy`, Spec-Kit Specs |
| **Security** | Vulnerability detection and compliance. | `security_review` (OWASP SAST), LLM-as-a-Judge, `cargo audit` |

---

## 1. Logical Verification (The Executor)

We ensure 100% mission reliability via **Sandbox Execution**.

- **Environment**: 
  - **SubprocessDriver**: `tokio::process::Command` inside gVisor pods (`runtimeClassName: gvisor`, RAM ≤ 30Mi, CPU ≤ 250m)
  - **FirecrackerDriver**: Micro-VM via KVM with `AF_VSOCK` host-to-guest communication (no host-level TCP bridges)
- **Workflow**:
  1. ZeroClaw generates code + unit tests.
  2. Sidecar triggers `cargo test` inside the isolated Sandbox.
  3. Feedback (stdout/stderr) is streamed back for self-correction (max 3 retries before `Agent-Stuck`).
- **Requirement**: ALL delivery-phase artifacts MUST pass their generated test suite.

---

## 2. Architectural Verification (The Planner)

Automated linting and architectural reviews ensure the code remains maintainable and true to the **Strategic Design**.

- **Standard Linters**: `clippy` for Rust. Enforced in CI pipeline (`cargo clippy --workspace -- -D warnings`).
- **Spec-Driven Compliance**: Rustant validates generated code against Spec-Kit artifacts (`spec.md`, `plan.md`, `tasks.md`) to prevent bounded context leakage.
- **Deep Review**: The Rustant audits code against the **[GLOSSARY](GLOSSARY)** and **[STRATEGIC-DESIGN](STRATEGIC-DESIGN)**.

---

## 3. Security Verification (The Guardrails)

The final gate before delivery. Any artifact with a security score below 8.0/10 is rejected and regressed to the planning phase.

- **Automated Scanning & Code Analysis**:
  - `security_review` MCP tool: OWASP Top 10 regex patterns (SQL injection, command injection, hardcoded secrets, path traversal).
  - `cargo audit`: Dependency vulnerability scanning.
  - LLM-as-a-Judge: Analyzes the diff for logical vulnerabilities; minimum score **8.0/10.0**.
- **Forensic Memory Wiping**: JIT credentials in RAM secured via `zeroize` crate. Secrets wiped within **4.33 μs**. Verified via Criterion benchmarks in CI.
- **Non-Human Identities**: Every agent action is signed with Ed25519 keypairs for SOC 2 / EU AI Act compliance.

---

## Aethelgard Auto-Remediation Loop

When CI/CD pipelines fail, the **DevOps Agent** triggers the auto-remediation loop:

1. Parse raw stdout/stderr from failing container.
2. Query R2R GraphRAG for similar historical error fixes.
3. Instruct Developer Agent via Hatchet to apply surgical code patch.
4. Commit and push patch → trigger pipeline rerun.
5. **Circuit Breaker**: Max 3 consecutive attempts. On 3rd failure, set `Agent-Stuck` and escalate to human.

---

## How to Validate Locally

### 1. Internal Units
```bash
cargo test
```

### 2. Integration Mocks
```bash
cargo test -p factory-infrastructure
```

### 3. CI Pipeline
```bash
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace -- --skip smoke
```
