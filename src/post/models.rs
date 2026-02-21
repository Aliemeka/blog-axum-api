use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::uuid};

// Post entity that represents a post in the database
#[derive(Clone, FromRow)]
pub struct Post {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub author_id: uuid::Uuid,
}

#[derive(Serialize)]
pub struct DisplayPost {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author_id: String,
}

#[derive(Serialize)]
pub struct PostList {
    pub data: Vec<DisplayPost>,
    pub total: usize,
}

#[derive(Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
    pub author_id: String,
}
