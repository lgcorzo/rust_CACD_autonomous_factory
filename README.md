# Autogen Team

[![check.yml](https://github.com/lgcorzo/llmops-python-package/actions/workflows/check.yml/badge.svg)](https://github.com/lgcorzo/llmops-python-package/actions/workflows/check.yml)
[![publish.yml](https://github.com/lgcorzo/llmops-python-package/actions/workflows/publish.yml/badge.svg)](https://github.com/lgcorzo/llmops-python-package/actions/workflows/publish.yml)
[![Documentation](https://img.shields.io/badge/documentation-available-brightgreen.svg)](https://lgcorzo.github.io/llmops-python-package/)
[![License](https://img.shields.io/github/license/lgcorzo/llmops-python-package)](https://github.com/lgcorzo/llmops-python-package/blob/main/LICENCE.txt)
[![Release](https://img.shields.io/github/v/release/lgcorzo/llmops-python-package)](https://github.com/lgcorzo/llmops-python-package/releases)

**Autogen Team** is a comprehensive MLOps and Agentic Framework Python package. It is designed to streamline the lifecycle of machine learning models and AI agents, supporting tasks such as training, tuning, evaluation, promotion, and inference (both batch and realtime).

Key features include:
- **MLOps Workflow:** Managed jobs for training, tuning, and promoting models.
- **Inference:** Support for both batch inference (via CLI) and realtime inference (via Kafka).
- **Agent Framework:** Integration with [AutoGen Studio](https://microsoft.github.io/autogen/) for building and managing multi-agent workflows.
- **Configuration Driven:** flexible execution using YAML configuration files.

## Table of Contents

- [Install](#install)
- [Usage](#usage)
  - [CLI (Batch Jobs)](#cli-batch-jobs)
  - [Realtime Inference (Kafka)](#realtime-inference-kafka)
  - [AutoGen Studio](#autogen-studio)
- [Configuration](#configuration)
- [Development](#development)
- [Project Structure](#project-structure)

## Install

### Prerequisites

- [Python >= 3.10](https://www.python.org/downloads/)
- [Poetry >= 1.8.2](https://python-poetry.org/)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/lgcorzo/llmops-python-package
   cd llmops-python-package
   ```

2. Install dependencies with Poetry:
   ```bash
   poetry install
   ```

## Usage

### CLI (Batch Jobs)

You can execute various MLOps jobs using the `autogen_team` CLI tool. The behavior is defined by configuration files located in the `confs/` directory.

```bash
# Run a training job
poetry run autogen_team confs/training.yaml

# Run a hyperparameter tuning job
poetry run autogen_team confs/tuning.yaml

# Run a model promotion job
poetry run autogen_team confs/promotion.yaml

# Run a batch inference job
poetry run autogen_team confs/inference.yaml

# Run evaluations
poetry run autogen_team confs/evaluations.yaml

# Run explanations (SHAP)
poetry run autogen_team confs/explanations.yaml
```

To see the configuration schema:
```bash
poetry run autogen_team --schema
```

### Realtime Inference (Kafka)

For realtime inference, the project provides a Kafka-integrated service. This is best run using Docker Compose.

The service listens to an input Kafka topic, runs predictions using the "Champion" model from MLflow, and publishes results to an output topic.

```bash
docker-compose up -d
```

This will start:
- **Kafka Service:** For messaging.
- **MLflow Service:** For model registry and tracking.
- **Inference Service:** The `autogen_team` container running the Kafka consumer/producer.

**Service Configuration (in `docker-compose.yml`):**
- `DEFAULT_INPUT_TOPIC`: Topic to consume data from.
- `DEFAULT_OUTPUT_TOPIC`: Topic to produce predictions to.
- `MLFLOW_TRACKING_URI`: URL of the MLflow server.

### AutoGen Studio

To run the AutoGen Studio UI for managing agents:

```bash
poetry run autogenstudio ui --port 8081
```

Access the UI at `http://localhost:8081`.

## Configuration

The project uses [OmegaConf](https://omegaconf.readthedocs.io/) and [Pydantic](https://docs.pydantic.dev/) for configuration management. Configuration files are located in `confs/`.

**Example `confs/training.yaml`:**
```yaml
job:
  KIND: TrainingJob
  inputs:
    KIND: ParquetReader
    path: data/inputs_train.parquet
  targets:
    KIND: ParquetReader
    path: data/targets_train.parquet
```

This configuration tells the application to run a `TrainingJob` using specified Parquet files for inputs and targets.

## Development

This project uses [Invoke](https://www.pyinvoke.org/) for task automation.

**Common Tasks:**

- **Check code quality:**
  ```bash
  inv checks
  ```
  This runs linting (Ruff), formatting (Ruff), type checking (Mypy), and testing (Pytest).

- **Format code:**
  ```bash
  inv format
  ```

- **Run tests:**
  ```bash
  inv checks.test
  ```

- **List all tasks:**
  ```bash
  inv --list
  ```

### Pre-commit Hooks

We use pre-commit hooks to ensure code quality before committing.
```bash
poetry run pre-commit install
```

## Project Structure

```
.
├── confs/                  # Configuration files (YAML)
├── data/                   # Data directory (inputs/targets)
├── src/
│   └── autogen_team/       # Main package source code
│       ├── controller/     # FastAPI/Kafka controllers
│       ├── core/           # Core logic (schemas, models)
│       ├── io/             # I/O utilities (datasets, registries, services)
│       ├── jobs/           # Job implementations (training, inference, etc.)
│       └── scripts.py      # CLI entry point
├── tasks/                  # Invoke tasks definitions
├── tests/                  # Unit and integration tests
├── Dockerfile              # Docker image definition
├── docker-compose.yml      # Docker Compose configuration
├── pyproject.toml          # Project metadata and dependencies
└── README.md               # Project documentation
```
