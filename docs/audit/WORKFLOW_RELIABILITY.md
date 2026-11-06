# Workflow Reliability & Authoritative FinOps Hard-Stop Review

**Feature**: `011-verified-autonomous-execution`  
**User Story**: User Story 2 — Enforce budget stops and reliable mission progress (`FR-009`, `FR-010`, `SC-003`)  
**Updated At (UTC)**: `2026-11-06T17:32:00Z`

---

## 1. Authoritative FinOps Hard-Stop Admission Boundary (`FR-009`, `SC-003`)

### 1.1 Previous Defect (`AUD-003`)
Prior to remediation, `AutonomousMissionWorkflow` constructed an isolated `FinOpsAgent::default()` inside the background budget monitor (`autonomous_mission.rs:303`) and a separate `FinOpsAgent::default()` inside each task admission closure (`rustant-plan` and `rustant-review`). Because each instance owned a fresh `Arc<AtomicBool>` (`halted = false`), hard-stop events detected by the monitor never propagated to mission admission checks or across worker processes.

### 1.2 Remediated Architecture (`T014`, `T015`)
1. **Shared Durable Store (`crates/factory-infrastructure/src/finops_stop_store.rs`)**:
   - `FinOpsStopStore` trait defines `read_state() -> Result<FinOpsStopState, String>` and `write_state(...) -> Result<FinOpsStopState, String>`.
   - `FinOpsStopState` persists `halted`, monotonically increasing `state_version` (`u64`), `reason`, `current_spend`, `max_daily_budget`, and `updated_at` (`DateTime<Utc>`).
   - `FileFinOpsStopStore` writes updates via atomic rename (`*.json.tmp` -> `*.json`), ensuring cross-thread and cross-process visibility on shared storage (`FINOPS_STOP_STORE_PATH`).
   - `FailingFinOpsStopStore` provides deterministic fault injection for read/write storage errors.
2. **Fail-Closed Admission Gate (`admit_mission_with_store` & `FinOpsAgent::evaluate_admission`)**:
   - Every admission check reads the authoritative `FinOpsStopStore`.
   - If `read_state()` returns an error (missing/corrupted store or I/O failure), admission **fails closed** with `FactoryError::FinOps`.
   - If `state.halted == true`, admission is denied with an explicit error embedding both `version=<state_version>` and `reason='<alert>'`.
   - `create_mission_workflow_with_clients` injects the shared `FinOpsStopStore` into the background monitor and all workflow admission gates (`rustant-plan`, `rustant-review`).

---

## 2. Mission Idempotency, Retry, Cancellation & Worker Restart (`FR-010`, `T016`)

### 2.1 `MissionExecutionCoordinator` (`crates/factory-application/src/workflows/autonomous_mission.rs`)
- **Idempotent Event Handling (`process_event`)**:
  - Maps incoming `event_key` (e.g., webhook delivery ID or poller cursor hash) to `mission_id` in `PersistedMissionStoreData`.
  - Duplicate deliveries for a `Completed` or `Running` mission return the existing `PersistedMissionRecord` immediately without re-invoking the workload closure or producing duplicate side effects.
- **Transient Failure & Retry Semantics**:
  - When a mission step fails transiently, the record transitions to `MissionLifecycleStatus::FailedRetryable`, increments `retry_count`, and records `last_error` without advancing the mission to `Completed`.
  - Subsequent retry delivery re-checks the FinOps admission gate, re-runs the failed step, and upon success transitions to `Completed` while preserving the accurate `retry_count`.
- **Step Checkpointing Across Worker Restarts (`execute_checkpointed_step`)**:
  - Completed step outputs (`rustant-plan`, `zeroclaw-execute`, etc.) are persisted in `completed_steps: HashMap<String, String>`.
  - When a new worker instance initializes from the persisted state file after a crash or restart, already-completed steps return their checkpointed output without re-executing side effects.
- **Explicit Cancellation & Compensation (`cancel_mission`)**:
  - Transitions the mission to `MissionLifecycleStatus::Cancelled` and records both `cancellation_reason` and `compensation_record` (e.g., JIT token revocation and workspace cleanup).
  - Any subsequent step or event targeting a `Cancelled` mission is rejected.

---

## 3. Active-Mission Policy Under FinOps Hard Stop (`FR-010`)

`ActiveMissionHardStopPolicy` defines two explicit, operator-selectable behaviors when a FinOps hard stop is triggered while missions are already in flight:
1. **`CancelAtStepBoundary` (Default Safety Policy)**:
   - Missions already running finish their current atomic step, and at the next `execute_checkpointed_step` boundary observe `stop_state.halted == true`, transition to `MissionLifecycleStatus::Cancelled`, record the `state_version` and step-checkpoint compensation, and halt further spend.
2. **`DrainAdmittedMissions`**:
   - Missions already admitted prior to the hard-stop `state_version` are permitted to drain to completion while 100% of new mission admissions are rejected.

---

## 4. Verification Evidence & Remaining Limitations

- **Verified by Automated Integration Tests (`verification_class: integration`)**:
  - `crates/factory-application/tests/finops_admission_tests.rs`:
    - `test_finops_hard_stop_blocks_mission_admission_across_workers_with_version` (8 concurrent admission requests across separate worker instances after monitor trips 90% hard stop; 100% rejected with `version=1`).
    - `test_finops_admission_fails_closed_on_store_read_error` (verifies fail-closed denial on store read error).
  - `crates/factory-application/tests/workflow_recovery_tests.rs`:
    - `test_duplicate_event_delivery_is_idempotent_without_duplicate_side_effects`
    - `test_retry_after_transient_failure_advances_once_without_advancing_incomplete_work`
    - `test_cancellation_compensation_and_worker_restart_recovery`
- **Remaining Limitations**:
  - Multi-node Kubernetes deployments where worker pods do not share a ReadWriteMany PVC or Redis/PostgreSQL backend for `FINOPS_STOP_STORE_PATH` require mounting a shared volume or backing `FinOpsStopStore` with PostgreSQL/Redis in cluster production. Live multi-pod cluster failover remains `NOT_VERIFIED` until exercised on `server.internal.lgcorzo`.

