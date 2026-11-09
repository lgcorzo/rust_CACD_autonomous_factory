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
