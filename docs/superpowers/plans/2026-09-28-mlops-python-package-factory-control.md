# Dark Gravity Control of `mlops-python-package` Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enable autonomous monitoring, polling, PR directives, and CI/CD error remediation of `lgcorzo/mlops-python-package` within the Dark Gravity Agent Factory and its GitOps cluster deployment.

**Architecture:** Update the Dark Gravity Poller configuration and CLI defaults to register `lgcorzo/mlops-python-package`, update the Kubernetes GitOps `poller-deployment.yaml` manifest in `gitops_internal_lgcorzo`, and add HITL branch/PR policies and Dark Gravity command protocols to `AGENTS.md` in `mlops-python-package`.

**Tech Stack:** Rust (Clap, Tokio, Axum), YAML, Kubernetes Manifests (Apps/v1 Deployment), Markdown.

## Global Constraints

- Target repository URL: `https://github.com/lgcorzo/mlops-python-package.git`
- GitHub repository slug: `lgcorzo/mlops-python-package`
- Strictly adhere to Human-in-the-Loop (HITL) PR policy: autonomous merging is forbidden.
- Zero breaking changes to existing repository polling configurations (`rust_CACD_autonomous_factory`, `lince-rs`, `fastapi-autogen-team`).

---

### Task 1: Dark Gravity Factory Configuration & Documentation

**Files:**
- Modify: `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/config/pipeline-remediation.yaml:20-25`
- Modify: `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/config/pipeline-remediation.env.example:14`
- Modify: `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/crates/factory-cli/src/main.rs:114-120`
- Modify: `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory/README.md:40-46`

**Interfaces:**
- Consumes: Poller CLI arguments and remediation config schema.
- Produces: Poller daemon recognizing `lgcorzo/mlops-python-package` as a monitored GitHub repository by default.

- [ ] **Step 1: Update remediation YAML configuration**
Add `"lgcorzo/mlops-python-package"` under `polling.github_repositories` in `config/pipeline-remediation.yaml`.

- [ ] **Step 2: Update remediation env example**
Set `GITHUB_REPOS=lgcorzo/rust_CACD_autonomous_factory,lgcorzo/mlops-python-package` in `config/pipeline-remediation.env.example`.

- [ ] **Step 3: Update CLI default value in `crates/factory-cli/src/main.rs`**
Update `github_repos` default value on `Poller` sub-command to `"lgcorzo/rust_CACD_autonomous_factory,lgcorzo/mlops-python-package"`.

- [ ] **Step 4: Update README target repository table**
Add `lgcorzo/mlops-python-package` row to `README.md` under **Integrated Target Repositories**.

- [ ] **Step 5: Verify build with cargo check**
Run `cargo check -p factory-cli` in `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory`.

- [ ] **Step 6: Commit changes in `rust_CACD_autonomous_factory`**
```bash
git add config/ crates/factory-cli/src/main.rs README.md
git commit -m "feat(poller): register mlops-python-package under factory control"
```

---

### Task 2: Update Kubernetes GitOps Poller Deployment

**Files:**
- Modify: `/mnt/F024B17C24B145FE/Repos/gitops_internal_lgcorzo/infrastructure/agents/poller-deployment.yaml:26-27`

**Interfaces:**
- Consumes: Cluster container environment variables.
- Produces: Production Kubernetes deployment environment variable `GITHUB_REPOS` configured with `lgcorzo/rust_CACD_autonomous_factory, lgcorzo/mlops-python-package`.

- [ ] **Step 1: Update GITHUB_REPOS environment variable**
Update `GITHUB_REPOS` in `infrastructure/agents/poller-deployment.yaml`:
```yaml
        - name: GITHUB_REPOS
          value: "lgcorzo/rust_CACD_autonomous_factory, lgcorzo/mlops-python-package"
```

- [ ] **Step 2: Validate YAML syntax**
Run python YAML parser check:
```bash
python3 -c "import yaml; yaml.safe_load(open('infrastructure/agents/poller-deployment.yaml'))"
```

- [ ] **Step 3: Commit changes in `gitops_internal_lgcorzo`**
```bash
git add infrastructure/agents/poller-deployment.yaml
git commit -m "feat(agents): add mlops-python-package to factory-poller GITHUB_REPOS"
```

---

### Task 3: Update Target Repository Governance & Agent Protocol

**Files:**
- Modify: `/mnt/F024B17C24B145FE/Repos/mlops-python-package/AGENTS.md:1-10`

**Interfaces:**
- Consumes: None.
- Produces: Formal agent guidelines, HITL PR gate, and Dark Gravity bot command specs in `mlops-python-package`.

- [ ] **Step 1: Append HITL Policy and Dark Gravity Protocol to `AGENTS.md`**
Add:
- Branch & Pull Request Policy: Human-in-the-Loop (HITL)
- Controlling Dark Gravity from GitHub Issues & PRs
- Mission triggering criteria and directive formats

- [ ] **Step 2: Verify AGENTS.md formatting**
Ensure markdown links and code fences are valid.

- [ ] **Step 3: Commit changes in `mlops-python-package`**
```bash
git add AGENTS.md
git commit -m "docs(agents): add HITL branch policy and Dark Gravity factory protocol"
```
