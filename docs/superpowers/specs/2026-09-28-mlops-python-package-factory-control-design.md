# Design Specification: Dark Gravity Control of `mlops-python-package`

**Date:** 2026-09-28  
**Status:** Approved  
**Target Repository:** `https://github.com/lgcorzo/mlops-python-package.git`  
**Factory Engine:** `lgcorzo/rust_CACD_autonomous_factory` (Dark Gravity)  
**Cluster Infrastructure:** `lgcorzo/gitops_internal_lgcorzo` (GitOps)  

---

## 1. Objective

Integrate the GitHub repository [`lgcorzo/mlops-python-package`](https://github.com/lgcorzo/mlops-python-package) into the Dark Gravity CA/CD Autonomous Agent Factory. The factory will autonomously monitor, poll, and execute missions on `mlops-python-package`, including:
- Ingestion of issues labeled `autonomous-mission` or `dark-gravity`.
- Processing PR comment directives (`@darkgravity /status`, `@darkgravity /validate`, `@darkgravity /refine`, etc.).
- Continuous monitoring of CI/CD workflow runs and automated error remediation missions upon failure.

---

## 2. Architecture & Components

```
┌────────────────────────────────────────────────────────┐
│                   GitHub Platform                      │
│            lgcorzo/mlops-python-package                │
│   (Issues, PR Discussions, GitHub Actions CI/CD)       │
└───────────────────────────▲────────────────────────────┘
                            │
              Polling (REST API / 30s Interval)
                            │
┌───────────────────────────┴────────────────────────────┐
│         Dark Gravity Factory Poller Daemon             │
│        (factory-poller in Kubernetes / local)          │
│                                                        │
│  - GitHub Poller (Issues + PR Comments + Workflows)    │
│  - Non-Human Identity (NHI) Ed25519 VC Generation      │
│  - Kafka Producer (mission-input topic)                │
└───────────────────────────┬────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────┐
│               Hatchet Orchestrator DAG                 │
│  Ingestion ➔ Rustant (Plan) ➔ ZeroClaw (Code/Validate) │
│          ➔ Rustant (Review) ➔ Delivery (GitOps PR)     │
└────────────────────────────────────────────────────────┘
```

---

## 3. Detailed Changes Across Repositories

### 3.1 Dark Gravity Factory (`rust_CACD_autonomous_factory`)

1. **Pipeline Remediation Configuration (`config/pipeline-remediation.yaml`)**:
   Add `"lgcorzo/mlops-python-package"` to `polling.github_repositories`:
   ```yaml
   polling:
     github_repositories:
       - "lgcorzo/rust_CACD_autonomous_factory"
       - "lgcorzo/mlops-python-package"
   ```

2. **Environment Template (`config/pipeline-remediation.env.example`)**:
   Update `GITHUB_REPOS` setting:
   ```bash
   GITHUB_REPOS=lgcorzo/rust_CACD_autonomous_factory,lgcorzo/mlops-python-package
   ```

3. **CLI Arguments Default (`crates/factory-cli/src/main.rs`)**:
   Update default value for `Poller.github_repos`:
   ```rust
   #[arg(
       long,
       env = "GITHUB_REPOS",
       default_value = "lgcorzo/rust_CACD_autonomous_factory,lgcorzo/mlops-python-package"
   )]
   github_repos: String,
   ```

4. **Documentation (`README.md`)**:
   Add `lgcorzo/mlops-python-package` to the **Integrated Target Repositories** table:
   ```markdown
   | [`lgcorzo/mlops-python-package`](https://github.com/lgcorzo/mlops-python-package) | MLOps & ML Pipelines | Automated testing, DVC/Poetry dependency validation & CI/CD error remediation |
   ```

---

### 3.2 GitOps Cluster Deployment (`gitops_internal_lgcorzo`)

1. **Poller Deployment (`infrastructure/agents/poller-deployment.yaml`)**:
   Update the `GITHUB_REPOS` container environment variable:
   ```yaml
   - name: GITHUB_REPOS
     value: "lgcorzo/rust_CACD_autonomous_factory, lgcorzo/mlops-python-package"
   ```

---

### 3.3 Target Repository Standards (`mlops-python-package`)

1. **Agent Governance (`AGENTS.md`)**:
   Append standard agent directives, Human-in-the-Loop (HITL) policy, and Dark Gravity bot command formats:
   - **Branch & Pull Request Policy: Human-in-the-Loop (HITL)**
     - Agents may create feature branches, commit code, push upstream, open PRs, and resolve CI/CD failures.
     - Autonomous merging into `main` is strictly forbidden.
     - Merging must remain a manual human approval action.
   - **Dark Gravity Directives & Autonomous Mission Protocol**
     - Issue labels: `autonomous-mission`, `dark-gravity`.
     - Resource limits syntax: `Resource limits: CPU: 500m, RAM: 512Mi, Timeout: 300s`.
     - PR interaction directives: `@darkgravity /status`, `@darkgravity /validate`, `@darkgravity /refine`, `@darkgravity /interact`.

---

## 4. Verification & Validation Plan

1. **Configuration Validation**:
   - Verify syntax of `config/pipeline-remediation.yaml` and `poller-deployment.yaml`.
2. **Factory CLI Build & Check**:
   - Run `cargo check -p factory-cli` in `rust_CACD_autonomous_factory` to ensure parsing of updated default values compiles cleanly.
3. **Repository Policy Verification**:
   - Confirm `AGENTS.md` in `mlops-python-package` correctly reflects OpenWiki and Dark Gravity HITL standards.
