use factory_application::workflows::deep_research::DeepSearchInput;
use hatchet_sdk::Hatchet;
use hatchet_sdk::Runnable;

#[tokio::main]
#[allow(clippy::collapsible_if)]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Connecting to Hatchet for Deep Search Trigger...");
    let hatchet = Hatchet::from_env().await?;

    // Parse --query
    let mut args = std::env::args().skip(1);
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let job_id = format!("ds-test-{}", timestamp);
    let mut query = "Research OpenZiti zero trust architectures in Rust applications".to_string();

    while let Some(arg) = args.next() {
        if arg == "--query" {
            if let Some(q) = args.next() {
                query = q;
            }
        }
    }

    let input = DeepSearchInput {
        job_id: job_id.clone(),
        query: query.clone(),
    };

    tracing::info!(
        "Triggering deep-search-workflow with Job ID: {} and Query: {}",
        job_id,
        query
    );
    match hatchet
        .workflow::<DeepSearchInput, factory_application::workflows::deep_research::DeepSearchOutput>(
            "deep-search-workflow",
        )
        .build()
        .unwrap()
        .run_no_wait(&input, None)
        .await
    {
        Ok(res) => tracing::info!("Deep Search triggered successfully! Result: {:?}", res),
        Err(e) => tracing::error!("Failed to trigger deep search: {}", e),
    }

    Ok(())
}
