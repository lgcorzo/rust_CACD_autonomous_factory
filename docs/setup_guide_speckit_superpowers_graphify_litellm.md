# Comprehensive Setup Guide: Spec Kit, Superpowers, Graphify, and LiteLLM Gateway Integration

This document provides a step-by-step installation, configuration, and operational guide for integrating **Spec Kit**, **Superpowers**, and **Graphify** powered by an internal **LiteLLM Gateway** (with models like `ollama/qwen2.5:7b` and `ollama/qwen2.5-coder:7b`).

---

## Architecture Overview

```mermaid
flowchart TD
    Agent[Coding Agent / Antigravity] --> Superpowers[Superpowers Plugin]
    Agent --> SpecKit[Spec Kit Workflow]
    Agent --> Graphify[Graphify Knowledge Graph]

    Graphify -- LLM Semantic Extraction --> LiteLLM[LiteLLM Gateway Proxy]
    SpecKit -- Specification & Planning --> PlanDocs[spec.md / plan.md / tasks.md]
    Superpowers -- Behavioral Skills --> TDD[TDD / Systematic Debugging / Verification]

    LiteLLM -- TLS Proxy --> Nginx[Nginx Reverse Proxy]
    Nginx -- Model Routing --> Ollama[Kubernetes Ollama Service]
    Ollama --> Qwen[ollama/qwen2.5:7b / qwen2.5-coder:7b]
```

---

## 1. LiteLLM Gateway Setup & Model Configuration

The LiteLLM Gateway acts as a central OpenAI-compatible endpoint (`https://litellm.ziti/v1` or `http://litellm.llm-apps.svc.cluster.local/v1`) providing unified access to local and cloud LLMs.

### 1.1 LiteLLM Model Routing ConfigMap

In `gitops_internal_lgcorzo/infrastructure/llm-apps/litellm/litellm-config-cm.yaml`:

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: litellm-config
  namespace: llm-apps
data:
  config.yaml: |
    model_list:
      - model_name: ollama/qwen2.5:7b
        litellm_params:
          model: ollama_chat/qwen2.5:7b
          api_base: "http://ollama-service.llm-apps.svc.cluster.local:11434"
          api_key: "EMPTY"
          num_ctx: 16384

      - model_name: ollama/qwen2.5-coder:7b
        litellm_params:
          model: ollama_chat/qwen2.5-coder:7b
          api_base: "http://ollama-service.llm-apps.svc.cluster.local:11434"
          api_key: "EMPTY"
          num_ctx: 32768
```

### 1.2 Nginx Reverse Proxy ConfigMap (`client_max_body_size`)

To prevent `HTTP 413 Request Entity Too Large` errors when Graphify or Spec Kit sends large context batches:

In `gitops_internal_lgcorzo/infrastructure/llm-apps/litellm/litellm-nginx-config-cm.yaml`:

```nginx
http {
    include       mime.types;
    default_type  application/octet-stream;
    sendfile        on;
    keepalive_timeout  65;
    client_max_body_size 100M;  # Required for large LLM payloads
    ...
}
```

### 1.3 LiteLLM Environment Variables

Set the following environment variables in your shell or agent environment:

```bash
export OPENAI_BASE_URL="https://litellm.ziti/v1"
export OPENAI_API_KEY="sk-1234"
export OPENAI_MODEL="ollama/qwen2.5:7b"
export LITELLM_API_BASE="https://litellm.ziti/v1"
```

---

## 2. Graphify Setup

`graphify` parses your repository into a navigable AST knowledge graph and uses LiteLLM Gateway models for semantic extraction over documentation and code architecture.

### 2.1 Installation

Install `graphify` using `uv` or `pip`:

```bash
uv tool install graphifyy
# or: pip install graphifyy
```

### 2.2 Register Agent Hooks for Graphify

To integrate Graphify into your coding assistant environment:

```bash
graphify antigravity install
# or for Codex: graphify codex install
```

### 2.4 AST-Only Graph Update (Free / No LLM Cost)

To update code symbols, dependencies, and file relationships without invoking LLMs:

```bash
graphify update .
```

### 2.5 Semantic Graph Extraction via LiteLLM Gateway

When running full semantic extraction over documentation using `ollama/qwen2.5:7b` via LiteLLM:

```bash
python3 -c '
import httpx, urllib3
urllib3.disable_warnings()
# Bypass internal self-signed TLS verification for LiteLLM gateway
_orig = httpx.Client.__init__
httpx.Client.__init__ = lambda self, *a, **kw: _orig(self, *a, **{**kw, "verify": False})

import os, sys
os.environ["OPENAI_BASE_URL"] = "https://litellm.ziti/v1"
os.environ["OPENAI_MODEL"] = "ollama/qwen2.5:7b"
os.environ["OPENAI_API_KEY"] = "sk-1234"

