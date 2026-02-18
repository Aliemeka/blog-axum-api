use axum::{Router, routing::get};
use tokio::net::TcpListener;

mod handlers;
mod models;
mod services;
use handlers::{get_author_by_id, get_authors, get_posts, health_handler, hello, submit_post};
use models::{AppState, SharedState};

#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn create_app() -> Router {
    let state = SharedState::new(AppState::new());
    Router::new()
        .route("/", get(hello))
        .route("/health", get(health_handler))
        .route("/authors", get(get_authors))
        .route("/authors/{id}", get(get_author_by_id))
        .route("/posts", get(get_posts).post(submit_post))
        .with_state(state)
}
