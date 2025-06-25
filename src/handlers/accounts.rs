use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{
        sse::{Event, Sse},
        Json,
    },
    Json as JsonExtract,
};
use futures::stream::{self, Stream};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::{convert::Infallible, time::Duration};
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use crate::models::account::{BatchUpload, BatchResponse};

// Event structure similar to your Go broker.Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerEvent {
    pub id: Option<i64>,
    pub pk: Option<i64>,
    pub account_id: Option<i64>,
    pub account_name: Option<String>,
    pub account_hostname: Option<String>,
    // pub batch_id: Option<i64>,
    // pub bet_id: Option<i64>,
    pub event: String,
}

// Global event broadcaster
pub type EventSender = broadcast::Sender<BrokerEvent>;

// Application state that includes the event broadcaster
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub event_sender: EventSender,
}

// Your existing structs
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub hostname: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub name: String,
    pub hostname: String,
}

// SSE endpoint handler
pub async fn sse_handler(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.event_sender.subscribe();
    let event_stream = BroadcastStream::new(rx)
        .filter_map(|result| {
            match result {
                Ok(broker_event) => {
                    // Convert BrokerEvent to SSE Event
                    let event_name = broker_event.event.clone();
                    let data = match serde_json::to_string(&broker_event) {
                        Ok(json) => json,
                        Err(_) => return None,
                    };
                    
                    Some(Ok(Event::default()
                        .event(event_name)
                        .data(data)))
                }
                Err(_) => None,
            }
        });

    // Add heartbeat similar to your ticker
    let heartbeat = stream::repeat_with(|| {
        Ok(Event::default()
            .event("ping")
            .data(chrono::Utc::now().to_rfc3339()))
    })
    .throttle(Duration::from_secs(10));

    let combined_stream = stream::select(event_stream, heartbeat);

    Sse::new(combined_stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive"),
    )
}

// GET /api/v1/accounts - Get all accounts
pub async fn get_accounts(
    State(state): State<AppState>,
) -> Result<Json<Vec<Account>>, StatusCode> {
    let accounts = sqlx::query_as::<_, Account>("SELECT * FROM accounts ORDER BY created_at DESC")
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(accounts))
}

// GET /api/v1/accounts/:id - Get account by ID
pub async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Account>, StatusCode> {
    let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = ?")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(account))
}

// Updated create_account with event publishing
pub async fn create_account(
    State(state): State<AppState>,
    JsonExtract(payload): JsonExtract<CreateAccountRequest>,
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
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Publish event (similar to your Go broker.Publish)
    let event = BrokerEvent {
        id: Some(chrono::Utc::now().timestamp_millis()),
        pk: Some(account.id),
        account_id: Some(account.id),
        account_name: Some(account.name.clone()),
        account_hostname: Some(account.hostname.clone()),
        // batch_id: None,
        // bet_id: None,
        event: "account_created".to_string(),
    };

    // Send event to all SSE subscribers
    if let Err(e) = state.event_sender.send(event) {
        eprintln!("Failed to send event: {}", e);
    }

    println!(
        "Account created - ID: {}, Name: {}, Hostname: {}",
        account.id, account.name, account.hostname
    );

    Ok(Json(account))
}

// Update account implementation
pub async fn update_account(
    State(state): State<AppState>,
    axum::extract::Path(account_id): axum::extract::Path<i64>,
    JsonExtract(payload): JsonExtract<CreateAccountRequest>,
) -> Result<Json<Account>, StatusCode> {
    let account = sqlx::query_as::<_, Account>(
        r#"
        UPDATE accounts 
        SET name = ?, hostname = ?, updated_at = datetime('now')
        WHERE id = ?
        RETURNING *
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.hostname)
    .bind(account_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Publish update event
    let event = BrokerEvent {
        id: Some(chrono::Utc::now().timestamp_millis()),
        pk: Some(account.id),
        account_id: Some(account.id),
        account_name: Some(account.name.clone()),
        account_hostname: Some(account.hostname.clone()),
        // batch_id: None,
        // bet_id: None,
        event: "account_updated".to_string(),
    };

    if let Err(e) = state.event_sender.send(event) {
        eprintln!("Failed to send event: {}", e);
    }

    println!(
        "Account updated - ID: {}, Name: {}, Hostname: {}",
        account.id, account.name, account.hostname
    );

    Ok(Json(account))
}

pub async fn delete_account(
    State(state): State<AppState>,
    axum::extract::Path(account_id): axum::extract::Path<i64>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM accounts WHERE id = ?")
        .bind(account_id)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Publish delete event
    let event = BrokerEvent {
        id: Some(chrono::Utc::now().timestamp_millis()),
        pk: Some(account_id),
        account_id: Some(account_id),
        account_name: None,
        account_hostname: None,
        // batch_id: None,
        // bet_id: None,
        event: "account_deleted".to_string(),
    };

    if let Err(e) = state.event_sender.send(event) {
        eprintln!("Failed to send event: {}", e);
    }

    println!("Account deleted - ID: {}", account_id);
    Ok(StatusCode::NO_CONTENT)
}

pub async fn create_batch(
    Path(account_id): Path<i64>,
    State(state): State<AppState>,
    Json(upload_batch): Json<BatchUpload>,
) -> Result<Json<BatchResponse>, (StatusCode, String)> {
    let batch = sqlx::query_as::<_, Batch>(
        r#"
        INSERT INTO batches (meta, account_id, created_at, updated_at)
        VALUES (?, ?, datetime('now'), datetime('now'))
        RETURNING *
        "#,
    )
    .bind(&upload_batch)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let event = BrokerEvent {
        id: Some(chrono::Utc::now().timestamp_millis()),
        pk: Some(batch.id),
        account_id: Some(account_id),
        batch_id: Some(batch.id),
        // bet_id: None,
        event: "batch_created".to_string(),
    };

    // Send event to all SSE subscribers
    if let Err(e) = state.event_sender.send(event) {
        eprintln!("Failed to send event: {}", e);
    }

    Ok(Json(batch))
}
