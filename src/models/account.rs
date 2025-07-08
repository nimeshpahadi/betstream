use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use chrono::{DateTime, Utc};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerEvent {
    pub id: Option<i64>,
    pub pk: Option<i64>,
    pub account_id: Option<i64>,
    pub batch_id: Option<i64>,
    pub bet_id: Option<i64>,
    pub event: String,
}

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

#[derive(Deserialize)]
pub struct BetUpdateRequest {
    pub pid: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBetStatusRequest {
    pub status: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBatchRequest {
    pub meta: JsonValue,
    pub bets: Vec<CreateBetRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBetRequest {
    pub id: i64,
    pub selection: String,
    pub stake: f64,
    pub cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Bet {
    pub pid: i64,
    pub id: i64,
    pub selection: String,
    pub stake: f64,
    pub cost: f64,
    pub status: String,
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
    pub id: i64,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
    pub meta: JsonValue,
    pub account_id: i64,
    pub bets: Vec<Bet>,
}
