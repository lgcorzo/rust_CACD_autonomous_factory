use rdkafka::Message;
use rdkafka::consumer::{Consumer, StreamConsumer};
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;

pub struct TelemetryExporter {
    kafka_brokers: String,
    openwebui_db_url: String,
    http_client: Client,
}

impl TelemetryExporter {
    pub fn new(kafka_brokers: String, openwebui_db_url: String) -> Self {
        Self {
            kafka_brokers,
            openwebui_db_url,
            http_client: Client::new(),
        }
    }

    /// Starts a background task consuming `agent-thought` from Kafka and exporting to OpenWebUI.
    pub async fn start_export_loop(self: Arc<Self>) -> anyhow::Result<()> {
        let consumer: StreamConsumer = rdkafka::ClientConfig::new()
            .set("group.id", "telemetry-exporter")
            .set("bootstrap.servers", &self.kafka_brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .create()
            .map_err(|e| anyhow::anyhow!("Kafka Consumer creation error: {}", e))?;

        consumer
            .subscribe(&["agent-thought"])
            .map_err(|e| anyhow::anyhow!("Can't subscribe to specified topic: {}", e))?;

        tracing::info!("Starting telemetry export loop from Kafka to OpenWebUI...");

        // Start listening
        tokio::spawn(async move {
            loop {
                match consumer.recv().await {
                    Err(e) => tracing::warn!("Kafka error: {}", e),
                    Ok(m) => {
                        let payload = match m.payload_view::<str>() {
                            None => "",
                            Some(Ok(s)) => s,
                            Some(Err(e)) => {
                                tracing::warn!(
                                    "Error while deserializing message payload: {:?}",
                                    e
                                );
                                ""
                            }
                        };

                        if let Ok(json) = serde_json::from_str::<Value>(payload) 
                            && let Err(e) = self.push_to_openwebui(&json).await 
                        {
                            tracing::error!("Failed to export telemetry to OpenWebUI: {}", e);
                        }
                    }
                }
            }
        });

        Ok(())
    }

    async fn push_to_openwebui(&self, thought: &Value) -> anyhow::Result<()> {
        // Here we mock a push to OpenWebUI's internal API or PostgreSQL directly.
        // In reality, this would likely be an HTTP call to an ingestion endpoint,
        // or a direct insert if we hold the openwebuidb credentials.

        // Let's use a mock REST call to demonstrate
        let url = format!("{}/api/v1/telemetry/ingest", self.openwebui_db_url);

        let res = self.http_client.post(&url).json(thought).send().await;

        match res {
            Ok(r) if r.status().is_success() => Ok(()),
            Ok(r) => Err(anyhow::anyhow!("OpenWebUI returned status: {}", r.status())),
            Err(e) => Err(anyhow::anyhow!("OpenWebUI request failed: {}", e)),
        }
    }
}
