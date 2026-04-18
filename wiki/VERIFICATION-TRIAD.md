# 🛡️ VERIFICATION-TRIAD: Quality Assurance

This document defines the **Verification Triad** standard for the **Dark Gravity** autonomous factory: Logical, Architectural, and Security verification.

---

## 🏗️ The Hierarchy of Truth

Our strategy ensures artifact quality through three simultaneous validation pillars.

| Pillar | Focus | Key Tooling |
| :--- | :--- | :--- |
| **Logical** | Functional correctness of the code. | `cargo test`, `pytest`, Sandbox |
| **Architectural** | Alignment with DDD patterns and Bounded Contexts. | Rustant, `clippy`, Custom Linters |
| **Security** | Vulnerability detection and Compliance. | `cargo audit`, LLM SAST, Sandbox |

---

## 1. Logical Verification (The Executor)

We ensure 100% mission reliability via **Sandbox Execution**.

- **Environment**: Isolated Firecracker MicroVMs.
- **Workflow**:
    1. Agent generates code + unit tests.
    2. Sidecar triggers `cargo test` inside the Sandbox.
    3. Feedback (stdout/stderr) is streamed back to the agent for self-correction.
- **Requirement**: ALL delivery-phase artifacts MUST pass their generated test suite.

---

## 2. Architectural Verification (The Planner)

Automated linting and architectural reviews ensure the code remains maintainable and true to the **Strategic Design**.

- **Standard Linters**: `clippy` for Rust, `eslint` for JS.
- **Deep Review**: The Planner agent audits the code against the **[GLOSSARY](GLOSSARY)** and **[STRATEGIC-DESIGN](STRATEGIC-DESIGN)** to prevent bounded context leakage.

---

## 3. Security Verification (The Guardrails)

The final gate before delivery. Any artifact with a security score below 8.0/10 is rejected and regressed to the planning phase.

- **Automated Scanning**: Checking for hardcoded secrets and unsafe dependencies.
- **LLM-as-a-Judge**: A security-tuned LLM analyzes the diff for logic-based vulnerabilities (e.g., bypass bugs).

---

## 🚀 How to Validate Locally

### 1. Internal Units
```bash
# Run all tests in the workspace crates
cargo test
```

### 2. Integration Mocks
```bash
# Verify infrastructure clients against wiremock
cargo test -p factory-infrastructure
```

### 3. Agent Missions (Dry Run)
```bash
# Run a local mission using the CLI interface
factory-cli mission "Add feature X" --dry-run
```
