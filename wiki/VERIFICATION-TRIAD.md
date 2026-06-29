# VERIFICATION-TRIAD: Quality Assurance

This document defines the **Verification Triad** standard for the **Dark Gravity** autonomous factory: Logical, Architectural, and Security verification.

---

## The Hierarchy of Truth

Our strategy ensures artifact quality through three simultaneous validation pillars.

| Pillar | Focus | Key Tooling |
| :--- | :--- | :--- |
| **Logical** | Functional correctness of the code. | `cargo test`, Sandbox (Firecracker/Subprocess) |
| **Architectural** | Alignment with DDD patterns and Bounded Contexts. | Rustant, `clippy`, domain models |
| **Security** | Vulnerability detection and compliance. | `security_review` (LLM-as-a-Judge), `cargo clippy` |

---

## 1. Logical Verification (The Executor)

We ensure mission reliability via **Sandbox Execution**.

- **Environment**: 
  - **SubprocessDriver**: `tokio::process::Command` for local execution
  - **FirecrackerDriver**: Micro-VM via KVM (planned implementation)
- **Workflow**:
  1. ZeroClaw generates code + unit tests.
  2. Runs tests via `run_tests` MCP tool inside the isolated Sandbox.
  3. Feedback (stdout/stderr) is streamed back for self-correction (max 3 retries).
- **Requirement**: ALL delivery-phase artifacts MUST pass their generated test suite.

---

## 2. Architectural Verification (The Planner)

Automated linting ensures the code remains maintainable and true to the **Strategic Design**.

- **Standard Linters**: `clippy` for Rust. Enforced in CI pipeline (`cargo clippy --workspace -- -D warnings`).
- **Domain Compliance**: Rustant validates generated code against core domain models and `SecurityValidator` constraints.

---

## 3. Security Verification (The Guardrails)

The final gate before delivery.

- **Automated Scanning**:
  - `security_review` MCP tool: LLM-as-a-Judge analysis of code diffs
- **Dependency Checking**: `cargo deny` / `cargo audit` (planned)
- **Sandbox Isolation**: Code executes in isolated Firecracker micro-VMs

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

---

## CRG-Verified Test Structure

Based on `code-review-graph` analysis, the test structure across the codebase includes:

| Crate | Test Configuration | Nodes |
|-------|-------------------|-------|
| `factory-mcp-server` | Unit + integration tests for 8 tools | 68 tool nodes, 68 test edges |
| `factory-infrastructure` | Mock-based tests (`wiremock`) | 42 client nodes, 15 test assertions |
| `factory-core` | Pure domain logic tests | 12 domain model nodes |
| `integration tests` | E2E security tests in `/tests/` | 4 security test nodes |

---

*Last updated: 2026-06-23 — Verified against actual codebase via CRG analysis*