from graphify.__main__ import main
sys.argv = ["graphify", "extract", ".", "--backend", "openai", "--token-budget", "15000"]
main()
'
```

---

## 3. Superpowers Plugin Setup

Superpowers provides behavioral skills for structured development, plan execution, and verification.

### 3.1 Installation

Ensure Superpowers is placed in your global customization root (`~/.gemini/config/plugins/superpowers`) or project customization root (`.agents/`):

```bash
mkdir -p ~/.gemini/config/plugins/superpowers
```

### 3.2 Key Superpowers Skills

| Skill | Purpose |
| :--- | :--- |
| `brainstorming` | Explores user intent, requirements, and design choices prior to writing code. |
| `writing-plans` | Converts feature requirements into actionable `implementation_plan.md`. |
| `executing-plans` | Executes implementation plans with review checkpoints. |
| `subagent-driven-development` | Dispatches independent tasks to specialized parallel subagents. |
| `systematic-debugging` | Enforces root-cause investigation before applying bug fixes. |
| `test-driven-development` | Enforces Red-Green-Refactor TDD cycle. |
| `verification-before-completion` | Requires empirical proof (tests passing, clean builds) before marking work complete. |

---

## 4. Spec Kit Workflow Setup

Spec Kit standardizes specification-driven development using `.specify/` specs, implementation plans, and ordered task lists.

### 4.1 Installation & Superpowers Bridge Integration

Add Spec Kit to your environment:
```bash
uv tool install specify-cli --from git+https://github.com/github/spec-kit.git
specify init --here --integration agy
```

#### Connecting Spec-Kit with Superpowers via Superpowers Bridge (`speckit-superpowers-bridge`)

**Superpowers Implementation Bridge** (`speckit-superpowers-bridge` v1.1.0) is the thin orchestrator between **Spec Kit** (design) and **Superpowers** (implementation). Installed into `.specify/extensions/speckit-superpowers-bridge/`, it exposes 3 specialized commands:

- `speckit.speckit-superpowers-bridge.handoff` — Create or update the Superpowers implementation handoff state.
- `speckit.speckit-superpowers-bridge.guard` — Guard Spec Kit and Superpowers commands from crossing bridge ownership boundaries.
- `speckit.speckit-superpowers-bridge.execute` — Execute Spec Kit `tasks.md` through the Superpowers bridge.

##### Option A: Native Spec Kit Extension CLI (Recommended)

Install directly via the Spec Kit extension CLI:

```bash
specify extension add speckit-superpowers-bridge
```

Verify installation:
```bash
specify extension list
```

##### Option B: Manual Archive Extraction (Fallback)

If installing offline or directly from the source repository:
```bash
mkdir -p .specify/extensions/speckit-superpowers-bridge
curl -L https://github.com/github/spec-kit/archive/refs/heads/main.zip -o spec-kit.zip
unzip -o spec-kit.zip -d /tmp/speckit_temp
cp -r /tmp/speckit_temp/spec-kit-main/extensions/speckit-superpowers-bridge/* .specify/extensions/speckit-superpowers-bridge/
rm -rf spec-kit.zip /tmp/speckit_temp
```

### 4.2 Repository Configuration

Add Spec Kit artifacts to `.gitignore` if scratch specs shouldn't be committed to version control:

```gitignore
# Spec Kit
.specify/
.agents/
```

### 4.3 Spec Kit Command Lifecycle

```mermaid
sequenceDiagram
    participant User
    participant Agent
    participant SpecKit

    User->>Agent: Request new feature
    Agent->>SpecKit: speckit-specify (Create spec.md)
    Agent->>SpecKit: speckit-plan (Generate plan.md & architecture)
    Agent->>SpecKit: speckit-tasks (Generate ordered tasks.md)
    Agent->>SpecKit: speckit-analyze (Cross-artifact consistency check)
    Agent->>SpecKit: speckit-implement (Execute tasks using TDD & Verification)
```

1. `/speckit-specify` — Create or update feature specification.
2. `/speckit-plan` — Generate design artifacts and implementation plan.
3. `/speckit-tasks` — Build dependency-ordered `tasks.md`.
4. `/speckit-analyze` — Run quality check across `spec.md`, `plan.md`, and `tasks.md`.
5. `/speckit-implement` — Execute tasks sequentially with automated verification.

### 4.4 Getting Started

1. **Initialize Project Governance**
   ```bash
   /speckit.constitution MyProject
   ```
   This creates the `.specify/` directory structure and interviews you about core principles, technology stack, and quality gates.

2. **Write Your First Spec**
   ```bash
   /speckit.specify "User authentication with email and password"
   ```
   This creates a feature specification at `specs/001-user-authentication/spec.md` with user stories, requirements, and success criteria.

3. **Brainstorm & Refine Requirements**
   ```bash
   /speckit.clarify                         # Eliminate ambiguities in spec.md
   ```
   Or invoke Superpowers' `brainstorming` skill directly to discover boundary conditions, error scenarios, security concerns, and UX pitfalls.

4. **Plan, Task, Guard & Execute**
   ```bash
   /speckit.plan                                        # Create technical implementation plan
   /speckit.tasks                                       # Generate ordered task breakdown
   /speckit.speckit-superpowers-bridge.handoff          # Create/update implementation handoff state
   /speckit.speckit-superpowers-bridge.guard            # Enforce bridge ownership boundaries
   /speckit.speckit-superpowers-bridge.execute          # Execute tasks.md with TDD discipline & subagents
   ```

---

## 5. Integrated End-to-End Development Workflow

When all components (**LiteLLM Gateway**, **Graphify**, **Superpowers**, **Spec-Kit**, and **Superpowers Bridge**) are installed and configured, the autonomous factory operates under a unified 5-phase execution workflow:

```mermaid
flowchart TD
    Phase1[1. Context & Exploration<br/>Graphify + LiteLLM] --> Phase2[2. Design & Alignment<br/>Superpowers Brainstorming]
    Phase2 --> Phase3[3. Specification & Planning<br/>Spec-Kit + Superpowers Bridge]
    Phase3 --> Phase4[4. TDD & Implementation<br/>Superpowers TDD & Debugging]
    Phase4 --> Phase5[5. Empirical Verification & AST Refresh<br/>Verification + Graphify Update]

    subgraph Infrastructure Layer
        LiteLLM[LiteLLM Gateway / Ollama Models]
    end

    Phase1 -. Model Queries .-> LiteLLM
    Phase3 -. LLM Extraction .-> LiteLLM
```

### Phase-by-Phase Execution Guide

#### Phase 1: Context & Architecture Exploration
- **Tools**: `Graphify`, `LiteLLM Gateway`
- **Action**: Before modifying code, query the AST Knowledge Graph to inspect existing symbols, callers, and module boundaries without consuming high token counts.
- **Commands**:
  ```bash
  graphify query "Explain the architecture and main dependencies of module X"
  ```

#### Phase 2: Requirements & Design Alignment
- **Tools**: `Superpowers` (`brainstorming` skill)
- **Action**: Explore user intent, business requirements, and potential edge cases before writing formal specs. Clarify architectural choices and trade-offs.

#### Phase 3: Specification & Task Planning
- **Tools**: `Spec-Kit`, `Superpowers Bridge` (`speckit-superpowers-bridge` installed into `.specify/extensions/speckit-superpowers-bridge/`)
- **Action**: Run the Spec-Kit pipeline. Superpowers Bridge injects Superpowers guidelines into each artifact generation step, ensuring strict governance and zero hallucinations.
- **Pipeline Execution**:
  1. `/speckit-specify` — Define business intent and requirement contracts in `spec.md`.
  2. `/speckit-plan` — Design system architecture and component structure in `plan.md`.
  3. `/speckit-tasks` — Decompose requirements into atomic, dependency-ordered tasks in `tasks.md`.
  4. `/speckit-analyze` — Execute cross-artifact consistency validation across `spec.md`, `plan.md`, and `tasks.md`.

#### Phase 4: Test-Driven Implementation & Debugging
- **Tools**: `Superpowers` (`test-driven-development`, `systematic-debugging`), `Spec-Kit` (`/speckit-implement`)
- **Action**: Execute tasks sequentially from `tasks.md`:
  - **TDD Cycle**: Write failing tests first (Red), implement code to pass (Green), and refactor cleanly (Refactor).
  - **Systematic Debugging**: If tests or compilation fail, analyze logs and trace root causes before modifying code.

#### Phase 5: Verification & Knowledge Graph Synchronization
- **Tools**: `Superpowers` (`verification-before-completion`), `Graphify` (`graphify update .`)
- **Action**:
  1. Run automated build and test suites (`cargo test`, `pytest`, `npm test`) to obtain empirical proof of success.
  2. Execute AST graph update to keep the project's knowledge graph in sync with the new code changes without incurring LLM API costs:
     ```bash
     graphify update .
     ```

---

## 6. Verification & Best Practices

1. **Verify Gateway Status**:
   ```bash
   curl -k -H "Authorization: Bearer sk-1234" https://litellm.ziti/v1/models
   ```
2. **Always Run AST Updates After Code Changes**:
   ```bash
   graphify update .
   ```
3. **Enforce Verification Before Completion**:
   Never mark tasks complete without running builds and unit tests (`cargo test`, `pytest`, `npm test`).


---

## 7. Command-by-Command Reference Guide

Here is the complete command-by-command reference guide for your combined architecture (**Spec-Kit** + **Spec Kit Superpowers Bridge** + **Superpowers** + **Graphify**).

This sequence outlines the exact command you call at each step, what happens behind the scenes, and how the engines coordinate to maintain absolute discipline without burning tokens.

### Step 0: Initialize Structural Memory (The Foundation)
Before you touch any feature specification, give your agent a structural map of the existing repository using Graphify.

- **Command**: `/graphify .` (or `graphify .` in terminal depending on your agent runtime)
- **What it does**: Uses local tree-sitter AST parsing (zero vector databases, zero cloud telemetry) to map your code, SQL schemas, and config files into local queryable files (`graphify-out/graph.json`, `GRAPH_REPORT.md`, and `graph.html`).
- **Why it matters**: Ensures your agent has instant topological awareness of dependencies, modules, and "god nodes" from second zero.

### Step 1: Establish Project Governance
Set the constitutional ground rules that apply to the entire application.

- **Command**: `/speckit.constitution`
- **What it does**: Scaffolds `.specify/memory/constitution.md` to define project-wide coding standards, testing rules, and architectural boundaries.
- **Bridge Behavior**: Spec-Kit securely owns this governance asset; no other tool can overwrite it.

### Step 2: Define Feature Requirements
Define what you want to build and why, independent of the tech stack.

- **Command**: `/speckit.specify` (followed by your feature description)
- **What it does**: Creates an isolated working branch and generates a clear user-focused requirements document at `specs/[feature-id]/spec.md`.

### Step 3: Socratic Clarification & Edge-Case Deep-Dive
Before locking down a technical plan, eliminate ambiguity.

- **Command**: `/speckit.clarify` (or Superpowers `brainstorming` skill)
- **What it does**: Invokes interactive Socratic dialogue to walk you through edge cases, failure modes, and trade-offs section by section before any code is written.

### Step 4: Technical Planning & Sizing
Map out the exact implementation strategy.

- **Command**: `/speckit.plan`
- **What it does**: Generates `specs/[feature-id]/plan.md`, specifying the tech stack, data contracts, and architecture.
- **Graphify Integration Synergy**: During this step, the agent queries `graph.json` via Graphify to ensure the new plan respects existing dependency boundaries rather than proposing conflicting patterns.

### Step 5: Atomic Task Decomposition
Break the plan down into manageable chunks.

- **Command**: `/speckit.tasks`
- **What it does**: Translates `plan.md` into an atomic task checklist inside `specs/[feature-id]/tasks.md`, structured into bite-sized steps complete with explicit ordering and test dependencies.

### Step 6: Bridge Guard, Handoff & Disciplined Execution (The TDD Core)
This is where `speckit-superpowers-bridge` orchestrates Superpowers' strictest engineering habits over Spec Kit tasks.

- **Commands**: 
  - `/speckit.speckit-superpowers-bridge.handoff` — Create/update the Superpowers implementation handoff state.
  - `/speckit.speckit-superpowers-bridge.guard` — Enforce ownership boundaries between design artifacts and implementation files.
  - `/speckit.speckit-superpowers-bridge.execute` — Execute Spec Kit `tasks.md` through the Superpowers bridge.
- **What it does**:
  1. **Git Worktree Isolation**: Automatically provisions an isolated workspace (`using-git-worktrees`) so your main branch stays completely clean.
  2. **Test-Driven Development (TDD)**: Forces a strict Red-Green-Refactor loop for every single task (`test-driven-development`). Any implementation code written before a failing unit test is automatically rejected or deleted.
  3. **Subagent-Driven Iteration**: Dispatches focused subagents (`subagent-driven-development`) to tackle tasks sequentially while cross-checking code quality.

### Step 7: Convergence & Multi-Angle Code Review
Validate that the code matches the specification contract down to the letter.

- **Command**: Superpowers `requesting-code-review` / `verification-before-completion`
- **What it does**: Assesses the codebase against `spec.md`, `plan.md`, and `tasks.md`. If converged cleanly, Superpowers initiates branch finishing (`finishing-a-development-branch`) to verify all system tests pass before queuing up the Pull Request.

---

### Summary Checklist for Daily Use

1. **Initialize/Refresh Memory**: `/graphify .`
2. **Specify Intent**: `/speckit.specify`
3. **Refine Design & Plan**: `/speckit.plan` → `/speckit.tasks`
4. **Handoff & Guard**: `/speckit.speckit-superpowers-bridge.handoff` → `/speckit.speckit-superpowers-bridge.guard`
5. **Execute with TDD**: `/speckit.speckit-superpowers-bridge.execute`
6. **Review & Merge**: Superpowers verification & finish branch

References:
- https://dev.to/mir_mursalin_ankur/graphify-code-review-graph-build-a-self-updating-knowledge-graph-for-claude-code-and-other-ai-j1m
