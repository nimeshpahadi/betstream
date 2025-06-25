use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
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
    pub account_name: Option<String>,
    pub account_hostname: Option<String>,
    pub batch_id: Option<i64>,
    pub bet_id: Option<i64>,
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


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Batch {
    pub id: i64,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub meta: JsonValue,
    pub account_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Bet {
    pub id: i64,
    pub selection: String,
    pub stake: f64,
    pub cost: f64,
    pub batch_id: i64,
}

// Request/Response DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchUpload {
    #[serde(flatten)]
    pub batch: BatchData,
    pub bets: Vec<BetData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchData {
    pub meta: JsonValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BetData {
    pub id: i64,
    pub selection: String,
    pub stake: f64,
    pub cost: f64,
}

#[derive(Debug, Serialize)]
pub struct BatchResponse {
    #[serde(flatten)]
    pub batch: Batch,
    pub bets: Vec<Bet>,
}
