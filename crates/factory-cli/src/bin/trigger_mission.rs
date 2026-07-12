use factory_application::workflows::autonomous_mission::MissionInput;
use hatchet_sdk::Hatchet;
use hatchet_sdk::Runnable;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Connecting to Hatchet...");
    let hatchet = Hatchet::from_env().await?;

    // Parse --payload
    let mut args = std::env::args().skip(1);
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let mut mission_id = format!("test-uuid-{}", timestamp);
    let mut goal = "Test goal".to_string();

    while let Some(arg) = args.next() {
        if arg == "--payload" {
            if let Some(payload_str) = args.next() {
                if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&payload_str) {
                    let event_id = payload["event_id"].as_str().unwrap_or("test-uuid-default");
                    mission_id = format!("{}-{}", event_id, timestamp);
                    goal = payload["message"].as_str().unwrap_or("Test goal").to_string();
                }
            }
        }
    }

    let input = MissionInput {
        mission_id: Some(mission_id.clone()),
        goal: goal.clone(),
        repository_path: String::new(),
    };

    tracing::info!("Triggering darkgravitymission with ID: {} and goal: {}", mission_id, goal);
    match hatchet
        .workflow::<MissionInput, factory_application::workflows::MissionOutput>(
            "darkgravitymission-dev-lgcorzo",
        )
        .build()
        .unwrap()
        .run_no_wait(&input, None)
        .await
    {
        Ok(res) => tracing::info!("Mission triggered successfully! Result: {:?}", res),
        Err(e) => tracing::error!("Failed to trigger mission: {}", e),
    }

    Ok(())
}
