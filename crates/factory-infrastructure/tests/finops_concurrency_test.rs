use factory_infrastructure::finops_stop_store::{FileFinOpsStopStore, FinOpsStopStore};
use std::thread;

#[test]
fn test_finops_concurrent_check_and_reserve_prevents_over_admission() {
    let path = std::env::temp_dir().join(format!(
        "finops_concurrency_test_{}.json",
        uuid::Uuid::new_v4()
    ));

    let init_store = FileFinOpsStopStore::new(path.clone());
    init_store
        .write_state(false, None, 0.0, 50.0)
        .expect("Failed to initialize FinOps store");

    let mut handles = Vec::new();
    for idx in 0..10 {
        let store_path = path.clone();
        handles.push(thread::spawn(move || {
            let worker_store = FileFinOpsStopStore::new(store_path);
            worker_store.check_and_reserve_budget(&format!("mission-{}", idx), 15.0)
        }));
    }

    let mut successes = 0;
    let mut rejections = 0;
    for h in handles {
        match h.join().expect("Thread panicked") {
            Ok(_) => successes += 1,
            Err(_) => rejections += 1,
        }
    }

    // With $50.0 max daily budget, 90% threshold is $45.0.
    // Three reservations of $15.0 reach $45.0 (latching halted=true on the 3rd); all remaining 7 must be rejected.
    assert_eq!(
        successes, 3,
        "Expected exactly 3 concurrent reservations of $15 to succeed before 90% ($45) latch"
    );
    assert_eq!(rejections, 7, "Expected remaining 7 threads to fail closed");

    let final_state = init_store.read_state().expect("Failed to read final state");
    assert!(final_state.halted, "Store must be latched in halted state");
    assert!(
        (final_state.current_spend - 45.0).abs() < f64::EPSILON,
        "Final spend must be exactly $45.0, got {}",
        final_state.current_spend
    );

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("lock"));
}

#[test]
fn test_finops_rejects_invalid_negative_nan_or_infinite_amounts() {
    let path = std::env::temp_dir().join(format!(
        "finops_validation_test_{}.json",
        uuid::Uuid::new_v4()
    ));
    let store = FileFinOpsStopStore::new(path.clone());

    // Reject negative current_spend or non-positive max_daily_budget in write_state
    assert!(store.write_state(false, None, -1.0, 50.0).is_err());
    assert!(store.write_state(false, None, f64::NAN, 50.0).is_err());
    assert!(store.write_state(false, None, f64::INFINITY, 50.0).is_err());
    assert!(store.write_state(false, None, 0.0, 0.0).is_err());
    assert!(store.write_state(false, None, 0.0, -50.0).is_err());
    assert!(store.write_state(false, None, 0.0, f64::NAN).is_err());
    assert!(store.write_state(false, None, 0.0, f64::INFINITY).is_err());

    store.write_state(false, None, 10.0, 50.0).unwrap();

    // Reject negative, NaN, or infinite estimated_cost in check_and_reserve_budget
    assert!(store.check_and_reserve_budget("neg-cost", -5.0).is_err());
    assert!(store
        .check_and_reserve_budget("nan-cost", f64::NAN)
        .is_err());
    assert!(store
        .check_and_reserve_budget("inf-cost", f64::INFINITY)
        .is_err());

    let state = store.read_state().unwrap();
    assert!(
        (state.current_spend - 10.0).abs() < f64::EPSILON,
        "Invalid reservations must not mutate current_spend"
    );

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("lock"));
}

#[test]
fn finops_child_process_worker_entrypoint() {
    let Ok(store_path) = std::env::var("FINOPS_CHILD_WORKER_STORE_PATH") else {
        // No-op when not invoked as a child OS process worker
        return;
    };
    let mission_id = std::env::var("FINOPS_CHILD_WORKER_MISSION_ID")
        .unwrap_or_else(|_| "child-mission".to_string());
    let store = FileFinOpsStopStore::new(std::path::PathBuf::from(store_path));
    match store.check_and_reserve_budget(&mission_id, 15.0) {
        Ok(_) => println!("FINOPS_CHILD_RESULT=ADMITTED"),
        Err(e) => println!("FINOPS_CHILD_RESULT=REJECTED:{}", e),
    }
}

