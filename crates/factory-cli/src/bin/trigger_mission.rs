use factory_application::workflows::autonomous_mission::MissionInput;
use hatchet_sdk::Hatchet;
use hatchet_sdk::Runnable;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Connecting to Hatchet...");
    let hatchet = Hatchet::from_env().await?;

    let input = MissionInput {
        mission_id: Some("mock-sentry-139".to_string()),
        goal: "Hotfix Crash: NullPointerException in payment processing module. Context: http://gitlab.com/issue/139".to_string(),
        repository_path: String::new(),
    };

    tracing::info!("Triggering darkgravitymission...");
    match hatchet
        .workflow::<MissionInput, factory_application::workflows::MissionOutput>(
            "darkgravitymission-test",
        )
        .build()
        .unwrap()
        .run_no_wait(&input, None)
        .await
    {
        Ok(_) => tracing::info!("Mission triggered successfully!"),
        Err(e) => tracing::error!("Failed to trigger mission: {}", e),
    }

    Ok(())
}
