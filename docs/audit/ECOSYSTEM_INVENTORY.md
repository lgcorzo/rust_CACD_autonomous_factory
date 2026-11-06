# Dark Gravity Ecosystem Repository Inventory

**Feature**: `011-verified-autonomous-execution`  
**Collected At (UTC)**: `2026-11-06T17:10:05Z`  
**Collector**: `antigravity-audit-collector`  
**Policy**: Inspect exact current revisions before making repository-specific claims (`FR-001`, `FR-002`, `FR-023`, `SC-007`). Inaccessible repositories are marked `not_accessible` without inferred findings.

---

## 1. Repository Snapshot Summary

| Repository | Canonical Remote / Location | Access Status | Inspected Branch | Inspected Full Commit SHA | Review Status | Primary CI / Validation Entry Points |
|---|---|---|---:|---|---|---|
| `rust_CACD_autonomous_factory` | `https://github.com/lgcorzo/rust_CACD_autonomous_factory.git` (`/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory`) | `accessible` | `011-verified-autonomous-execution` (`main`: `a894328a1d7c4c63a159a5d3795e6fb2a54ca8e3`) | `5464b9ca025f64751ab3144cc82f2c3f75c88ca7` | `in_progress` | `.github/workflows/pipeline.yml`, `.github/workflows/docs-to-wiki.yml`, `Makefile` (`cargo fmt`, `cargo clippy`, `cargo test --workspace`, `scripts/generate_evidence.sh`) |
| `gitops_internal_lgcorzo` | `https://github.com/lgcorzo/gitops_internal_lgcorzo.git` (`/mnt/F024B17C24B145FE/Repos/gitops_internal_lgcorzo`) | `accessible` | `main` | `48b9c0e333ce541fc741c056bd67e2932fdc50e9` | `in_progress` | `.github/workflows/flux-build.yml`, `.github/workflows/lint-and-validate.yml`, `.github/workflows/security-scan.yml`, `.github/workflows/trivy-scan.yml`, `kubectl kustomize infrastructure/agents` |
| `mlops-python-package` | `https://github.com/lgcorzo/mlops-python-package.git` (`/mnt/F024B17C24B145FE/Repos/mlops-python-package`) | `accessible` | `main` | `bf05a509b1e41597c823d030f2108c9e9c00ed83` | `not_started` (scheduled in Phase 8 `T035`) | `.github/workflows/check.yml`, `.github/workflows/publish.yml`, `.github/workflows/docs-to-wiki.yml`, `.github/workflows/openwiki-update.yml`, `uv run pytest`, `uv run ruff`, `uv run mypy` |
| `fastapi-autogen-team` | `https://github.com/lgcorzo/fastapi-autogen-team.git` (`/mnt/F024B17C24B145FE/Repos/fastapi-autogen-team`) | `accessible` | `main` | `6b9c4513eea630a04255a68412f2042cd61dbea1` | `not_started` (scheduled in Phase 8 `T036`) | `.github/workflows/check.yml`, `.github/workflows/publish.yml`, `.github/workflows/gitops-trigger.yml`, `.github/workflows/docs-to-wiki.yml`, `cargo test`, `cargo clippy` |
| `lgcorzo.github.io` | `https://github.com/lgcorzo/lgcorzo.github.io.git` (`/mnt/F024B17C24B145FE/Repos/lgcorzo.github.io`) | `accessible` | `main` | `e69db33ccceb3e7fdfaaa7bca5d593dedb296f7c` | `not_started` (scheduled in Phase 8 `T038`–`T043`) | Static HTML/CSS/JSON (`index.html`, `architecture.html`, `lifecycle.html`, `notebooks.html`, `fork-policy.html`, `sovereignty.html`); `.github/workflows/site-consistency.yml` to be added in `T041` |
| `autonomous_factory` | `https://gitlab.com/lgcorzo-lab/autonomous_factory.git` (`/mnt/F024B17C24B145FE/Repos/autonomous_factory`) | `accessible` | `main` | `4a8f2f3eb0b4484c0627919a5e3020bc1beb02f7` | `in_progress` | `.gitlab-ci.yml`, `legacy_code_documenter/` Python scripts (`pytest`) |
| `lince-rs` | `https://github.com/lgcorzo/lince-rs` (not cloned locally; GitHub API returns `NOT_FOUND`) | `not_accessible` | `N/A` | `N/A` | `blocked` | `N/A` — Repository is not present in `/mnt/F024B17C24B145FE/Repos/` and cannot be resolved via GitHub API (`gh repo view lgcorzo/lince-rs`). No code-level or runtime claims are inferred (`FR-002`, `SC-007`). |

---

## 2. Detailed Snapshot Records

### 2.1 `rust_CACD_autonomous_factory`
- **Repository**: `https://github.com/lgcorzo/rust_CACD_autonomous_factory.git`
- **Observed At**: `2026-11-06T17:10:05Z`
- **Access Status**: `accessible`
- **Revision**:
  - Active feature branch: `011-verified-autonomous-execution` @ `5464b9ca025f64751ab3144cc82f2c3f75c88ca7`
  - Default branch (`main`): `a894328a1d7c4c63a159a5d3795e6fb2a54ca8e3`
