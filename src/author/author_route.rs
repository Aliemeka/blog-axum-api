use crate::app_state::{AppState, SharedState};
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
};
use serde_json::json;
use std::sync::Arc;

// Author routes that handles all requests related to authors
pub fn author_routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(author_handler))
}

// Author handler that simulates fetching authors from the database
async fn author_handler(State(state): State<SharedState>) -> impl IntoResponse {
    let authors = sqlx::query("SELECT 1").fetch_one(&state.db_pool).await;
    if authors.is_ok() {
        return (
            StatusCode::OK,
            Json(json!({
                "status": "ok",
                "message": "Authors retrieved successfully",
                "authors": []
            })),
        );
    }
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "status": "error",
            "message": "Authors not found"
        })),
    )
}
