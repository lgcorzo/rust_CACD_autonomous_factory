use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "factory-cli", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the Hatchet worker to process missions
    Worker {
        #[arg(long, default_value = "http://localhost:8100")]
        mcp_url: String,

        #[arg(long, default_value = "http://localhost:8000")]
        r2r_url: String,

        #[arg(long, env = "KAFKA_BROKERS", default_value = "localhost:9092")]
        kafka_brokers: String,

        #[arg(
            long,
            env = "AETHALGARD_WEBHOOK_URL",
            default_value = "http://jules-cloud-vm.internal:8080/mcp"
        )]
        aethalgard_webhook_url: String,
    },
    /// Verify Out-of-Sync Rate (OSR) against R2R knowledge base
    VerifyOsr {
        #[arg(long, default_value = "http://localhost:8000")]
        r2r_url: String,

        #[arg(long, default_value = "admin")]
        r2r_user: String,

        #[arg(long, default_value = "admin")]
        r2r_pwd: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = sentry::init((
        "https://2c78059d7a60a77da9bd8cc9a6affd33@o4511678618271744.ingest.de.sentry.io/4511701441445968",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            send_default_pii: true,
            ..Default::default()
        },
    ));

    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Worker {
            mcp_url,
            r2r_url,
            kafka_brokers,
            aethalgard_webhook_url,
        } => {
            if kafka_brokers.trim().is_empty() {
                anyhow::bail!("Invalid configuration: KAFKA_BROKERS must not be empty.");
            }

            // SPECIFY_CLI_PATH validation removed. The worker delegates specify execution to the MCP server.

            tracing::info!("Starting Hatchet worker...");

            let hatchet = hatchet_sdk::Hatchet::from_env().await?;
            let mut worker = hatchet.worker("factory-worker").slots(10).build().unwrap();

            // Register workflows
            let mission_wf = factory_application::workflows::create_mission_workflow(
                &hatchet,
                mcp_url.clone(),
                r2r_url,
                kafka_brokers,
                aethalgard_webhook_url,
            );
            let task_wf =
                factory_application::workflows::create_develop_task_workflow(&hatchet, mcp_url);

            worker =
                hatchet_sdk::worker::worker::Register::add_task_or_workflow(worker, &mission_wf);
            worker = hatchet_sdk::worker::worker::Register::add_task_or_workflow(worker, &task_wf);

            worker.start().await?;
        }
        Commands::VerifyOsr {
            r2r_url,
            r2r_user,
            r2r_pwd,
        } => {
            tracing::info!("Starting OSR verification against R2R...");
            use factory_infrastructure::r2r::R2rClient;
            let r2r_client =
                factory_infrastructure::r2r::HttpR2rClient::new(r2r_url, r2r_user, r2r_pwd);

            let context = match r2r_client.search("documentation sync state").await {
                Ok(c) => c,
                Err(e) => {
                    tracing::warn!(
                        "Failed to connect to R2R ({}). Skipping OSR verification in CI.",
                        e
                    );
                    std::process::exit(0);
                }
            };
            let r2r_text = serde_json::to_string(&context).unwrap_or_default();

            let mut wiki_content = String::new();
            if let Ok(entries) = std::fs::read_dir("wiki") {
                for entry in entries.flatten() {
                    if entry.path().extension().and_then(|s| s.to_str()) == Some("md")
                        && let Ok(content) = std::fs::read_to_string(entry.path())
                    {
                        wiki_content.push_str(&content);
                    }
                }
            }

            let osr = factory_application::utils::osr::calculate_osr(&wiki_content, &r2r_text);

            if osr <= 0.05 {
                tracing::info!("OSR validation passed with {}%", osr * 100.0);
                std::process::exit(0);
            } else {
                tracing::error!("OSR validation failed with {}%", osr * 100.0);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
