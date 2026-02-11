# Autogen Team

[![check.yml](https://github.com/lgcorzo/llmops-python-package/actions/workflows/check.yml/badge.svg)](https://github.com/lgcorzo/llmops-python-package/actions/workflows/check.yml)
[![publish.yml](https://github.com/lgcorzo/llmops-python-package/actions/workflows/publish.yml/badge.svg)](https://github.com/lgcorzo/llmops-python-package/actions/workflows/publish.yml)
[![Documentation](https://img.shields.io/badge/documentation-available-brightgreen.svg)](https://lgcorzo.github.io/llmops-python-package/)
[![License](https://img.shields.io/github/license/lgcorzo/llmops-python-package)](https://github.com/lgcorzo/llmops-python-package/blob/main/LICENCE.txt)
[![Release](https://img.shields.io/github/v/release/lgcorzo/llmops-python-package)](https://github.com/lgcorzo/llmops-python-package/releases)

**Autogen Team** is a comprehensive MLOps and Agentic Framework Python package. It is designed to streamline the lifecycle of machine learning models and AI agents, supporting tasks such as training, tuning, evaluation, promotion, and inference (both batch and realtime).

Key features include:

- **MLops Workflow:** Managed jobs for training, tuning, and promoting models.
- **Inference:** Support for both batch inference (via CLI), asynchronous managed inference (via Hatchet), and realtime inference (via Kafka).
- **Orchestration:** Integrated with [Hatchet](https://hatchet.run/) for managing distributed and long-running ML workflows with built-in retries and monitoring.
- **Agent Framework:** Integration with [AutoGen Studio](https://microsoft.github.io/autogen/) for building and managing multi-agent workflows.
- **Autonomous AI Tools:** Standardized [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server providing tools for mission planning, code generation, sandboxed testing, and security analysis.
- **Configuration Driven:** flexible execution using YAML configuration files.

## Table of Contents

- [Autogen Team](#autogen-team)
  - [Table of Contents](#table-of-contents)
  - [Install](#install)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
  - [Usage](#usage)
    - [CLI (Batch Jobs)](#cli-batch-jobs)
    - [Asynchronous Inference (Hatchet)](#asynchronous-inference-hatchet)
    - [Realtime Inference (Kafka)](#realtime-inference-kafka)
    - [AutoGen Studio](#autogen-studio)
  - [Configuration](#configuration)
  - [Development](#development)
    - [Pre-commit Hooks](#pre-commit-hooks)
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

# Run realtime inference (Kafka)
poetry run invoke projects.kafka

# Run MCP server
poetry run invoke projects.mcp

# Run with a custom configuration
poetry run invoke projects.mcp --prompts=confs/mcp_prompts.yaml
```

To see the configuration schema:

```bash
poetry run autogen_team --schema
```

### Asynchronous Inference (Hatchet)

For long-running inference tasks, you can use the Hatchet-powered asynchronous execution. This job triggers a workflow in the Hatchet engine and returns immediately, allowing for independent processing.

```bash
# Run asynchronous inference
poetry run invoke projects.run --job hatchet_inference
```

**Requirements:**

- Hatchet SDK configuration (`HATCHET_CLIENT_TOKEN`, `HATCHET_NAMESPACE`).
- A running Hatchet worker listening for the `InferenceWorkflow`.

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

### MCP Server (Model Context Protocol)

The project includes an MCP server that exposes 6 AI-powered tools for mission planning, code execution, testing, and security analysis.

**Available Tools:**
- `plan_mission`: Decompose goals into task DAGs.
- `execute_code`: Generate and validate code changes.
- `run_tests`: Run pytest in an isolated sandbox.
- `security_review`: OWASP & RAG-based security analysis.
- `retrieve_context`: Semantic search via R2R RAG.
- `index_code`: Index files into R2R knowledge graph.

**Running Locally:**
```bash
# Option 1: Using Invoke (Recommended)
poetry run invoke projects.mcp
# Run with a custom prompts config
poetry run invoke projects.mcp --prompts=confs/custom_prompts.yaml

# Option 2: Direct execution
poetry run python -m autogen_team.application.mcp.mcp_server
```

**Customizing Prompts:**
The agent's system prompts and instructions are configuration-driven and can be customized in **[confs/mcp_prompts.yaml](file:///home/lgcorzo/llmops-python-package/confs/mcp_prompts.yaml)**. This allows tuning the behavior of `plan_mission`, `execute_code`, and `security_review` without code changes.

**Running in Kubernetes:**
The server is deployed in the `agents` namespace. It uses `LITELLM_API_BASE` and `R2R_BASE_URL` for backend integrations.

```bash
kubectl apply -k k8s/base/
```

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
  inv checks
  ```

- **List all tasks:**

  ```bash
  inv --list
  ```

- **Run all project tasks (Orchestration):**

  ```bash
  inv all
  ```
  This orchestrates the entire lifecycle: environment setup, MLOps jobs, evaluations, the **Kafka inference service**, and the **MCP server**.

- **Run app (Kafka standalone):**

  ```bash
  poetry run python src/autogen_team/controller/kafka_app.py
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
│       ├── application/    # Application layer (jobs and MCP server)
│       │   └── mcp/        # MCP Server implementation and tools
│       ├── core/           # Shared kernel (schemas, exceptions)
│       ├── data_access/    # Data access domain (adapters, repositories)
│       ├── evaluation/     # Evaluation domain (metrics, services)
│       ├── infrastructure/ # Infrastructure layer (io, services, messaging)
│       ├── models/         # Models domain (entities, repositories)
│       ├── registry/       # Registry domain (adapters, repositories)
│       ├── jobs/           # Backward compatibility re-exports
│       ├── settings.py     # Application settings
│       └── scripts.py      # CLI entry point
├── tasks/                  # Invoke tasks definitions
├── tests/                  # Unit and integration tests
├── Dockerfile              # Docker image definition
├── docker-compose.yml      # Docker Compose configuration
├── pyproject.toml          # Project metadata and dependencies
└── README.md               # Project documentation
```
