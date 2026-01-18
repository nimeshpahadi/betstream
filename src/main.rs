mod models;
mod handlers;

use axum::{
    routing::{get, post, put, patch, delete},
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
    update_account_batch_bet,
    update_account_batch_bets,
    complete_account_batch,
    sse_handler,
    AppState
};
use tokio::sync::broadcast;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use models::account::{
    Account, CreateAccountRequest, Batch, BatchResponse, 
    Bet, CreateBatchRequest, CreateBetRequest, 
    UpdateBetStatusRequest, BetUpdateRequest, BetStatus
};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::accounts::get_accounts,
        handlers::accounts::create_account,
        handlers::accounts::get_account,
        handlers::accounts::update_account,
        handlers::accounts::delete_account,
        handlers::accounts::create_batch,
        handlers::accounts::account_batches,
        handlers::accounts::update_account_batch_bet,
        handlers::accounts::update_account_batch_bets,
        handlers::accounts::complete_account_batch,
    ),
    components(
        schemas(
            Account, 
            CreateAccountRequest, 
            Batch, 
            BatchResponse, 
            Bet, 
            CreateBatchRequest, 
            CreateBetRequest, 
            UpdateBetStatusRequest, 
            BetUpdateRequest,
            BetStatus
        )
    ),
    tags(
        (name = "accounts", description = "Account management endpoints"),
        (name = "batches", description = "Batch management endpoints"),
        (name = "bets", description = "Bet management endpoints")
    ),
    info(
        title = "Betstream API",
        version = "1.0.0",
        description = "API for managing betting accounts, batches, and bets",
        contact(
            name = "API Support",
            email = "support@betstream.com"
        )
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    init();
    
    // Get database URL from environment or use default SQLite file
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./betstream.db?mode=rwc".to_string());
    
    // Create database connection pool
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to SQLite");
    
    // ENABLE WAL MODE
    sqlx::query("PRAGMA journal_mode = WAL;")
        .execute(&pool)
        .await
        .expect("Failed to enable WAL mode");
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    // Create app state
    let app_state = create_app_state(pool);
    
    // Build router
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(|| async { "Betting API 🎲" }))
        .route("/health", get(|| async { "OK" }))
        // Account routes
        .route("/api/v1/accounts", get(get_accounts))
        .route("/api/v1/accounts", post(create_account))
        .route("/api/v1/accounts/:id", get(get_account))
        .route("/api/v1/accounts/:id", put(update_account))
        .route("/api/v1/accounts/:id", delete(delete_account))
        .route("/api/v1/accounts/:id/batches", post(create_batch))
        .route("/api/v1/accounts/:id/batches", get(account_batches))
        .route("/api/v1/accounts/:id/batches/:batch_id/bets/:bet_id", patch(update_account_batch_bet))
        .route("/api/v1/accounts/:id/batches/:batch_id/bets", patch(update_account_batch_bets))
        .route("/api/v1/accounts/:id/batches/:batch_id", delete(complete_account_batch))
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
    println!("📚 Swagger UI available at http://localhost:3001/swagger-ui");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

pub fn create_app_state(pool: SqlitePool) -> AppState {
    let (event_sender, _) = broadcast::channel(1000);
    AppState { pool, event_sender }
}
