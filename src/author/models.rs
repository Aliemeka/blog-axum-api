use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::uuid::Uuid};

// Author entity that represents an author in the database
#[derive(Clone, FromRow)]
pub struct Author {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct DisplayAuthor {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct AuthorList {
    pub data: Vec<DisplayAuthor>,
    pub total: usize,
}

#[derive(Deserialize)]
pub struct CreateAuthor {
    pub name: String,
    pub email: String,
}
