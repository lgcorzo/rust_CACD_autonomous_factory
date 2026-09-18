use factory_application::workflows::autonomous_mission::MissionInput;
use hatchet_sdk::Hatchet;
use hatchet_sdk::Runnable;

#[tokio::main]
#[allow(clippy::collapsible_if)]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Connecting to Hatchet...");
    let hatchet = Hatchet::from_env().await?;

    // Parse --payload
    let mut args = std::env::args().skip(1);
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut mission_id = format!("test-uuid-{}", timestamp);
    let mut goal = "Test goal".to_string();

    let mut repository_path = String::new();
    let mut source_platform: Option<String> = None;
    let mut repository: Option<String> = None;
    let mut issue_number: Option<u64> = None;

    while let Some(arg) = args.next() {
        if arg == "--payload" {
            if let Some(payload_str) = args.next() {
                if let Ok(input_parsed) = serde_json::from_str::<MissionInput>(&payload_str) {
                    if let Some(m) = input_parsed.mission_id {
                        mission_id = m;
                    }
                    if !input_parsed.goal.is_empty() {
                        goal = input_parsed.goal;
                    }
                    if !input_parsed.repository_path.is_empty() {
                        repository_path = input_parsed.repository_path;
                    }
                    source_platform = input_parsed.source_platform;
                    repository = input_parsed.repository;
                    issue_number = input_parsed.issue_number;
                } else if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&payload_str)
                {
                    if let Some(mid) = payload.get("mission_id").and_then(|v| v.as_str()) {
                        mission_id = mid.to_string();
                    } else if let Some(event_id) = payload.get("event_id").and_then(|v| v.as_str())
                    {
                        mission_id = format!("{}-{}", event_id, timestamp);
                    }
                    if let Some(msg) = payload
                        .get("message")
                        .or_else(|| payload.get("goal"))
                        .and_then(|v| v.as_str())
                    {
                        goal = msg.to_string();
                    }
                    if let Some(sp) = payload.get("source_platform").and_then(|v| v.as_str()) {
                        source_platform = Some(sp.to_string());
                    }
                    if let Some(repo) = payload
                        .get("repository")
                        .or_else(|| payload.get("repository_path"))
                        .and_then(|v| v.as_str())
                    {
                        repository = Some(repo.to_string());
                    }
                    if let Some(num) = payload.get("issue_number").and_then(|v| v.as_u64()) {
                        issue_number = Some(num);
                    }
                    if let Some(rp) = payload.get("repository_path").and_then(|v| v.as_str()) {
                        repository_path = rp.to_string();
                    }
                }
            }
        }
    }

    let input = MissionInput {
        mission_id: Some(mission_id.clone()),
        goal: goal.clone(),
        repository_path,
        source_platform,
        repository,
        issue_number,
    };

    tracing::info!(
        "Triggering darkgravitymission with ID: {} and goal: {}",
        mission_id,
        goal
    );
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
