use crate::app_state::{AppState, SharedState};
use crate::config::{AppConfig, connect_db};
use crate::router::create_router;
use tokio::net::TcpListener;

// Creates and starts the Axum application
pub async fn create_app() {
    // Get the environment variables
    let app_config = AppConfig::from_env().expect("Failed to load configuration from environment");

    // Get the db pool from the database url returned by AppConfig.database_url
    let db_pool = connect_db(&app_config.database_url)
        .await
        .expect("Failed to connect to the database");

    let app_state = SharedState::new(AppState::new(db_pool));

    let app = create_router(app_state);

    let server_address = format!("127.0.0.1:{}", app_config.server_port);

    let listener = TcpListener::bind(&server_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
