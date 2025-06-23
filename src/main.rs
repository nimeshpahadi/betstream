mod models;
mod handlers;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    init();

    // Get database URL from environment or use default SQLite file
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./manual-betting-server.db".to_string());

    // Create database connection pool with create_if_missing option
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(
            database_url.parse::<sqlx::sqlite::SqliteConnectOptions>()?
                .create_if_missing(true)
        )
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Build router
    let app = Router::new()
        .route("/", get(|| async { "Betting API 🎲" }))
        .route("/health", get(|| async { "OK" }))
        // Account routes
        .route("/api/v1/accounts", get(handlers::get_accounts))
        .route("/api/v1/accounts", post(handlers::create_account))
        .route("/api/v1/accounts/:id", get(handlers::get_account))
        .route("/api/v1/accounts/:id", put(handlers::update_account))
        .route("/api/v1/accounts/:id", delete(handlers::delete_account))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    println!("🚀 Server running on http://localhost:3001");
    
    axum::serve(listener, app).await?;

    Ok(())
}
