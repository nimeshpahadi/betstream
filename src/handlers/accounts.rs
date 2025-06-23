use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use sqlx::SqlitePool;
use crate::models::{Account, CreateAccountRequest, UpdateAccountRequest};

// GET /api/v1/accounts - Get all accounts
pub async fn get_accounts(State(pool): State<SqlitePool>) -> Result<Json<Vec<Account>>, StatusCode> {
    let accounts = sqlx::query_as::<_, Account>("SELECT * FROM accounts ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(accounts))
}

// GET /api/v1/accounts/:id - Get account by ID
pub async fn get_account(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Account>, StatusCode> {
    let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(account))
}

// POST /api/v1/accounts - Create new account
pub async fn create_account(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateAccountRequest>,
) -> Result<Json<Account>, StatusCode> {
    let account = sqlx::query_as::<_, Account>(
        r#"
        INSERT INTO accounts (name, hostname, created_at, updated_at)
        VALUES (?, ?, datetime('now'), datetime('now'))
        RETURNING *
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.hostname)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(account))
}

// PUT /api/v1/accounts/:id - Update account
pub async fn update_account(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAccountRequest>,
) -> Result<Json<Account>, StatusCode> {
    // First check if account exists
    let _existing = sqlx::query("SELECT id FROM accounts WHERE id = ?")
        .bind(&id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // Build dynamic update query
    let mut query = "UPDATE accounts SET updated_at = datetime('now')".to_string();
    let mut params: Vec<String> = vec![];
    let mut param_count = 0;

    if let Some(name) = &payload.name {
        query.push_str(", name = ?");
        params.push(name.clone());
        param_count += 1;
    }

    if let Some(hostname) = &payload.hostname {
        query.push_str(", hostname = ?");
        params.push(hostname.clone());
        param_count += 1;
    }

    query.push_str(" WHERE id = ? RETURNING *");

    let mut sqlx_query = sqlx::query_as::<_, Account>(&query);
    
    for param in &params {
        sqlx_query = sqlx_query.bind(param);
    }
    
    sqlx_query = sqlx_query.bind(&id);

    let account = sqlx_query
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(account))
}

// DELETE /api/v1/accounts/:id - Delete account
pub async fn delete_account(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM accounts WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
