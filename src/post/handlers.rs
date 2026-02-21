use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostFilter {
    pub author_id: Option<u16>,
}

// pub async fn get_posts(
//     State(state): State<SharedState>,
//     Query(filter): Query<PostFilter>,
// ) -> Json<PostList> {
//     let posts = find_posts(state, filter.author_id).await;
//     let total = posts.len();
//     Json(PostList { posts, total })
// }
