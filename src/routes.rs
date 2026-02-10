use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use nanoid::nanoid;
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

// Application state
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub base_url: String,
}

// Health check response
#[derive(Serialize)]
struct HealthResponse {
    status: String,
    database: String,
}

// Request payload for shortening
#[derive(Serialize, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

// Response payload for shortening
#[derive(Serialize, Deserialize)]
pub struct ShortenResponse {
    pub short_code: String,
    pub short_url: String,
    pub original_url: String,
}

// Stats response
#[derive(Serialize, Deserialize)]
pub struct StatsResponse {
    pub short_code: String,
    pub original_url: String,
    pub clicks: i64,
    pub created_at: String,
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
    "Welcome to the Axum URL Shortener!"
}

// Shorten URL endpoint
async fn shorten_url(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ShortenRequest>,
) -> impl IntoResponse {
    if payload.url.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(json!({ "error": "url is required" })))
            .into_response();
    }

    let mut attempts = 0;
    let short_code = loop {
        attempts += 1;
        if attempts > 5 {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "failed to generate short code" })),
            )
                .into_response();
        }

        let code = nanoid!(7);
        let stmt = Statement::from_sql_and_values(
            state.db.get_database_backend(),
            "INSERT INTO urls (short_code, original_url) VALUES (?, ?)",
            vec![code.clone().into(), payload.url.clone().into()],
        );

        match state.db.execute(stmt).await {
            Ok(_) => break code,
            Err(err) => {
                let msg = err.to_string();
                if msg.contains("Duplicate") {
                    continue;
                }
                tracing::error!("Failed to insert url: {}", msg);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "failed to save url" })),
                )
                    .into_response();
            }
        }
    };

    let response = ShortenResponse {
        short_code: short_code.clone(),
        short_url: format!("{}/{}", state.base_url, short_code),
        original_url: payload.url,
    };

    (StatusCode::CREATED, Json(response)).into_response()
}

// Redirect endpoint
async fn redirect_to_url(
    State(state): State<Arc<AppState>>,
    Path(short_code): Path<String>,
) -> impl IntoResponse {
    let stmt = Statement::from_sql_and_values(
        state.db.get_database_backend(),
        "SELECT original_url FROM urls WHERE short_code = ? LIMIT 1",
        vec![short_code.clone().into()],
    );

    let row = match state.db.query_one(stmt).await {
        Ok(row) => row,
        Err(err) => {
            tracing::error!("Failed to query url: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "database error").into_response();
        }
    };

    let Some(row) = row else {
        return (StatusCode::NOT_FOUND, "short code not found").into_response();
    };

    let original_url: String = match row.try_get("", "original_url") {
        Ok(url) => url,
        Err(err) => {
            tracing::error!("Failed to read url: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "database error").into_response();
        }
    };

    let update_stmt = Statement::from_sql_and_values(
        state.db.get_database_backend(),
        "UPDATE urls SET clicks = clicks + 1 WHERE short_code = ?",
        vec![short_code.into()],
    );

    if let Err(err) = state.db.execute(update_stmt).await {
        tracing::error!("Failed to update clicks: {}", err);
    }

    Redirect::temporary(&original_url).into_response()
}

// Stats endpoint
async fn stats(
    State(state): State<Arc<AppState>>,
    Path(short_code): Path<String>,
) -> impl IntoResponse {
    let stmt = Statement::from_sql_and_values(
        state.db.get_database_backend(),
        "SELECT original_url, clicks, DATE_FORMAT(created_at, '%Y-%m-%dT%H:%i:%s') AS created_at FROM urls WHERE short_code = ? LIMIT 1",
        vec![short_code.clone().into()],
    );

    let row = match state.db.query_one(stmt).await {
        Ok(row) => row,
        Err(err) => {
            tracing::error!("Failed to query stats: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "database error").into_response();
        }
    };

    let Some(row) = row else {
        return (StatusCode::NOT_FOUND, "short code not found").into_response();
    };

    let original_url: String = match row.try_get("", "original_url") {
        Ok(url) => url,
        Err(err) => {
            tracing::error!("Failed to read url: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "database error").into_response();
        }
    };

    let clicks: i64 = match row.try_get("", "clicks") {
        Ok(value) => value,
        Err(err) => {
            tracing::error!("Failed to read clicks: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "database error").into_response();
        }
    };

    let created_at: String = match row.try_get("", "created_at") {
        Ok(value) => value,
        Err(err) => {
            tracing::error!("Failed to read created_at: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "database error").into_response();
        }
    };

    let response = StatsResponse {
        short_code,
        original_url,
        clicks,
        created_at,
    };

    (StatusCode::OK, Json(response)).into_response()
}

// Configure all routes
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
    .route("/shorten", post(shorten_url))
    .route("/stats/:short_code", get(stats))
    .route("/:short_code", get(redirect_to_url))
        .with_state(state)
}
