mod models;
mod handlers;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::sqlite::SqlitePool;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::fmt::init;
use handlers::accounts::{
    create_account,
    update_account,
    get_accounts,
    get_account,
    delete_account,
    create_batch,
    account_batches,
    sse_handler,
    AppState
};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    init();

    // Get database URL from environment or use default SQLite file
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./manual-betting-server.db?mode=rwc".to_string());

    // Create database connection pool with create_if_missing option
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to SQLite");

    // Run migrations if needed
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Create app state with event broadcaster
    let app_state = create_app_state(pool);

    // Build router
    let app = Router::new()
        .route("/", get(|| async { "Betting API 🎲" }))
        .route("/health", get(|| async { "OK" }))
        // Account routes
        .route("/api/v1/accounts", get(get_accounts))
        .route("/api/v1/accounts", post(create_account))
        .route("/api/v1/accounts/:id", get(get_account))
        .route("/api/v1/accounts/:id", put(update_account))
        .route("/api/v1/accounts/:id", delete(delete_account))
        .route("/api/v1/accounts/:id/batches", post(create_batch))
        // .route("/api/v1/accounts/:id/batches/:batch_id", delete(delete_account_batch))
        .route("/api/v1/accounts/:id/batches", get(account_batches))
        // .route("/api/v1/accounts/:id/batches/:batch_id", get(account_batch))
        .route("/sse", get(sse_handler))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(app_state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    println!("🚀 Server running on http://localhost:3001");
    
    axum::serve(listener, app).await?;

    Ok(())
}


pub fn create_app_state(pool: SqlitePool) -> AppState {
    let (event_sender, _) = broadcast::channel(1000);
    AppState { pool, event_sender }
}