---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-core::config — Factory Model Configuration"
source_path: "crates/factory-core/src/config.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-core::config — Factory Model Configuration."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-core::config — Factory Model Configuration

> **Source**: `crates/factory-core/src/config.rs`  
> **Layer**: Domain (innermost onion layer)  
> **Role**: Centralized configuration management for LLM model endpoints, agent-specific model bindings, and LiteLLM proxy resolution.

---

## Architecture & Class Diagram

```mermaid
classDiagram
    class AgentModelConfig {
        +String default_model
        +String planner_model
        +HashMap~String, String~ agents
        +load() AgentModelConfig
        +from_file(path) Result~AgentModelConfig~
        +get_model(agent_name) &str
        +get_planner_model() &str
        +get_default_model() &str
    }

    class ModelDefaults {
        <<constants>>
        +DEFAULT_FACTORY_MODEL: "ollama/qwen2.5:7b"
        +DEFAULT_PLANNER_MODEL: "gpt-oss-120b"
    }

    AgentModelConfig ..> ModelDefaults : fallbacks to
```

## Configuration Resolution Hierarchy

1. **Environment Variables**:
   - `FACTORY_MODELS_CONFIG`: Path to custom YAML/JSON configuration.
   - `LITELLM_MODEL`: Overrides the default agent model.
   - `LITELLM_PLANNER_MODEL` / `PLANNER_MODEL`: Overrides the planner model.
2. **Configuration Files** (searched in order):
   - `config/models.yaml`, `config/models.yml`, `config/models.json`
   - `models.yaml`, `models.json`
   - `../config/models.yaml`, `../../config/models.yaml`
3. **Hardcoded Fallbacks**:
   - Default Factory Model: `ollama/qwen2.5:7b` (via LiteLLM proxy)
   - Default Planner Model: `gpt-oss-120b`

---

> *Related: [lib.rs](lib.md) · [Tactical Design](../../architecture/tactical_design.md)*
