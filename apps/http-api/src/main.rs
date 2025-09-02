use std::env;
use std::fs;
use std::sync::Arc;

use http_api::{app_state::AppState, config, routes::units_router};
use sqlx::PgPool;
use tracing::info;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(OpenApi)]
    #[openapi(info(
        title = "Van Phu Binh API",
        version = "1.0.0",
        description = "API for managing Van Phu Binh Internal System"
    ))]
    pub struct ApiDoc;

    init_tracing();

    let config = config::Config::new()?;
    info!("Starting VPB ERP Backend...");

    let pool = PgPool::connect(&config.db_url).await?;
    let app_state = Arc::new(AppState { pool });

    let openapi = ApiDoc::openapi();
    let (app, openapi_doc) = OpenApiRouter::with_openapi(openapi.clone())
        .nest("/units", units_router::init())
        .with_state(app_state)
        .split_for_parts();

    let env = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
    if env == "development" || env == "dev" {
        generate_openapi_json(&openapi_doc)?;
    }

    let app = app.merge(Scalar::with_url("/docs", openapi_doc.clone()));

    let listener = tokio::net::TcpListener::bind(config.addr).await?;
    info!("Listening on http://{}", config.addr);

    // Spawn the server in a background task
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .expect("Server error");
    });

    // Wait for the server to finish (i.e., until shutdown)
    server.await?;

    info!("Server shutdown complete");

    Ok(())
}

fn generate_openapi_json(api: &utoipa::openapi::OpenApi) -> Result<(), Box<dyn std::error::Error>> {
    let openapi_json = serde_json::to_string_pretty(api)?;
    let output_path = "./apps/http-api/openapi.json";
    fs::write(output_path, &openapi_json)?;
    info!("✅ Generated {}", output_path);
    info!("📄 {} bytes written", openapi_json.len());
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
