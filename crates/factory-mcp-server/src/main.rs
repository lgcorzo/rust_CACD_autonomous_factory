use axum::{
    routing::{get, post},
    Router,
};
use factory_mcp_server::McpServer;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Initializing MCP Server...");
    let server = Arc::new(McpServer::new());
    server.register_default_tools().await?;

    // Security: Restrict CORS to allowed origins and specific methods/headers.
    // Defaults to a restrictive policy (denying all) if ALLOWED_ORIGINS is unset.
    let allowed_origins_raw = std::env::var("ALLOWED_ORIGINS").unwrap_or_default();
    let cors = CorsLayer::new()
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/mcp", post(McpServer::post_handler))
        .route("/sse", get(McpServer::sse_handler));

    let app = if allowed_origins_raw.is_empty() {
        app.layer(cors)
    } else {
        let origins = allowed_origins_raw
            .split(',')
            .map(|s| s.trim().parse().expect("Invalid origin in ALLOWED_ORIGINS"))
            .collect::<Vec<axum::http::HeaderValue>>();
        app.layer(cors.allow_origin(origins))
    }
    .with_state(server);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8100".to_string())
        .parse::<u16>()?;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("MCP Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
