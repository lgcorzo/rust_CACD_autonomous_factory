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

> *Related: [CLI main.rs](crates_factory-cli_src_main.md) · [autonomous_mission.rs](crates_factory-application_src_workflows_autonomous_mission.md)*
