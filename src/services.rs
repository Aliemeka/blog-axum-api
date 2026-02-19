use crate::models::{CreatePost, Post, SharedState};

pub async fn create_post(state: SharedState, payload: CreatePost) -> Option<Post> {
    state
        .add_new_post(payload.title, payload.content, payload.author_id)
        .await
}

pub async fn find_posts(state: SharedState, author_id: Option<u16>) -> Vec<Post> {
    state.find_posts(author_id).await
}
