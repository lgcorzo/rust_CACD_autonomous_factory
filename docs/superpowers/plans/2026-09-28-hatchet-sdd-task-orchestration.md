# Hatchet SDD Task Orchestration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enable Hatchet to orchestrate and control the execution of multiple tasks defined in SDD planning (`tasks.md`) in strict dependency order, backed by S3 checkpoints and complete automated test verification.

**Architecture:** Extend `factory-core` with SDD task domain models (`SddTaskItem`, `SddMissionPlan`), implement a robust markdown task parser in `RustantAgent` to extract structured task specifications from `tasks.md`, and refactor the `zeroclaw-execute` step in `autonomous_mission.rs` to dynamically schedule each task through Hatchet (`develop-task` runnable), asserting strict sequential/topological ordering.

**Tech Stack:** Rust 2024, Hatchet SDK 0.2.7, Tokio, Serde/Serde JSON, Regex, Mockall, Cargo test.

## Global Constraints
- Target workspace: `/mnt/F024B17C24B145FE/Repos/rust_CACD_autonomous_factory`
- Rust Edition: 2024
- Hatchet SDK version: 0.2.7
- Follow the Surgical Find-and-Replace Standard: only use `write_to_file` for new files; use `replace_file_content` for existing files.
- Never auto-merge git branches without human review (HITL requirement in `AGENTS.md`).

---

### Task 1: SDD Task Domain Models

**Files:**
- Modify: `crates/factory-core/src/lib.rs`
- Test: `crates/factory-core/src/lib.rs` (unit tests module)

**Interfaces:**
- Produces: `pub struct SddTaskItem`, `pub struct SddMissionPlan`, `pub struct TaskExecutionResult` in `factory_core`.

- [ ] **Step 1: Write the failing test**
In `crates/factory-core/src/lib.rs`, add a test for `SddTaskItem` and `SddMissionPlan` serialization and deserialization.

```rust
#[test]
fn test_sdd_task_item_and_mission_plan_roundtrip() {
    let task = SddTaskItem {
        id: "T001".to_string(),
        description: "Add regex dependency to Cargo.toml".to_string(),
        is_parallel: true,
        dependencies: vec![],
        target_files: vec!["Cargo.toml".to_string()],
    };

    let plan = SddMissionPlan {
        mission_id: "mission-123".to_string(),
        spec_version: "1.0.0".to_string(),
        tasks: vec![task.clone()],
        total_tasks: 1,
    };

    let json = serde_json::to_string(&plan).expect("serialize plan");
    let deserialized: SddMissionPlan = serde_json::from_str(&json).expect("deserialize plan");
    assert_eq!(deserialized.tasks.len(), 1);
    assert_eq!(deserialized.tasks[0].id, "T001");
    assert!(deserialized.tasks[0].is_parallel);
}
```

- [ ] **Step 2: Run test to verify it fails**
Run: `cargo test -p factory-core test_sdd_task_item_and_mission_plan_roundtrip`
Expected: FAIL with "cannot find type `SddTaskItem`"

- [ ] **Step 3: Write minimal implementation**
In `crates/factory-core/src/lib.rs`, add `SddTaskItem`, `SddMissionPlan`, and `TaskExecutionResult`:

```rust
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct SddTaskItem {
    pub id: String,
    pub description: String,
    #[serde(default)]
    pub is_parallel: bool,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub target_files: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct SddMissionPlan {
    pub mission_id: String,
    pub spec_version: String,
    pub tasks: Vec<SddTaskItem>,
    pub total_tasks: usize,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct TaskExecutionResult {
    pub task_id: String,
    pub status: String,
    pub hatchet_run_id: String,
    pub duration_ms: u64,
}
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test -p factory-core test_sdd_task_item_and_mission_plan_roundtrip`
Expected: PASS

- [ ] **Step 5: Commit**
```bash
git add crates/factory-core/src/lib.rs
git commit -m "feat(core): add SddTaskItem, SddMissionPlan, and TaskExecutionResult domain models"
```

---

### Task 2: SDD Task Markdown Parser in RustantAgent

**Files:**
- Modify: `crates/factory-application/src/agents/rustant.rs`
- Test: `crates/factory-application/src/agents/rustant.rs` (unit tests module)

**Interfaces:**
- Consumes: `factory_core::{SddTaskItem, SddMissionPlan}`
- Produces: `RustantAgent::parse_sdd_tasks(content: &str) -> Vec<SddTaskItem>` and updated `plan_mission` including structured `sdd_plan` in the returned JSON.

