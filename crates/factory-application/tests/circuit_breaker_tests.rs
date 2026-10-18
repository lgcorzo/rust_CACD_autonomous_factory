use factory_application::workflows::circuit_breaker::{
    CircuitBreakerGuard, CircuitBreakerStatus, compute_diff_hash, normalize_diff,
};

#[test]
fn test_circuit_breaker_detects_abab_oscillation_and_persistent_trip() {
    let mut guard = CircuitBreakerGuard::new(5, 8.0);
    let diff_a = "+ let token = \"secret1\"; eval(user_input);";
    let diff_b = "+ let token = \"secret2\"; exec(command_str);";

    // Attempt 1: diff_a
    let (status1, _) = guard.evaluate_diff(diff_a);
    assert!(matches!(
        status1,
        CircuitBreakerStatus::Retrying { attempt: 1, .. }
    ));
    assert!(!guard.is_tripped());

    // Attempt 2: diff_b
    let (status2, _) = guard.evaluate_diff(diff_b);
    assert!(matches!(
        status2,
        CircuitBreakerStatus::Retrying { attempt: 2, .. }
    ));
    assert!(!guard.is_tripped());

    // Attempt 3: diff_a again -> Oscillation deadlock detected!
    let (status3, _) = guard.evaluate_diff(diff_a);
    match status3 {
        CircuitBreakerStatus::AgentStuck { reason } => {
            assert!(
                reason.contains("Oscillation deadlock") || reason.contains("Deadlock detected"),
                "Expected oscillation deadlock alert: {}",
                reason
            );
        }
        _ => panic!(
            "Expected AgentStuck status on A-B-A oscillation, got {:?}",
            status3
        ),
    }

    assert!(guard.is_tripped(), "Guard must remain tripped");

    // Any subsequent evaluation while tripped must immediately return AgentStuck
    let (status4, _) = guard.evaluate_diff("+ pub fn clean() {}");
    assert!(matches!(status4, CircuitBreakerStatus::AgentStuck { .. }));

    // Resetting clears the trip latch
    guard.reset();
    assert!(!guard.is_tripped());
    let (status5, _) = guard.evaluate_diff("+ pub fn clean() {}");
    assert_eq!(status5, CircuitBreakerStatus::Passed);
}

#[test]
fn test_diff_normalization_ignores_git_headers_and_whitespace() {
    let diff_with_headers = "diff --git a/src/lib.rs b/src/lib.rs\nindex 1234..5678 100644\n--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -1,3 +1,3 @@\n+   fn    test()   {   }\n";
    let raw_diff = "+ fn test() { }\n";

    let hash1 = compute_diff_hash(diff_with_headers);
    let hash2 = compute_diff_hash(raw_diff);

    assert_eq!(
        hash1, hash2,
        "Normalized diff hash must match regardless of diff headers/whitespace"
    );
    assert_eq!(normalize_diff(diff_with_headers), "+ fn test() { }");
}
