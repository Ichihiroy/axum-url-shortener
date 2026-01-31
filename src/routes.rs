use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Application state
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

// Health check response
#[derive(Serialize)]
struct HealthResponse {
    status: String,
    database: String,
}

// Example user structure
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<u32>,
    pub name: String,
    pub email: String,
}

// Health check endpoint
async fn health_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let db_status = match state.db.ping().await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    let response = HealthResponse {
        status: "ok".to_string(),
        database: db_status.to_string(),
    };

    (StatusCode::OK, Json(response))
}

// Root endpoint
async fn root() -> &'static str {
    "Welcome to Axum API with MySQL!"
}

// Example POST endpoint
async fn create_user(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    // Here you would typically save to database
    tracing::info!("Creating user: {} - {}", payload.name, payload.email);
    
    let user = User {
        id: Some(1), // This would come from database
        name: payload.name,
        email: payload.email,
    };

    (StatusCode::CREATED, Json(user))
}

// Configure all routes
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/users", post(create_user))
        .with_state(state)
}
