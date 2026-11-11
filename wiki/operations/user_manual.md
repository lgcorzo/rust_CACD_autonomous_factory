---
iso_doc_type: "Procedure"
iso_viewpoint: "DeploymentView"
type: "operations"
title: "User Manual — Dark Gravity CA/CD Autonomous Factory"
description: "ISO 42010 DeploymentView / ISO 15289 Procedure documentation for User Manual — Dark Gravity CA/CD Autonomous Factory."
tags: ['iso42010', 'okf', 'operations', 'deployment']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "false"
# 2026-11-10 DG-RVP-018 A2: auto-generated "true" withdrawn; pending human review
last_verified_commit: "fbdc5848"
---

# User Manual — Dark Gravity CA/CD Autonomous Factory

> **Purpose**: Comprehensive step-by-step guides for all operational scenarios.
> The generated local walkthrough source is `docs/screencast_walkthrough.md`. The referenced video asset is not present in this checkout.

---

## 1. Issue-Triggered Autonomous Missions

### Step 1: Create a GitHub Issue

Create an issue in the monitored repository with the required labels:

```markdown
Title: Fix compilation error in config.rs module

Labels: autonomous-mission, dark-gravity

Body:
This issue requests an autonomous fix for the compilation error
in the `config.rs` module caused by a missing `Default` implementation.

Resource limits: CPU: 250m, RAM: 30Mi, Timeout: 300s
```

The example uses the `30Mi`/`250m` declared workload cap in the Custom Resource Definition (CRD) and admission policies. The sandbox job builder currently defaults to `512Mi` memory and `500m` CPU; this conflicts with that cap.

**Required labels** (at least one): `autonomous-mission`, `dark-gravity`

### Step 2: Resource Limits Syntax

Include resource limits in the issue body:

```
Resource limits: CPU: <millicores>, RAM: <memory>, Timeout: <seconds>
```

### Step 3: Monitor Progress

The factory will:
1. **Ingest** the issue via PollerDaemonService
2. **Plan** the mission via RustantAgent
3. **Code** the fix via ZeroClawAgent in a gVisor user-space sandbox runtime
4. **Validate** tests. The repository does not establish a SAST gate for this flow.
5. **Review** the changes
6. **Deliver** a PR for human review (human-in-the-loop, or HITL, Vertex 4)

---

## 2. PR Interactive Directive Commands

Tag the bot in any PR comment:

| Command | Example | Description |
|:---|:---|:---|
| `/status` | `@darkgravity /status` | Query factory health and directed acyclic graph (DAG) state |
| `/interact` | `@darkgravity /interact How does the config module work?` | Natural-language query |
| `/refine` | `@darkgravity /refine Fix the off-by-one error in line 42` | Request code edits |
| `/validate` | `@darkgravity /validate` | Request validation; a SAST scanner and its results are not established here |

Also accepts `@dark-gravity` prefix.

---

## 3. CLI trigger_mission Usage

```bash
# Manual mission trigger
factory-cli trigger-mission \
  --repo "lgcorzo/rust_CACD_autonomous_factory" \
  --title "Fix compilation error in config.rs" \
  --labels "autonomous-mission,dark-gravity"

# Run functional test suite
factory-cli run-functional-suite

# Trigger deep knowledge search
factory-cli trigger-deep-search --query "circuit breaker patterns"
```

---

## 4. Environment Configuration

### LiteLLM Model Switching

```bash
# Default agent model
export LITELLM_MODEL="ollama/qwen2.5:7b"

# Planner model (higher capability)
export LITELLM_PLANNER_MODEL="gemini-3.8-flash-low"

# LiteLLM API endpoint. The active deployment endpoint is TO CONFIRM.
export LITELLM_API_BASE="http://litellm.llm-apps.svc.cluster.local:4000/v1"

# FinOps configuration
export FINOPS_MAX_DAILY_BUDGET="50"
export FINOPS_TEAM="dark-gravity-ops"
export FINOPS_EPIC="HAZITEK-2026"
```

### Key Environment Variables

| Variable | Default | Purpose |
|:---|:---|:---|
| `LITELLM_MODEL` | `ollama/qwen2.5:7b` | Default LLM for agents |
| `LITELLM_PLANNER_MODEL` | `gemini-3.8-flash-low` | High-capability planner LLM |
| `LITELLM_API_BASE` | `http://litellm.llm-apps.svc.cluster.local:4000` in the MCP server | API base; deployment endpoint is **TO CONFIRM** |
| `HATCHET_API_URL` | `http://hatchet:8080` | Hatchet orchestrator |
| `FINOPS_MAX_DAILY_BUDGET` | `50` | Daily budget limit; currency is **TO CONFIRM** |
| `FACTORY_MODELS_CONFIG` | — | Custom model config path |

---

## 5. Kubernetes Manifest Setup

The checked-in `deploy/` manifests define no `Namespace` objects. Their admission-policy bindings select the namespace name `development`. Other checked-in configuration refers to `factory-sandbox` and the `openziti` namespace label. Whether these namespaces exist in a deployment, and the complete deployed namespace inventory, are **TO CONFIRM**.

---

> *Related: [Production Operations](production_operations.md) · [HITL Governance](../security/hitl_governance.md)*
