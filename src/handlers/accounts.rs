use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{
        sse::{Event, Sse, KeepAlive},
        Json,
    },
    Json as JsonExtract,
};
use futures::stream::Stream;
use sqlx::SqlitePool;
use std::{convert::Infallible, time::Duration};
use tokio::sync::broadcast;
use tokio_stream::wrappers::{BroadcastStream, IntervalStream};
use tokio_stream::StreamExt;
use crate::models::account::*;


// Global event broadcaster
pub type EventSender = broadcast::Sender<BrokerEvent>;

// Application state that includes the event broadcaster
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub event_sender: EventSender,
}

// SSE endpoint handler
pub async fn sse_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let db = state.pool.clone();
    let rx = state.event_sender.subscribe();

    let event_stream = BroadcastStream::new(rx)
        .then(move |result| {
            let db = db.clone();
            async move {
                let event = match result {
                    Ok(e) => e,
                    Err(_) => return None,
                };

                let event_name = event.event.clone();

                let payload = match event_name.as_str() {
                    "account_created" | "account_updated" => {
                        let row = sqlx::query_as::<_, Account>(
                            "SELECT * FROM accounts WHERE id = ?",
                        )
                        .bind(event.account_id)
                        .fetch_one(&db)
                        .await
                        .ok()?;
                        serde_json::to_string(&row).ok()
                    }

                    "account_deleted" => {
                        Some(serde_json::json!({ "id": event.pk }).to_string())
                    }
                    "batch_created" | "batch_deleted" => {
                        if event_name == "batch_deleted" {
                            // For deleted batches, create minimal response since batch may not exist in DB
                            let batch_response = BatchResponse {
                                id: event.pk.unwrap_or(0),
                                completed: true,
                                created_at: chrono::Utc::now().to_rfc3339(),
                                updated_at: chrono::Utc::now().to_rfc3339(),
                                meta: serde_json::json!({}),
                                account_id: event.account_id.unwrap_or(0),
                                bets: vec![],
                            };
                            serde_json::to_string(&batch_response).ok()
                        } else {
                            // First get the batch
                            let batch = sqlx::query_as::<_, Batch>(
                                "SELECT * FROM batches WHERE id = ?",
                            )
                            .bind(event.pk)
                            .fetch_one(&db)
                            .await
                            .ok()?;

                            // Then get the bets for this batch
                            let bets = sqlx::query_as::<_, Bet>(
                                "SELECT * FROM bets WHERE batch_id = ? ORDER BY id",
                            )
                            .bind(event.pk)
                            .fetch_all(&db)
                            .await
                            .unwrap_or_default();

                            // Create a BatchResponse object to send
                            let batch_response = BatchResponse {
                                id: batch.id,
                                completed: batch.completed,
                                created_at: batch.created_at.to_rfc3339(),
                                updated_at: batch.updated_at.to_rfc3339(),
                                meta: batch.meta,
                                account_id: batch.account_id,
                                bets,
                            };

                            serde_json::to_string(&batch_response).ok()
                        }
                    }
                    "bet_status_updated" => {
                        let row = sqlx::query_as::<_, Bet>(
                            "SELECT * FROM bets WHERE id = ?",
                        )
                        .bind(event.bet_id)
                        .fetch_one(&db)
                        .await
                        .ok()?;
                        serde_json::to_string(&row).ok()
                    }

                    _ => serde_json::to_string(&event).ok(),
                };

                payload.map(|data| {
                    Ok(Event::default()
                        .event(event_name)
                        .data(data))
                })
            }
        })
        .filter_map(|e| e);

    let heartbeat = IntervalStream::new(tokio::time::interval(Duration::from_secs(10)))
        .map(|_| {
            Ok(Event::default()
                .event("ping")
                .data(chrono::Utc::now().to_rfc3339()))
        });

    let stream = tokio_stream::StreamExt::merge(event_stream, heartbeat);

    Sse::new(stream).keep_alive(
        KeepAlive::new()
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
        .map_err(|e| {
            eprintln!("Database error fetching accounts: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(accounts))
}

// GET /api/v1/accounts/:id - Get account by ID
pub async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Account>, StatusCode> {
    let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = ?")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            eprintln!("Database error fetching account: {}", e);
            StatusCode::NOT_FOUND
        })?;

    Ok(Json(account))
}

// Create account with event publishing
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
        eprintln!("Database error inserting account: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let event = BrokerEvent {
        id: Some(chrono::Utc::now().timestamp_millis()),
        pk: Some(account.id),
        account_id: Some(account.id),
        batch_id: None,
        bet_id: None,
        event: "account_created".to_string(),
    };

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
    Path(account_id): Path<i64>,
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
        eprintln!("Database error updating account: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let event = BrokerEvent {
        id: Some(chrono::Utc::now().timestamp_millis()),
        pk: Some(account.id),
        account_id: Some(account.id),
        batch_id: None,
        bet_id: None,
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
    Path(account_id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    // Get batches before deletion (for SSE events)
    let batches = sqlx::query_as::<_, Batch>(
        "SELECT * FROM batches WHERE account_id = ?",
    )
    .bind(account_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching batches: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Delete account (cascade will handle batches and bets)
    sqlx::query("DELETE FROM accounts WHERE id = ?")
        .bind(account_id)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            eprintln!("Database error deleting account: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Send events for cascaded batch deletions
    for batch in batches {
        let _ = state.event_sender.send(BrokerEvent {
            id: Some(chrono::Utc::now().timestamp_millis()),
            pk: Some(batch.id),
            account_id: Some(account_id),
            batch_id: Some(batch.id),
            bet_id: None,
            event: "batch_deleted".to_string(),
        });
    }

    // Send account deleted event
    let event = BrokerEvent {
        id: Some(chrono::Utc::now().timestamp_millis()),
        pk: Some(account_id),
        account_id: Some(account_id),
        batch_id: None,
        bet_id: None,
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
    JsonExtract(payload): JsonExtract<CreateBatchRequest>,
) -> Result<Json<BatchResponse>, StatusCode> {
    let mut tx = state.pool.begin().await.map_err(|e| {
        eprintln!("Transaction begin error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let meta_json = serde_json::to_string(&payload.meta).map_err(|e| {
        eprintln!("JSON serialization error: {}", e);
        StatusCode::BAD_REQUEST
    })?;

    let batch = sqlx::query_as::<_, Batch>(
        r#"
        INSERT INTO batches (meta, account_id, created_at, updated_at)
        VALUES (?, ?, datetime('now'), datetime('now'))
        RETURNING *
        "#,
    )
    .bind(&meta_json)
    .bind(account_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        eprintln!("Database error creating batch: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut bets = Vec::new();
    for bet_request in payload.bets {
        let bet = sqlx::query_as::<_, Bet>(
            r#"
            INSERT INTO bets (id, selection, stake, cost, batch_id)
            VALUES (?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(bet_request.id)
        .bind(&bet_request.selection)
        .bind(bet_request.stake)
        .bind(bet_request.cost)
        .bind(batch.id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("Database error creating bet: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        bets.push(bet);
    }

    tx.commit().await.map_err(|e| {
        eprintln!("Transaction commit error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let event = BrokerEvent {
        id: Some(chrono::Utc::now().timestamp_millis()),
        pk: Some(batch.id),
        account_id: Some(account_id),
        batch_id: Some(batch.id),
        bet_id: None,
        event: "batch_created".to_string(),
    };

    if let Err(e) = state.event_sender.send(event) {
        eprintln!("Failed to send event: {}", e);
    }

    let response = BatchResponse {
        id: batch.id,
        completed: batch.completed,
        created_at: batch.created_at.to_rfc3339(),
        updated_at: batch.updated_at.to_rfc3339(),
        meta: batch.meta,
        account_id: batch.account_id,
        bets,
    };

    println!(
        "Batch created - ID: {}, Account: {}, Bets: {}",
        response.id, response.account_id, response.bets.len()
    );

    Ok(Json(response))
}

pub async fn account_batches(
    Path(account_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<Vec<BatchResponse>>, StatusCode> {
    let batches = sqlx::query_as::<_, Batch>(
        r#"
        SELECT * FROM batches 
        WHERE account_id = ? 
        ORDER BY created_at DESC
        "#,
    )
    .bind(account_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching batches: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut batch_responses = Vec::new();

    for batch in batches {
        let bets = sqlx::query_as::<_, Bet>(
            r#"
            SELECT * FROM bets 
            WHERE batch_id = ? 
            ORDER BY id
            "#,
        )
        .bind(batch.id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            eprintln!("Database error fetching bets for batch {}: {}", batch.id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        let response = BatchResponse {
            id: batch.id,
            completed: batch.completed,
            created_at: batch.created_at.to_rfc3339(),
            updated_at: batch.updated_at.to_rfc3339(),
            meta: batch.meta.clone(),
            account_id: batch.account_id,
            bets,
        };

        batch_responses.push(response);
    }

    println!(
        "Retrieved {} batches for account {}",
        batch_responses.len(),
        account_id
    );

    Ok(Json(batch_responses))
}

pub async fn update_account_batch_bets(
    Path((account_id, batch_id)): Path<(i64, i64)>,
    State(state): State<AppState>,
    Json(bets): Json<Vec<BetUpdateRequest>>,
) -> Result<Json<Vec<Bet>>, StatusCode> {
    let mut tx = state.pool.begin().await.map_err(|e| {
        eprintln!("Transaction begin error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut updated_bets = Vec::new();
    for bet in bets {
        let result = sqlx::query_as::<_, Bet>(
            r#"
            UPDATE bets SET status = 'successful'
            WHERE pid = ? AND batch_id = ?
            RETURNING *
            "#,
        )
        .bind(bet.pid)
        .bind(batch_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("Failed to update bet status to successful: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        updated_bets.push(result);
    }

    tx.commit().await.map_err(|e| {
        eprintln!("Transaction commit error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let _ = state.event_sender.send(BrokerEvent {
        id: Some(chrono::Utc::now().timestamp_millis()),
        pk: Some(batch_id),
        account_id: Some(account_id),
        batch_id: Some(batch_id),
        bet_id: None,
        event: "batch_updated".to_string(),
    });

    Ok(Json(updated_bets))
}

pub async fn update_account_batch_bet(
    Path((account_id, batch_id, bet_id)): Path<(i64, i64, i64)>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateBetStatusRequest>,
) -> Result<Json<Bet>, StatusCode> {
    let updated_bet = sqlx::query_as::<_, Bet>(
        r#"
        UPDATE bets 
        SET status = ? 
        WHERE pid = ? AND batch_id = ? 
        RETURNING pid, id, selection, stake, cost, status, batch_id
        "#
    )
    .bind(payload.status.to_string())
    .bind(bet_id)
    .bind(batch_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to update bet: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let event = BrokerEvent {
        id: Some(chrono::Utc::now().timestamp_millis()),
        pk: Some(bet_id),
        account_id: Some(account_id),
        batch_id: Some(batch_id),
        bet_id: Some(bet_id),
        event: "bet_status_updated".to_string(),
    };

    let _ = state.event_sender.send(event);

    Ok(Json(updated_bet))
}

pub async fn delete_account_batch(
    State(state): State<AppState>,
    Path((account_id, batch_id)): Path<(i64, i64)>,
) -> Result<(), StatusCode> {
    sqlx::query(
        r#"
        UPDATE batches 
        SET completed = 1, updated_at = datetime('now')
        WHERE id = ? AND account_id = ?
        "#,
    )
    .bind(batch_id)
    .bind(account_id)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to mark batch as completed: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let _ = state.event_sender.send(BrokerEvent {
        id: Some(chrono::Utc::now().timestamp_millis()),
        pk: Some(batch_id),
        account_id: Some(account_id),
        batch_id: Some(batch_id),
        bet_id: None,
        event: "batch_deleted".to_string(),
    });

    Ok(())
}

