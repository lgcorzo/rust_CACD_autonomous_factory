use factory_mcp_server::{build_mcp_router, McpServer};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    rustls::crypto::ring::default_provider()
        .install_default()
        .ok();

    tracing::info!("Initializing MCP Server...");
    let server = Arc::new(McpServer::new());
    server.register_default_tools().await?;

    // Security: Restrict CORS to allowed origins and specific methods/headers.
    // Note (FR-011): CORS is browser cross-origin policy only and is NEVER treated as caller authentication.
    // Privileged routes (/mcp, /sse, /feedback) enforce bearer authentication via build_mcp_router.
    let allowed_origins_raw = std::env::var("ALLOWED_ORIGINS").unwrap_or_default();
    let cors = CorsLayer::new()
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ]);

    let app = build_mcp_router(server);

    let app = if allowed_origins_raw.is_empty() {
        app.layer(cors)
    } else {
        let origins = allowed_origins_raw
            .split(',')
            .map(|s| s.trim().parse().expect("Invalid origin in ALLOWED_ORIGINS"))
            .collect::<Vec<axum::http::HeaderValue>>();
        app.layer(cors.allow_origin(origins))
    };

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8100".to_string())
        .parse::<u16>()?;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("MCP Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
