use crate::models::{Author, AuthorList, CreatePost, PostList, SharedState};
use crate::services::{create_post, find_posts};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::json;

pub async fn health_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "message": "Server is running"
        })),
    )
}

pub async fn hello() -> &'static str {
    "Hello, World!"
}

pub async fn get_authors(State(state): State<SharedState>) -> Json<AuthorList> {
    let authors = state.authors.read().await;
    let data: Vec<Author> = authors
        .iter()
        .map(|author| Author {
            id: author.id,
            name: author.name.clone(),
            email: author.email.clone(),
        })
        .collect();
    let total = authors.len();
    Json(AuthorList { data, total })
}

pub async fn get_author_by_id(
    State(state): State<SharedState>,
    Path(id): Path<u16>,
) -> impl IntoResponse {
    let authors = state.authors.read().await;
    let author_id = id;
    if let Some(author) = authors.iter().find(|a| a.id == author_id) {
        (
            StatusCode::OK,
            Json(json!({
                "data": author.clone()
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "Author not found"
            })),
        )
    }
}

pub async fn submit_post(
    State(state): State<SharedState>,
    Json(payload): Json<CreatePost>,
) -> impl IntoResponse {
    match create_post(state, payload).await {
        Some(post) => (
            StatusCode::CREATED,
            Json(json!({
                "data": post
            })),
        ),
        None => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": "Failed to create post. Author may not exist."
            })),
        ),
    }
}
