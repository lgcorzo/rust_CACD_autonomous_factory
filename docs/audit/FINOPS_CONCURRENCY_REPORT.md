# FinOps Concurrency & Distributed Consistency Report

**Component**: `crates/factory-infrastructure/src/finops_stop_store.rs`  
**Standard**: DG-RVP-014 §5  
**Evaluated At**: 2026-11-09T23:40:00Z  
**Status**: `VERIFIED`

---

## 1. Input Validation & Hard-Stop Semantics

1. **Strict Input Validation**:
   - `write_state`: Fails closed if `current_spend` is non-finite, `NaN`, or `< 0.0`, or if `max_daily_budget` is non-finite, `NaN`, or `<= 0.0`.
   - `check_and_reserve_budget`: Fails closed if `estimated_cost` is non-finite, `NaN`, or `< 0.0`.
2. **Deterministic Hard-Stop**:
   - Hard-stop ceiling is defined as exactly `90%` of `max_daily_budget` (e.g. `$45.0` on a `$50.0` budget).
   - Any admission attempt where `current_spend >= threshold` or `current_spend + estimated_cost > threshold` is denied.
   - If projected spend reaches the threshold, the store atomically latches `halted = true` with a detailed audit reason and increments `state_version`.

---

## 2. Multi-Process Concurrency & Fault Recovery

Tested in `crates/factory-infrastructure/tests/finops_concurrency_test.rs`:
1. **Multi-Thread Concurrency**: 10 threads competing for `$15` reservations on a `$50` budget. Exactly 3 succeed ($45 spend) and 7 fail closed.
2. **Multi-Process Concurrency (`test_finops_cross_process_concurrency_prevents_over_admission`)**:
   - Spawns 10 separate OS child processes (`std::process::Command::new(exe)`).
   - Advisory file locks (`fs2::FileExt::lock_exclusive()`) synchronize access across OS processes.
   - Exactly 3 child processes report `FINOPS_CHILD_RESULT=ADMITTED`; 7 report `FINOPS_CHILD_RESULT=REJECTED`.
3. **Fault Injection & Recovery (`test_finops_fail_closed_on_corrupt_truncated_unavailable_and_recovers_after_restart`)**:
   - Corrupt JSON state fails closed.
   - Truncated state fails closed.
   - Leftover temp files from killed processes do not poison committed state.
   - Unavailability of storage directory fails closed.
   - Successful restart recovery cleanly loads the persisted version and continues admissions.

---

## 3. Deployment Topology & Storage Boundaries

> [!IMPORTANT]
> `FileFinOpsStopStore` relies on POSIX advisory locks and atomic `rename(2)`. This guarantees cross-process consistency on a single host or across pods mounting a shared POSIX `ReadWriteMany` (RWX) volume. It does not provide distributed consistency across separate local filesystems in a multi-node cluster. Multi-node clusters without shared POSIX storage must back `FinOpsStopStore` with a distributed transactional store (e.g. PostgreSQL).
