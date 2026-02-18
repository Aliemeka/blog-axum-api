use axum::routing::post;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Clone)]
pub struct Author {
    pub id: u16,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct AuthorList {
    pub data: Vec<Author>,
    pub total: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: u16,
    pub title: String,
    pub content: String,
    pub author_id: u16,
}

#[derive(Serialize)]
pub struct PostList {
    pub posts: Vec<Post>,
    pub total: usize,
}

impl Author {
    pub fn new(id: u16, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}

#[derive(Deserialize, Clone)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
    pub author_id: u16,
}

pub struct AppState {
    pub authors: RwLock<Vec<Author>>,
    pub posts: RwLock<Vec<Post>>,
    pub next_author_id: RwLock<u16>,
    pub next_post_id: RwLock<u16>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            authors: RwLock::new(seed_authors()),
            posts: RwLock::new(Vec::new()),
            next_author_id: RwLock::new(3),
            next_post_id: RwLock::new(1),
        }
    }

    pub async fn get_author_by_id(&self, author_id: u16) -> Option<Author> {
        let authors = self.authors.read().await;
        authors.iter().find(|a| a.id == author_id).cloned()
    }

    pub async fn add_new_post(
        &self,
        title: String,
        content: String,
        author_id: u16,
    ) -> Option<Post> {
        let mut posts = self.posts.write().await;
        let mut next_post_id = self.next_post_id.write().await;
        if let Some(_author) = self.get_author_by_id(author_id).await {
            let post = Post {
                id: *next_post_id,
                title,
                content,
                author_id,
            };
            posts.push(post.clone());
            *next_post_id += 1;
            Some(post)
        } else {
            None
        }
    }

    pub async fn find_posts(&self, author_id: Option<u16>) -> Vec<Post> {
        let posts = self.posts.read().await;
        match author_id {
            None => posts.clone(),
            Some(id) => posts
                .iter()
                .filter(|p| p.author_id == id)
                .cloned()
                .collect(),
        }
    }
}

fn seed_authors() -> Vec<Author> {
    vec![
        Author::new(
            1,
            "Emeka Allison".to_string(),
            "emekaallison4@gmail.com".to_string(),
        ),
        Author::new(2, "John Doe".to_string(), "johndoe@mail.com".to_string()),
    ]
}

pub type SharedState = Arc<AppState>;
