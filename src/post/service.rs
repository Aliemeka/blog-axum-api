use crate::post::models::{CreatePost, Post};
use crate::{app_state::SharedState, error::AppError};
use sqlx::{query, types::uuid};

pub async fn save_new_post(post: CreatePost, state: SharedState) -> Result<bool, AppError> {
    if let Ok(author_id_as_uuid) = uuid::Uuid::parse_str(&post.author_id) {
        let result = query("INSERT INTO posts (title, content, author_id) VALUES ($1, $2, $3)")
            .bind(&post.title)
            .bind(&post.content)
            .bind(author_id_as_uuid)
            .execute(&state.db_pool)
            .await;
        if let Err(e) = result {
            eprintln!("DB error saving post: {e}");
            return Err(AppError::InternalServerError(
                "Failed to save post".to_string(),
            ));
        }
        return Ok(true);
    }
    Err(AppError::UnProcessableEntity {
        field: "author_id".to_string(),
        message: "Invalid UUID".to_string(),
    })
}

pub async fn get_all_posts(state: SharedState) -> Result<Vec<Post>, AppError> {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(&state.db_pool)
        .await;
    match posts {
        Ok(posts) => Ok(posts),
        Err(e) => {
            eprintln!("DB error fetching posts: {e}");
            Err(AppError::InternalServerError(
                "Failed to fetch posts".to_string(),
            ))
        }
    }
}
