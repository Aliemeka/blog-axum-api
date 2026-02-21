mod app;
mod app_state;
mod author;
mod config;
mod error;
mod handlers;
mod models;
mod post;
mod router;
mod services;

use app::create_app;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    // Load in environment variables from .env file
    dotenv().ok();

    // Start the application
    create_app().await;
}
