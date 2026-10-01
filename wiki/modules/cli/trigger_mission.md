---
iso_doc_type: "Specification"
iso_viewpoint: "ComponentView"
type: "module"
title: "factory-cli::trigger_mission — Manual Mission Trigger"
source_path: "crates/factory-cli/src/bin/trigger_mission.rs"
description: "ISO 42010 ComponentView / ISO 15289 Specification documentation for factory-cli::trigger_mission — Manual Mission Trigger."
tags: ['iso42010', 'okf', 'component_view', 'rust', 'ast']
timestamp: "2026-10-01T14:00:00Z"
generated: "agent:okf-professional-documenter"
verified: "true"
last_verified_commit: "fbdc5848"
---

# factory-cli::trigger_mission — Manual Mission Trigger

> **Source**: `crates/factory-cli/src/bin/trigger_mission.rs`
> **Layer**: Interface

---

Manually triggers an autonomous mission by creating a `PolledIssueEvent` and dispatching it to the Hatchet DAG, bypassing the poller. Useful for development and testing.

## Usage

```bash
factory-cli trigger-mission \
  --repo "lgcorzo/rust_CACD_autonomous_factory" \
  --title "Fix compilation error in config.rs" \
  --labels "autonomous-mission,dark-gravity"
```

---

> *Related: [CLI main.rs](main.md) · [autonomous_mission.rs](../application/workflows/autonomous_mission.md)*