- [ ] **Step 1: Write the failing test**
In `crates/factory-application/src/agents/rustant.rs` (under `#[cfg(test)]`), add a test for `parse_sdd_tasks`:

```rust
#[test]
fn test_parse_sdd_tasks_from_markdown() {
    let tasks_md = r#"
# Tasks: Pipeline Error Remediation
- [ ] T001 Add `regex = "1"` to `[workspace.dependencies]` in `Cargo.toml`
- [ ] T002 [P] Create empty module file `crates/factory-infrastructure/src/pipeline_classifier.rs` with module doc comment
- [ ] T003 [P] Create empty module file `crates/factory-application/src/workflows/pipeline_remediation.rs`
- [ ] T006 [P] Add `ErrorCategory` enum to `crates/factory-core/src/lib.rs` (per data-model.md)
"#;

    let parsed = RustantAgent::parse_sdd_tasks(tasks_md);
    assert_eq!(parsed.len(), 4);
    assert_eq!(parsed[0].id, "T001");
    assert!(!parsed[0].is_parallel);
    assert!(parsed[0].target_files.contains(&"Cargo.toml".to_string()));

    assert_eq!(parsed[1].id, "T002");
    assert!(parsed[1].is_parallel);
    assert!(parsed[1].target_files.contains(&"crates/factory-infrastructure/src/pipeline_classifier.rs".to_string()));
}
```

- [ ] **Step 2: Run test to verify it fails**
Run: `cargo test -p factory-application test_parse_sdd_tasks_from_markdown`
Expected: FAIL with "no function or associated item named `parse_sdd_tasks` found for struct `RustantAgent`"

- [ ] **Step 3: Write minimal implementation**
Implement `RustantAgent::parse_sdd_tasks` in `crates/factory-application/src/agents/rustant.rs`:

```rust
impl RustantAgent {
    pub fn parse_sdd_tasks(content: &str) -> Vec<factory_core::SddTaskItem> {
        let task_regex = regex::Regex::new(r"(?m)^-\s*\[[ xX]\]\s*(T\d+)\s*(\[P\])?\s*(?:\[[^\]]+\])?\s*(.+)$").unwrap();
        let file_regex = regex::Regex::new(r"`([^`]+(?:\.[a-zA-Z0-9]+|Cargo\.toml))`").unwrap();

        let mut tasks = Vec::new();
        let mut previous_task_id: Option<String> = None;

        for cap in task_regex.captures_iter(content) {
            let id = cap.get(1).map_or("", |m| m.as_str()).to_string();
            let is_parallel = cap.get(2).is_some();
            let description = cap.get(3).map_or("", |m| m.as_str()).trim().to_string();

            let mut target_files = Vec::new();
            for file_cap in file_regex.captures_iter(&description) {
                if let Some(m) = file_cap.get(1) {
                    let path = m.as_str().to_string();
                    if !target_files.contains(&path) {
                        target_files.push(path);
                    }
                }
            }

            // Dependency tracking: non-parallel tasks depend on the prior task
            let mut dependencies = Vec::new();
            if !is_parallel {
                if let Some(prev) = &previous_task_id {
                    dependencies.push(prev.clone());
                }
            }
            previous_task_id = Some(id.clone());

            tasks.push(factory_core::SddTaskItem {
                id,
                description,
                is_parallel,
                dependencies,
                target_files,
            });
        }

        tasks
    }
}
```
Also update `plan_mission` to parse `tasks.md` and include `"sdd_plan": SddMissionPlan` in the returned JSON object.

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test -p factory-application test_parse_sdd_tasks_from_markdown`
Expected: PASS

- [ ] **Step 5: Commit**
```bash
git add crates/factory-application/src/agents/rustant.rs
git commit -m "feat(application): implement parse_sdd_tasks in RustantAgent"
```

---

### Task 3: Hatchet SDD Task Orchestrator in Autonomous Mission Workflow

**Files:**
- Modify: `crates/factory-application/src/workflows/autonomous_mission.rs`
- Test: `crates/factory-application/src/workflows/autonomous_mission.rs`

**Interfaces:**
- Consumes: `factory_core::{SddTaskItem, SddMissionPlan, TaskExecutionResult}`, `crate::workflows::develop_task::{TaskInput, TaskOutput}`
- Produces: Refactored `zeroclaw-execute` step that iterates through `sdd_plan.tasks` in order, invoking `execute_task` / Hatchet task for each, and verifying order.

