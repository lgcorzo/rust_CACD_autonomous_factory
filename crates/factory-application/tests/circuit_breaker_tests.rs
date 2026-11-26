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

#[test]
fn test_swarm_circuit_breaker_all_five_triggers_and_hitl_override() {
    use factory_application::workflows::circuit_breaker::{
        SwarmCircuitBreakerGuard, SwarmFreezeReason,
    };

    let lanes = vec!["lane-1".to_string(), "lane-2".to_string()];

    // 1. +15% plan baseline overrun
    let mut guard = SwarmCircuitBreakerGuard::new(10.0);
    assert!(guard.evaluate_spend(11.5, 11.5, 0.5, &lanes).is_none());
    let reason = guard.evaluate_spend(11.6, 11.6, 0.5, &lanes);
    assert!(matches!(
        reason,
        Some(SwarmFreezeReason::PlanBaselineOverrun { .. })
    ));
    assert!(guard.is_frozen());
    assert_eq!(guard.state.frozen_lanes, lanes);

    // HITL Vertex 3 override unfreezes
    assert!(guard.apply_hitl_override("vertex-3-operator", "Approved +20% budget"));
    assert!(!guard.is_frozen());

    // 2. > $1.00/min burn velocity
    let mut guard_vel = SwarmCircuitBreakerGuard::new(100.0);
    let reason = guard_vel.evaluate_spend(5.0, 5.0, 1.05, &lanes);
    assert!(matches!(
        reason,
        Some(SwarmFreezeReason::BurnVelocityExceeded { .. })
    ));
    assert!(guard_vel.is_frozen());

    // 3. $45/$50 daily HardStop
    let mut guard_hard = SwarmCircuitBreakerGuard::new(100.0);
    let reason = guard_hard.evaluate_spend(10.0, 45.0, 0.2, &lanes);
    assert!(matches!(
        reason,
        Some(SwarmFreezeReason::DailyHardStopReached { .. })
    ));
    assert!(guard_hard.is_frozen());

    // 4. > 6 retries within 60s sliding window
    let mut guard_retry = SwarmCircuitBreakerGuard::new(100.0);
    for ts in 100..106 {
        assert!(guard_retry.record_retry_attempt(ts, &lanes).is_none());
    }
    let reason = guard_retry.record_retry_attempt(106, &lanes);
    assert!(matches!(
        reason,
        Some(SwarmFreezeReason::RetryWindowExceeded {
            retries_in_window: 7,
            max_retries: 6,
            window_secs: 60,
        })
    ));
    assert!(guard_retry.is_frozen());

    // 5. A-B-A-B diff oscillation halt
    let mut guard_osc = SwarmCircuitBreakerGuard::new(100.0);
    assert!(
        guard_osc
            .record_lane_diff("lane-1", "+ a = 1", &lanes)
            .is_none()
    );
    assert!(
        guard_osc
            .record_lane_diff("lane-1", "+ a = 2", &lanes)
            .is_none()
    );
    assert!(
        guard_osc
            .record_lane_diff("lane-1", "+ a = 1", &lanes)
            .is_none()
    );
    let reason = guard_osc.record_lane_diff("lane-1", "+ a = 2", &lanes);
    assert!(matches!(
        reason,
        Some(SwarmFreezeReason::DiffOscillationHalt { .. })
    ));
    assert!(guard_osc.is_frozen());
}

#[tokio::test]
async fn test_finops_evaluate_swarm_mission_spend_hardstop_and_hooks() {
    use factory_application::agents::finops::FinOpsAgent;
    use factory_application::workflows::circuit_breaker::SwarmFreezeReason;
    use factory_core::FinOpsTag;
    use factory_infrastructure::KafkaClient;
    use factory_infrastructure::finops_stop_store::{FileFinOpsStopStore, FinOpsStopStore};
    use std::sync::{Arc, Mutex};

    struct MockKafka {
        published: Mutex<Vec<(String, String, Vec<u8>)>>,
    }
    #[async_trait::async_trait]
    impl KafkaClient for MockKafka {
        async fn publish(&self, topic: &str, key: &str, payload: &[u8]) -> anyhow::Result<()> {
            self.published.lock().unwrap().push((
                topic.to_string(),
                key.to_string(),
                payload.to_vec(),
            ));
            Ok(())
        }
    }

    let stop_path = std::env::temp_dir().join(format!("finops_stop_{}.json", std::process::id()));
    let _ = std::fs::remove_file(&stop_path);
    let stop_store = Arc::new(FileFinOpsStopStore::new(stop_path));
    let agent = FinOpsAgent::with_stop_store(
        "http://localhost:4000".to_string(),
        "test-key".to_string(),
        FinOpsTag {
            team: "team".to_string(),
            epic: "epic".to_string(),
            microservice: "svc".to_string(),
            environment: "dev".to_string(),
            cost_center: "cc".to_string(),
        },
        stop_store.clone(),
    );
    let kafka = MockKafka {
        published: Mutex::new(Vec::new()),
    };
    let revoked = Arc::new(Mutex::new(Vec::new()));
    let revoked_clone = revoked.clone();

    let eval = agent
        .evaluate_swarm_mission_spend(
            "mission-swarm-1",
            &[4.0, 5.0, 3.0], // 12.0 total
            100.0,
            45.0, // $45/$50 HardStop
            50.0,
            0.5,
            Some(&kafka),
            Some(move |mid: String| {
                let r = revoked_clone.clone();
                async move {
                    r.lock().unwrap().push(mid);
                    Ok(())
                }
            }),
        )
        .await
        .expect("evaluate_swarm_mission_spend should succeed");

    assert_eq!(eval.total_mission_spend_usd, 12.0);
    assert!(matches!(
        eval.freeze_reason,
        Some(SwarmFreezeReason::DailyHardStopReached { .. })
    ));
    assert!(eval.hard_stop_latched);
    assert!(eval.vault_revocation_triggered);
    assert!(eval.kafka_alert_emitted);
    assert!(stop_store.read_state().unwrap().halted);
    assert_eq!(revoked.lock().unwrap().as_slice(), &["mission-swarm-1"]);

    let events = kafka.published.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].0, "budget-exceeded");
    let payload: serde_json::Value = serde_json::from_slice(&events[0].2).unwrap();
    assert_eq!(payload["event_type"], "budget-exceeded");
    assert_eq!(payload["mission_id"], "mission-swarm-1");
}
