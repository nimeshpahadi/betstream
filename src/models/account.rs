use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

use sqlx::SqlitePool;
use tokio::sync::broadcast;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub hostname: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub name: String,
    pub hostname: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAccountRequest {
    pub name: Option<String>,
    pub hostname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerEvent {
    pub id: Option<i64>,
    pub pk: Option<i64>,
    pub account_id: Option<i64>,
    // pub batch_id: Option<i64>,
    // pub bet_id: Option<i64>,
    pub event: String,
}

// Global event broadcaster
pub type EventSender = broadcast::Sender<BrokerEvent>;
pub type EventReceiver = broadcast::Receiver<BrokerEvent>;

// Application state that includes the event broadcaster
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub event_sender: EventSender,
}
