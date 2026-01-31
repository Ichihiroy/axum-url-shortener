mod database;
mod routes;

use std::sync::Arc;
use tower_http::{trace::TraceLayer, cors::CorsLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Initialize tracing (logging)
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    tracing::info!("Starting Axum server with MySQL...");

    // Establish database connection
    let db = database::establish_connection()
        .await
        .expect("Failed to connect to database");

    // Test the connection
    database::test_connection(&db)
        .await
        .expect("Failed to ping database");

    // Create application state
    let state = Arc::new(routes::AppState { db });

    // Build our application with routes
    let app = routes::create_router(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    // Get server configuration from environment
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    // Run it with hyper
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("🚀 Server listening on http://{}", addr);
    tracing::info!("📊 Health check available at http://{}/health", addr);

    axum::serve(listener, app).await?;

    Ok(())
}