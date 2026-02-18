use crate::models::{AppState, CreatePost, Post};
use std::sync::Arc;

pub async fn create_post(state: Arc<AppState>, payload: CreatePost) -> Option<Post> {
    state
        .add_new_post(payload.title, payload.content, payload.author_id)
        .await
}

pub async fn find_posts(state: Arc<AppState>, author_id: Option<u16>) -> Vec<Post> {
    state.find_posts(author_id).await
}