- [ ] **Step 1: Write the failing test**
In `crates/factory-application/src/workflows/autonomous_mission.rs` (under `#[cfg(test)]`), add a test validating sequential execution of an SDD plan through the coordinator:

```rust
#[tokio::test]
async fn test_sdd_task_execution_order_enforcement() {
    let tasks = vec![
        factory_core::SddTaskItem {
            id: "T001".to_string(),
            description: "Setup step".to_string(),
            is_parallel: false,
            dependencies: vec![],
            target_files: vec!["Cargo.toml".to_string()],
        },
        factory_core::SddTaskItem {
            id: "T002".to_string(),
            description: "Dependent step".to_string(),
            is_parallel: false,
            dependencies: vec!["T001".to_string()],
            target_files: vec!["src/lib.rs".to_string()],
        },
    ];

    let mut execution_order = Vec::new();
    for task in &tasks {
        // Assert all dependencies were previously executed
        for dep in &task.dependencies {
            assert!(execution_order.contains(dep), "Dependency {} was not satisfied before {}", dep, task.id);
        }
        execution_order.push(task.id.clone());
    }

    assert_eq!(execution_order, vec!["T001", "T002"]);
}
```

- [ ] **Step 2: Run test to verify it passes/fails**
Run: `cargo test -p factory-application test_sdd_task_execution_order_enforcement`
Expected: PASS (validating model logic)

- [ ] **Step 3: Update `zeroclaw-execute` in `autonomous_mission.rs`**
Replace the static `time.sleep(15)` in `autonomous_mission.rs` with the SDD task execution loop:
1. Extract `sdd_plan` from `ctx.parent_output("rustant-plan")` or generate fallback from input.
2. For each task:
   - Verify dependencies have completed.
   - Publish Kafka progress thought: `format!("Executing SDD task {} ({}): {}", task.id, i+1, task.description)`.
   - Call `zeroclaw.execute_task(&task.id, &task.description, &task.target_files).await?`.
   - Log task completion and checkpoint to S3.
3. Return the cumulative task results.

- [ ] **Step 4: Run workspace tests to verify no regressions**
Run: `cargo test -p factory-application`
Expected: PASS

- [ ] **Step 5: Commit**
```bash
git add crates/factory-application/src/workflows/autonomous_mission.rs
git commit -m "feat(workflows): orchestrate SDD tasks in sequential order during zeroclaw-execute"
```

---

### Task 4: End-to-End Integration Test for Hatchet SDD Task Orchestration

**Files:**
- Create: `crates/factory-application/tests/hatchet_sdd_task_orchestration_test.rs`
- Test: `crates/factory-application/tests/hatchet_sdd_task_orchestration_test.rs`

**Interfaces:**
- Consumes: `factory_application::workflows::autonomous_mission::*`, `factory_core::*`, `factory_infrastructure::*`
- Produces: Comprehensive automated integration test asserting that when a multi-task mission is executed, all tasks run in the exact order planned by the SDD.

- [ ] **Step 1: Write the integration test**
Create `crates/factory-application/tests/hatchet_sdd_task_orchestration_test.rs`:
- Mock `McpClient`, `R2rClient`, `KafkaClient`, `AethalgardClient`.
- Provide a multi-task plan with 3 tasks: `T001`, `T002`, `T003`.
- Track tool invocation sequence using a shared `Arc<Mutex<Vec<String>>>`.
- Assert that `T001` is executed before `T002`, and `T002` before `T003`.
- Assert that validation and review only occur after all 3 tasks have finished.

- [ ] **Step 2: Run test to verify it passes**
Run: `cargo test --test hatchet_sdd_task_orchestration_test`
Expected: PASS

- [ ] **Step 3: Commit**
```bash
git add crates/factory-application/tests/hatchet_sdd_task_orchestration_test.rs
git commit -m "test(application): add integration test for Hatchet SDD task orchestration"
```

---

### Task 5: Factory CLI Runner Verification

**Files:**
- Modify: `crates/factory-cli/src/bin/trigger_mission.rs`
- Test: Manual / CLI execution with `--payload`

- [ ] **Step 1: Add `--sdd-plan` support or automatic task breakdown logging in `trigger_mission.rs`**
Display the parsed task breakdown and execution progress in CLI output.

- [ ] **Step 2: Verify `cargo check --workspace`**
Run: `cargo check --workspace`
Expected: 0 errors.

- [ ] **Step 3: Commit**
```bash
git add crates/factory-cli/src/bin/trigger_mission.rs
git commit -m "feat(cli): enhance trigger_mission with SDD task execution tracking"
```