- **Workspace Crates**:
  - `crates/factory-core`
  - `crates/factory-infrastructure`
  - `crates/factory-application`
  - `crates/factory-mcp-server`
  - `crates/factory-cli`
- **Validation Entry Points**:
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `bash scripts/generate_evidence.sh`
- **Limitations**:
  - Live Kubernetes cluster (`server.internal.lgcorzo`) runtime checks require active cluster reachability and operator authorization (`T011`, `T027`, `T028`).

### 2.2 `gitops_internal_lgcorzo`
- **Repository**: `https://github.com/lgcorzo/gitops_internal_lgcorzo.git`
- **Observed At**: `2026-11-06T17:10:05Z`
- **Access Status**: `accessible`
- **Revision**: `main` @ `48b9c0e333ce541fc741c056bd67e2932fdc50e9`
- **Validation Entry Points**:
  - `kubectl kustomize infrastructure/agents`
  - `.github/workflows/flux-build.yml`
  - `.github/workflows/lint-and-validate.yml`
  - `.github/workflows/security-scan.yml`
  - `.github/workflows/trivy-scan.yml`
- **Limitations**:
  - Manifest rendering (`kubectl kustomize`) proves `declared` configuration only, not `reconciled`, `enforced`, or `runtime_verified` state (`FR-015`, `FR-027`).

### 2.3 `mlops-python-package`
- **Repository**: `https://github.com/lgcorzo/mlops-python-package.git`
- **Observed At**: `2026-11-06T17:10:05Z`
- **Access Status**: `accessible`
- **Revision**: `main` @ `bf05a509b1e41597c823d030f2108c9e9c00ed83`
- **Validation Entry Points**:
  - `.github/workflows/check.yml` (`ruff`, `mypy`, `bandit`, `pytest`, `cyclonedx-py`)
  - `.github/workflows/publish.yml`
- **Limitations**:
  - Full compatibility and reproducibility review scheduled in `T035` (`docs/audit/reviews/mlops-python-package.md`).

### 2.4 `fastapi-autogen-team`
- **Repository**: `https://github.com/lgcorzo/fastapi-autogen-team.git`
- **Observed At**: `2026-11-06T17:10:05Z`
- **Access Status**: `accessible`
- **Revision**: `main` @ `6b9c4513eea630a04255a68412f2042cd61dbea1`
- **Validation Entry Points**:
  - `.github/workflows/check.yml` (`cargo fmt`, `cargo clippy`, `cargo test`)
  - `.github/workflows/publish.yml`
  - `.github/workflows/gitops-trigger.yml`
- **Limitations**:
  - Note: Despite the historical repository name `fastapi-autogen-team`, the inspected revision (`6b9c4513...`) is a Rust (`axum` + `rig-core` + `tokio`) microservice (`rust-agent-team`). Naming compatibility and API review are scheduled in `T036` (`docs/audit/reviews/fastapi-autogen-team.md`).

### 2.5 `lgcorzo.github.io`
- **Repository**: `https://github.com/lgcorzo/lgcorzo.github.io.git`
- **Observed At**: `2026-11-06T17:10:05Z`
- **Access Status**: `accessible`
- **Revision**: `main` @ `e69db33ccceb3e7fdfaaa7bca5d593dedb296f7c`
- **Validation Entry Points**:
  - Static HTML pages (`index.html`, `architecture.html`, `lifecycle.html`, `notebooks.html`, `fork-policy.html`, `sovereignty.html`) and shared stylesheet (`assets/site.css`).
  - Automated claim consistency checker (`scripts/check_public_claims.py` and `tests/test_public_claims.py`) to be added in `T038`–`T041`.

### 2.6 `autonomous_factory` (Supporting Legacy GitLab Repository)
- **Repository**: `https://gitlab.com/lgcorzo-lab/autonomous_factory.git`
- **Observed At**: `2026-11-06T17:10:05Z`
- **Access Status**: `accessible` (local clone present at `/mnt/F024B17C24B145FE/Repos/autonomous_factory`)
- **Revision**: `main` @ `4a8f2f3eb0b4484c0627919a5e3020bc1beb02f7`
- **Validation Entry Points**:
  - `.gitlab-ci.yml`
  - `legacy_code_documenter/` Python package and skills.
- **Limitations**:
  - Historical predecessor repository on GitLab prior to the Rust CA/CD migration in `rust_CACD_autonomous_factory`.

### 2.7 `lince-rs`
- **Repository**: `lince-rs`
- **Observed At**: `2026-11-06T17:10:05Z`
- **Access Status**: `not_accessible`
- **Revision**: `N/A`
- **Review Status**: `blocked`
- **Limitations**:
  - Neither a local checkout in `/mnt/F024B17C24B145FE/Repos/` nor a remote GitHub repository (`lgcorzo/lince-rs`) is accessible. Per `FR-002` and `SC-007`, zero code, dependency, or runtime findings are inferred for `lince-rs`.

