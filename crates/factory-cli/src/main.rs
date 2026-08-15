use clap::{Parser, Subcommand};

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;
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
        #[arg(long, env = "MCP_URL", default_value = "http://localhost:8100")]
        mcp_url: String,

        #[arg(long, env = "R2R_URL", default_value = "http://localhost:8000")]
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
    /// Start the Outbound Poller daemon for GitHub/GitLab issues and PR comments
    Poller {
        #[arg(long, env = "GITHUB_REPOS", default_value = "lgcorzo/rust_CACD_autonomous_factory")]
        github_repos: String,

        #[arg(long, env = "GITLAB_PROJECTS", default_value = "lgcorzo-lab/autonomous_factory")]
        gitlab_projects: String,

        #[arg(long, env = "POLLING_INTERVAL_SECS", default_value = "30")]
        interval_secs: u64,

        #[arg(long, env = "MCP_URL", default_value = "http://localhost:8100")]
        mcp_url: String,

        #[arg(long, env = "R2R_URL", default_value = "http://localhost:8000")]
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

            tracing::info!("Starting Hatchet worker...");

            let token =
                std::env::var("HATCHET_CLIENT_TOKEN").expect("HATCHET_CLIENT_TOKEN must be set");
            let server_url = std::env::var("HATCHET_CLIENT_REST_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
            let grpc_url = std::env::var("HATCHET_CLIENT_GRPC_URL")
                .unwrap_or_else(|_| "127.0.0.1:7070".to_string());
            let tls_strategy =
                std::env::var("HATCHET_CLIENT_TLS_STRATEGY").unwrap_or_else(|_| "none".to_string());
            let hatchet =
                hatchet_sdk::Hatchet::from_token(&server_url, &grpc_url, &token, &tls_strategy)
                    .await?;
            let mut worker = hatchet.worker("factory-worker").slots(10).build().unwrap();

            // Register workflows
            let mission_wf = factory_application::workflows::create_mission_workflow(
                &hatchet,
                mcp_url.clone(),
                r2r_url.clone(),
                kafka_brokers,
                aethalgard_webhook_url,
            );
            let task_wf =
                factory_application::workflows::create_develop_task_workflow(&hatchet, mcp_url);
            let deep_research_wf =
                factory_application::workflows::create_deep_research_workflow(&hatchet, r2r_url);

            worker =
                hatchet_sdk::worker::worker::Register::add_task_or_workflow(worker, &mission_wf);
            worker = hatchet_sdk::worker::worker::Register::add_task_or_workflow(worker, &task_wf);
            worker = hatchet_sdk::worker::worker::Register::add_task_or_workflow(
                worker,
                &deep_research_wf,
            );

            worker.start().await?;
        }
        Commands::Poller {
            github_repos,
            gitlab_projects,
            interval_secs,
            mcp_url,
            r2r_url,
            kafka_brokers,
            aethalgard_webhook_url,
        } => {
            use factory_application::poller_service::PollerDaemonService;
            use factory_application::workflows::comment_control::CommentControlService;
            use factory_infrastructure::{
                GitPlatformPoller, InMemoryCursorStore, PostgresCursorStore,
                HttpGithubClient, HttpGitlabClient, McpHttpClient,
                HttpR2rClient, HttpAethalgardClient, HttpSemanticaClient,
                KafkaClient,
            };
            use std::sync::Arc;

            tracing::info!("Starting Dark Gravity Native Outbound Poller Daemon...");

            let github_token = std::env::var("GITHUB_API_TOKEN").unwrap_or_default();
            let gitlab_token = std::env::var("GITLAB_API_TOKEN").unwrap_or_default();
            let gitlab_url = std::env::var("GITLAB_URL").unwrap_or_else(|_| "https://gitlab.com".to_string());
            let db_url = std::env::var("DATABASE_URL").unwrap_or_default();

            let gh_client: Option<Arc<dyn factory_infrastructure::GithubClient>> = if !github_token.is_empty() {
                Some(Arc::new(HttpGithubClient::new(github_token)))
            } else {
                None
            };

            let gl_client: Option<Arc<dyn factory_infrastructure::GitlabClient>> = if !gitlab_token.is_empty() {
                Some(Arc::new(HttpGitlabClient::new(gitlab_url, gitlab_token)))
            } else {
                None
            };

            let cursor_store: Arc<dyn factory_infrastructure::CursorStore> = if !db_url.is_empty() {
                Arc::new(PostgresCursorStore::new(db_url))
            } else {
                Arc::new(InMemoryCursorStore::new())
            };

            let poller = Arc::new(GitPlatformPoller::new(
                gh_client.clone(),
                gl_client.clone(),
                cursor_store,
            ));

            let mcp_client = Arc::new(McpHttpClient::new(mcp_url));
            let r2r_user = std::env::var("R2R_SUPERUSER_EMAIL").unwrap_or_else(|_| "lgcorzo@gmail.com".to_string());
            let r2r_pwd = std::env::var("R2R_SUPERUSER_PASSWORD").unwrap_or_else(|_| "admin".to_string());
            let r2r_client = Arc::new(HttpR2rClient::new(r2r_url, r2r_user, r2r_pwd));
            let aethalgard_client = Arc::new(HttpAethalgardClient::new(aethalgard_webhook_url));

            let kafka_client: Arc<dyn KafkaClient> = if kafka_brokers == "mock" || kafka_brokers.is_empty() {
                #[cfg(not(feature = "production"))]
                {
                    Arc::new(factory_infrastructure::SimpleMockKafkaClient::new(&kafka_brokers).unwrap())
                }
                #[cfg(feature = "production")]
                {
                    Arc::new(factory_infrastructure::RdKafkaClient::new(&kafka_brokers)?)
                }
            } else {
                #[cfg(not(feature = "production"))]
                {
                    Arc::new(factory_infrastructure::SimpleMockKafkaClient::new(&kafka_brokers).unwrap())
                }
                #[cfg(feature = "production")]
                {
                    Arc::new(factory_infrastructure::RdKafkaClient::new(&kafka_brokers)?)
                }
            };

            let semantica_url = std::env::var("SEMANTICA_URL").unwrap_or_default();
            let semantica_client = if !semantica_url.is_empty() {
                Some(Arc::new(HttpSemanticaClient::new(semantica_url, None)) as Arc<dyn factory_infrastructure::SemanticaClient>)
            } else {
                None
            };

            let comment_service = Arc::new(CommentControlService::new(
                gh_client,
                gl_client,
                mcp_client,
                r2r_client,
                aethalgard_client,
            ));

            let daemon = PollerDaemonService::new(
                poller,
                kafka_client,
                semantica_client,
                comment_service,
            );

            let gh_repos_list: Vec<String> = github_repos
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let gl_projects_list: Vec<String> = gitlab_projects
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            tracing::info!(
                "Poller active: interval={}s, github_repos={:?}, gitlab_projects={:?}",
                interval_secs, gh_repos_list, gl_projects_list
            );

            loop {
                let stats = daemon.poll_once(&gh_repos_list, &gl_projects_list).await;
                if stats.issues_ingested > 0 || stats.directives_processed > 0 {
                    tracing::info!(
                        "Poll cycle: {} issues ingested, {} directives processed",
                        stats.issues_ingested, stats.directives_processed
                    );
                }
                if !stats.errors.is_empty() {
                    for err in stats.errors {
                        tracing::warn!("Poller cycle warning: {}", err);
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(interval_secs)).await;
            }
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
