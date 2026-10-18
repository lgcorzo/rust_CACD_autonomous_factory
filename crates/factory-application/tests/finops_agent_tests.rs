use factory_application::agents::finops::{BudgetEvaluation, FinOpsAgent};
use factory_core::FinOpsTag;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn test_tag() -> FinOpsTag {
    FinOpsTag {
        team: "test-team".to_string(),
        epic: "E1.0".to_string(),
        microservice: "test-svc".to_string(),
        environment: "staging".to_string(),
        cost_center: "test-cc".to_string(),
    }
}

#[tokio::test]
async fn test_finops_client_poll_and_hardstop_latch() {
    let mock_server = MockServer::start().await;

    // Return spend that hits HardStop threshold ($46.00 with $50.00 max daily budget)
    Mock::given(method("GET"))
        .and(path("/spend/logs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "total_spend": 46.00
        })))
        .mount(&mock_server)
        .await;

    let agent = FinOpsAgent::new(mock_server.uri(), "mock-api-key".to_string(), test_tag());

    assert!(!agent.is_halted(), "Agent must not be halted initially");

    let eval = agent
        .poll_spend_and_evaluate(50.0, None)
        .await
        .expect("Poll spend failed");

    assert!(
        matches!(eval, BudgetEvaluation::HardStop { .. }),
        "Expected HardStop, got {:?}",
        eval
    );
    assert!(
        agent.is_halted(),
        "Agent must be halted after HardStop breached"
    );
}

#[tokio::test]
async fn test_finops_http_error_handling() {
    let mock_server = MockServer::start().await;

    // Return 500 error
    Mock::given(method("GET"))
        .and(path("/spend/logs"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let agent = FinOpsAgent::new(mock_server.uri(), "mock-api-key".to_string(), test_tag());

    let res = agent.poll_spend_and_evaluate(50.0, None).await;
    assert!(res.is_err(), "Must return an error on 500 status code");
}

#[test]
fn test_finops_first_cycle_anomaly_suppression() {
    let agent = FinOpsAgent::new(
        "http://localhost:4000".to_string(),
        "key".to_string(),
        test_tag(),
    );

    // First cycle: previous_spend is None, even with $25.00 spend
    let eval = agent.evaluate_spend_delta_normalized(25.0, None, 1.0, 50.0);
    assert_eq!(
        eval,
        BudgetEvaluation::Healthy {
            spend: 25.0,
            velocity: 0.0,
        },
        "First cycle must not trigger velocity anomaly"
    );

    // Second cycle: delta +$2.50 over 1 min -> velocity 2.50 (> 1.0)
    let eval_anomaly = agent.evaluate_spend_delta_normalized(27.5, Some(25.0), 1.0, 50.0);
    assert!(matches!(
        eval_anomaly,
        BudgetEvaluation::VelocityAnomaly { .. }
    ));
}
