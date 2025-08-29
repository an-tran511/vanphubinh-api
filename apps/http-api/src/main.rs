use std::sync::Arc;

use axum::Router;
use http_api::{app_state::AppState, config};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: config::Config = config::Config::new()?;

    init_tracing();

    info!("Starting VPB ERP Backend...");

    let pool = PgPool::connect(&config.db_url).await?;

    let app_state = Arc::new(AppState { pool });

    let app = Router::new()
        .with_state(CorsLayer::permissive())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(config.addr).await?;
    info!("Listening on http://{}", config.addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Server shutdown complete");

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl+c");
    info!("Received shutdown signal");
}

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "http_api=info,tower_http=debug".into());

    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}
