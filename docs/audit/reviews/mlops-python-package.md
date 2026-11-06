# Supporting Repository Review: `mlops-python-package` (`FR-023`, `FR-026`, `FR-030`, `SC-012`)

**Repository**: `https://github.com/lgcorzo/mlops-python-package.git` (`/mnt/F024B17C24B145FE/Repos/mlops-python-package`)  
**Inspected Branch**: `main`  
**Inspected Full Commit SHA**: `bf05a509b1e41597c823d030f2108c9e9c00ed83`  
**Reviewed At (UTC)**: `2026-11-06T18:08:36Z`  

---

## 1. Packaging, Interpreter, and Dependency Groups (`T035`)

- **Package Metadata (`pyproject.toml`)**:
  - Name: `regression_model_template` (`version = "2.1.1"`).
  - Build backend: `poetry.core.masonry.api` (`poetry-core`), using PEP 621 `[project]` metadata.
  - Supported Python interpreter constraint: `requires-python = ">=3.12,<3.13"`.
  - CLI entrypoint: `regression_model_template = "regression_model_template.scripts:main"`.
- **Dependency Groups**:
  - `[project.dependencies]`: Core ML, data, serving, and observability runtime dependencies (`mlflow >=3.17.2,<4.0.0`, `scikit-learn >=1.5.0,<2.0.0`, `pandas >=2.2.2,<3.0.0`, `pandera >=0.20.1,<0.21.0`, `pyarrow >=17.0.0,<24.0.0`, `pydantic >=2.7.4,<3.0.0`, `dvc >=3.67.1,<4.0.0`, `dvc-s3 >=3.2.1,<4.0.0`, `mlserver >=1.7.1,<2.0.0`, `confluent-kafka >=2.8.2,<3.0.0`, `fastapi >=0.124.0,<1.0.0`, `opentelemetry-* >=1.45.1,<2.0.0`) plus explicit security floors for transitive packages (`cryptography >=50.0.0`, `gitpython >=3.2.1`, `tornado >=6.5.8`, `aiohttp >=3.14.4`, `pillow >=13.0.0`, `brotli >=1.2.1`, `graphql-core >=3.2.12`, `certifi >=2026.10.19`).
  - `[tool.poetry.group.checks.dependencies]`: `bandit`, `coverage`, `mypy`, `pytest`, `pytest-cov`, `pytest-xdist`, `pandera[mypy]`, `ruff`, `pytest-mock`, `pytest-asyncio`.
  - `[tool.poetry.group.commits.dependencies]`: `commitizen`, `pre-commit`.
  - `[tool.poetry.group.dev.dependencies]`: `invoke`, `pip-audit`.
  - `[tool.poetry.group.docs.dependencies]`: `pdoc`.
  - `[tool.poetry.group.notebooks.dependencies]`: `ipykernel`, `nbformat`.

---

## 2. Reproducibility, Lockfile Integrity, and Test Outcomes

- **Lockfile Parity**:
  - Command: `poetry check --lock`
  - Result: `All set!` (exit code `0`). `poetry.lock` is synchronized with `pyproject.toml`.
- **Automated Test Execution**:
  - Command: `poetry run pytest -q` (Python `3.12.4`, `pytest-9.1.1`)
  - Result: **`98 passed, 3 xfailed in 185.86s`** (exit code `0`).
  - Covered security & governance suites include:
    - `tests/controller/test_hitl_governance.py` (`5/5 passed`): Verifies HITL branch protection and `/status`, `/validate`, `/refine` directives.
    - `tests/controller/test_kafka_app_dos.py` (`6/6 passed`): Verifies max rows/columns bounds and rate limiter middleware.
    - `tests/controller/test_kafka_app_security.py` & `test_log_leakage.py` (`6/6 passed`): Verifies input sanitization, security headers, and exception/credential redaction in logs.
- **CI Configuration (`.github/workflows/check.yml`)**:
  - Runs `poetry install --with checks` and `poetry run invoke checks` (`ruff`, `mypy`, `bandit`, `pytest`, `coverage`) on pull requests targeting `main` with `MLFLOW_ALLOW_FILE_STORE="true"`.

---

## 3. Compatibility & Governance Assessment (`FR-026`, `FR-030`, `SC-012`)

1. **Fork Governance**: Documented in `README.md` as an independently maintained derivative of `fmind/mlops-python-package` (MIT license preserved in `LICENSE.txt`), adapted from `bikes` to `regression_model_template` with Kafka inference controllers and Dark Gravity HITL rules.
2. **Resource Claim Nuance**: `README.md` lines 27 & 69 state that the test suite is calibrated for gVisor limits (`<= 30 MiB RAM`). While ADR-008 `development` ValidatingAdmissionPolicies enforce `30Mi` for micro-containers, full `pytest` execution loading `scikit-learn`, `mlflow`, `shap`, and `pandas` requires standard Python scientific memory (`~250-512 MiB` as configured in `infrastructure/agents/sandbox-job.yaml` and `launch_sandbox_pod.rs`). Public claims must distinguish ADR-008 `development` admission policies (`30Mi`) from `factory-sandboxes` execution limits (`512Mi`).

