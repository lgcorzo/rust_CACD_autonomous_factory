# Autogen Team: Autonomous Agentic Core

[![check.yml](https://github.com/lgcorzo/llmops-python-package/actions/workflows/check.yml/badge.svg)](https://github.com/lgcorzo/llmops-python-package/actions/workflows/check.yml)
[![publish.yml](https://github.com/lgcorzo/llmops-python-package/actions/workflows/publish.yml/badge.svg)](https://github.com/lgcorzo/llmops-python-package/actions/workflows/publish.yml)
[![Documentation](https://img.shields.io/badge/documentation-available-brightgreen.svg)](https://lgcorzo.github.io/llmops-python-package/)
[![License](https://img.shields.io/github/license/lgcorzo/llmops-python-package)](https://github.com/lgcorzo/llmops-python-package/blob/main/LICENCE.txt)
[![Release](https://img.shields.io/github/v/release/lgcorzo/llmops-python-package)](https://github.com/lgcorzo/llmops-python-package/releases)

**Autogen Team** has evolved from an MLOps library into a **Long-Term Agentic System** serving as the intelligence core for the **Dark Gravity CA/CD Autonomous Agent Factory**.

Built upon strict **Domain-Driven Design (DDD)** standards and solid **LLMOps principles**, this Python package provides the robust architectural foundation required to orchestrate multi-agent autonomous workloads spanning long-running software development lifecycles.

## 🎯 Architecture & Guiding Principles

### 1. Domain-Driven Design (DDD)
The codebase enforces strict separation of concerns to handle the deep complexity of autonomous multi-agent interactions, ensuring absolute maintainability over the long term.

- **`core/`**: Shared kernel (schemas, business-independent exceptions, security protocols).
- **`application/`**: Orchestrates use-cases without knowing infrastructure details. Includes Hatchet **Workflows** (e.g., `autonomous_mission.py`), the **Agents** themselves, and the **MCP Server**.
- **`infrastructure/`**: The outermost layer connecting to the world. Encompasses A2A (Agent-to-Agent) Kafka messaging protocols, Hatchet API clients, MinIO connections, and Firecracker MicroVM Sandbox Services.
- **Bounded Contexts**: Independent domains such as `models`, `data_access`, `registry`, and `evaluation` operate with dedicated Data Adapter and Repository patterns.

### 2. Solid LLMOps Foundation
While prioritizing long-term autonomous agents, the systemic backbone uses proven LLMOps pipelines. The factory natively supports offline evaluations, asynchronous inferences, and RAG context persistence via **R2R** and **pgvector**.

### 3. CA/CD Autonomous Agent Factory Integration
Within the Dark Gravity Zero-Trust Cluster, this package drives the `opencode` workers.
Missions are managed through **Hatchet** (yielding durable state and parallel "fan-out" execution), utilizing **Model Context Protocol (MCP)** tools for completely automated planning, code execution in sandboxes, testing, and security scanning.

---

## Table of Contents

- [Install](#install)
- [The Agentic CA/CD Factory](#the-agentic-cacd-factory)
- [Legacy Batch & Realtime Inference](#legacy-batch--realtime-inference)
- [Configuration](#configuration)
- [Development Workflow](#development-workflow)
- [Project Structure Map](#project-structure-map)

---

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

---

## The Agentic CA/CD Factory

The flagship implementation of this package is the **Autonomous Mission Workflow** which transforms software requirements into fully reviewed, production-ready Pull Requests.

**Core Agentic Components:**
- **Hatchet Workflow DSL**: `AutonomousMissionWorkflow` handles task DAG planning, fan-out execution of worker tasks in parallel (using `aio_run_many`), and aggregation/review with durable execution state.
- **OpenCode Workers**: Dynamically scaled by **KEDA** based on Kafka queues to act as robust edge-workers executing the Hatchet flows.
- **MCP Server (Model Context Protocol)**: Exposes a standardized suite of deep integrations to the LLMs.
- **Zero Trust Network**: All A2A (Agent to Agent) communication is encrypted and validated dynamically via OpenZiti APIs.

### Available MCP Tools for Agents:
Driven by configuration (`confs/mcp_prompts.yaml`), the MCP exposes:
- `plan_mission`: Break down a goal into a Task DAG (Planner Agent).
- `execute_code`: Generate and inject code changes securely within an ephemeral Sandbox (Coder Agent).
- `run_tests`: Run isolated test suites over PR diffs in a MicroVM (Tester Agent).
- `security_review`: Comprehensive OWASP & RAG-backed vulnerability analysis (Reviewer Agent).
- `retrieve_context` & `index_code`: Semantic queries over the codebase via an R2R Knowledge Graph.

### Running the Agent/MCP Ecosystem Locally:

```bash
# Start the MCP Server
poetry run invoke projects.mcp

# With custom system prompts for agents:
poetry run invoke projects.mcp --prompts=confs/mcp_prompts.yaml
```

---

## Legacy Batch & Realtime Inference

The package still seamlessly supports legacy MLOps topologies managed by OmegaConf configurations.

### CLI (Batch Jobs)
```bash
poetry run autogen_team confs/training.yaml
poetry run autogen_team confs/evaluations.yaml
# etc.
```

### Asynchronous & Realtime Systems
- **Hatchet Inference**: Offload batch inference asynchronously `poetry run invoke projects.run --job hatchet_inference`
- **Kafka Streaming**: Realtime inference streams using Docker Compose stack. Run `docker-compose up -d` (Includes Kafka, MLflow, Inference Service UI on port 8081).

---

## Configuration

We aggressively decouple configuration from code using [OmegaConf](https://omegaconf.readthedocs.io/) and [Pydantic](https://docs.pydantic.dev/). System prompts for the Autonomous team live in `confs/mcp_prompts.yaml`, and job logic in corresponding YAML files.

Access the JSON schema for configurations:
```bash
poetry run autogen_team --schema
```

---

## Development Workflow

Task automation is heavily driven by [Invoke](https://www.pyinvoke.org/).

- **Format**: `inv format` (Ruff)
- **Check (Lint/Type/Test)**: `inv checks` (Mypy, Pytest, Ruff)
- **Comprehensive Lifecycle Test**: `inv all` (E2E job orchestration, Kafka, MCP test boot)
- **List Tasks**: `inv --list`

Ensure you install pre-commit hooks before committing:
```bash
poetry run pre-commit install
```

---

## Project Structure Map

```text
.
├── confs/                  # YAML configurations (Jobs, MCP Prompts)
├── data/                   # Data directory (inputs/targets)
├── src/
│   └── autogen_team/       # Main package source code
│       ├── application/    # Application Layer: Workflows, MCP Server, Agents, Jobs
│       ├── core/           # Core Layer: Shared schemas, common security interfaces
│       ├── data_access/    # Bounded Context: Data adapters & repositories
│       ├── evaluation/     # Bounded Context: Eval metrics & services
│       ├── infrastructure/ # Infrastructure Layer: Hatchet/Minio/Kafka Integrations, Sandbox
│       ├── models/         # Bounded Context: Model registries & entities
│       └── registry/       # Bounded Context: Loggers and state registry
```
