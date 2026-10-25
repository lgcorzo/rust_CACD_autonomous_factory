# MLOps Python Package — Dark Gravity Factory Control & HITL Governance

[![Dark Gravity Target](https://img.shields.io/badge/Dark_Gravity-Managed_Target-purple.svg)](https://github.com/lgcorzo/rust_CACD_autonomous_factory)
[![HITL Gate](https://img.shields.io/badge/HITL-Strict_Human_Review-blue.svg)](https://github.com/lgcorzo/rust_CACD_autonomous_factory#5-branch--pull-request-policy-human-in-the-loop-hitl)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

This repository contains the standard **MLOps Python Package** for ML pipeline development, automated experiment tracking (DVC/MLflow), and Poetry dependency management, autonomously monitored and governed by the **Dark Gravity CA/CD Autonomous Agent Factory**.

---

## 🌌 Dark Gravity Autonomous Integration

This repository is integrated as an active target repository under the control of the **Dark Gravity Autonomous Software Engineering Factory**.

- **Outbound Polling & Event Ingestion**: Dark Gravity's `GitPlatformPoller` continuously monitors GitHub issues, work items, and pull requests in this repository.
- **Automated CI/CD Error Remediation**: Failed GitHub Actions pipeline runs (e.g., Poetry dependency conflicts, linting failures, pytest test assertion failures) are automatically classified by Dark Gravity's `PipelineErrorClassifier` and remediated via autonomous micro-missions.
- **Zero-Trust Security & Non-Human Identity (NHI)**: All automated actions in this repository carry Ed25519-signed W3C Verifiable Credentials issued under `did:factory:dark-gravity-euskadi`.

---

## 🎮 Directing Dark Gravity on PRs and Issues

Developers and ML engineers can interact directly with Dark Gravity agents (`Rustant` planner, `ZeroClaw` executor, `QAObserver`) using comments on GitHub Pull Requests or Issues:

```text
@darkgravity /<command> [optional instruction/prompt]
```

### Supported Directives

| Directive | Command Action | Agent Target |
| :--- | :--- | :--- |
| `@darkgravity /status` | Returns current factory health, DAG pipeline state, and sandbox status. | System Poller |
| `@darkgravity /interact <prompt>` | Answers queries regarding ML pipeline design, dependencies, or tests. | Rustant (Planner) |
| `@darkgravity /spec <prompt>` | Re-evaluates specification design or generates testing contracts. | Rustant (Planner) |
| `@darkgravity /refine <instruction>` | Applies surgical code or configuration fixes in a gVisor sandbox. | ZeroClaw (Executor) |
| `@darkgravity /validate` | Executes `poetry run pytest` and security audit gates in sandbox. | QAObserver / SAST |
| `@darkgravity /retry` | Restarts the verification loop for failing CI checks. | Remediation Loop |

---

## 🛡️ Human-in-the-Loop (HITL) Governance Rules

To ensure safety, stability, and regulatory compliance (EU AI Act & SOC 2 Type II), Dark Gravity enforces strict **Human-in-the-Loop (HITL)** governance rules across `mlops-python-package`:

1. **Strict Manual Merge Policy on Core Repositories**: Dark Gravity agents create isolated feature branches, run isolated test suites, and open Pull Requests. **Under Dark Gravity governance, PRs on core production repositories (such as `mlops-python-package`) are NEVER automatically merged** — human maintainer review and sign-off is mandatory. Routine patch bumps on peripheral fleet repos are handled by separate fleet triage automation.
2. **Mandatory Human Sign-off**: Every pull request targeting core services requires human review, inspection of automated test evidence, and explicit manual merge by an authorized human maintainer.
3. **Sandbox Execution Clamping**: All code synthesis and validation tests executed by Dark Gravity run inside ephemeral gVisor (`runsc`) micro-containers strictly clamped to allocated resources ($\le 30\text{ MiB}$ RAM / $\le 250\text{m}$ CPU for micro-sandboxes; up to $512\text{ MiB}$ RAM / $500\text{m}$ CPU for full compiler builds).
4. **Self-Referential Safety & Escalation**: In the event of recurring CI failures (3 consecutive identical error fingerprints), Dark Gravity automatically escalates the issue to human maintainers via a GitHub Issue.

---

For complete details on Dark Gravity V7 Architecture, empirical evidence trees, and system benchmarks, refer to the root [Dark Gravity Workspace README](https://github.com/lgcorzo/rust_CACD_autonomous_factory).
