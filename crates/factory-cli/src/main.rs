use clap::{Parser, Subcommand};
use factory_application::workflows::{AutonomousMissionWorkflow, DevelopTaskWorkflow};
use hatchet_sdk::worker::Worker;
use tracing_subscriber::EnvFilter;

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
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Worker { mcp_url } => {
            tracing::info!("Starting Hatchet worker...");
            
            let hatchet = hatchet_sdk::clients::hatchet::Hatchet::from_env().await?;
            let mut worker = hatchet.worker("factory-worker")
                .slots(10)
                .build()
                .unwrap();
            
            // Register workflows
            let mission_wf = factory_application::workflows::create_mission_workflow(&hatchet, mcp_url.clone());
            let task_wf = factory_application::workflows::create_develop_task_workflow(&hatchet, mcp_url);
            
            worker = hatchet_sdk::worker::worker::Register::add_task_or_workflow(worker, &mission_wf);
            worker = hatchet_sdk::worker::worker::Register::add_task_or_workflow(worker, &task_wf);
            
            worker.start().await?;
        }
    }

    Ok(())
}
