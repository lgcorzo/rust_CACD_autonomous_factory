# User Manual — Dark Gravity CA/CD Autonomous Factory

> **Purpose**: Comprehensive step-by-step guides for all operational scenarios.

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

Resource limits: CPU: 500m, RAM: 512Mi, Timeout: 300s
```

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
3. **Code** the fix via ZeroClawAgent in a gVisor sandbox
4. **Validate** tests and SAST gate
5. **Review** the changes
6. **Deliver** a PR for human review (HITL Vertex 4)

---

## 2. PR Interactive Directive Commands

Tag the bot in any PR comment:

| Command | Example | Description |
|:---|:---|:---|
| `/status` | `@darkgravity /status` | Query factory health and DAG state |
| `/interact` | `@darkgravity /interact How does the config module work?` | Natural-language query |
| `/refine` | `@darkgravity /refine Fix the off-by-one error in line 42` | Request code edits |
| `/validate` | `@darkgravity /validate` | Run full test + SAST verification |

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
export LITELLM_PLANNER_MODEL="gpt-oss-120b"

# LiteLLM API endpoint
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
| `LITELLM_PLANNER_MODEL` | `gpt-oss-120b` | High-capability planner LLM |
| `LITELLM_API_BASE` | `http://litellm:4000/v1` | LiteLLM proxy endpoint |
| `HATCHET_API_URL` | `http://hatchet:8080` | Hatchet orchestrator |
| `FINOPS_MAX_DAILY_BUDGET` | `50` | Daily spend limit (USD) |
| `FACTORY_MODELS_CONFIG` | — | Custom model config path |

---

## 5. Kubernetes Manifest Setup

Deploy via FluxCD GitOps:

```yaml
# K8s namespace layout
apiVersion: v1
kind: Namespace
metadata:
  name: dark-gravity
  labels:
    app.kubernetes.io/part-of: dark-gravity
---
# Core namespaces:
# - dark-gravity: Factory services
# - orchestrators: Hatchet, Kafka
# - llm-apps: LiteLLM, R2R
# - sandbox-exec: gVisor agent pods
# - observability: Sentry, Grafana
```

---

> *Related: [Production Operations](PRODUCTION-OPERATIONS.md) · [HITL Governance](HITL-GOVERNANCE.md)*
