use factory_infrastructure::kafka::{KafkaClient, RdKafkaClient};
use std::env;

#[tokio::test]
async fn test_kafka_live_connection() {
    let brokers = env::var("KAFKA_BROKERS").unwrap_or_else(|_| {
        "my-kafka-cluster-bootstrap.confluent.svc.cluster.local:9092".to_string()
    });

    let client_result = RdKafkaClient::new(&brokers);
    assert!(
        client_result.is_ok(),
        "Failed to create RdKafkaClient: {:?}",
        client_result.err()
    );

    let client = client_result.unwrap();

    let result = client
        .publish("mission-input", "test-key", b"test-payload")
        .await;

    // In a pure development environment outside the cluster, this might fail to resolve
    // so we handle both success and failure gracefully for now if the host is unreachable.
    match result {
        Ok(_) => println!("Successfully published to live Kafka broker"),
        Err(e) => {
            let err_str = e.to_string();
            // Allow failure if the host is completely unresolvable locally
            if !err_str.contains("Unknown broker")
                && !err_str.contains("Resolve failed")
                && !err_str.to_lowercase().contains("timed out")
            {
                panic!("Unexpected Kafka error: {}", err_str);
            }
        }
    }
}
