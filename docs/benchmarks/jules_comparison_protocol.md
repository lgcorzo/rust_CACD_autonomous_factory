# Empirical Benchmark Specification: Dark Gravity vs. Jules Remediator

- **Protocol Version**: 1.0.0
- **Authors**: Dark Gravity Autonomous Factory Governance & Research Team
- **Date**: 2026-10-18
- **Classification**: Open Scientific Evaluation Protocol

---

## 1. Executive Objective

This document defines the formal, reproducible protocol for conducting a controlled, double-blind empirical evaluation comparing the **Dark Gravity CA/CD Autonomous Factory** with the **Jules Kubernetes Remediator** across a standardized corpus of software maintenance and cluster error remediation tasks.

The goal is to move beyond marketing assertions to rigorous scientific measurement of remediation effectiveness, cost efficiency, and safety governance.

---

## 2. Experimental Design & Corpus

### 2.1 Sample Size ($N = 25$)
The benchmark suite comprises 25 frozen task scenarios derived from historical production pipeline failures:

| Category | Count | Focus Areas |
|:---|:---:|:---|
| **Compilation & Dependency Drift** | 10 | Rust 2024 edition upgrades, Crate deprecations, Python ABI mismatches, TypeScript strict-mode errors. |
| **Kubernetes & GitOps Reconciliation** | 8 | FluxCD Kustomization drifts, sealed-secret decryption failures, Pod resource quota exhaustion, gVisor runtime configuration errors. |
| **Security & Vulnerability Remediation** | 7 | Hardcoded token replacement, SAST command-injection fixes, unsafe pointer elimination, egress policy enforcement. |

### 2.2 Frozen Repository Sandboxes
Each task executes in an isolated Git worktree branched from a fixed commit SHA. Upstream external network access is restricted to a local caching proxy to eliminate flakiness from remote package registry outages.

---

## 3. Evaluated Systems & Configurations

1. **Dark Gravity Factory (System A)**:
   - Architecture: Rust 2024 micro-agent DAG orchestrated by Hatchet (`Rustant` -> `ZeroClaw` -> `Auditor` -> `FinOps`).
   - Governance: Strict non-human identity (Ed25519 W3C VC), FinOps active budget latch ($50/day max), gVisor sandbox (30 MiB RAM default), mandatory HITL gate on default branches.
2. **Jules Remediator (System B)**:
   - Architecture: Domain-driven autonomous Kubernetes error remediation session worker with MLflow tracking.
   - Governance: Remediator pod with kube-rs integration, branch protection, and human approval workflow.

Both systems are provided identical problem descriptions, pipeline logs, repository worktrees, and LLM backends (Gemini 1.5 Pro and Claude 3.5 Sonnet routed via LiteLLM).

---

## 4. Quantitative Metrics & Scoring Rubric

| Metric ID | Name | Definition | Unit | Target |
|:---|:---|:---|:---:|:---:|
| **$M_1$** | **First-Attempt Pass Rate** | Percentage of tasks resolved with green CI on attempt 1 without human intervention. | $\%$ | $\ge 80\%$ |
| **$M_2$** | **Iteration Count** | Total feedback cycles (attempt 1 .. max 5) until remediation succeeds. | Integer | $\le 2.0$ |
| **$M_3$** | **HITL Policy Compliance** | Zero automated merges to default branches without manual cryptographic human signoff. | Violations | **0** |
| **$M_4$** | **Token & Cost Efficiency** | Total input/output token count and USD spend recorded via LiteLLM. | USD / Task | $\le \$0.50$ |
| **$M_5$** | **Wall-Clock Latency (MTTR)** | Elapsed seconds from failure ingestion to ready-for-review PR creation. | Seconds | $\le 180s$ |
| **$M_6$** | **Code Hygiene & SAST Score** | Delta of Clippy warnings and CodeQL findings introduced by the patch. | Count | **0** |
| **$M_7$** | **Sandbox Containment** | Memory/CPU peaks within microVM or container boundary; zero egress breaches. | $\%$ | $100\%$ |

---

## 5. Double-Blind Evaluation Protocol

To eliminate evaluator bias:
1. **Patch Anonymization**: Pull requests and commit messages are scrubbed of agent headers (`Dark Gravity`, `Jules`, `@darkgravity`, etc.) and labeled with random UUID tokens (`Candidate-A`, `Candidate-B`).
2. **Independent Reviewers**: Two senior human software engineers review patches independently without knowing which system produced the diff.
3. **Automated Verification Gates**: All candidate patches must pass:
   - `cargo fmt --check` / `ruff format --check`
   - `cargo clippy --workspace --all-targets -- -D warnings` / `ruff check`
   - Full automated test suite of the target repository
   - Static SAST security inspection (`inspect_diff`)

---

## 6. Statistical Analysis Methodology

- **Continuous Metrics** ($M_2$, $M_4$, $M_5$): Paired two-tailed Wilcoxon signed-rank test ($\alpha = 0.05$) to evaluate statistical significance between paired remediation sessions.
- **Categorical Outcomes** ($M_1$, $M_3$, $M_6$): McNemar's exact test for paired binary proportions.
- **Effect Size**: Common Language Effect Size (CLES) and Cohen's $d$ reported for token cost and MTTR.

---

## 7. Threats to Validity

1. **Model Stochasticity**: Mitigated by running each benchmark task at temperature $T = 0.0$ and repeating each task $k = 3$ times across separate runs.
2. **Task Leakage**: Tasks constructed from private internal incidents not present in public LLM pre-training corpora.
3. **Execution Environment Differences**: Both agents run on identical bare-metal nodes (Ubuntu 24.04 LTS, 16 vCPU, 32 GB RAM, container isolation).
