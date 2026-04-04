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
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Worker { mcp_url } => {
            tracing::info!("Starting Hatchet worker...");
            
            let mut worker = Worker::new().await?;
            
            // Register workflows
            let mission_wf = AutonomousMissionWorkflow::new(mcp_url.clone());
            let task_wf = DevelopTaskWorkflow::new(mcp_url);
            
            worker.register_workflow(mission_wf)?;
            worker.register_workflow(task_wf)?;
            
            worker.start().await?;
        }
    }

    Ok(())
}
