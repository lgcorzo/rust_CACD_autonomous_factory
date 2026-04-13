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

    let app = Router::new()
        .route("/mcp", post(McpServer::post_handler))
        .route("/sse", get(McpServer::sse_handler))
        .layer(CorsLayer::permissive())
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
