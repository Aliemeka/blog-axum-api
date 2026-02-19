use crate::app_state::SharedState;
use crate::author::author_route::author_routes;
use crate::handlers::{health_handler, hello};
use axum::{Router, routing::get};

// Routes declared here
pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .nest("/authors", author_routes())
        .route("/", get(hello))
        .route("/health", get(health_handler))
        .with_state(state)
}