#[test]
fn test_finops_cross_process_concurrency_prevents_over_admission() {
    let path = std::env::temp_dir().join(format!(
        "finops_multiprocess_test_{}.json",
        uuid::Uuid::new_v4()
    ));

    let init_store = FileFinOpsStopStore::new(path.clone());
    init_store
        .write_state(false, None, 0.0, 50.0)
        .expect("Failed to initialize FinOps store");

    let exe = std::env::current_exe().expect("Failed to resolve current test executable");
    let mut children = Vec::new();
    for idx in 0..10 {
        let child = std::process::Command::new(&exe)
            .arg("--exact")
            .arg("finops_child_process_worker_entrypoint")
            .arg("--nocapture")
            .env("FINOPS_CHILD_WORKER_STORE_PATH", &path)
            .env(
                "FINOPS_CHILD_WORKER_MISSION_ID",
                format!("proc-mission-{}", idx),
            )
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn child worker process");
        children.push(child);
    }

    let mut admitted = 0;
    let mut rejected = 0;
    for child in children {
        let out = child
            .wait_with_output()
            .expect("Failed waiting on child worker process");
        assert!(out.status.success(), "Child process exited abnormally");
        let stdout = String::from_utf8_lossy(&out.stdout);
        if stdout.contains("FINOPS_CHILD_RESULT=ADMITTED") {
            admitted += 1;
        } else if stdout.contains("FINOPS_CHILD_RESULT=REJECTED:") {
            rejected += 1;
        } else {
            panic!("Unexpected child process output: {}", stdout);
        }
    }

    assert_eq!(
        admitted, 3,
        "Expected exactly 3 OS processes to reserve $15 before 90% ($45) hard-stop latch"
    );
    assert_eq!(
        rejected, 7,
        "Expected remaining 7 OS processes to fail closed"
    );

    let final_state = init_store.read_state().expect("Failed to read final state");
    assert!(final_state.halted);
    assert_eq!(final_state.state_version, 4); // 1 init + 3 admissions
    assert!((final_state.current_spend - 45.0).abs() < f64::EPSILON);

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("lock"));
}

#[test]
fn test_finops_fail_closed_on_corrupt_truncated_unavailable_and_recovers_after_restart() {
    let dir = std::env::temp_dir().join(format!("finops_faults_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("state.json");

    // Corrupt state -> fail closed
    std::fs::write(&path, b"{not json").unwrap();
    let store = FileFinOpsStopStore::new(path.clone());
    assert!(store.check_and_reserve_budget("corrupt", 1.0).is_err());
    assert!(store.read_state().is_err());

    // Truncated state (process killed mid-write without atomic rename) -> fail closed
    std::fs::write(&path, b"{\"halted\": false, \"state_ver").unwrap();
    assert!(store.check_and_reserve_budget("truncated", 1.0).is_err());

    // Crash during write leaves only a *.json.tmp; committed state must stay authoritative
    store.write_state(false, None, 10.0, 50.0).unwrap();
    std::fs::write(path.with_extension("json.tmp"), b"{garbage").unwrap();
    let restarted = FileFinOpsStopStore::new(path.clone());
    let s = restarted.read_state().unwrap();
    assert!((s.current_spend - 10.0).abs() < f64::EPSILON);

    // Recovery after restart: new instance continues from persisted version
    let after = restarted
        .check_and_reserve_budget("after-restart", 5.0)
        .unwrap();
    assert_eq!(after.state_version, s.state_version + 1);

    // Storage unavailable (parent path is a regular file) -> fail closed
    let blocker = dir.join("blocker");
    std::fs::write(&blocker, b"x").unwrap();
    let unavailable = FileFinOpsStopStore::new(blocker.join("state.json"));
    assert!(unavailable
        .check_and_reserve_budget("no-storage", 1.0)
        .is_err());
    assert!(unavailable.write_state(false, None, 0.0, 50.0).is_err());

    // Concurrent stop-state latch vs reservations: once halted, every reservation is rejected
    restarted
        .write_state(true, Some("operator stop".into()), 15.0, 50.0)
        .unwrap();
    assert!(restarted
        .check_and_reserve_budget("post-halt", 0.0)
        .is_err());

    let _ = std::fs::remove_dir_all(&dir);
}
